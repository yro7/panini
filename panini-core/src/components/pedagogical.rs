use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::explanation::{MarkupKind, StructuredExplanation};
use crate::traits::LinguisticDefinition;

/// Produces a pedagogical explanation of the sentence in the learner's language.
///
/// Version 2 emits structure — an analysis list and an optional grammar recap —
/// where version 1 emitted an HTML blob. Inline markup survives the change, in
/// the compact `[text](kind)` form described in [`crate::explanation`]: which
/// words are verbs or grammar terms is a judgement about the sentence, which is
/// the model's job, while how a verb should look is the client's.
#[derive(Debug, Clone, Default)]
pub struct PedagogicalExplanation;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for PedagogicalExplanation {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for PedagogicalExplanation {
    fn name(&self) -> &'static str {
        "Pedagogical Explanation"
    }

    fn schema_key(&self) -> &'static str {
        "pedagogical_explanation"
    }

    fn output_version(&self) -> u32 {
        2
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["analysis"],
            "description": markup_instructions(),
            "properties": {
                "analysis": {
                    "type": "array",
                    "minItems": 1,
                    "description": concat!(
                        "One entry per point of analysis, in the learner's interface language. ",
                        "Cover the grammar the skill is testing. Do NOT analyse every trivial ",
                        "word -- merge related points into one entry rather than padding the list. ",
                        "No introductory or concluding chatter: no \"Here is...\", no \"Great example!\". ",
                        "Do NOT restate the translation, literal or natural -- it is already shown ",
                        "to the learner separately."
                    ),
                    "items": { "type": "string" }
                },
                "grammar_recap": {
                    // Nullable rather than absent: strict structured-output
                    // modes force every property into `required`, so "leave it
                    // out" is not something the model can be asked for. Null is
                    // the only way to say "there is none" that survives.
                    "type": ["object", "null"],
                    "additionalProperties": false,
                    "required": ["title", "rules"],
                    "description": concat!(
                        "A short summary of the declension, conjugation or rule the sentence ",
                        "exercises. Set this to null when the sentence exercises no rule worth ",
                        "restating -- an empty recap is worse than none."
                    ),
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "What the recap is about, e.g. \"Accusative (Biernik)\"."
                        },
                        "rules": {
                            "type": "array",
                            "description": "One entry per rule, e.g. \"Nouns take [-a](morpheme)\".",
                            "items": { "type": "string" }
                        }
                    }
                }
            }
        })
    }

    fn prompt_fragment(&self, _lang: &L, ctx: &ComponentContext) -> String {
        let mut s = format!(
            "Write a clear, pedagogically useful explanation of the sentence in {}.",
            ctx.learner_ui_language.to_name()
        );
        if !ctx.linguistic_background.is_empty() {
            s.push_str(
                " When genuinely helpful, use the learner's known languages as cognitive bridges.",
            );
        }
        s
    }

    /// Reject an explanation with nothing in it.
    ///
    /// Only emptiness is worth failing on: it is the one outcome a retry can
    /// plausibly fix. Malformed inline markup is deliberately *not* rejected —
    /// [`crate::explanation::parse_markup`] degrades a bad span to plain text,
    /// so failing the whole extraction over one would trade a cosmetic blemish
    /// for a missing explanation.
    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let explanation: StructuredExplanation =
            serde_json::from_value(section.clone()).map_err(|error| {
                format!("pedagogical explanation does not match the schema: {error}")
            })?;

        if explanation.is_empty() {
            return Err("pedagogical explanation is empty".to_string());
        }
        Ok(())
    }
}

