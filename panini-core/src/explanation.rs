//! The structured pedagogical explanation, and the inline markup its text carries.
//!
//! Version 1 of this component emitted an HTML blob. That put presentation in
//! the payload, forced every client to sanitise untrusted markup before it could
//! render anything, and spent most of its output tokens on tags. Version 2 emits
//! structure — an analysis list and an optional grammar recap — and keeps the
//! one thing the model is genuinely better placed to decide than the client:
//! which words in its own prose are verbs, nouns, grammar terms, or target-
//! language forms.
//!
//! # The inline markup
//!
//! Marked spans are written `[text](kind)`:
//!
//! ```text
//! [Czytamy](verb) is first person plural, and [ciekawy](noun) takes the [accusative](grammar).
//! ```
//!
//! A span is only a span when `kind` is one this build knows. `[see note](1)`
//! and `[a bracket]` are ordinary text, which is why there is no escape
//! character to specify, to teach the model, or to get wrong: the delimiters are
//! only special in the one arrangement that means something.
//!
//! # Parsing never fails
//!
//! [`parse_markup`] is total. Anything it does not recognise comes back as
//! literal text, so a malformed span costs its styling and nothing else. That is
//! the right trade for text a learner is mid-study with: rendering
//! `[Czytamy](verrb)` unstyled is a blemish, while failing the card over it, or
//! dropping the sentence, is a broken card.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A pedagogical explanation of one sentence, as version 2 stores it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct StructuredExplanation {
    /// The analysis, one entry per point. Each carries inline markup.
    pub analysis: Vec<String>,
    /// A summary of the rules the sentence exercises. Absent when the sentence
    /// exercises nothing worth restating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grammar_recap: Option<GrammarRecap>,
}

impl StructuredExplanation {
    /// The explanation as plain text, markup removed.
    ///
    /// For consumers that cannot render spans — the study agent's prompt, logs,
    /// search indexing.
    #[must_use]
    pub fn to_plain_text(&self) -> String {
        let mut lines: Vec<String> = self.analysis.iter().map(|line| plain_text(line)).collect();

        if let Some(recap) = &self.grammar_recap {
            lines.push(plain_text(&recap.title));
            lines.extend(recap.rules.iter().map(|rule| plain_text(rule)));
        }
        lines.join("\n")
    }

    /// Whether this explanation carries nothing worth rendering.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.analysis.iter().all(|line| line.trim().is_empty())
            && self
                .grammar_recap
                .as_ref()
                .is_none_or(|recap| recap.rules.is_empty() && recap.title.trim().is_empty())
    }
}

/// A summary of the declensions, conjugations or rules a sentence exercises.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GrammarRecap {
    /// What the recap is about, e.g. "Accusative (Biernik)".
    pub title: String,
    /// One entry per rule. Each carries inline markup.
    pub rules: Vec<String>,
}

/// What a marked span means.
///
/// Semantic rather than presentational: the payload says a word is a verb, and
/// the client decides what a verb looks like. `Italic` and `Bold` are the
/// exception, and exist because emphasis in running prose is genuinely
/// typographic — there is no domain fact underneath it to name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkupKind {
    Verb,
    Noun,
    /// A grammatical term or rule name: a case, a tense, an agreement.
    Grammar,
    /// A word or phrase quoted in the language being learned, as opposed to the
    /// learner's interface language. Clients use it to pick the right font and
    /// text direction, and to mark the language for screen readers — without it
    /// an Arabic form quoted inside French prose renders wrong.
    Target,
    /// An affix or ending shown on its own: `-a`, `-ego`, `-ler`.
    Morpheme,
    Italic,
    Bold,
}

impl MarkupKind {
    /// Every kind, in the order the prompt lists them.
    pub const ALL: &'static [Self] = &[
        Self::Verb,
        Self::Noun,
        Self::Grammar,
        Self::Target,
        Self::Morpheme,
        Self::Italic,
        Self::Bold,
    ];

    /// The token used inside `(…)`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verb => "verb",
            Self::Noun => "noun",
            Self::Grammar => "grammar",
            Self::Target => "target",
            Self::Morpheme => "morpheme",
            Self::Italic => "italic",
            Self::Bold => "bold",
        }
    }

    /// Parse a kind token, or `None` when it names nothing this build knows.
    #[must_use]
    pub fn parse(token: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|kind| kind.as_str() == token)
    }
}

