use std::fmt::Debug;

use crate::alignment::wire_v2;
use crate::component::{AnalysisComponent, ComponentContext};
use crate::traits::LinguisticDefinition;

/// Aligns the sentence with its translation, segment by segment — the compact
/// successor to [`super::translation_alignment::TranslationAlignment`].
///
/// Same alignment semantics and same stored format (`post_process` emits the
/// resolved [`crate::alignment::AlignedTranslation`]), but the LLM emits the
/// [`wire_v2`] shape: words as nested arrays instead of per-segment objects
/// with boundary flags, and link references that collapse to bare strings when
/// unambiguous. Measured on real song alignments, the v1 shape was verbose
/// enough to overflow `max_tokens` on every call; v2 exists to cut that
/// output by an order of magnitude without losing any expressiveness.
///
/// Not yet registered in any production pipeline — exercised by the live
/// probe (`app/tests/live_alignment_probe_v2.rs`) until the format is
/// validated on real model output.
#[derive(Debug, Clone, Default)]
pub struct TranslationAlignmentV2;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for TranslationAlignmentV2 {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for TranslationAlignmentV2 {
    fn name(&self) -> &'static str {
        "Translation Alignment v2"
    }

    fn schema_key(&self) -> &'static str {
        "translation_alignment"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        let r#gen = schemars::SchemaGenerator::default();
        let schema = r#gen.into_root_schema_for::<wire_v2::AlignedTranslation>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, _lang: &L, ctx: &ComponentContext) -> String {
        format!(
            "Translate the sentence into {ui_lang}; `t.x` is that idiomatic translation. \
             Align the two sentences segment by segment:\n\
             - Split BOTH sentences into words (`s` for the source sentence, `t.w` for the \
               translation), in reading order. Each word is an ARRAY of segment strings. A \
               whole word is a one-element array like [\"plaży\"]; split a word into several \
               segments (stem, affixes, clitics, fused plural marks) whenever a sub-word unit \
               corresponds to a separate unit in the other sentence — mandatory for \
               agglutinative morphology, e.g. [\"Ev\", \"ler\", \"im\", \"de\"]. The stem is a \
               segment too.\n\
             - The segments of one word concatenate to that word exactly as written — no added \
               hyphens, no normalization, NEVER any whitespace inside a segment. Each \
               punctuation mark is its own one-element word, left unlinked. NEVER merge two \
               whitespace-separated words into one array — a multi-word unit is expressed by \
               one link spanning several segments, not by merging words.\n\
             - `l` holds the links; they are many-to-many and reference segments by their \
               exact text. A reference is the segment string itself when that text is unique \
               among the sentence's segments; when it appears more than once, use \
               {{\"s\": text, \"o\": occurrence}} with `o` 1-based in reading order. \
               Discontinuous units go in one link (e.g. French `ne…pas` → one link with two \
               source references).\n\
             - Link ONLY segments that genuinely correspond in meaning or function — pairing \
               segments because they sit at the same position is wrong. A segment with no \
               counterpart in the other sentence appears in no link at all — never force a \
               correspondence.\n\
             - `lit`: a word-by-word literal rendering of the source sentence in {ui_lang}, \
               exposing its structure the way \"pomme de terre\" is literally \"apple of \
               earth\". Follow the source's own word order and morphology, not {ui_lang} \
               idiom. Null when it would read the same as `t.x`.",
            ui_lang = ctx.learner_ui_language
        )
    }

    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let alignment: wire_v2::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        alignment.resolve().map(|_| ())
    }

    fn post_process(&self, _lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let alignment: wire_v2::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        let resolved = alignment.resolve()?;
        *section = serde_json::to_value(&resolved).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn needs_pedagogical_context(&self) -> bool {
        false
    }
}
