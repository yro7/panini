use std::fmt::Debug;

use crate::alignment::wire_v4;
use crate::component::{AnalysisComponent, ComponentContext};
use crate::traits::LinguisticDefinition;

/// Aligns the sentence with its translation, segment by segment, using the
/// [`wire_v4`] format: correspondences are group numbers carried by the
/// segments rather than a separate table of references.
///
/// Same task and same output as [`super::TranslationAlignment`] — only the
/// shape the model is asked for differs. See [`wire_v4`] for why.
#[derive(Debug, Clone, Default)]
pub struct TranslationAlignmentV4;

/// A resolved alignment shown in full, because the numbering rule reads as
/// ambiguous in prose: told only that "segments carrying the same number
/// correspond", models number a repeated word once — both `samochód` as 2,
/// both `voiture` as 2 — which collapses two correspondences into one and
/// leaves other groups one-sided. The example is chosen to make that reading
/// impossible: a repeated noun that must take two different numbers, and a
/// discontinuous negation that must take one.
///
/// Deliberately not a benchmark fixture, and in a language pair none of them
/// use: an example that overlaps the evaluation set measures the example.
const EXAMPLE: &str = r#"{
  "source": [["Der"], ["Hund"], ["sieht"], ["den"], ["Hund"], ["nicht"], ["."]],
  "source_groups": [[1], [2], [3], [4], [5], [6], [0]],
  "translation": "Le chien ne voit pas le chien.",
  "target": [["Le"], ["chien"], ["ne"], ["voit"], ["pas"], ["le"], ["chien"], ["."]],
  "target_groups": [[1], [2], [6], [3], [6], [4], [5], [0]],
  "literal": "Le chien voit le chien ne-pas."
}"#;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for TranslationAlignmentV4 {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for TranslationAlignmentV4 {
    fn name(&self) -> &'static str {
        "Translation Alignment"
    }

    fn schema_key(&self) -> &'static str {
        "translation_alignment"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        let r#gen = schemars::SchemaGenerator::default();
        let schema = r#gen.into_root_schema_for::<wire_v4::AlignedTranslation>();
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
             - The segments of one word concatenate to that word exactly as written — no added \
               hyphens, no normalization, NEVER any whitespace inside a segment. Each \
               punctuation mark is its own one-element word. A word array covers ONE \
               whitespace-delimited word: never put two whitespace-separated words in the same \
               array, and never split one written word across two arrays — \"Let's\" is one \
               word, either [\"Let's\"] or [\"Let\", \"'s\"], never two words.\n\
             - `source_groups` and `target_groups` number the segments: they mirror `source` \
               and `target` exactly, one number per segment, same nesting. Segments carrying \
               the SAME number correspond to each other. That is the entire link mechanism: a \
               many-to-many correspondence is just several segments sharing a number, and a \
               discontinuous unit needs nothing special — French `ne … pas` translating to one \
               word is those three segments all carrying, say, 2.\n\
             - Number only what genuinely corresponds in meaning or function; pairing segments \
               because they sit at the same position is wrong. A segment with no counterpart in \
               the other sentence takes group 0 — punctuation usually does. Every non-zero \
               number must appear on BOTH sides, at least once each. Numbering is otherwise \
               free: start at 1 and go up in source reading order.\n\
             - `literal`: a word-by-word literal rendering of the source sentence in {ui_lang}, \
               exposing its structure the way \"pomme de terre\" is literally \"apple of \
               earth\". Follow the source's own word order and morphology, not {ui_lang} \
               idiom. Null when it would read the same as `translation`.\n\
             \n\
             Worked example — German \"Der Hund sieht den Hund nicht.\" into French:\n\
             {example}\n\
             Read what the numbers do there. The two \"Hund\" are two DIFFERENT \
             correspondences, 2 and 5, each pairing with its own \"chien\" — a number marks one \
             correspondence, it does NOT mark \"the same word\", so never give two occurrences \
             of a repeated word the same number. \"ne\" and \"pas\" share 6 with each other and \
             with \"nicht\": one discontinuous unit, one number, no special structure. Both \
             full stops take 0 because neither corresponds to anything. And \"n'aime\" would be \
             ONE word of two segments, [\"n'\", \"aime\"], never two words.",
            ui_lang = ctx.learner_ui_language,
            example = EXAMPLE
        )
    }

    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let alignment: wire_v4::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        alignment.resolve().map(|_| ())
    }

    fn post_process(&self, _lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let alignment: wire_v4::AlignedTranslation =
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
    use super::{EXAMPLE, wire_v4};

    /// An example the resolver would reject teaches the model to fail. It also
    /// has to keep saying what it is shown for, so the two claims the prose
    /// makes about it are asserted here rather than trusted.
    #[test]
    fn the_worked_example_resolves_and_says_what_it_claims() {
        let wire: wire_v4::AlignedTranslation =
            serde_json::from_str(EXAMPLE).expect("the example should be valid v4 JSON");
        let resolved = wire.resolve().expect("the example should resolve");

        assert_eq!(resolved.source.text, "Der Hund sieht den Hund nicht.");

        // Source ids: Der 0, Hund 1, sieht 2, den 3, Hund 4, nicht 5, . 6.
        // Target ids: Le 0, chien 1, ne 2, voit 3, pas 4, le 5, chien 6, . 7.
        let of = |ids: Vec<u32>| {
            resolved
                .links
                .iter()
                .find(|l| l.source == ids)
                .map(|l| l.target.clone())
        };
        assert_eq!(of(vec![1]), Some(vec![1]), "the first Hund pairs alone");
        assert_eq!(of(vec![4]), Some(vec![6]), "the second Hund pairs alone");
        assert_eq!(
            of(vec![5]),
            Some(vec![2, 4]),
            "nicht reaches both ne and pas as one discontinuous link"
        );
        assert!(
            resolved.links.iter().all(|l| !l.source.contains(&6)),
            "the full stops stay out of every link"
        );
    }
}
