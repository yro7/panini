use std::fmt::Debug;

use serde::de::DeserializeOwned;

use crate::aggregable::digest::AggregationSink;
use crate::traits::{IsoLang, LinguisticDefinition};

/// A language the learner already speaks, with proficiency level.
#[derive(Debug, Clone)]
pub struct LanguageLevel {
    pub iso_639_3: IsoLang,
    pub level: ProficiencyLevel,
}

/// Proficiency scale used by Panglotive's learner profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProficiencyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Fluent,
    Native,
}

/// TODO: refactor avec l'imp de Panglotive
impl ProficiencyLevel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
            Self::Fluent => "Fluent",
            Self::Native => "Native",
        }
    }
}

impl std::fmt::Display for ProficiencyLevel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Context passed to components during schema/prompt generation.
pub struct ComponentContext<'a> {
    pub targets: &'a [String],
    pub learner_ui_language: IsoLang,
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

    /// The version of this component's **persisted** shape, stamped onto every
    /// section it produces so readers can dispatch on it.
    ///
    /// This versions the section as it looks *after* [`Self::post_process`] —
    /// what actually reaches storage — not the shape the LLM is asked to emit.
    /// `TranslationAlignment` and `TranslationAlignmentV2` are the worked
    /// example: wildly different wire schemas, both resolving to the same
    /// `AlignedTranslation`, therefore both version 1. Changing how you ask the
    /// model is not a storage migration.
    ///
    /// Bump this whenever the stored JSON changes shape in a way a reader of
    /// the previous shape would not understand. A bump is a deliberate act: it
    /// comes with a migration from the previous version, so sections already in
    /// the database stay readable. Purely additive changes that older readers
    /// tolerate do not need a bump.
    ///
    /// This covers only variation owned by the component's own code. Variation
    /// coming from the language definition (a morphology enum gaining or losing
    /// variants) is not expressible as a hand-written integer and is tracked
    /// separately by the language digest.
    fn output_version(&self) -> u32 {
        1
    }

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
    ///
    /// # Errors
    /// Returns a validation error string if the section does not conform to expected constraints.
    fn validate(&self, _lang: &L, _section: &serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    /// Post-process this component's section of the parsed JSON (in place).
    ///
    /// # Errors
    /// Returns an error string if post-processing logic fails.
    fn post_process(&self, _lang: &L, _section: &mut serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    /// Whether this component is compatible with the given language.
    /// Incompatible components are silently skipped.
    fn is_compatible(&self, _lang: &L) -> bool {
        true
    }

    /// Whether this component's prompt needs the pedagogical context blocks
    /// (`<learner_profile>`, `<skill_context>`, `<user_context>`).
    ///
    /// Mechanical components (morphology, alignment, …) override this to
    /// `false` so single-axis prompts don't carry generation instructions
    /// that are noise for their task. The composer includes the blocks if
    /// any component in the call requests them.
    fn needs_pedagogical_context(&self) -> bool {
        true
    }

    /// Returns `Some(self)` for components that produce aggregable data.
    ///
    /// Override to return `Some(self)` in components that implement [`Aggregating<L>`].
    /// Default returns `None` — non-aggregable components carry no aggregation logic.
    fn as_aggregating(&self) -> Option<&dyn Aggregating<L>> {
        None
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
    #[must_use]
    pub const fn new(raw: serde_json::Value, requested_keys: Vec<&'static str>) -> Self {
        Self {
            raw,
            requested_keys,
        }
    }

    /// Deserialize a component's section into a concrete type.
    ///
    /// # Errors
    /// Returns `ExtractionResultError::KeyNotFound` if the key is not in the result.
    /// Returns `ExtractionResultError::DeserializeError` if the section fails to deserialize into `T`.
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
    #[must_use]
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
    #[must_use]
    pub fn requested_keys(&self) -> &[&'static str] {
        &self.requested_keys
    }

    /// Consume and return the raw JSON value.
    #[must_use]
    pub fn into_raw(self) -> serde_json::Value {
        self.raw
    }
}

// ─── AggregationError ────────────────────────────────────────────────────────

/// Typed error for [`Aggregating::aggregate_section`].
#[derive(Debug, thiserror::Error)]
pub enum AggregationError {
    #[error("failed to deserialize section '{key}': {source}")]
    Deserialize {
        key: &'static str,
        #[source]
        source: serde_json::Error,
    },
    #[error("aggregation hook for '{key}' failed: {message}")]
    Hook { key: &'static str, message: String },
}

// ─── Aggregating sub-trait ────────────────────────────────────────────────────

/// Extension of [`AnalysisComponent`] for components that produce aggregable data.
///
/// Components opt in by overriding `as_aggregating` on `AnalysisComponent` to
/// return `Some(self)`. Non-aggregable components (`PedagogicalExplanation`,
/// `LeipzigGloss`, etc.) do nothing.
pub trait Aggregating<L: LinguisticDefinition>: AnalysisComponent<L> {
    /// Project this component's JSON section into aggregation contributions.
    ///
    /// Called per-card with the deserialized (and post-processed) section value.
    /// Implementations deserialize the section and push contributions to `sink`
    /// via [`AggregationSink::record_contribution`] or the typed shim
    /// [`AggregationSink::record`].
    fn aggregate_section(
        &self,
        lang: &L,
        section: &serde_json::Value,
        sink: &mut dyn AggregationSink,
    ) -> Result<(), AggregationError>;
}

// ─── Marker trait ─────────────────────────────────────────────────────────────

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
