//! Set GOOGLE_API_KEY, or LFG_ENV_FILE pointing to a dotenv file containing it.
//! Optional: LFG_MODEL, LFG_OUTPUT.
//! With no arguments, analyze seven sentences; otherwise pass ISO-639-3 and text.
use anyhow::{Context, Result, ensure};
use panini_core::traits::IsoLang;
use panini_core::{LfgAnalysis, LfgValue};
use panini_engine::prompts::ExtractorPrompts;
use panini_engine::structured_llm::{
    StructuredLlmError, StructuredLlmExecutor, StructuredLlmFuture, StructuredLlmRequest,
    StructuredLlmResponse,
};
use panini_engine::{ExtractionOptions, ExtractionRequest};
use panini_langs::registry;
use rig::{
    client::CompletionClient, completion::CompletionModel, message::Message, providers::gemini,
};
use serde_json::json;
use std::{
    env,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = api_key()?;
    let model_name =
        env::var("LFG_MODEL").unwrap_or_else(|_| "gemini-3.1-flash-lite-preview".into());
    let client = gemini::Client::new(&api_key)?;
    let model = client.completion_model(&model_name);
    let executor = GeminiExecutor(model);
    let prompts = ExtractorPrompts::load(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../prompts/default.yml"),
    )?;
    let args: Vec<String> = env::args().skip(1).collect();
    ensure!(
        args.is_empty() || args.len() == 2,
        "Usage: lfg_f_structure [ISO-639-3 'sentence']"
    );
    let cases: Vec<(&str, &str, Option<&str>)> = if args.is_empty() {
        vec![
            ("eng", "The driver starts the tractor.", None),
            ("eng", "The tractor was repaired by the farmer.", None),
            ("eng", "The farmer tries to leave.", Some("SUBJ")),
            (
                "eng",
                "The farmer persuades his neighbor to leave.",
                Some("OBJ"),
            ),
            ("eng", "The farmer who bought the tractor left.", None),
            ("fra", "Le fermier essaie de partir.", Some("SUBJ")),
            ("gle", "Tosaíonn an tiománaí an tarracóir.", None),
        ]
    } else {
        vec![(&args[0], &args[1], None)]
    };
    let mut records = Vec::new();
    let mut failures = 0;
    for (iso, text, expected_controller) in cases {
        eprintln!("Analyzing {iso}: {text}");
        let language = IsoLang::from_639_3(iso).context("Unknown ISO-639-3 code")?;
        let request = ExtractionRequest {
            content: text.to_string(),
            targets: vec![],
            pedagogical_context: None,
            skill_path: None,
            learner_ui_language: IsoLang::Eng,
            translation_alignment_directives: None,
            linguistic_background: vec![],
            user_prompt: None,
        };
        let mut options = ExtractionOptions::new(&prompts, "panini-lfg-live-example");
        options.temperature = 0.0;
        options.max_tokens = 8192;
        options.attempt_timeout = Duration::from_secs(60);
        options.total_timeout = Duration::from_secs(180);
        let started = Instant::now();
        let outcome = registry::extract_erased_with_components_executor(
            language,
            &executor,
            &request,
            Some(&["lfg_f_structure"]),
            options,
        )
        .await;
        match outcome {
            Ok(result) => {
                let analysis: LfgAnalysis = result.get("lfg_f_structure")?;
                analysis.validate().map_err(anyhow::Error::msg)?;
                let root = analysis
                    .structures
                    .iter()
                    .find(|s| s.id == analysis.root)
                    .unwrap();
                let controller = root.attributes.get("XCOMP-CONTROLLER").and_then(|value| {
                    if let LfgValue::Atom(function) = value {
                        Some(function.as_str())
                    } else {
                        None
                    }
                });
                let control_shared = root
                    .attributes
                    .get("XCOMP")
                    .and_then(|v| match v {
                        LfgValue::Reference(r) => {
                            analysis.structures.iter().find(|s| s.id == r.target)
                        }
                        _ => None,
                    })
                    .is_some_and(|xcomp| {
                        controller.is_some_and(|function| {
                            matches!(root.attributes.get(function), Some(LfgValue::Reference(_)))
                                && root.attributes.get(function) == xcomp.attributes.get("SUBJ")
                        })
                    });
                if expected_controller.is_some()
                    && (controller != expected_controller || !control_shared)
                {
                    failures += 1;
                }
                eprintln!(
                    "  Valid graph: {} structures; controller: {controller:?}; shared control: {control_shared}",
                    analysis.structures.len()
                );
                records.push(json!({
                    "language": iso, "text": text, "elapsed_seconds": started.elapsed().as_secs_f64(),
                    "graph_valid": true, "expected_controller": expected_controller, "controller":controller,
                    "control_shared": control_shared, "result": result.into_raw(),
                }));
            }
            Err(error) => {
                failures += 1;
                let message = error.to_string().replace(&api_key, "<redacted>");
                let raw_response = error
                    .downcast_ref::<panini_engine::ExtractionError>()
                    .and_then(|e| match e {
                        panini_engine::ExtractionError::Parse(parse) => {
                            Some(parse.raw_response.replace(&api_key, "<redacted>"))
                        }
                        _ => None,
                    });
                eprintln!("  Failed: {message}");
                records
                    .push(json!({"language":iso,"text":text,"graph_valid":false,"error":message,"raw_response":raw_response}));
            }
        }
    }
    let report = json!({"model":model_name,"failures":failures,"cases":records});
    let output = serde_json::to_string_pretty(&report)?;
    if let Ok(path) = env::var("LFG_OUTPUT") {
        std::fs::write(&path, &output)?;
        eprintln!("Saved live report to {path}");
    }
    println!("{output}");
    ensure!(
        failures == 0,
        "{failures} live checks failed; inspect the report"
    );
    Ok(())
}

