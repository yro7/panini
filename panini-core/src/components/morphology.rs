use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::domain::ExtractedFeature;
use crate::traits::LinguisticDefinition;

/// Produces morphological feature extraction for target and context words.
#[derive(Debug, Clone, Default)]
pub struct MorphologyAnalysis;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for MorphologyAnalysis {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for MorphologyAnalysis {
    fn name(&self) -> &'static str {
        "Morphology Analysis"
    }

    fn schema_key(&self) -> &'static str {
        "morphology"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        // Generate the schema for ExtractedFeature<L::Morphology> and build the
        // morphology object with target_features and context_features.
        let r#gen = schemars::SchemaGenerator::default();
        let feature_schema = r#gen.into_root_schema_for::<Vec<ExtractedFeature<L::Morphology>>>();
        let feature_value = serde_json::to_value(&feature_schema).unwrap();

        // Extract $defs if present — they'll be hoisted by the composer
        let mut fragment = serde_json::json!({
            "type": "object",
            "properties": {
                "target_features": feature_value,
                "context_features": feature_value
            },
            "required": ["target_features", "context_features"]
        });

        // Hoist $defs from the inner schema to our fragment level for the composer
        if let Some(defs) = feature_value.get("$defs") {
            fragment["$defs"] = defs.clone();
            // Remove $defs from the nested copies
            if let Some(props) = fragment.get_mut("properties") {
                for key in ["target_features", "context_features"] {
                    if let Some(obj) = props.get_mut(key).and_then(|p| p.as_object_mut()) {
                        obj.remove("$defs");
                    }
                }
            }
        }

        fragment
    }

    fn prompt_fragment(&self, lang: &L, _ctx: &ComponentContext) -> String {
        format!(
            "Extract morphological features from every word in the sentence, following the JSON schema exactly.\n\
             Language-specific extraction directives for {}:\n{}",
            lang.name(),
            lang.extraction_directives()
        )
    }

    fn output_instruction(&self) -> Option<&str> {
        Some(concat!(
            "MORPHOLOGY RULES \n",
            "Fill the `morphology` key with exactly two lists:\n",
            "  \"target_features\"  — morphological features for each constituent word of\n",
            "                       the TARGET WORDS supplied in the user message.\n",
            "  \"context_features\" — morphological features for every OTHER word in the\n",
            "                       sentence (for grammatical context).\n\n",
            "1. The `pos` key is an internally-tagged discriminant. Use ONLY the variants\n",
            "   defined in the schema. NEVER invent variants such as \"phrase\", \"punctuation\",\n",
            "   \"clause\", or anything not listed.\n",
            "2. Punctuation tokens (`,` `.` `!` `?` `—` `«» ` etc.) must be OMITTED entirely.\n",
            "3. Every field MUST exactly match the enum variants in the schema (e.g., do not \n",
            "   use \"masculine\" as a value for a \"number\" field; use \"singular\", \"dual\", or \"plural\").\n",
            "4. Optional fields must match the schema's definition (omit if not applicable).\n",
            "5. Lemmas must always be in the dictionary (citation) form of the target language."
        ))
    }

    fn pre_process(&self, raw: &str) -> String {
        crate::text_processing::normalize_pos_tags(raw)
    }
}
