//! Shared morphological enums for cross-language reuse.
//!
//! **Semantic bijection principle**: An enum belongs here only if its variants
//! carry the *same meaning* across every language that uses it. "Masculine" in
//! Arabic is the same concept as "Masculine" in Russian — that is a true
//! bijection. By contrast, tenses, moods, and cases are language-specific
//! systems with no universal mapping and therefore stay in each language module.

use panini_macro::ClosedValues;
use serde::{Deserialize, Serialize};

// Person

/// Grammatical person
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    ClosedValues,
)]
#[closed_values(crate = "crate")]
#[serde(rename_all = "snake_case")]
pub enum Person {
    First,
    Second,
    Third,
}

// Number

/// Binary grammatical number (singular / plural).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    ClosedValues,
)]
#[closed_values(crate = "crate")]
#[serde(rename_all = "snake_case")]
pub enum BinaryNumber {
    Singular,
    Plural,
}

/// Ternary grammatical number (singular / dual / plural).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    ClosedValues,
)]
#[closed_values(crate = "crate")]
#[serde(rename_all = "snake_case")]
pub enum TernaryNumber {
    Singular,
    Dual,
    Plural,
}

// Gender

/// Two-gender system (masculine / feminine).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    ClosedValues,
)]
#[closed_values(crate = "crate")]
#[serde(rename_all = "snake_case")]
pub enum BinaryGender {
    Masculine,
    Feminine,
}

/// Three-gender system (masculine / feminine / neuter).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    ClosedValues,
)]
#[closed_values(crate = "crate")]
#[serde(rename_all = "snake_case")]
pub enum TernaryGender {
    Masculine,
    Feminine,
    Neuter,
}

// Aspect

/// Slavic verbal aspect (perfective / imperfective).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    ClosedValues,
)]
#[closed_values(crate = "crate")]
#[serde(rename_all = "snake_case")]
pub enum SlavicAspect {
    Perfective,
    Imperfective,
}

// Voice

/// Binary voice (active / passive).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    ClosedValues,
)]
#[closed_values(crate = "crate")]
#[serde(rename_all = "snake_case")]
pub enum BinaryVoice {
    Active,
    Passive,
}

// Part of speech

/// Universal part-of-speech tag — the single typed PoS currency across the system.
///
/// The [Universal Dependencies UPOS](https://universaldependencies.org/u/pos/)
/// categories **except `AUX` and `PUNCT`**.
///
/// Auxiliaries are modelled as plain verbs: the AUX/VERB split is a UD flattening
/// that dilutes lexicon analysis, and an exercise needing auxiliaries specifically
/// asks the LLM for them.
///
/// Punctuation is not analysed at all. It carries no morphology, is never a
/// lexicon item, and never a target — analysing it only added tokens every
/// consumer had to filter back out. `MorphologyAnalysis` tells the model to omit
/// punctuation; one emitted anyway fails to deserialize exactly like any other
/// value outside the schema, and is not worth special-casing.
///
/// Plus `Classifier` for languages (e.g. Mandarin) whose morphology model exposes
/// measure words as a first-class part of speech.
///
/// Every `{Language}Morphology` enum's outer variants are named identically to
/// these, so `#[derive(MorphologyInfo)]` maps each variant to its `Upos` with a
/// pure `Self::X => Upos::X` arm (see `MorphologyInfo::pos`).
///
/// Serde uses the PascalCase variant name verbatim (no `rename_all`): this is the
/// wire contract for `part_of_speech` / `pos_filter` and matches `pos_label()`.
/// It is deliberately distinct from the `#[serde(tag = "pos", rename_all =
/// "snake_case")]` channel used *inside* morphology JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
pub enum Upos {
    /// Adjective (ADJ).
    Adjective,
    /// Adposition (ADP).
    Adposition,
    /// Adverb (ADV).
    Adverb,
    /// Coordinating conjunction (CCONJ).
    CoordinatingConjunction,
    /// Determiner (DET).
    Determiner,
    /// Interjection (INTJ).
    Interjection,
    /// Noun (NOUN).
    Noun,
    /// Numeral (NUM).
    Numeral,
    /// Particle (PART).
    Particle,
    /// Pronoun (PRON).
    Pronoun,
    /// Proper noun (PROPN).
    ProperNoun,
    /// Subordinating conjunction (SCONJ).
    SubordinatingConjunction,
    /// Symbol (SYM).
    Symbol,
    /// Verb (VERB).
    Verb,
    /// Other / unknown (UD tag X).
    Other,
    /// Measure word / classifier — not a UD UPOS tag; used by languages (Mandarin)
    /// that model classifiers as a distinct part of speech.
    Classifier,
}

impl Upos {
    /// Every `Upos` value, in declaration order.
    pub const ALL: [Upos; 16] = [
        Upos::Adjective,
        Upos::Adposition,
        Upos::Adverb,
        Upos::CoordinatingConjunction,
        Upos::Determiner,
        Upos::Interjection,
        Upos::Noun,
        Upos::Numeral,
        Upos::Particle,
        Upos::Pronoun,
        Upos::ProperNoun,
        Upos::SubordinatingConjunction,
        Upos::Symbol,
        Upos::Verb,
        Upos::Other,
        Upos::Classifier,
    ];

    /// The PascalCase label — identical to the enum variant name and to the
    /// legacy `pos_label()` string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Upos::Adjective => "Adjective",
            Upos::Adposition => "Adposition",
            Upos::Adverb => "Adverb",
            Upos::CoordinatingConjunction => "CoordinatingConjunction",
            Upos::Determiner => "Determiner",
            Upos::Interjection => "Interjection",
            Upos::Noun => "Noun",
            Upos::Numeral => "Numeral",
            Upos::Particle => "Particle",
            Upos::Pronoun => "Pronoun",
            Upos::ProperNoun => "ProperNoun",
            Upos::SubordinatingConjunction => "SubordinatingConjunction",
            Upos::Symbol => "Symbol",
            Upos::Verb => "Verb",
            Upos::Other => "Other",
            Upos::Classifier => "Classifier",
        }
    }
}

impl std::fmt::Display for Upos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error returned when a string is not a known [`Upos`] label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseUposError(pub String);

impl std::fmt::Display for ParseUposError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown part-of-speech label: {:?}", self.0)
    }
}

impl std::error::Error for ParseUposError {}

impl std::str::FromStr for Upos {
    type Err = ParseUposError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Upos::ALL
            .into_iter()
            .find(|u| u.as_str() == s)
            .ok_or_else(|| ParseUposError(s.to_owned()))
    }
}
