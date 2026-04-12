use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_async_runtimes::tokio::future_into_py;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

use panini_engine::prompts::{ExtractionRequest, ExtractorPrompts};
use panini_langs::registry;
use rig::client::CompletionClient;
use rig::providers::{anthropic, gemini, openai};

const DEFAULT_PROMPTS_YAML: &str =
    include_str!("../../panini-cli/prompts/default.yml");

fn default_prompts() -> ExtractorPrompts {
    serde_yml::from_str(DEFAULT_PROMPTS_YAML)
        .expect("embedded default prompts must be valid YAML")
}

/// Helper to load Prompts from a dictionary, a path, or fall back to embedded defaults.
fn load_prompts(prompts_input: Option<&Bound<'_, PyAny>>) -> PyResult<ExtractorPrompts> {
    let Some(input) = prompts_input else {
        return Ok(default_prompts());
    };
    if let Ok(path_str) = input.extract::<String>() {
        ExtractorPrompts::load(&path_str).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Failed to load prompts from path: {}", e))
        })
    } else if input.is_instance_of::<PyDict>() {
        depythonize(input.as_any()).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Invalid prompts dictionary: {}", e))
        })
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "prompts must be a string path, a dictionary, or None",
        ))
    }
}

/// Helper to create an extraction request.
fn create_request(text: String, targets: Vec<String>, ui_language: String) -> ExtractionRequest {
    ExtractionRequest {
        content: text,
        targets,
        pedagogical_context: None,
        skill_path: None,
        learner_ui_language: ui_language,
        linguistic_background: vec![],
        user_prompt: None,
    }
}

async fn do_extract(
    provider: String,
    model_name: String,
    api_key: String,
    language: String,
    text: String,
    targets: Vec<String>,
    ui_language: String,
    prompts: ExtractorPrompts,
    temperature: f32,
    max_tokens: u32,
    components: Option<Vec<String>>,
) -> anyhow::Result<Value> {
    let request = create_request(text, targets, ui_language);

    let component_refs: Option<Vec<&str>> =
        components.as_ref().map(|c| c.iter().map(|s| s.as_str()).collect());

    match provider.as_str() {
        "openai" => {
            let client = openai::Client::new(&api_key)
                .map_err(|e| anyhow::anyhow!("OpenAI init error: {e}"))?;
            let model = client.completion_model(&model_name);
            let result = registry::extract_erased_with_components(
                &language,
                &model,
                &request,
                component_refs.as_deref(),
                temperature,
                max_tokens,
                None,
                &prompts,
            )
            .await?;
            Ok(result.into_raw())
        }
        "anthropic" => {
            let client = anthropic::Client::new(&api_key)
                .map_err(|e| anyhow::anyhow!("Anthropic init error: {e}"))?;
            let model = client.completion_model(&model_name);
            let result = registry::extract_erased_with_components(
                &language,
                &model,
                &request,
                component_refs.as_deref(),
                temperature,
                max_tokens,
                None,
                &prompts,
            )
            .await?;
            Ok(result.into_raw())
        }
        "google" => {
            let client = gemini::Client::new(&api_key)
                .map_err(|e| anyhow::anyhow!("Google init error: {e}"))?;
            let model = client.completion_model(&model_name);
            let result = registry::extract_erased_with_components(
                &language,
                &model,
                &request,
                component_refs.as_deref(),
                temperature,
                max_tokens,
                None,
                &prompts,
            )
            .await?;
            Ok(result.into_raw())
        }
        _ => Err(anyhow::anyhow!("Unsupported provider: {provider}")),
    }
}

/// Extracts morphological features for one or more target words synchronously.
#[pyfunction]
#[pyo3(signature = (provider, model, api_key, language, text, targets, prompts=None, temperature=0.2, max_tokens=4096, ui_language="English".to_string(), components=None))]
#[allow(clippy::too_many_arguments)]
pub fn extract(
    py: Python<'_>,
    provider: String,
    model: String,
    api_key: String,
    language: String,
    text: String,
    targets: Vec<String>,
    prompts: Option<Bound<'_, PyAny>>,
    temperature: f32,
    max_tokens: u32,
    ui_language: String,
    components: Option<Vec<String>>,
) -> PyResult<PyObject> {
    let prompts_obj = load_prompts(prompts.as_ref())?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start Tokio runtime: {}", e))
        })?;

    let result = rt
        .block_on(async move {
            do_extract(
                provider,
                model,
                api_key,
                language,
                text,
                targets,
                ui_language,
                prompts_obj,
                temperature,
                max_tokens,
                components,
            )
            .await
        })
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Extraction error: {}", e)))?;

    Ok(pythonize(py, &result)?.into())
}

/// Extracts morphological features for one or more target words asynchronously.
#[pyfunction]
#[pyo3(signature = (provider, model, api_key, language, text, targets, prompts=None, temperature=0.2, max_tokens=4096, ui_language="English".to_string(), components=None))]
#[allow(clippy::too_many_arguments)]
pub fn async_extract<'py>(
    py: Python<'py>,
    provider: String,
    model: String,
    api_key: String,
    language: String,
    text: String,
    targets: Vec<String>,
    prompts: Option<Bound<'_, PyAny>>,
    temperature: f32,
    max_tokens: u32,
    ui_language: String,
    components: Option<Vec<String>>,
) -> PyResult<Bound<'py, PyAny>> {
    let prompts_obj = load_prompts(prompts.as_ref())?;

    future_into_py(py, async move {
        let result = do_extract(
            provider,
            model,
            api_key,
            language,
            text,
            targets,
            ui_language,
            prompts_obj,
            temperature,
            max_tokens,
            components,
        )
        .await
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Extraction error: {}", e)))?;

        Python::with_gil(|py| -> PyResult<PyObject> { Ok(pythonize(py, &result)?.into()) })
    })
}

/// Returns the built-in default extraction prompts as a Python dict.
#[pyfunction]
pub fn get_default_prompts(py: Python<'_>) -> PyResult<PyObject> {
    let prompts = default_prompts();
    Ok(pythonize(py, &prompts)?.into())
}
