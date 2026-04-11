use std::fmt::Debug;

use serde::de::DeserializeOwned;

use crate::traits::LinguisticDefinition;

/// A language the learner already speaks, with proficiency level.
#[derive(Debug, Clone)]
pub struct LanguageLevel {
    pub iso_639_3: String,
    pub level: String,
}

/// Context passed to components during schema/prompt generation.
pub struct ComponentContext<'a> {
    pub targets: &'a [String],
    pub learner_ui_language: &'a str,
    pub pedagogical_context: Option<&'a str>,
    pub skill_path: Option<&'a str>,
    pub linguistic_background: &'a [LanguageLevel],
}

/// A composable analysis component that contributes a section to the extraction schema,
/// prompt, and output processing pipeline.
///
/// Each component owns one top-level key in the JSON output.
/// Components are parameterized by the language definition `L` so they can
/// access language-specific types and methods.
pub trait AnalysisComponent<L: LinguisticDefinition>: Send + Sync + Debug {
    /// Human-readable name for logging/display.
    fn name(&self) -> &'static str;

    /// The top-level JSON key this component produces (e.g. `"morphology"`).
    fn schema_key(&self) -> &'static str;

    /// Returns the JSON Schema fragment for this component's output.
    /// This will be placed under `properties[schema_key]` in the composed schema.
    fn schema_fragment(&self, lang: &L) -> serde_json::Value;

    /// Returns prompt text describing what this component expects from the LLM.
    fn prompt_fragment(&self, lang: &L, ctx: &ComponentContext) -> String;

    /// Optional extra output instructions (appended to the output section).
    fn output_instruction(&self) -> Option<&str> {
        None
    }

    /// Pre-process the raw LLM JSON text before parsing.
    /// Applied to the full JSON string; components are chained in order.
    fn pre_process(&self, raw: &str) -> String {
        raw.to_string()
    }

    /// Validate this component's section of the parsed JSON.
    fn validate(&self, _lang: &L, _section: &serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    /// Post-process this component's section of the parsed JSON (in place).
    fn post_process(&self, _lang: &L, _section: &mut serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    /// Whether this component is compatible with the given language.
    /// Incompatible components are silently skipped.
    fn is_compatible(&self, _lang: &L) -> bool {
        true
    }
}

// ─── ExtractionResult ────────────────────────────────────────────────────────

/// Error type for `ExtractionResult` accessor methods.
#[derive(Debug, thiserror::Error)]
pub enum ExtractionResultError {
    #[error("key not found: {key}")]
    KeyNotFound { key: String },
    #[error("deserialization error for key '{key}': {source}")]
    DeserializeError {
        key: String,
        source: serde_json::Error,
    },
}

/// Container for the composed extraction result.
///
/// Holds the raw JSON value (an object with one key per component)
/// and provides typed accessors.
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    raw: serde_json::Value,
    requested_keys: Vec<&'static str>,
}

impl ExtractionResult {
    /// Create a new `ExtractionResult` from a raw JSON object and the list
    /// of component keys that were requested.
    pub fn new(raw: serde_json::Value, requested_keys: Vec<&'static str>) -> Self {
        Self {
            raw,
            requested_keys,
        }
    }

    /// Deserialize a component's section into a concrete type.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, ExtractionResultError> {
        let section = self
            .raw
            .get(key)
            .ok_or_else(|| ExtractionResultError::KeyNotFound {
                key: key.to_string(),
            })?;
        serde_json::from_value(section.clone()).map_err(|e| {
            ExtractionResultError::DeserializeError {
                key: key.to_string(),
                source: e,
            }
        })
    }

    /// Get the raw JSON value for a component's key.
    pub fn get_raw(&self, key: &str) -> Option<&serde_json::Value> {
        self.raw.get(key)
    }

    /// Iterate over all (key, value) pairs in the raw JSON object.
    pub fn iter_raw(&self) -> impl Iterator<Item = (&str, &serde_json::Value)> {
        self.raw
            .as_object()
            .into_iter()
            .flat_map(|obj| obj.iter().map(|(k, v)| (k.as_str(), v)))
    }

    /// The keys that were requested (i.e., the compatible components).
    pub fn requested_keys(&self) -> &[&'static str] {
        &self.requested_keys
    }

    /// Consume and return the raw JSON value.
    pub fn into_raw(self) -> serde_json::Value {
        self.raw
    }
}

/// Marker trait for compile-time validation of component-language compatibility.
///
/// Used by `#[derive(PaniniResult)]` to enforce that a component is valid for
/// the language `L`. Universal components implement this for all `L: LinguisticDefinition`.
/// Restricted components (e.g. `MorphemeSegmentation`) add trait bounds
/// (e.g. `L: Agglutinative`), causing a compile error if used with an incompatible language.
pub trait ComponentRequires<L> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_typed_value() {
        let raw = serde_json::json!({
            "pedagogical_explanation": "This is a test.",
            "morphology": { "target_features": [], "context_features": [] }
        });
        let result = ExtractionResult::new(raw, vec!["pedagogical_explanation", "morphology"]);

        let explanation: String = result.get("pedagogical_explanation").unwrap();
        assert_eq!(explanation, "This is a test.");
    }

    #[test]
    fn get_missing_key_returns_key_not_found() {
        let raw = serde_json::json!({ "morphology": {} });
        let result = ExtractionResult::new(raw, vec!["morphology"]);

        let err = result.get::<String>("nonexistent").unwrap_err();
        assert!(matches!(err, ExtractionResultError::KeyNotFound { .. }));
    }

    #[test]
    fn get_raw_returns_section() {
        let raw = serde_json::json!({ "morphology": { "target_features": [] } });
        let result = ExtractionResult::new(raw, vec!["morphology"]);

        assert!(result.get_raw("morphology").is_some());
        assert!(result.get_raw("nonexistent").is_none());
    }

    #[test]
    fn iter_raw_returns_all_entries() {
        let raw = serde_json::json!({
            "a": 1,
            "b": 2,
            "c": 3
        });
        let result = ExtractionResult::new(raw, vec![]);

        let keys: Vec<&str> = result.iter_raw().map(|(k, _)| k).collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"a"));
        assert!(keys.contains(&"b"));
        assert!(keys.contains(&"c"));
    }

    #[test]
    fn into_raw_consumes() {
        let raw = serde_json::json!({ "key": "value" });
        let result = ExtractionResult::new(raw.clone(), vec!["key"]);
        assert_eq!(result.into_raw(), raw);
    }
}
