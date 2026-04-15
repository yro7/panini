#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
use pythonize::pythonize;

use panini_langs::registry;

#[pyclass]
pub struct LanguageInfo {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub scripts: Vec<String>,
    #[pyo3(get)]
    pub typological_features: Vec<String>,
    #[pyo3(get)]
    pub directives: String,
}

#[pyfunction]
pub fn get_language_info(lang_code: &str) -> PyResult<LanguageInfo> {
    macro_rules! dispatch_info {
        ($($lang:ident),*) => {
            match lang_code {
                $(
                    <panini_langs::$lang as panini_core::traits::LinguisticDefinition>::ISO_CODE => {
                        use panini_core::traits::LinguisticDefinition;
                        let lang = panini_langs::$lang;
                        Ok(LanguageInfo {
                            name: lang.name().to_string(),
                            scripts: lang.supported_scripts().iter().map(|s| s.code().to_string()).collect(),
                            typological_features: lang.typological_features().iter().map(|f| format!("{:?}", f)).collect(),
                            directives: lang.extraction_directives().to_string(),
                        })
                    }
                )*
                _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unsupported language: {}", lang_code))),
            }
        }
    }

    dispatch_info!(Polish, Turkish, Arabic, French, Italian)
}

#[pyfunction]
pub fn get_morphology_schema(lang_code: &str) -> PyResult<PyObject> {
    macro_rules! dispatch_schema {
        ($($lang:ident),*) => {
            match lang_code {
                $(
                    <panini_langs::$lang as panini_core::traits::LinguisticDefinition>::ISO_CODE => {
                        use panini_core::component::AnalysisComponent;
                        use panini_core::components::MorphologyAnalysis;
                        let lang = panini_langs::$lang;
                        let schema = MorphologyAnalysis.schema_fragment(&lang);
                        Python::with_gil(|py| -> PyResult<PyObject> {
                            Ok(pythonize(py, &schema)?.into())
                        })
                    }
                )*
                _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unsupported language: {}", lang_code))),
            }
        }
    }

    dispatch_schema!(Polish, Turkish, Arabic, French, Italian)
}

/// Returns the list of supported ISO 639-3 language codes.
#[pyfunction]
pub fn supported_languages() -> Vec<&'static str> {
    registry::supported_languages().to_vec()
}

/// Returns the current package version.
#[pyfunction]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
