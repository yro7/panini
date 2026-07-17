use std::fmt::Debug;

use crate::alignment::wire;
use crate::component::{AnalysisComponent, ComponentContext};
use crate::traits::LinguisticDefinition;

/// Aligns the sentence with its translation, segment by segment.
///
/// Both sentences are split into addressable segments — whole tokens, or
/// single morphemes where a sub-word unit corresponds to a separate unit in
/// the other language — joined by many-to-many [`crate::alignment::AlignmentLink`]s.
/// This is the bilingual counterpart of the monolingual Leipzig gloss: the
/// gloss aligns a sentence with its analysis, this component aligns it with
/// its translation.
#[derive(Debug, Clone, Default)]
pub struct TranslationAlignment;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for TranslationAlignment {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for TranslationAlignment {
    fn name(&self) -> &'static str {
        "Translation Alignment"
    }

    fn schema_key(&self) -> &'static str {
        "translation_alignment"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        let r#gen = schemars::SchemaGenerator::default();
        let schema = r#gen.into_root_schema_for::<wire::AlignedTranslation>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, _lang: &L, ctx: &ComponentContext) -> String {
        format!(
            "Translate the sentence into {ui_lang} and align the two sentences segment by segment:\n\
             - `source.text` is the original sentence verbatim; `target.text` is an idiomatic \
               {ui_lang} translation.\n\
             - Split BOTH sentences into segments, in reading order. Default to one segment per \
               word; split a word into several segments (stem, affixes, clitics, fused plural \
               marks) whenever a sub-word unit corresponds to a separate unit in the other \
               sentence — mandatory for agglutinative morphology. The stem is a segment too.\n\
             - Segments must cover every non-whitespace character exactly once: the `surface` \
               strings of one word concatenate to that word exactly as written — no added \
               hyphens, no normalization. Punctuation is its own word, left unlinked.\n\
             - `starts_new_token` is true when the segment begins a new word (words are \
               separated by whitespace; each punctuation mark counts as its own word), false \
               when it continues the previous word. NEVER mark two whitespace-separated words \
               as one word — a multi-word unit is expressed by one link spanning several \
               segments, not by merging words.\n\
             - `gloss`: for grammatical morphemes and function words, UPPER CASE standard \
               Leipzig abbreviations (e.g. NOM, ACC, GEN, DAT, PL, SG, PST, PRS, FUT, INF, \
               PTCP, NEG, DEF, INDF, PASS, CAUS, REFL, SBJV, IMP, PROG, PFV, IPFV, COMP, REL, \
               DEM, COP, AUX, LOC). Compose as many atoms as needed, joined by `.` (1SG.POSS, \
               PST.PFV); person+number fuse without a dot (1SG, never 1.SG); `N` prefixes an \
               atom for \"non-\" (NPST). Use null for content stems and punctuation — do not \
               gloss a content word with its meaning. The gloss is a best-effort tooltip: \
               stray labels are cleaned up rather than rejected, but staying within these \
               atoms keeps it intact.\n\
             - `links` are many-to-many and reference segments by their `surface` text. When \
               the same surface appears more than once among a sentence's segments, add \
               `occurrence` (1-based, in reading order) to say which one is meant. \
               `Lexical` for content↔content, `Grammatical` when a grammatical unit is involved \
               (a case suffix may map to a preposition, a person suffix to a pronoun), `Phrasal` \
               for idioms aligned as a whole because word-by-word links would mislead. \
               Discontinuous units go in one link (e.g. French `ne…pas`).\n\
             - Link ONLY segments that genuinely correspond in meaning or function — pairing \
               segments because they sit at the same position is wrong. A segment with no \
               counterpart in the other sentence appears in no link at all — never force a \
               correspondence.\n\
             - `literal_translation`: a word-by-word literal rendering of the source sentence in \
               {ui_lang}, exposing its structure the way \"pomme de terre\" is literally \"apple \
               of earth\". Follow the source's own word order and morphology, not {ui_lang} \
               idiom. Null when it would read the same as `target.text`.",
            ui_lang = ctx.learner_ui_language
        )
    }

    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let alignment: wire::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        alignment.resolve().map(|_| ())
    }

    fn post_process(&self, _lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let alignment: wire::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        let resolved = alignment.resolve()?;
        *section = serde_json::to_value(&resolved).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn needs_pedagogical_context(&self) -> bool {
        false
    }
}
