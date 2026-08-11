use std::fmt::Debug;

use crate::alignment::wire_v6;
use crate::component::{AnalysisComponent, ComponentContext};
use crate::traits::LinguisticDefinition;

/// Aligns the sentence with its translation segment by segment using the
/// [`wire_v6`] format: 0-based flat segment indices.
#[derive(Debug, Clone, Default)]
pub struct TranslationAlignmentV6;

const EXAMPLE_V6: &str = r#"{
  "source": [["Der"], ["Hund"], ["sieht"], ["den"], ["Hund"], ["nicht"], ["."]],
  "translation": "Le chien ne voit pas le chien.",
  "target": [["Le"], ["chien"], ["ne"], ["voit"], ["pas"], ["le"], ["chien"], ["."]],
  "literal": "Le chien voit le chien ne-pas.",
  "links": [
    { "source": [0], "target": [0] },
    { "source": [1], "target": [1] },
    { "source": [2], "target": [3] },
    { "source": [3], "target": [5] },
    { "source": [4], "target": [6] },
    { "source": [5], "target": [2, 4] }
  ]
}"#;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for TranslationAlignmentV6 {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for TranslationAlignmentV6 {
    fn name(&self) -> &'static str {
        "Translation Alignment V6"
    }

    fn schema_key(&self) -> &'static str {
        "translation_alignment"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        let r#gen = schemars::SchemaGenerator::default();
        let schema = r#gen.into_root_schema_for::<wire_v6::AlignedTranslation>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, _lang: &L, ctx: &ComponentContext) -> String {
        format!(
            "Translate the sentence into {ui_lang}; `translation` is that idiomatic \
             translation. Align the two sentences segment by segment:\n\
             - Split BOTH sentences into words (`source` for the source sentence, `target` for \
               the translation), in reading order. Each word is an ARRAY of segment strings. A \
               whole word is a one-element array like [\"plaży\"]; split a word into several \
               segments (stem, affixes, clitics, fused plural marks) whenever a sub-word unit \
               corresponds to a separate unit in the other sentence — mandatory for \
               agglutinative morphology, e.g. [\"Ev\", \"ler\", \"im\", \"de\"]. The stem is a \
               segment too.\n\
             - Segments across words are assigned 0-based flat segment indices in reading order \
               (0, 1, 2, 3...). For example, if `source` is [[\"Je\"], [\"d'\", \"eau\"]], the \
               segment indices are: \"Je\" -> 0, \"d'\" -> 1, \"eau\" -> 2.\n\
             - `links` holds many-to-many correspondences referencing these 0-based flat segment \
               indices: `{{\"source\": [0], \"target\": [1]}}`. Discontinuous units go in one \
               link (e.g. French `ne…pas` -> one link with two target segment indices).\n\
             - Link ONLY segments that genuinely correspond in meaning or function. Punctuation \
               or unlinked elements are left out of all links.\n\
             - `literal`: a word-by-word literal rendering of the source sentence in {ui_lang}, \
               exposing its structure the way \"pomme de terre\" is literally \"apple of \
               earth\". Null when it would read the same as `translation`.\n\
             \n\
             Worked example — German \"Der Hund sieht den Hund nicht.\" into French:\n\
             {example}",
            ui_lang = ctx.learner_ui_language,
            example = EXAMPLE_V6
        )
    }

    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let alignment: wire_v6::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        alignment.resolve().map(|_| ())
    }

    fn post_process(&self, _lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let alignment: wire_v6::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        let resolved = alignment.resolve()?;
        *section = serde_json::to_value(&resolved).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn needs_pedagogical_context(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{EXAMPLE_V6, wire_v6};

    #[test]
    fn the_worked_example_resolves() {
        let wire: wire_v6::AlignedTranslation =
            serde_json::from_str(EXAMPLE_V6).expect("the example should be valid v6 JSON");
        let resolved = wire.resolve().expect("the example should resolve");
        assert_eq!(resolved.source.text, "Der Hund sieht den Hund nicht.");
        assert_eq!(resolved.links.len(), 6);
    }
}
