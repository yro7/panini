use rig::completion::{CompletionModel, CompletionRequestBuilder};
use rig::message::Message;
use std::{future::Future, pin::Pin};

/// Error returned by a structured LLM transport implementation.
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct StructuredLlmError {
    pub message: String,
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

#[derive(Debug)]
struct AnyhowErrorWrapper(anyhow::Error);

impl std::fmt::Display for AnyhowErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AnyhowErrorWrapper {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl StructuredLlmError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    #[must_use]
    pub fn with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(source),
        }
    }
}

impl From<anyhow::Error> for StructuredLlmError {
    fn from(error: anyhow::Error) -> Self {
        let msg = error.to_string();
        Self {
            message: msg,
            source: Some(Box::new(AnyhowErrorWrapper(error))),
        }
    }
}

impl From<rig::completion::request::CompletionError> for StructuredLlmError {
    fn from(error: rig::completion::request::CompletionError) -> Self {
        let msg = error.to_string();
        Self {
            message: msg,
            source: Some(Box::new(error)),
        }
    }
}

/// Prior failed LLM output and validation feedback for self-correction.
#[derive(Debug, Clone, Copy)]
pub struct StructuredLlmRetryContext<'a> {
    pub raw_response: &'a str,
    pub error: &'a str,
}

/// Structured output request passed to an LLM transport.
pub struct StructuredLlmRequest<'a> {
    pub system_prompt: &'a str,
    pub user_content: &'a str,
    pub schema: &'a schemars::Schema,
    pub temperature: f32,
    pub max_tokens: u32,
    pub user_id: &'a str,
    pub timeout: std::time::Duration,
    pub retry_context: Option<StructuredLlmRetryContext<'a>>,
}

/// Structured output response returned by an LLM transport.
#[derive(Debug, Clone)]
pub struct StructuredLlmResponse {
    pub text: String,
    pub tokens_in: u32,
    pub tokens_out: u32,
}

pub type StructuredLlmFuture<'a> =
    Pin<Box<dyn Future<Output = Result<StructuredLlmResponse, StructuredLlmError>> + Send + 'a>>;

/// Transport boundary for structured LLM calls used by Panini extraction.
pub trait StructuredLlmExecutor: Send + Sync {
    fn execute_structured<'a>(
        &'a self,
        request: StructuredLlmRequest<'a>,
    ) -> StructuredLlmFuture<'a>;
}

pub(crate) struct RigStructuredLlmExecutor<'a, M> {
    model: &'a M,
}

impl<'a, M> RigStructuredLlmExecutor<'a, M> {
    pub(crate) fn new(model: &'a M) -> Self {
        Self { model }
    }
}

impl<M> StructuredLlmExecutor for RigStructuredLlmExecutor<'_, M>
where
    M: CompletionModel + Sync,
{
    fn execute_structured<'a>(
        &'a self,
        request: StructuredLlmRequest<'a>,
    ) -> StructuredLlmFuture<'a> {
        Box::pin(async move {
            let mut builder: CompletionRequestBuilder<M> = self
                .model
                .completion_request(request.user_content)
                .preamble(request.system_prompt.to_string())
                .temperature(f64::from(request.temperature))
                .max_tokens(u64::from(request.max_tokens))
                .output_schema(request.schema.clone())
                .additional_params(serde_json::json!({
                    "user": request.user_id,
                    "panglotive_call_type": "extraction_morphology"
                }));

            if let Some(retry) = request.retry_context {
                builder = builder
                    .message(Message::assistant(retry.raw_response))
                    .message(Message::user(format!(
                        "Your output is not conform to what I'm expecting. \
                         Please look at the error and correct yourself: {}",
                        retry.error
                    )));
            }

            let completion_response = builder.send().await.map_err(StructuredLlmError::from)?;

            let text = completion_response
                .choice
                .into_iter()
                .find_map(|c| {
                    if let rig::completion::message::AssistantContent::Text(t) = c {
                        Some(t.text)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| StructuredLlmError::new("LLM returned no text content"))?;

            Ok(StructuredLlmResponse {
                text,
                tokens_in: completion_response.usage.input_tokens as u32,
                tokens_out: completion_response.usage.output_tokens as u32,
            })
        })
    }
}