impl fmt::Display for MarkupKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One stretch of explanation text, marked or plain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkupRun {
    pub text: String,
    /// `None` for unmarked prose.
    pub kind: Option<MarkupKind>,
}

/// Split marked-up text into runs.
///
/// Total: unrecognised markup comes back as literal text rather than an error.
/// Adjacent plain stretches are merged, so a caller never sees an empty run and
/// never sees two plain runs in a row.
#[must_use]
pub fn parse_markup(input: &str) -> Vec<MarkupRun> {
    let mut runs: Vec<MarkupRun> = Vec::new();
    let mut plain = String::new();
    let mut rest = input;

    while let Some(open) = rest.find('[') {
        match parse_span(&rest[open..]) {
            Some((text, kind, consumed)) => {
                plain.push_str(&rest[..open]);
                if !plain.is_empty() {
                    runs.push(MarkupRun {
                        text: std::mem::take(&mut plain),
                        kind: None,
                    });
                }
                runs.push(MarkupRun {
                    text: text.to_string(),
                    kind: Some(kind),
                });
                rest = &rest[open + consumed..];
            }
            None => {
                // Not a span. Keep the bracket as text and carry on looking
                // after it, so `[a] and [b](verb)` still finds the second one.
                let next = open + '['.len_utf8();
                plain.push_str(&rest[..next]);
                rest = &rest[next..];
            }
        }
    }

    plain.push_str(rest);
    if !plain.is_empty() {
        runs.push(MarkupRun {
            text: plain,
            kind: None,
        });
    }
    runs
}

/// Strip markup, keeping the marked text itself.
#[must_use]
pub fn plain_text(input: &str) -> String {
    parse_markup(input)
        .into_iter()
        .map(|run| run.text)
        .collect()
}

