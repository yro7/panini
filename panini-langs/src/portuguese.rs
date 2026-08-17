use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature,
    Upos,
};

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
    panini_macro::ClosedValues,
)]
#[serde(rename_all = "snake_case")]
pub enum PortugueseTense {
    Present,
    Preterite,
    Imperfect,
    Pluperfect,
    Future,
    Conditional,
}

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
    panini_macro::ClosedValues,
)]
#[serde(rename_all = "snake_case")]
pub enum PortugueseMood {
    Indicative,
    Subjunctive,
    Imperative,
    Conditional,
    Infinitive,
    PersonalInfinitive,
    Gerund,
    Participle,
}

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
    panini_macro::ClosedValues,
)]
#[serde(rename_all = "snake_case")]
pub enum PortuguesePronounCase {
    Subject,
    DirectObject,
    IndirectObject,
    Reflexive,
    Prepositional,
}

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
    panini_macro::ClosedValues,
)]
#[serde(rename_all = "snake_case")]
pub enum PortuguesePronounType {
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
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    panini_macro::ClosedValues,
)]
#[serde(rename_all = "snake_case")]
pub enum PortugueseDeterminerType {
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
pub enum PortugueseMorphology {
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
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: PortugueseDeterminerType,
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
        pronoun_type: PortuguesePronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<PortuguesePronounCase>,
    },
    ProperNoun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        tense: PortugueseTense,
        mood: PortugueseMood,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
    },
    Other {
        lemma: String,
    },
}

pub struct Portuguese;

impl LinguisticDefinition for Portuguese {
    type Morphology = PortugueseMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Por;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        PortugueseMorphology::PIVOT_TENSE,
        PortugueseMorphology::PIVOT_MOOD,
        PortugueseMorphology::PIVOT_GENDER,
        PortugueseMorphology::PIVOT_NUMBER,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[TypologicalFeature::Conjugation(&[Upos::Verb])]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization: Provide the canonical dictionary citation form (masculine singular for nouns, adjectives, and determiners; infinitive for verbs).\n\
         2. Gender and Number: Always specify Gender (masculine/feminine) and Number (singular/plural) for Nouns, Adjectives, Determiners, and Proper Nouns.\n\
         3. Verbs: Specify Tense, Mood, Person, and Number for finite forms. European Portuguese fully distinguishes 2nd person singular ('tu') from 3rd person polite address ('você' / null subject). For the Personal Infinitive (infinitivo pessoal), tag mood as 'personal_infinitive' and include Person and Number. For past participles (particípio) agreeing with a subject/object in passive or adjectival constructions, specify Gender and Number.\n\
         4. Pronouns: Identify the pronoun type and specify the case (subject, direct_object, indirect_object, reflexive, prepositional) where applicable. Distinguish clitic objects (me, te, o, a, lhe, nos, vos, os, as, lhes) and contracted combinations (mo, to, lho).\n\
         5. Contractions: Split contracted prepositions (e.g., 'no' -> 'em' + 'o', 'da' -> 'de' + 'a', 'num' -> 'em' + 'um', 'deste' -> 'de' + 'este', 'pelo' -> 'por' + 'o', 'à' -> 'a' + 'a').\n\
         6. Clitic Pronouns: Separate hyphenated enclitic, proclitic, or mesoclitic pronouns (e.g., 'deu-me' -> 'dar' + 'me', 'viu-o' -> 'ver' + 'o', 'dar-lhe-ia' -> 'dar' + 'lhe')."
    }
}
