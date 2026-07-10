use panini_core::component::{AnalysisComponent, ExtractionResult};
use panini_core::traits::LinguisticDefinition;
use rig::completion::CompletionModel;
use std::time::Duration;

use crate::composer::{compose_prompt, compose_schema};
use crate::llm_utils::clean_llm_json;
use crate::prompts::{ExtractionRequest, ExtractorPrompts};
use crate::structured_llm::{
    RigStructuredLlmExecutor, StructuredLlmError, StructuredLlmExecutor, StructuredLlmRequest,
    StructuredLlmRetryContext,
};

// ─── Error types ──────────────────────────────────────────────────────────────

/// Detailed reason for an extraction failure.
#[derive(Debug, thiserror::Error)]
pub enum ExtractionFailureReason {
    /// LLM output could not be parsed as JSON.
    #[error("Invalid JSON syntax: {0}")]
    JsonSyntax(String),

    /// JSON output did not match the required schema.
    #[error("Schema validation failed: {0}")]
    Schema(String),

    /// A specific component failed its internal validation.
    #[error("Validation failed for component '{key}': {message}")]
    ComponentValidation { key: &'static str, message: String },

    /// A specific component failed its internal post-processing.
    #[error("Post-processing failed for component '{key}': {message}")]
    ComponentPostProcess { key: &'static str, message: String },
}

/// Error returned when feature extraction parsing fails, carrying the raw LLM output and structured reason.
#[derive(Debug, thiserror::Error)]
#[error("{reason}")]
pub struct ExtractionParseError {
    pub raw_response: String,
    pub reason: ExtractionFailureReason,
}

/// Typed error enum for the extraction pipeline.
#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    /// LLM provider errors (rig-core completion failures, network, auth, etc.)
    #[error("LLM completion failed: {0}")]
    Llm(#[from] rig::completion::request::CompletionError),

    /// LLM transport errors from an injected structured executor.
    #[error("LLM executor failed: {0}")]
    StructuredLlm(#[from] StructuredLlmError),

    /// JSON serialization/deserialization errors (schema conversion, response parsing)
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Prompt composition errors (missing placeholders, I/O, etc.)
    #[error("prompt composition failed: {0}")]
    PromptComposition(#[from] crate::prompts::PromptBuilderError),

    /// LLM returned no text content in its response
    #[error("LLM returned no text content")]
    EmptyResponse,

    /// Schema validation or component validation/parse failure — carries the raw
    /// LLM output so callers can retry with `PreviousAttempt`
    #[error("{0}")]
    Parse(#[from] ExtractionParseError),

    /// Failed to map raw `ExtractionResult` into a typed consumer struct
    /// (used by `#[derive(PaniniResult)]` generated code)
    #[error("failed to map extracted components to result struct")]
    ResultMapping(#[from] panini_core::component::ExtractionResultError),
}

// ─── Extraction options ───────────────────────────────────────────────────────

/// Previous failed attempt context for LLM self-correction retry.
struct PreviousAttempt {
    pub raw_response: String,
    pub error: String,
}

/// Configuration for the retry mechanism
#[derive(Clone, Debug)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub initial_backoff_secs: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 2,
            initial_backoff_secs: 1,
        }
    }
}

/// Bundles extraction parameters
#[derive(Clone)]
pub struct ExtractionOptions<'a> {
    pub temperature: f32,
    pub max_tokens: u32,
    pub extractor_prompts: &'a ExtractorPrompts,
    pub retry: RetryConfig,
    pub timeout: Duration,
    pub user_id: &'a str,
}

impl<'a> ExtractionOptions<'a> {
    #[must_use]
    pub fn new(extractor_prompts: &'a ExtractorPrompts, user_id: &'a str) -> Self {
        Self {
            temperature: 0.2,
            max_tokens: 4096,
            extractor_prompts,
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            user_id,
        }
    }
}

// ─── Composable entry point ───────────────────────────────────────────────────

/// Extracts features using composable `AnalysisComponent`s.
///
/// This is the entry-point that supports selecting which analyses to include.
/// It includes an automatic self-correction loop (Retry) in case of validation errors.
///
/// # Errors
/// Returns an extraction error if the LLM completion fails, or JSON parsing
/// / validation fails after all retry attempts are exhausted.
pub async fn extract_with_components<L, M>(
    language: &L,
    model: &M,
    request: &ExtractionRequest,
    components: &[&dyn AnalysisComponent<L>],
    options: ExtractionOptions<'_>,
) -> Result<ExtractionResult, ExtractionError>
where
    L: LinguisticDefinition + Send + Sync,
    M: CompletionModel + Sync,
{
    let executor = RigStructuredLlmExecutor::new(model);
    extract_with_components_executor(language, &executor, request, components, options).await
}

/// Extracts features using an injected structured LLM executor.
///
/// This entry-point lets applications enforce their own LLM policy boundary
/// while Panini keeps the same schema, retry, validation, and component logic.
pub async fn extract_with_components_executor<L, E>(
    language: &L,
    executor: &E,
    request: &ExtractionRequest,
    components: &[&dyn AnalysisComponent<L>],
    options: ExtractionOptions<'_>,
) -> Result<ExtractionResult, ExtractionError>
where
    L: LinguisticDefinition + Send + Sync,
    E: StructuredLlmExecutor,
{
    // --- 1. Filter to compatible components once ---
    let compatible: Vec<&dyn AnalysisComponent<L>> = components
        .iter()
        .filter(|c| c.is_compatible(language))
        .copied()
        .collect();

    let requested_keys: Vec<&'static str> = compatible.iter().map(|c| c.schema_key()).collect();

    // --- 2. Compose schema once ---
    let schema_value = compose_schema(language, &compatible);
    let schema: schemars::Schema = serde_json::from_value(schema_value.clone())?;

    // --- 3. Compose prompt once ---
    let system_prompt = compose_prompt(language, request, options.extractor_prompts, &compatible)?;

    let user_message = format!(
        "Extract features from this card:\n{}\n\nTARGET WORDS: {:?}",
        request.content, request.targets
    );

    let mut prev_attempt: Option<PreviousAttempt> = None;
    let mut backoff = backoff::ExponentialBackoffBuilder::new()
        .with_initial_interval(Duration::from_secs(options.retry.initial_backoff_secs))
        .with_multiplier(2.0)
        .with_max_elapsed_time(Some(options.timeout))
        .build();

    let start_time = std::time::Instant::now();

    loop {
        let elapsed = start_time.elapsed();
        if elapsed >= options.timeout {
            return Err(ExtractionError::StructuredLlm(StructuredLlmError::new(
                "total timeout exceeded",
            )));
        }
        let remaining = options.timeout - elapsed;

        let result = perform_single_shot_extraction(
            language,
            executor,
            &schema,
            &schema_value,
            &system_prompt,
            &user_message,
            &compatible,
            &requested_keys,
            &options,
            remaining,
            prev_attempt.as_ref(),
        )
        .await;

        match result {
            Ok(res) => return Ok(res),
            Err(e) => {
                // Only retry on parsing/validation errors
                if let ExtractionError::Parse(pe) = &e
                    && let Some(wait) = backoff::backoff::Backoff::next_backoff(&mut backoff)
                {
                    let err_msg = pe.reason.to_string();
                    tracing::warn!(
                        ?wait,
                        error = %err_msg,
                        "Extraction validation failed, retrying with self-correction..."
                    );
                    prev_attempt = Some(PreviousAttempt {
                        raw_response: pe.raw_response.clone(),
                        error: err_msg,
                    });
                    tokio::time::sleep(wait).await;
                    continue;
                }
                return Err(e);
            }
        }
    }
}

/// Internal function to perform a single extraction attempt.
#[allow(clippy::too_many_arguments)]
async fn perform_single_shot_extraction<L, E>(
    language: &L,
    executor: &E,
    schema: &schemars::Schema,
    schema_value: &serde_json::Value,
    system_prompt: &str,
    user_message: &str,
    compatible: &[&dyn AnalysisComponent<L>],
    requested_keys: &[&'static str],
    options: &ExtractionOptions<'_>,
    remaining_total_timeout: Duration,
    previous_attempt: Option<&PreviousAttempt>,
) -> Result<ExtractionResult, ExtractionError>
where
    L: LinguisticDefinition + Send + Sync,
    E: StructuredLlmExecutor,
{
    let attempt_timeout = std::cmp::min(options.timeout, remaining_total_timeout);

    // 4. Run LLM request through the injected transport with timeout wrapper.
    let retry_context = previous_attempt.map(|prev| StructuredLlmRetryContext {
        raw_response: &prev.raw_response,
        error: &prev.error,
    });

    let raw_text = tokio::time::timeout(
        attempt_timeout,
        executor.execute_structured(StructuredLlmRequest {
            system_prompt,
            user_content: user_message,
            schema,
            temperature: options.temperature,
            max_tokens: options.max_tokens,
            user_id: options.user_id,
            timeout: attempt_timeout,
            retry_context,
        }),
    )
    .await
    .map_err(|_| StructuredLlmError::new("LLM request timed out"))??
    .text;

    // 5. Chain pre_process from each component
    let cleaned = clean_llm_json(&raw_text);
    let mut processed = cleaned.to_string();
    for comp in compatible {
        processed = comp.pre_process(&processed);
    }

    // 6. Parse JSON
    let mut json_value: serde_json::Value = match serde_json::from_str(&processed) {
        Ok(v) => v,
        Err(e) => {
            let err_msg = format!("{e}");
            tracing::warn!(error = %err_msg, "Failed to parse JSON syntax");
            return Err(ExtractionParseError {
                raw_response: processed,
                reason: ExtractionFailureReason::JsonSyntax(err_msg),
            }
            .into());
        }
    };

    // 7. Validate composed schema
    if let Ok(validator) = jsonschema::validator_for(schema_value) {
        let schema_errors: Vec<_> = validator.iter_errors(&json_value).collect();
        if !schema_errors.is_empty() {
            let mut err_msgs = Vec::new();
            for err in schema_errors {
                err_msgs.push(format!("- Path: {}: {}", err.instance_path(), err));
            }
            let err_msg = err_msgs.join("\n");
            tracing::warn!(error = %err_msg, "Schema validation failed — retrying");
            return Err(ExtractionParseError {
                raw_response: processed,
                reason: ExtractionFailureReason::Schema(err_msg),
            }
            .into());
        }
    }

    // 8. Per-component validate + post_process
    for comp in compatible {
        let key = comp.schema_key();
        if let Some(section) = json_value.get(key) {
            comp.validate(language, section)
                .map_err(|e| ExtractionParseError {
                    raw_response: processed.clone(),
                    reason: ExtractionFailureReason::ComponentValidation { key, message: e },
                })?;
        }
    }

    for comp in compatible {
        let key = comp.schema_key();
        if let Some(section) = json_value.get_mut(key) {
            comp.post_process(language, section)
                .map_err(|e| ExtractionParseError {
                    raw_response: processed.clone(),
                    reason: ExtractionFailureReason::ComponentPostProcess { key, message: e },
                })?;
        }
    }

    // 9. Return ExtractionResult
    Ok(ExtractionResult::new(json_value, requested_keys.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use panini_core::aggregable::{Aggregable, FieldDescriptor};
    use panini_core::component::ComponentContext;
    use panini_core::traits::{IsoLang, MorphologyInfo, Script};
    use serde::{Deserialize, Serialize};
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use crate::structured_llm::{StructuredLlmFuture, StructuredLlmResponse};

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
    #[serde(tag = "pos", rename_all = "lowercase")]
    enum TestMorphology {
        Word { lemma: String },
    }

    impl Aggregable for TestMorphology {
        fn group_key(&self) -> String {
            self.pos_label().to_string()
        }

        fn instance_descriptors(&self) -> Vec<FieldDescriptor> {
            vec![]
        }

        fn observations(&self) -> Vec<Vec<(String, String)>> {
            vec![vec![]]
        }
    }

    impl MorphologyInfo for TestMorphology {
        type PosTag = TestPosTag;

        fn lemma(&self) -> &str {
            match self {
                Self::Word { lemma } => lemma,
            }
        }

        fn pos_tag(&self) -> Self::PosTag {
            TestPosTag::Word
        }

        fn pos(&self) -> panini_core::traits::Upos {
            panini_core::traits::Upos::Other
        }

        // `Word` is a synthetic test PoS with no UD tag; keep the string label.
        fn pos_label(&self) -> &'static str {
            "Word"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestPosTag {
        Word,
    }

    struct TestLang;

    impl LinguisticDefinition for TestLang {
        type Morphology = TestMorphology;
        type GrammaticalFunction = ();

        const ISO_LANG: IsoLang = IsoLang::Eng;

        fn supported_scripts(&self) -> &[Script] {
            &[Script::LATN]
        }

        fn default_script(&self) -> Script {
            Script::LATN
        }

        fn extraction_directives(&self) -> &str {
            "Extract test values."
        }
    }

    #[derive(Debug)]
    struct AlphaComponent;

    impl AnalysisComponent<TestLang> for AlphaComponent {
        fn name(&self) -> &'static str {
            "Alpha"
        }

        fn schema_key(&self) -> &'static str {
            "alpha"
        }

        fn schema_fragment(&self, _lang: &TestLang) -> serde_json::Value {
            serde_json::json!({ "type": "string" })
        }

        fn prompt_fragment(&self, _lang: &TestLang, _ctx: &ComponentContext) -> String {
            "Extract alpha.".to_string()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ExecutorCall {
        user_id: String,
        had_retry_context: bool,
    }

    struct FakeExecutor {
        responses: Mutex<VecDeque<String>>,
        calls: Mutex<Vec<ExecutorCall>>,
    }

    impl FakeExecutor {
        fn new(responses: Vec<&str>) -> Self {
            Self {
                responses: Mutex::new(
                    responses
                        .into_iter()
                        .map(std::string::ToString::to_string)
                        .collect(),
                ),
                calls: Mutex::new(Vec::new()),
            }
        }
    }

    impl StructuredLlmExecutor for FakeExecutor {
        fn execute_structured<'a>(
            &'a self,
            request: StructuredLlmRequest<'a>,
        ) -> StructuredLlmFuture<'a> {
            Box::pin(async move {
                self.calls.lock().unwrap().push(ExecutorCall {
                    user_id: request.user_id.to_string(),
                    had_retry_context: request.retry_context.is_some(),
                });
                let text =
                    self.responses.lock().unwrap().pop_front().ok_or_else(|| {
                        StructuredLlmError::new("fake executor response exhausted")
                    })?;
                Ok(StructuredLlmResponse {
                    text,
                    tokens_in: 1,
                    tokens_out: 1,
                })
            })
        }
    }

    fn test_prompts() -> ExtractorPrompts {
        ExtractorPrompts {
            system_role: "system".to_string(),
            target_language: "{language}".to_string(),
            extraction_directives: "{directives}".to_string(),
            learner_profile: crate::prompts::LearnerProfile {
                ui_language: "{name}".to_string(),
                linguistic_background_intro: "Known languages:".to_string(),
                linguistic_background_entry: "{iso}:{level}".to_string(),
            },
            skill_context: crate::prompts::SkillContextPrompts {
                skill_tree_path: "{path}".to_string(),
                pedagogical_focus: "{instructions}".to_string(),
            },
            user_context: "{context_description}".to_string(),
            output_instruction: "Return valid JSON.".to_string(),
        }
    }

    #[tokio::test]
    async fn executor_entrypoint_uses_injected_transport_for_retry() {
        let executor =
            FakeExecutor::new(vec![r#"{"alpha": 1}"#, r#"{"alpha": "valid after retry"}"#]);
        let request = ExtractionRequest {
            content: "content".to_string(),
            targets: vec!["content".to_string()],
            pedagogical_context: None,
            skill_path: None,
            learner_ui_language: "English".to_string(),
            linguistic_background: vec![],
            user_prompt: None,
        };
        let options = ExtractionOptions {
            temperature: 0.2,
            max_tokens: 256,
            extractor_prompts: &test_prompts(),
            retry: RetryConfig {
                max_retries: 2,
                initial_backoff_secs: 0,
            },
            timeout: Duration::from_secs(5),
            user_id: "test-user",
        };

        let result = extract_with_components_executor(
            &TestLang,
            &executor,
            &request,
            &[&AlphaComponent as &dyn AnalysisComponent<TestLang>],
            options,
        )
        .await
        .expect("executor-backed extraction should retry and succeed");

        let alpha: String = result.get("alpha").expect("alpha should deserialize");
        assert_eq!(alpha, "valid after retry");
        assert_eq!(
            *executor.calls.lock().unwrap(),
            vec![
                ExecutorCall {
                    user_id: "test-user".to_string(),
                    had_retry_context: false,
                },
                ExecutorCall {
                    user_id: "test-user".to_string(),
                    had_retry_context: true,
                },
            ]
        );
    }
}
