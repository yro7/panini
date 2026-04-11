use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::morpheme::WordSegmentation;
use crate::traits::{LinguisticDefinition, TypologicalFeature};

/// Produces morpheme segmentation for agglutinative languages.
///
/// This component is only compatible with languages that have the
/// `Agglutination` typological feature.
#[derive(Debug, Clone, Default)]
pub struct MorphemeSegmentation;

/// Compile-time compatibility gate: `MorphemeSegmentation` can only be used
/// with languages that implement `Agglutinative`. The `#[derive(PaniniResult)]`
/// macro emits a `ComponentRequires<L>` bound for each component, so using
/// `MorphemeSegmentation` with a non-agglutinative language causes a compile error.
impl<L: LinguisticDefinition + crate::morpheme::Agglutinative>
    crate::component::ComponentRequires<L> for MorphemeSegmentation
where
    <L::Morphology as crate::traits::MorphologyInfo>::PosTag:
        std::fmt::Debug + Clone + Copy + PartialEq + Eq + std::hash::Hash + 'static,
    L::GrammaticalFunction: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + schemars::JsonSchema
        + Send
        + Sync
        + 'static,
{
}

impl<L: LinguisticDefinition> AnalysisComponent<L> for MorphemeSegmentation {
    fn name(&self) -> &'static str {
        "Morpheme Segmentation"
    }

    fn schema_key(&self) -> &'static str {
        "morpheme_segmentation"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        let r#gen = schemars::SchemaGenerator::default();
        let schema = r#gen.into_root_schema_for::<Vec<WordSegmentation<L::GrammaticalFunction>>>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, lang: &L, _ctx: &ComponentContext) -> String {
        lang.extra_extraction_directives().unwrap_or_default()
    }

    fn post_process(&self, lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        // Deserialize into typed form, run language post-processing, then re-serialize
        let mut segmentation: Option<Vec<WordSegmentation<L::GrammaticalFunction>>> =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;

        lang.post_process_extraction(&mut segmentation)?;

        *section = serde_json::to_value(&segmentation).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn is_compatible(&self, lang: &L) -> bool {
        lang.typological_features()
            .contains(&TypologicalFeature::Agglutination)
    }
}
