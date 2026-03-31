use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::morpheme::WordSegmentation;
use crate::traits::{LinguisticDefinition, TypologicalFeature};

/// Produces morpheme segmentation for agglutinative languages.
///
/// This component is only compatible with languages that have the
/// `Agglutination` typological feature.
#[derive(Debug, Clone)]
pub struct MorphemeSegmentation;

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
        lang.extra_extraction_directives()
            .unwrap_or_default()
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
