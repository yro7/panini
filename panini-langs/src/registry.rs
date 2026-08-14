//! Language registry for panini.
//!
//! Provides the type-erased extraction entry-point `extract_erased_with_components()`
//! which dispatches on an ISO code and runs the composable component pipeline.

use anyhow::{Result, anyhow};
use rig::completion::CompletionModel;

use panini_core::component::{AnalysisComponent, ExtractionResult};
use panini_core::components::{
    LeipzigGloss, MorphemeSegmentation, MorphologyAnalysis, MultiwordExpressions,
    PedagogicalExplanation, TranslationAlignment,
};
use panini_engine::{
    ExtractionOptions, ExtractionRequest, extract_with_components, extract_with_components_executor,
};

/// Helper: build the component list for a concrete language and dispatch.
async fn extract_for_language<L, M>(
    lang: &L,
    model: &M,
    request: &ExtractionRequest,
    component_keys: Option<&[&str]>,
    options: ExtractionOptions<'_>,
) -> Result<ExtractionResult>
where
    L: panini_core::LinguisticDefinition + Send + Sync,
    L::Morphology: std::fmt::Debug
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + schemars::JsonSchema
        + panini_core::MorphologyInfo
        + Send
        + Sync,
    L::MorphemeFunction: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + schemars::JsonSchema
        + Send
        + Sync,
    M: CompletionModel + Sync,
{
    let pedagogical = PedagogicalExplanation;
    let morphology = MorphologyAnalysis;
    let multiword = MultiwordExpressions;
    let morpheme_seg = MorphemeSegmentation;
    let leipzig = LeipzigGloss;
    let translation = TranslationAlignment;

    let all_components: Vec<(&str, &dyn AnalysisComponent<L>)> = vec![
        ("pedagogical_explanation", &pedagogical),
        ("morphology", &morphology),
        ("multiword_expressions", &multiword),
        ("morpheme_segmentation", &morpheme_seg),
        ("leipzig_gloss", &leipzig),
        ("translation_alignment", &translation),
    ];

    let selected: Vec<&dyn AnalysisComponent<L>> = component_keys.map_or_else(
        || all_components.iter().map(|(_, c)| *c).collect(),
        |keys| {
            all_components
                .iter()
                .filter(|(k, _)| keys.contains(k))
                .map(|(_, c)| *c)
                .collect()
        },
    );

    Ok(extract_with_components(lang, model, request, &selected, options).await?)
}

async fn extract_for_language_executor<L, E>(
    lang: &L,
    executor: &E,
    request: &ExtractionRequest,
    component_keys: Option<&[&str]>,
    options: ExtractionOptions<'_>,
) -> Result<ExtractionResult>
where
    L: panini_core::LinguisticDefinition + Send + Sync,
    L::Morphology: std::fmt::Debug
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + schemars::JsonSchema
        + panini_core::MorphologyInfo
        + Send
        + Sync,
    L::MorphemeFunction: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + schemars::JsonSchema
        + Send
        + Sync,
    E: panini_engine::structured_llm::StructuredLlmExecutor,
{
    let pedagogical = PedagogicalExplanation;
    let morphology = MorphologyAnalysis;
    let multiword = MultiwordExpressions;
    let morpheme_seg = MorphemeSegmentation;
    let leipzig = LeipzigGloss;
    let translation = TranslationAlignment;

    let all_components: Vec<(&str, &dyn AnalysisComponent<L>)> = vec![
        ("pedagogical_explanation", &pedagogical),
        ("morphology", &morphology),
        ("multiword_expressions", &multiword),
        ("morpheme_segmentation", &morpheme_seg),
        ("leipzig_gloss", &leipzig),
        ("translation_alignment", &translation),
    ];

    let selected: Vec<&dyn AnalysisComponent<L>> = component_keys.map_or_else(
        || all_components.iter().map(|(_, c)| *c).collect(),
        |keys| {
            all_components
                .iter()
                .filter(|(k, _)| keys.contains(k))
                .map(|(_, c)| *c)
                .collect()
        },
    );

    Ok(extract_with_components_executor(lang, executor, request, &selected, options).await?)
}

