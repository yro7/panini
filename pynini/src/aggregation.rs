#![allow(clippy::useless_conversion, clippy::needless_pass_by_value)]
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pythonize::{depythonize, pythonize};
use std::collections::HashMap;

use panini_core::aggregable::digest::{
    AggregationResult, AggregationSink, Aggregator, BasicAggregator, Dimension, Distribution,
    GroupResult, Inventory,
};
use panini_core::aggregable::Aggregable;
use panini_core::component::{AggregationError, AnalysisComponent, ExtractionResult};
use panini_core::components::{MorphemeSegmentation, MorphologyAnalysis};
use panini_core::traits::LinguisticDefinition;

#[pyclass(name = "Distribution")]
#[derive(Clone)]
pub struct PyDistribution {
    pub(crate) inner: Distribution,
}

#[pymethods]
impl PyDistribution {
    #[getter]
    fn possible_values(&self) -> Vec<String> {
        self.inner.possible.clone()
    }

    #[getter]
    fn counts(&self) -> HashMap<String, usize> {
        self.inner.counts.clone()
    }

    #[getter]
    fn coverage_percent(&self) -> f64 {
        self.inner.coverage_percent()
    }

    fn seen_count(&self) -> usize {
        self.inner.seen_count()
    }

    const fn total_count(&self) -> usize {
        self.inner.total_count()
    }
}

#[pyclass(name = "Inventory")]
#[derive(Clone)]
pub struct PyInventory {
    pub(crate) inner: Inventory,
}

#[pymethods]
impl PyInventory {
    #[getter]
    fn counts(&self) -> HashMap<String, usize> {
        self.inner.counts.clone()
    }
}

#[pyclass(name = "GroupResult")]
pub struct PyGroupResult {
    pub(crate) inner: GroupResult,
}

#[pymethods]
impl PyGroupResult {
    #[getter]
    const fn total(&self) -> usize {
        self.inner.total
    }

    #[getter]
    fn dimensions(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        for (name, dim) in &self.inner.dimensions {
            match dim {
                Dimension::Dist(d) => {
                    dict.set_item(name, PyDistribution { inner: d.clone() }.into_py(py))?;
                }
                Dimension::Inv(i) => {
                    dict.set_item(name, PyInventory { inner: i.clone() }.into_py(py))?;
                }
            }
        }
        Ok(dict.into())
    }
}

#[pyclass(name = "AggregationResult")]
pub struct PyAggregationResult {
    pub(crate) inner: AggregationResult,
}

#[pymethods]
impl PyAggregationResult {
    #[getter]
    fn by_group(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        for (group, res) in &self.inner.by_group {
            dict.set_item(group, PyGroupResult { inner: res.clone() }.into_py(py))?;
        }
        Ok(dict.into())
    }

    fn total_count(&self) -> usize {
        self.inner.total_count()
    }

    fn group_count(&self) -> usize {
        self.inner.group_count()
    }

    fn print(&self) {
        self.inner.print();
    }
}

#[pyclass(name = "BasicAggregator")]
#[derive(Default)]
pub struct PyBasicAggregator {
    inner: Option<BasicAggregator>,
}

#[pymethods]
impl PyBasicAggregator {
    #[new]
    fn new() -> Self {
        Self {
            inner: Some(BasicAggregator::new()),
        }
    }

    /// Records an extraction result into the aggregator.
    /// `data` should be the dictionary returned from `extract()`.
    fn record(&mut self, lang_code: &str, data: Bound<'_, PyAny>) -> PyResult<()> {
        let value: serde_json::Value = depythonize(&data)?;
        self.do_record(lang_code, value, None)
    }

    /// Records an extraction result with a custom pivot callback.
    /// `pivot_callback` receives the analysis dictionary and returns a string key.
    fn record_pivoted(
        &mut self,
        lang_code: &str,
        data: Bound<'_, PyAny>,
        pivot_callback: PyObject,
    ) -> PyResult<()> {
        let value: serde_json::Value = depythonize(&data)?;
        self.do_record(lang_code, value, Some(pivot_callback))
    }

    fn finish(&mut self) -> PyResult<PyAggregationResult> {
        let agg = self.inner.take().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Aggregator has already been finished")
        })?;
        Ok(PyAggregationResult {
            inner: agg.finish(),
        })
    }
}

impl PyBasicAggregator {
    fn inner_mut(&mut self) -> PyResult<&mut BasicAggregator> {
        self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Aggregator has already been finished")
        })
    }

    fn do_record(
        &mut self,
        lang_code: &str,
        value: serde_json::Value,
        pivot_callback: Option<PyObject>,
    ) -> PyResult<()> {
        let agg = self.inner_mut()?;

        macro_rules! dispatch_record {
            ($($lang:ident),*) => {
                match lang_code {
                    $(
                        s if s == <panini_langs::$lang as LinguisticDefinition>::ISO_LANG.to_639_3() => {
                            use panini_langs::$lang;
                            use panini_core::domain::ExtractedFeature;

                            if let Some(cb) = &pivot_callback {
                                // Pivoted path: typed iteration so Python callback receives
                                // a structured morphology object (field-accessible dict).
                                if let Some(morph) = value.get("morphology") {
                                    for field in ["target_features", "context_features"] {
                                        if let Some(val) = morph.get(field) {
                                            let features: Vec<ExtractedFeature<<$lang as LinguisticDefinition>::Morphology>> =
                                                serde_json::from_value(val.clone())
                                                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(
                                                        format!("Failed to parse {}: {}", field, e)
                                                    ))?;
                                            for feat in features {
                                                let key = Python::with_gil(|py| -> PyResult<String> {
                                                    let dict = pythonize(py, &feat.morphology)?;
                                                    let res = cb.call1(py, (dict,))?;
                                                    res.extract::<String>(py)
                                                })?;
                                                agg.record(&feat.morphology.pivoted(|_| key.clone()));
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Non-pivoted path: component-driven dispatch.
                                let lang = $lang;
                                let morph = MorphologyAnalysis;
                                let seg = MorphemeSegmentation;
                                let components: &[&dyn AnalysisComponent<$lang>] = &[&morph, &seg];
                                let extraction = ExtractionResult::new(
                                    value.clone(),
                                    components.iter().map(|c| c.schema_key()).collect(),
                                );
                                for comp in components.iter().filter(|c| c.is_compatible(&lang)) {
                                    if let Some(a) = comp.as_aggregating() {
                                        if let Some(section) = extraction.get_raw(comp.schema_key()) {
                                            a.aggregate_section(&lang, section, agg)
                                                .map_err(|e: AggregationError| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                                        }
                                    }
                                }
                            }
                        }
                    )*
                    _ => return Err(pyo3::exceptions::PyValueError::new_err(format!("Unsupported language: {}", lang_code))),
                }
            }
        }

        dispatch_record!(Polish, Turkish, Arabic, French, Italian);
        Ok(())
    }
}