/// The inline markup contract, stated once for the whole component.
fn markup_instructions() -> String {
    let kinds = MarkupKind::ALL
        .iter()
        .map(|kind| format!("`{kind}`"))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "PEDAGOGICAL EXPLANATION\n\
         Write everything in the learner's interface language.\n\n\
         Every string field below may carry inline markup written `[text](kind)`, \
         where kind is one of: {kinds}.\n\
         - `verb`, `noun`, `grammar` mark the grammatical role of a word in your prose \
         (`grammar` is for terms and rule names: cases, tenses, agreements).\n\
         - `target` marks a word quoted in the language being learned, as opposed to \
         the interface language. Always use it when quoting a form of the studied sentence.\n\
         - `morpheme` marks an affix or ending shown on its own, such as [-ego](morpheme).\n\
         - `italic` and `bold` are plain emphasis; prefer a semantic kind when one fits.\n\n\
         Mark what carries the lesson, not every word. Do not nest markup. \
         Square brackets that do not form `[text](kind)` with a kind from that list \
         are read as ordinary text, so write freely otherwise.\n\
         Emit NO HTML and NO markdown: `<b>`, `**bold**` and `<span>` are shown to the \
         learner verbatim and look like a bug."
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::MorphologyAnalysis;
    use serde_json::json;

    /// A language stub: this component's output does not depend on one.
    fn schema() -> serde_json::Value {
        let component = PedagogicalExplanation;
        AnalysisComponent::<crate::test_support::StubLanguage>::schema_fragment(
            &component,
            &crate::test_support::StubLanguage,
        )
    }

    fn validate(section: serde_json::Value) -> Result<(), String> {
        AnalysisComponent::<crate::test_support::StubLanguage>::validate(
            &PedagogicalExplanation,
            &crate::test_support::StubLanguage,
            &section,
        )
    }

    #[test]
    fn output_version_is_two() {
        assert_eq!(
            AnalysisComponent::<crate::test_support::StubLanguage>::output_version(
                &PedagogicalExplanation
            ),
            2,
            "the structured shape is a different stored shape from the v1 HTML blob"
        );
    }

    /// The schema is what teaches the model the markup contract, so every kind
    /// the parser accepts has to appear in it. A kind added to the enum and
    /// forgotten here would simply never be produced.
    #[test]
    fn schema_documents_every_markup_kind() {
        let description = schema()["description"]
            .as_str()
            .expect("schema carries a description")
            .to_string();

        for kind in MarkupKind::ALL {
            assert!(
                description.contains(kind.as_str()),
                "markup kind `{kind}` is missing from the schema description"
            );
        }
    }

    #[test]
    fn schema_requires_a_non_empty_analysis() {
        let schema = schema();
        assert_eq!(schema["required"], json!(["analysis"]));
        assert_eq!(schema["properties"]["analysis"]["minItems"], 1);
    }

    #[test]
    fn a_populated_explanation_validates() {
        assert!(
            validate(json!({
                "analysis": ["[Czytamy](verb) is first person plural."],
                "grammar_recap": { "title": "Accusative", "rules": ["Nouns take [-a](morpheme)"] }
            }))
            .is_ok()
        );
    }

    #[test]
    fn a_recap_free_explanation_validates() {
        assert!(validate(json!({ "analysis": ["A point."] })).is_ok());
        assert!(
            validate(json!({ "analysis": ["A point."], "grammar_recap": null })).is_ok(),
            "null is how the model says there is no recap"
        );
    }

    /// Strict structured-output modes force every property into `required`, so
    /// a schema cannot ask for a field to be left out. `grammar_recap` has to
    /// be nullable instead, or the model is obliged to invent a recap for every
    /// sentence that exercises no rule.
    #[test]
    fn the_recap_is_nullable_rather_than_optional() {
        assert_eq!(
            schema()["properties"]["grammar_recap"]["type"],
            json!(["object", "null"])
        );
        assert!(
            schema()["description"]
                .as_str()
                .expect("description")
                .contains("null")
                || schema()["properties"]["grammar_recap"]["description"]
                    .as_str()
                    .expect("recap description")
                    .contains("null"),
            "the instruction must tell the model to send null, not to omit"
        );
    }

    #[test]
    fn an_empty_explanation_is_rejected() {
        assert!(validate(json!({ "analysis": [] })).is_err());
        assert!(validate(json!({ "analysis": ["   "] })).is_err());
    }

    /// Malformed markup must survive validation: the parser degrades it to
    /// plain text, and failing here would cost the learner the explanation.
    #[test]
    fn malformed_markup_does_not_fail_validation() {
        assert!(
            validate(json!({ "analysis": ["A typo [Czytamy](verrb) and an [unclosed span"] }))
                .is_ok()
        );
    }

    #[test]
    fn a_v1_html_string_no_longer_validates() {
        assert!(
            validate(json!("<p>Analysis</p>")).is_err(),
            "the v1 shape is a different version, not something v2 should accept"
        );
    }

    /// Guards against the component pair that motivated versioning the stored
    /// shape: two components under one schema key must not disagree about it.
    #[test]
    fn schema_key_is_unchanged_from_version_one() {
        assert_eq!(
            AnalysisComponent::<crate::test_support::StubLanguage>::schema_key(
                &PedagogicalExplanation
            ),
            "pedagogical_explanation"
        );
        assert_eq!(
            AnalysisComponent::<crate::test_support::StubLanguage>::schema_key(&MorphologyAnalysis),
            "morphology",
            "sanity: distinct components own distinct keys"
        );
    }
}