/// Macro to generate the registry functions for all languages.
/// Each language must be a unit struct implementing `LinguisticDefinition`.
///
/// Names are resolved as `$crate::<Ident>`, so a language only has to be listed
/// in the `generate_registry!` call below — the `pub use` in `lib.rs` is what
/// puts it at the crate root. No import in this module.
macro_rules! generate_registry {
    ($($lang:ident),* $(,)?) => {
        /// Extracts features using composable components for any supported language.
        ///
        /// `component_keys` selects which analyses to include (e.g. `["pedagogical_explanation", "morphology"]`).
        /// If `None`, all compatible components are used.
        ///
        /// # Errors
        /// Returns an error if the language code is unsupported, or if extraction fails.
        pub async fn extract_erased_with_components<M: CompletionModel + Sync>(
            lang_code: &str,
            model: &M,
            request: &ExtractionRequest,
            component_keys: Option<&[&str]>,
            options: ExtractionOptions<'_>,
        ) -> Result<ExtractionResult> {
            match lang_code {
                $(
                    s if s == <$crate::$lang as panini_core::LinguisticDefinition>::ISO_LANG.to_639_3() => {
                        extract_for_language(
                            &$crate::$lang,
                            model,
                            request,
                            component_keys,
                            options,
                        )
                        .await
                    }
                )*
                _ => Err(anyhow!("Unsupported language: {lang_code}")),
            }
        }

        /// Extracts features using composable components and an injected structured executor for any supported language.
        ///
        /// `component_keys` selects which analyses to include (e.g. `["pedagogical_explanation", "morphology"]`).
        /// If `None`, all compatible components are used.
        ///
        /// # Errors
        /// Returns an error if the language code is unsupported, or if extraction fails.
        pub async fn extract_erased_with_components_executor<E: panini_engine::structured_llm::StructuredLlmExecutor>(
            lang_code: &str,
            executor: &E,
            request: &ExtractionRequest,
            component_keys: Option<&[&str]>,
            options: ExtractionOptions<'_>,
        ) -> Result<ExtractionResult> {
            match lang_code {
                $(
                    s if s == <$crate::$lang as panini_core::LinguisticDefinition>::ISO_LANG.to_639_3() => {
                        extract_for_language_executor(
                            &$crate::$lang,
                            executor,
                            request,
                            component_keys,
                            options,
                        )
                        .await
                    }
                )*
                _ => Err(anyhow!("Unsupported language: {lang_code}")),
            }
        }

        /// Returns all supported ISO 639-3 language codes.
        pub fn supported_languages() -> Vec<&'static str> {
            vec![$(<$crate::$lang as panini_core::LinguisticDefinition>::ISO_LANG.to_639_3()),*]
        }
    };
}

// Generate the registry for all supported languages
generate_registry!(
    Polish,
    Turkish,
    Arabic,
    French,
    Italian,
    Danish,
    MandarinChinese
);

#[cfg(test)]
mod tests {
    use crate::{Arabic, Danish, Polish, Turkish};
    use panini_core::component::AnalysisComponent;
    use panini_core::components::*;
    use panini_core::morpheme::Agglutinative;

    fn assert_agglutinative_inventory_valid<L: Agglutinative>() {
        if let Err(err) = L::validate_inventory() {
            panic!("Morpheme inventory validation failed: {err}");
        }
    }

    #[test]
    fn test_agglutinative_inventories_integrity() {
        assert_agglutinative_inventory_valid::<Turkish>();
    }

    #[test]
    fn morpheme_segmentation_compatible_with_turkish() {
        let comp = MorphemeSegmentation;
        assert!(comp.is_compatible(&Turkish));
    }

    #[test]
    fn morpheme_segmentation_incompatible_with_polish() {
        let comp = MorphemeSegmentation;
        assert!(!comp.is_compatible(&Polish));
    }

    #[test]
    fn morpheme_segmentation_incompatible_with_arabic() {
        let comp = MorphemeSegmentation;
        assert!(!comp.is_compatible(&Arabic));
    }

    #[test]
    fn morpheme_segmentation_incompatible_with_danish() {
        let comp = MorphemeSegmentation;
        assert!(!comp.is_compatible(&Danish));
    }

    #[test]
    fn all_other_components_universal() {
        let ped = PedagogicalExplanation;
        let morph = MorphologyAnalysis;
        let multi = MultiwordExpressions;
        let translation = TranslationAlignment;

        assert!(ped.is_compatible(&Turkish));
        assert!(morph.is_compatible(&Turkish));
        assert!(multi.is_compatible(&Turkish));
        assert!(translation.is_compatible(&Turkish));

        assert!(ped.is_compatible(&Polish));
        assert!(morph.is_compatible(&Polish));
        assert!(multi.is_compatible(&Polish));
        assert!(translation.is_compatible(&Polish));

        assert!(ped.is_compatible(&Arabic));
        assert!(morph.is_compatible(&Arabic));
        assert!(multi.is_compatible(&Arabic));
        assert!(translation.is_compatible(&Arabic));
    }

    #[test]
    fn translation_alignment_schema_has_expected_shape() {
        let comp = TranslationAlignment;
        let schema = comp.schema_fragment(&Turkish);
        let props = schema["properties"].as_object().expect("object schema");
        assert!(props.contains_key("s"));
        assert!(props.contains_key("t"));
        assert!(props.contains_key("l"));

        // Links reference segments by surface + 1-based occurrence (SegRef: {s, o}).
        let ref_props = &schema["$defs"]["SegRef"]["properties"];
        assert!(ref_props.get("s").is_some(), "schema: {schema}");
        assert!(ref_props.get("o").is_some(), "schema: {schema}");

        // Both sides of a link are schema-required to be non-empty.
        let link_source = &schema["$defs"]["Link"]["properties"]["s"];
        assert_eq!(link_source["minItems"], 1, "schema: {schema}");

        // In wire_v3, `s` is a 2D array of strings (words → segment strings),
        // `t` $refs TargetSentence (x: string, w: 2D array of strings),
        // and `l` is an array of Links.
        let s_type = schema["properties"]["s"]["type"]
            .as_str()
            .expect("s should be an array");
        assert_eq!(s_type, "array", "schema: {schema}");

        let target_ref = schema["properties"]["t"]["$ref"]
            .as_str()
            .expect("t should $ref a schema def");
        assert!(target_ref.ends_with("/TargetSentence"), "schema: {schema}");
        let target_props = &schema["$defs"]["TargetSentence"]["properties"];
        assert!(target_props.get("x").is_some(), "schema: {schema}");
        assert!(target_props.get("w").is_some(), "schema: {schema}");
    }
}
