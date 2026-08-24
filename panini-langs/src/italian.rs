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
pub enum ItalianTense {
    Present,
    Past,
    Future,
    Imperfect,
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
pub enum ItalianMood {
    Indicative,
    Subjunctive,
    Conditional,
    Imperative,
    Infinitive,
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
    /// Adjective
    Adjective {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    /// Adposition
    Adposition { lemma: String },
    /// Adverb
    Adverb { lemma: String },
    /// Coordinating conjunction
    CoordinatingConjunction { lemma: String },
    /// Determiner
    Determiner {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    /// Interjection
    Interjection { lemma: String },
    /// Noun
    Noun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    /// Numeral
    Numeral { lemma: String },
    /// Particle
    Particle { lemma: String },
    /// Pronoun
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
    /// Proper noun
    ProperNoun { lemma: String },
    /// Subordinating conjunction
    SubordinatingConjunction { lemma: String },
    /// Symbol
    Symbol { lemma: String },
    /// Verb
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
    /// Other, for unanalyzable tokens
    Other { lemma: String },
}

pub struct Italian;

impl LinguisticDefinition for Italian {
    type Morphology = ItalianMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Ita;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        ItalianMorphology::PIVOT_TENSE,
        ItalianMorphology::PIVOT_MOOD,
        ItalianMorphology::PIVOT_GENDER,
        ItalianMorphology::PIVOT_NUMBER,
        ItalianMorphology::PIVOT_CLITIC,
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
        "1. Lemmatization: provide the dictionary form — infinitive for verbs, masculine singular for nouns, adjectives and determiners.\n\
         2. Gender and Number: always specify both for Nouns, Adjectives and Determiners. Gender is lexical and cannot be read off the ending: 'il problema', 'il poeta', 'il cinema' are masculine, 'la mano' and 'la radio' are feminine, and nouns in -e may be either.\n\
         3. Compound tenses: treat the auxiliary and the past participle as two separate verb tokens. 'ho parlato' is 'ho' (present, indicative) plus 'parlato' (past, participle), and the same applies to trapassato, futuro anteriore, congiuntivo passato and condizionale passato. Only the simple tenses — presente, imperfetto, passato remoto, futuro semplice — are a single verb token.\n\
         4. Verbs: always specify Tense and Mood. Specify Person and Number only for finite forms; omit both for infinitive, gerund and participle.\n\
         5. Past participles: add Gender and Number only when the form actually marks agreement — with the subject under 'essere' ('e andata', 'sono partiti') and with a preceding direct-object clitic under 'avere' ('l'ho vista', 'le ho comprate'). Omit them for the invariable form ('ho comprato le mele').\n\
         6. Pronouns: classify the type, and set clitic true for the unstressed forms ('mi', 'ti', 'lo', 'la', 'gli', 'le', 'ci', 'vi', 'ne', 'si') and false for the stressed ones ('io', 'me', 'lui', 'questo'). Specify Person, Gender and Number only where the form marks them: omit Person for non-personal pronouns, and omit Gender for forms that do not distinguish it ('mi', 'ti', 'ci', 'vi', 'ne', 'gli').\n\
         7. Clitic clusters: split an attached or combined clitic group into the verb and each clitic, each keeping its own lemma — dammelo -> 'dare' + 'me' + 'lo'; andarci -> 'andare' + 'ci'; parlandogli -> 'parlare' + 'gli'; glielo -> 'gli' + 'lo'.\n\
         8. Elisions: restore an elided word to its full form as its own token — l'ho visto -> 'lo' + 'ho' + 'visto'; un'amica -> 'una' + 'amica'; l'altra -> 'la' + 'altra'.\n\
         9. Preposizioni articolate: split articulated prepositions into the preposition lemma and the determiner (e.g., 'della' -> 'di' + 'la' as feminine singular, 'nei' -> 'in' + 'i' as masculine plural)."
    }
}
