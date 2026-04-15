use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, LinguisticDefinition, Person, Script, TypologicalFeature,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum ItalianTense {
    Present,
    Past,
    Future,
    Imperfect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum ItalianMood {
    Indicative,
    Subjunctive,
    Conditional,
    Imperative,
    Infinitive,
    Gerund,
    Participle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum ItalianPronounType {
    Personal,
    Possessive,
    Demonstrative,
    Relative,
    Interrogative,
    Indefinite,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    panini_macro::MorphologyInfo,
)]
#[serde(tag = "pos")]
#[serde(rename_all = "snake_case")]
pub enum ItalianMorphology {
    /// An adjective (ADJ).
    Adjective {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    /// An adposition (ADP).
    Adposition { lemma: String },
    /// An adverb (ADV).
    Adverb { lemma: String },
    /// An auxiliary (AUX).
    Auxiliary {
        lemma: String,
        tense: ItalianTense,
        mood: ItalianMood,
        person: Person,
        number: BinaryNumber,
    },
    /// A coordinating conjunction (CCONJ).
    CoordinatingConjunction { lemma: String },
    /// A determiner (DET).
    Determiner {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    /// An interjection (INTJ).
    Interjection { lemma: String },
    /// A noun (NOUN).
    Noun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    /// A numeral (NUM).
    Numeral { lemma: String },
    /// A particle (PART).
    Particle { lemma: String },
    /// A pronoun (PRON).
    Pronoun {
        lemma: String,
        pronoun_type: ItalianPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        clitic: bool,
    },
    /// A proper noun (PROPN).
    ProperNoun { lemma: String },
    /// Punctuation (PUNCT).
    Punctuation { lemma: String },
    /// A subordinating conjunction (SCONJ).
    SubordinatingConjunction { lemma: String },
    /// A symbol (SYM).
    Symbol { lemma: String },
    /// A verb (VERB).
    Verb {
        lemma: String,
        tense: ItalianTense,
        mood: ItalianMood,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
    },
    /// Other (X) for unanalyzable tokens.
    Other { lemma: String },
}

pub struct Italian;

impl LinguisticDefinition for Italian {
    type Morphology = ItalianMorphology;
    type GrammaticalFunction = ();

    const ISO_CODE: &'static str = "ita";

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[TypologicalFeature::Conjugation]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Identify the dictionary form (lemma) for all words (e.g., masculine singular for adjectives, infinitive for verbs).\n\
         2. For Nouns, Adjectives, and Determiners: specify Gender (masculine/feminine) and Number (singular/plural).\n\
         3. For Verbs: specify Tense, Mood, Person, and Number. For past participles (Participio Passato), also include Gender and Number if they agree with the subject or object.\n\
         4. For Pronouns: classify the type (personal, possessive, etc.) and note if it is a clitic (e.g., 'lo', 'la', 'ne', 'ci').\n\
         5. Preposizioni articolate: split articulated prepositions (e.g., 'della') into the preposition lemma ('di') and the determiner features ('la' -> feminine singular)."
    }
}
