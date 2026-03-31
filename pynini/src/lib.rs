use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pyo3_async_runtimes::tokio::future_into_py;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

use panini_engine::prompts::{ExtractionRequest, ExtractorPrompts};
use panini_langs::registry;
use rig::client::CompletionClient;
use rig::providers::{anthropic, gemini, openai};

/// Helper to load Prompts from a dictionary or a path.
fn load_prompts(prompts_input: &Bound<'_, PyAny>) -> PyResult<ExtractorPrompts> {
    if let Ok(path) = prompts_input.downcast::<PyString>() {
        let path_str = path.to_str()?;
        ExtractorPrompts::load(path_str).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Failed to load prompts from path: {}", e))
        })
    } else if prompts_input.is_instance_of::<PyDict>() {
        depythonize(prompts_input.as_any()).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Invalid prompts dictionary: {}", e))
        })
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "prompts must be a string path or a dictionary",
        ))
    }
}

/// Helper to create request.
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
#[pyo3(signature = (provider, model, api_key, language, text, targets, prompts, temperature=0.2, max_tokens=4096, ui_language="English".to_string(), components=None))]
#[allow(clippy::too_many_arguments)]
fn extract(
    py: Python<'_>,
    provider: String,
    model: String,
    api_key: String,
    language: String,
    text: String,
    targets: Vec<String>,
    prompts: Bound<'_, PyAny>,
    temperature: f32,
    max_tokens: u32,
    ui_language: String,
    components: Option<Vec<String>>,
) -> PyResult<PyObject> {
    let prompts_obj = load_prompts(&prompts)?;

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
#[pyo3(signature = (provider, model, api_key, language, text, targets, prompts, temperature=0.2, max_tokens=4096, ui_language="English".to_string(), components=None))]
#[allow(clippy::too_many_arguments)]
fn async_extract<'py>(
    py: Python<'py>,
    provider: String,
    model: String,
    api_key: String,
    language: String,
    text: String,
    targets: Vec<String>,
    prompts: Bound<'_, PyAny>,
    temperature: f32,
    max_tokens: u32,
    ui_language: String,
    components: Option<Vec<String>>,
) -> PyResult<Bound<'py, PyAny>> {
    let prompts_obj = load_prompts(&prompts)?;

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

/// The pynini Python module implemented in Rust.
#[pymodule]
fn pynini(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    m.add_function(wrap_pyfunction!(async_extract, m)?)?;
    Ok(())
}
