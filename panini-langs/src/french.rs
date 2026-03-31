use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, BinaryVoice, IsoLang, LinguisticDefinition, MorphologyInfo, Person,
    Script, TypologicalFeature,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrenchTense {
    Present,
    Imperfect,
    Future,
    Conditional,
    SimplePast,
    Past,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrenchMood {
    Indicative,
    Subjunctive,
    Imperative,
    Conditional,
    Infinitive,
    Participle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrenchPronounCase {
    Subject,
    DirectObject,
    IndirectObject,
    Reflexive,
    Tonic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrenchPronounType {
    Personal,
    Possessive,
    Demonstrative,
    Relative,
    Interrogative,
    Indefinite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FrenchDeterminerType {
    Article,
    Possessive,
    Demonstrative,
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
pub enum FrenchMorphology {
    Adjective {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
    },
    Auxiliary {
        lemma: String,
        tense: FrenchTense,
        mood: FrenchMood,
        person: Person,
        number: BinaryNumber,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: FrenchDeterminerType,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    Numeral {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        pronoun_type: FrenchPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<FrenchPronounCase>,
    },
    ProperNoun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    Punctuation {
        lemma: String,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        tense: FrenchTense,
        mood: FrenchMood,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        voice: BinaryVoice,
    },
    Other {
        lemma: String,
    },
}

pub struct French;

impl LinguisticDefinition for French {
    type Morphology = FrenchMorphology;
    type GrammaticalFunction = ();

    fn iso_code(&self) -> IsoLang {
        IsoLang::Fra
    }

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[TypologicalFeature::Conjugation]
    }

    fn extraction_directives(&self) -> &str {
        "1. Lemmatization: Provide the dictionary form (masculine singular for adjectives/nouns, infinitive for verbs).\n\
         2. Gender and Number: Always specify for Nouns, Adjectives, and Determiners.\n\
         3. Verbs: Distinguish between simple tenses (Present, Imperfect, etc.). For compound tenses (e.g., Passé Composé), treat the auxiliary and the past participle as separate tokens.\n\
         4. Pronouns: Identify the type (personal, demonstrative, etc.) and specify the case (subject, tonic, clitic COD/COI) where applicable.\n\
         5. Clitics: Separate elided forms (e.g., 'l'amine' -> 'le' + 'amine', 'j'aime' -> 'je' + 'aime').\n\
         6. Contractions: Split contracted prepositions (e.g., 'au' -> 'à' + 'le', 'du' -> 'de' + 'le')."
    }
}