/// Try to read `[text](kind)` at the start of `candidate`, which begins with `[`.
///
/// Returns the span's text, its kind, and how many bytes it occupied.
fn parse_span(candidate: &str) -> Option<(&str, MarkupKind, usize)> {
    let after_open = &candidate['['.len_utf8()..];

    // A span's text holds no brackets of its own: nesting would need an
    // escaping story, and none of the seven kinds compose meaningfully anyway.
    let close = after_open.find(']')?;
    if after_open[..close].contains('[') {
        return None;
    }
    let text = &after_open[..close];

    let after_close = &after_open[close + ']'.len_utf8()..];
    let kind_body = after_close.strip_prefix('(')?;
    let kind_end = kind_body.find(')')?;
    let kind = MarkupKind::parse(&kind_body[..kind_end])?;

    let consumed = candidate.len() - kind_body[kind_end + ')'.len_utf8()..].len();
    Some((text, kind, consumed))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn marked(text: &str, kind: MarkupKind) -> MarkupRun {
        MarkupRun {
            text: text.to_string(),
            kind: Some(kind),
        }
    }

    fn plain(text: &str) -> MarkupRun {
        MarkupRun {
            text: text.to_string(),
            kind: None,
        }
    }

    #[test]
    fn parses_a_marked_span_between_plain_text() {
        assert_eq!(
            parse_markup("Here [Czytamy](verb) appears."),
            vec![
                plain("Here "),
                marked("Czytamy", MarkupKind::Verb),
                plain(" appears."),
            ]
        );
    }

    #[test]
    fn parses_several_spans_of_different_kinds() {
        assert_eq!(
            parse_markup(
                "[ciekawy](noun) takes the [accusative](grammar), ending in [-ego](morpheme)"
            ),
            vec![
                marked("ciekawy", MarkupKind::Noun),
                plain(" takes the "),
                marked("accusative", MarkupKind::Grammar),
                plain(", ending in "),
                marked("-ego", MarkupKind::Morpheme),
            ]
        );
    }

    #[test]
    fn text_without_markup_is_one_run() {
        assert_eq!(
            parse_markup("Nothing marked here."),
            vec![plain("Nothing marked here.")]
        );
    }

    /// The reason there is no escape character: brackets only mean something in
    /// the one arrangement that names a known kind.
    #[test]
    fn brackets_that_name_no_kind_stay_literal() {
        for input in [
            "See [note 1] for details.",
            "Reference [see note](1) here.",
            "A typo [Czytamy](verrb) slipped in.",
            "An unclosed [span (verb) here.",
        ] {
            assert_eq!(
                parse_markup(input),
                vec![plain(input)],
                "{input} should survive as literal text"
            );
        }
    }

    /// A malformed span must not swallow the ones after it.
    #[test]
    fn a_literal_bracket_does_not_hide_a_later_span() {
        assert_eq!(
            parse_markup("[a] and [b](verb)"),
            vec![plain("[a] and "), marked("b", MarkupKind::Verb)]
        );
    }

    #[test]
    fn nested_brackets_are_not_a_span() {
        let input = "[outer [inner]](verb)";
        assert_eq!(
            plain_text(input),
            input,
            "nesting has no meaning, so it stays literal"
        );
    }

    /// Every byte of the input reaches the output, whatever the markup does.
    /// Losing a learner's explanation text is worse than losing its styling.
    #[test]
    fn parsing_never_drops_text() {
        for input in [
            "Here [Czytamy](verb) appears.",
            "[a] and [b](verb)",
            "[unclosed",
            "]](( )[",
            "",
            "[](verb)",
            "[é](target) with [ünïcode](noun)",
        ] {
            let rebuilt: String = parse_markup(input)
                .iter()
                .map(|run| match run.kind {
                    Some(kind) => format!("[{}]({kind})", run.text),
                    None => run.text.clone(),
                })
                .collect();
            assert_eq!(rebuilt, input, "round-trip lost text for {input:?}");
        }
    }

    #[test]
    fn multibyte_text_is_not_split_mid_character() {
        assert_eq!(
            parse_markup("Arabic [كِتَاب](target) means book"),
            vec![
                plain("Arabic "),
                marked("كِتَاب", MarkupKind::Target),
                plain(" means book"),
            ]
        );
    }

    #[test]
    fn empty_input_yields_no_runs() {
        assert_eq!(parse_markup(""), vec![]);
    }

    #[test]
    fn plain_text_keeps_the_marked_words() {
        assert_eq!(
            plain_text("[Czytamy](verb) takes the [accusative](grammar)."),
            "Czytamy takes the accusative."
        );
    }

    #[test]
    fn every_kind_round_trips_through_its_token() {
        for kind in MarkupKind::ALL {
            assert_eq!(MarkupKind::parse(kind.as_str()), Some(*kind));
        }
    }

    #[test]
    fn explanation_renders_recap_into_plain_text() {
        let explanation = StructuredExplanation {
            analysis: vec!["[Czytamy](verb) is first person plural.".to_string()],
            grammar_recap: Some(GrammarRecap {
                title: "Accusative (Biernik)".to_string(),
                rules: vec!["Nouns take [-a](morpheme)".to_string()],
            }),
        };

        assert_eq!(
            explanation.to_plain_text(),
            "Czytamy is first person plural.\nAccusative (Biernik)\nNouns take -a"
        );
    }

    #[test]
    fn an_explanation_with_no_content_reports_empty() {
        assert!(
            StructuredExplanation {
                analysis: vec![],
                grammar_recap: None,
            }
            .is_empty()
        );
        assert!(
            StructuredExplanation {
                analysis: vec!["   ".to_string()],
                grammar_recap: None,
            }
            .is_empty()
        );
        assert!(
            !StructuredExplanation {
                analysis: vec!["Something".to_string()],
                grammar_recap: None,
            }
            .is_empty()
        );
    }

    #[test]
    fn recap_is_omitted_from_json_when_absent() {
        let value = serde_json::to_value(StructuredExplanation {
            analysis: vec!["point".to_string()],
            grammar_recap: None,
        })
        .expect("explanation serializes");

        assert!(
            value
                .as_object()
                .expect("object")
                .get("grammar_recap")
                .is_none(),
            "an absent recap costs no output tokens"
        );
    }
}
