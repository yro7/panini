use pyo3::prelude::*;

mod aggregation;
mod extraction;
mod metadata;

/// The panini Python module implemented in Rust.
#[pymodule]
fn panini(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Extraction
    m.add_function(wrap_pyfunction!(extraction::extract, m)?)?;
    m.add_function(wrap_pyfunction!(extraction::async_extract, m)?)?;
    m.add_function(wrap_pyfunction!(extraction::get_default_prompts, m)?)?;

    // Metadata
    m.add_function(wrap_pyfunction!(metadata::supported_languages, m)?)?;
    m.add_function(wrap_pyfunction!(metadata::version, m)?)?;
    m.add_class::<metadata::LanguageInfo>()?;
    m.add_function(wrap_pyfunction!(metadata::get_language_info, m)?)?;
    m.add_function(wrap_pyfunction!(metadata::get_morphology_schema, m)?)?;

    // Aggregation
    m.add_class::<aggregation::PyDistribution>()?;
    m.add_class::<aggregation::PyInventory>()?;
    m.add_class::<aggregation::PyGroupResult>()?;
    m.add_class::<aggregation::PyAggregationResult>()?;
    m.add_class::<aggregation::BasicAggregatorWrapper>()?;
    // Rename to BasicAggregator for Python
    m.add("BasicAggregator", m.getattr("BasicAggregatorWrapper")?)?;

    Ok(())
}