/// The default Rig adapter adds Panglotive gateway metadata. Direct Gemini
/// rejects those fields, so this example supplies its transport through the
/// public executor seam; composition, validation and retry stay in Panini.
struct GeminiExecutor<M>(M);

impl<M: CompletionModel + Sync> StructuredLlmExecutor for GeminiExecutor<M> {
    fn execute_structured<'a>(
        &'a self,
        request: StructuredLlmRequest<'a>,
    ) -> StructuredLlmFuture<'a> {
        Box::pin(async move {
            let mut builder = self
                .0
                .completion_request(request.user_content)
                .preamble(request.system_prompt.to_string())
                .temperature(f64::from(request.temperature))
                .max_tokens(u64::from(request.max_tokens))
                .output_schema(request.schema.clone());
            if let Some(retry) = request.retry_context {
                builder = builder
                    .message(Message::assistant(retry.raw_response))
                    .message(Message::user(format!(
                        "Correct your previous analysis: {}",
                        retry.error
                    )));
            }
            let response = builder.send().await.map_err(StructuredLlmError::from)?;
            let text = response
                .choice
                .into_iter()
                .find_map(|content| {
                    if let rig::completion::message::AssistantContent::Text(text) = content {
                        Some(text.text)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| StructuredLlmError::new("Gemini returned no text"))?;
            Ok(StructuredLlmResponse {
                text,
                tokens_in: response.usage.input_tokens as u32,
                tokens_out: response.usage.output_tokens as u32,
            })
        })
    }
}

fn api_key() -> Result<String> {
    if let Ok(key) = env::var("GOOGLE_API_KEY") {
        return Ok(key);
    }
    let path = env::var("LFG_ENV_FILE").context("Set GOOGLE_API_KEY or LFG_ENV_FILE")?;
    // Read just the API key; avoid mutating the process environment in Tokio.
    #[allow(deprecated)]
    let entries = dotenv::from_path_iter(path)
        .map_err(|_| anyhow::anyhow!("Unable to read credentials file"))?;
    for entry in entries {
        let (name, value) =
            entry.map_err(|_| anyhow::anyhow!("Unable to parse credentials file"))?;
        if name == "GOOGLE_API_KEY" {
            return Ok(value);
        }
    }
    anyhow::bail!("GOOGLE_API_KEY is absent from credentials file")
}
