use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, LinguisticDefinition, Person, Script, TypologicalFeature,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum DanishGender {
    Common, // Fælleskøn (n-ord)
    Neuter, // Intetkøn (t-ord)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum DanishDefiniteness {
    Indefinite, // Ubestemt
    Definite,   // Bestemt
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum DanishCase {
    Nominative, // Nominativ
    Genitive,   // Genitiv (-s)
    Objective,  // Akkusativ/Dativ (mostly for pronouns)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum DanishTense {
    Present, // Nutid (præsens)
    Past,    // Datid (præteritum)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum DanishMood {
    Indicative, // Fremsættende måde
    Imperative, // Bydemåde
    Infinitive, // Navnemåde
    Participle, // Tillægsform
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum DanishVoice {
    Active,  // Aktiv
    Passive, // Passiv (s-passiv or auxiliary)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum DanishDegree {
    Positive,    // Positiv
    Comparative, // Komparativ
    Superlative, // Superlativ
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::MorphologyInfo)]
#[serde(tag = "pos")]
#[serde(rename_all = "snake_case")]
pub enum DanishMorphology {
    Adjective {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<DanishGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        definiteness: Option<DanishDefiniteness>,
        degree: DanishDegree,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
    },
    Auxiliary {
        lemma: String,
        tense: Option<DanishTense>,
        mood: DanishMood,
        voice: DanishVoice,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<DanishGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        gender: DanishGender,
        number: BinaryNumber,
        definiteness: DanishDefiniteness,
        case: DanishCase,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<DanishGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: DanishCase,
    },
    ProperNoun {
        lemma: String,
        case: DanishCase,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<DanishTense>,
        mood: DanishMood,
        voice: DanishVoice,
    },
    Other {
        lemma: String,
    },
}

pub struct Danish;

impl LinguisticDefinition for Danish {
    type Morphology = DanishMorphology;
    type GrammaticalFunction = ();

    const ISO_CODE: &'static str = "dan";

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
        "1. Lemmatization: Nouns should be in singular indefinite form. Verbs should be in the infinitive. Adjectives should be in the masculine/common singular positive form.\n\
         2. Nouns: Specify gender (common/neuter), number (singular/plural), definiteness (indefinite/definite), and case (usually nominative unless ending in -s for genitive).\n\
         3. Verbs: Identify tense (present/past) for finite forms. Distinguish between active and passive (s-passive) forms. Identify mood (indicative, imperative, infinitive, participle).\n\
         4. Adjectives: Specify degree (positive, comparative, superlative). Note agreement in gender, number, and definiteness where applicable (e.g., 'stort' vs 'stor').\n\
         5. Pronouns: Identify person, number, and case (subjective 'jeg' vs objective 'mig').\n\
         6. Definiteness: Distinguish between the suffixed definite article (e.g., 'manden') and the standalone definite determiner used with adjectives (e.g., 'den gamle mand')."
    }
}