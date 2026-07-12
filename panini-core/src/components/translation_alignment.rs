use std::fmt::Debug;

use crate::alignment::AlignedTranslation;
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
        let schema = r#gen.into_root_schema_for::<AlignedTranslation>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, _lang: &L, ctx: &ComponentContext) -> String {
        format!(
            "Translate the sentence into {ui_lang} and align the two sentences segment by segment:\n\
             - `source.text` is the original sentence verbatim; `target.text` is an idiomatic \
               {ui_lang} translation.\n\
             - Split BOTH sentences into segments, in reading order. Default to one segment per \
               token; split a token into several segments (stem, affixes, clitics, fused plural \
               marks) whenever a sub-word unit corresponds to a separate unit in the other \
               sentence — mandatory for agglutinative morphology. The stem is a segment too.\n\
             - Segments must cover every non-whitespace character exactly once: the `surface` \
               strings of one `token` concatenate to that token exactly as written — no added \
               hyphens, no normalization. Punctuation is its own token, left unlinked.\n\
             - `id` is unique per sentence (use the segment's position); `token` is the 0-based \
               token index, strictly increasing.\n\
             - `gloss`: for grammatical morphemes and function words, UPPER CASE standard \
               Leipzig abbreviations ONLY (e.g. NOM, ACC, GEN, DAT, PL, SG, PST, PRS, FUT, INF, \
               PTCP, NEG, DEF, INDF, PASS, CAUS, REFL, SBJV, IMP, PROG, PFV, IPFV, COMP, REL, \
               DEM, COP, AUX, LOC) — non-standard labels are rejected (PRES→PRS, PAST→PST). \
               Compose as many atoms as needed, joined by `.` (1SG.POSS, PST.PFV); person+number \
               fuse without a dot (1SG, never 1.SG); `N` prefixes an atom for \"non-\" (NPST). \
               Use null for content stems and punctuation.\n\
             - `links` are many-to-many: `Lexical` for content↔content, `Grammatical` when a \
               grammatical unit is involved (a case suffix may map to a preposition, a person \
               suffix to a pronoun), `Phrasal` for idioms aligned as a whole because word-by-word \
               links would mislead. Discontinuous units go in one link (e.g. French `ne…pas`).\n\
             - A segment with no counterpart in the other sentence appears in no link at all — \
               never force a correspondence.",
            ui_lang = ctx.learner_ui_language
        )
    }

    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let alignment: AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        alignment.validate_structure()
    }

    fn post_process(&self, _lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let mut alignment: AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        alignment.locate_spans()?;
        *section = serde_json::to_value(&alignment).map_err(|e| e.to_string())?;
        Ok(())
    }
}
