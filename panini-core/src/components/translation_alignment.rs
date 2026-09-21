use std::fmt::Debug;

use crate::alignment::wire_v3;
use crate::component::{AnalysisComponent, ComponentContext};
use crate::traits::LinguisticDefinition;

/// Aligns the sentence with its translation, segment by segment.
///
/// Both sentences are split into addressable segments — whole tokens, or
/// single morphemes where a sub-word unit corresponds to a separate unit in
/// the other language — joined by many-to-many [`crate::alignment::AlignmentLink`]s.
/// Uses the compact [`wire_v3`] format for efficient LLM extraction with
/// uniform `{s, o}` segment references.
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
        let schema = r#gen.into_root_schema_for::<wire_v3::AlignedTranslation>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, lang: &L, ctx: &ComponentContext) -> String {
        let mut fragment = format!(
            "Translate the sentence into {ui_lang}; `t.x` is that idiomatic translation. \
             CRITICAL: `t` MUST be an object containing BOTH `x` (the complete translation) \
             and `w` (its token arrays); NEVER emit `t` as a bare array. \
             Align the two sentences segment by segment:\n\
             - Split BOTH sentences into words (`s` for the source sentence, `t.w` for the \
               translation), in reading order. Each word is an ARRAY of segment strings. A \
               whole word is a one-element array like [\"plaży\"]; split a word into several \
               segments (stem, affixes, clitics, fused plural marks) whenever a sub-word unit \
               corresponds to a separate unit in the other sentence — mandatory for \
               agglutinative morphology. The stem is a segment too.\n\
             - The segments of one word concatenate to that word exactly as written — no added \
               hyphens, no normalization, NEVER any whitespace inside a segment. Each \
               punctuation mark is its own one-element word, left unlinked. NEVER merge two \
               whitespace-separated words into one array — a multi-word unit is expressed by \
               one link spanning several segments, not by merging words. Conversely, NEVER \
               split one written word into several arrays: a clitic written attached to its \
               host (an article, a conjunction, a preposition, a pronoun suffix) is a segment \
               of that word's array, never a word of its own.\n\
             - `l` holds the links; they are many-to-many and reference segments as \
               {{\"s\": text, \"o\": occurrence}} where `s` is the segment's exact text and \
               `o` is the 1-based position among segments with that same text in reading \
               order. Use `o`: 1 when the surface is unique. \
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
            ui_lang = ctx.learner_ui_language.to_name()
        );
        // Per-language segmentation inventories, source first: they say what
        // each language *can* split, the rules above decide whether to.
        if let Some(directives) = lang.alignment_directives() {
            fragment.push_str(&format!(
                "\n\nSource sentence ({}) — language-specific segmentation:\n{directives}",
                lang.name()
            ));
        }
        if let Some(directives) = ctx.translation_alignment_directives {
            fragment.push_str(&format!(
                "\n\nTranslation ({}) — language-specific segmentation:\n{directives}",
                ctx.learner_ui_language.to_name()
            ));
        }
        fragment
    }

    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let alignment: wire_v3::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        alignment.resolve().map(|_| ())
    }

    fn post_process(&self, _lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let alignment: wire_v3::AlignedTranslation =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        let resolved = alignment.resolve()?;
        *section = serde_json::to_value(&resolved).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn needs_pedagogical_context(&self) -> bool {
        false
    }

    fn needs_extraction_directives(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{StubLanguage, StubMorphology};
    use crate::traits::{IsoLang, Script};

    /// A stub that declares what it can split; `StubLanguage` declares nothing.
    #[derive(Debug)]
    struct SplittingLanguage;

    impl LinguisticDefinition for SplittingLanguage {
        type Morphology = StubMorphology;
        type MorphemeFunction = ();

        const ISO_LANG: IsoLang = IsoLang::Tur;

        fn supported_scripts(&self) -> &[Script] {
            &[Script::LATN]
        }

        fn default_script(&self) -> Script {
            Script::LATN
        }

        fn extraction_directives(&self) -> &str {
            ""
        }

        fn alignment_directives(&self) -> Option<&'static str> {
            Some("1. Case suffixes are segments.")
        }
    }

    fn context() -> ComponentContext<'static> {
        ComponentContext {
            targets: &[],
            learner_ui_language: IsoLang::Fra,
            translation_alignment_directives: None,
            pedagogical_context: None,
            skill_path: None,
            linguistic_background: &[],
        }
    }

    #[test]
    fn a_language_without_directives_gets_no_source_section() {
        let prompt = AnalysisComponent::<StubLanguage>::prompt_fragment(
            &TranslationAlignment,
            &StubLanguage,
            &context(),
        );

        assert!(!prompt.contains("Source sentence ("));
        assert!(!prompt.contains("Translation ("));
        assert!(prompt.contains("Translate the sentence into French"));
    }

    #[test]
    fn source_directives_follow_the_general_rules_under_the_language_name() {
        let prompt = AnalysisComponent::<SplittingLanguage>::prompt_fragment(
            &TranslationAlignment,
            &SplittingLanguage,
            &context(),
        );

        let heading = "Source sentence (Turkish) — language-specific segmentation:\n1. Case suffixes are segments.";
        assert_eq!(prompt.matches(heading).count(), 1);
        assert!(prompt.find(heading).unwrap() > prompt.find("`lit`:").unwrap());
    }

    #[test]
    fn translation_directives_come_after_the_source_ones() {
        let ctx = ComponentContext {
            translation_alignment_directives: Some("1. `ne…pas` is one discontinuous unit."),
            ..context()
        };
        let prompt = AnalysisComponent::<SplittingLanguage>::prompt_fragment(
            &TranslationAlignment,
            &SplittingLanguage,
            &ctx,
        );

        let source = prompt.find("Source sentence (Turkish)").unwrap();
        let translation = prompt
            .find("Translation (French) — language-specific segmentation:\n1. `ne…pas`")
            .unwrap();
        assert!(source < translation);
    }
}
