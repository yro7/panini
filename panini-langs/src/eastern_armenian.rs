use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature, Upos,
};

/// Syntactic case in Standard Eastern Armenian.
///
/// The seven functions are kept distinct even though nominative–accusative and
/// genitive–dative are frequently syncretic on the surface. The extractor must
/// resolve the case from syntax, not just from the ending.
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
pub enum EasternArmenianCase {
    Nominative,
    Accusative,
    Genitive,
    Dative,
    Ablative,
    Instrumental,
    Locative,
}

/// Whether a nominal expression carries the Armenian definite article.
///
/// Definiteness is normally expressed by the enclitic `-ը` / `-ն`; possessive
/// suffixes also make the noun phrase definite.
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
pub enum EasternArmenianDefiniteness {
    Indefinite,
    Definite,
}

/// The grammatical human/non-human contrast used by nominal morphology.
///
/// It matters especially for differential object marking: human direct objects
/// normally use a dative-shaped form, while non-human direct objects normally
/// use a nominative-shaped form.
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
pub enum EasternArmenianAnimacy {
    Human,
    NonHuman,
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
pub enum EasternArmenianDegree {
    Positive,
    Comparative,
    Superlative,
    AbsoluteSuperlative,
}

/// Morphological form occupied by a verb token.
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
pub enum EasternArmenianVerbForm {
    Finite,
    Infinitive,
    Participle,
    Converb,
}

/// The seven participial forms of Standard Eastern Armenian.
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
pub enum EasternArmenianParticipleType {
    Resultative,
    Subject,
    Imperfective,
    Future,
    FutureAdjectival,
    Perfect,
    Processual,
}

/// Mood of a finite verb or auxiliary.
///
/// Necessitative constructions are periphrastic (`պետք է`, `պիտի`) but are a
/// first-class cell of the Eastern Armenian verbal system.
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
pub enum EasternArmenianMood {
    Indicative,
    Imperative,
    Subjunctive,
    Conditional,
    Necessitative,
}

/// Tense carried by a finite verb token.
///
/// Compound constructions are split into their participle and auxiliary, so
/// perfect and prospective meanings belong to the participle type plus the
/// auxiliary tense rather than to extra synthetic tense values here.
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
pub enum EasternArmenianTense {
    Present,
    Imperfect,
    Past,
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
pub enum EasternArmenianVoice {
    Active,
    Passive,
    Causative,
    Middle,
    Reciprocal,
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
pub enum EasternArmenianPolarity {
    Affirmative,
    Negative,
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
pub enum EasternArmenianMorphology {
    /// Attributive adjectives are normally indeclinable in Modern Eastern Armenian.
    Adjective {
        lemma: String,
        degree: EasternArmenianDegree,
    },
    /// Armenian has both prepositions and postpositions; `case` is the case
    /// governed by this occurrence.
    Adposition {
        lemma: String,
        case: EasternArmenianCase,
    },
    Adverb {
        lemma: String,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        animacy: EasternArmenianAnimacy,
        number: BinaryNumber,
        case: EasternArmenianCase,
        definiteness: EasternArmenianDefiniteness,
        /// Person encoded by a possessive suffix such as `-ս` or `-դ`.
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_person: Option<Person>,
        /// Number of the possessor when the suffix distinguishes it.
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_number: Option<BinaryNumber>,
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
        number: Option<BinaryNumber>,
        case: EasternArmenianCase,
    },
    ProperNoun {
        lemma: String,
        animacy: EasternArmenianAnimacy,
        number: BinaryNumber,
        case: EasternArmenianCase,
        definiteness: EasternArmenianDefiniteness,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_number: Option<BinaryNumber>,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    /// Finite forms, infinitives, participles and converbs.
    Verb {
        lemma: String,
        verb_form: EasternArmenianVerbForm,
        voice: EasternArmenianVoice,
        polarity: EasternArmenianPolarity,
        /// Finite forms and necessitative auxiliaries only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<EasternArmenianMood>,
        /// Finite forms only; compound constructions put tense on the auxiliary.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<EasternArmenianTense>,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms, and nominalized non-finite forms when number is marked.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Present only when `verb_form` is `participle`.
        #[serde(skip_serializing_if = "Option::is_none")]
        participle_type: Option<EasternArmenianParticipleType>,
        /// Infinitives and participles can be nominalized and declined.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<EasternArmenianCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        definiteness: Option<EasternArmenianDefiniteness>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_number: Option<BinaryNumber>,
    },
    Other {
        lemma: String,
    },
}

impl EasternArmenianMorphology {
    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_participle_type(&self) -> Option<String> {
        match self {
            Self::Verb {
                participle_type, ..
            } => participle_type
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <EasternArmenianMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <EasternArmenianTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    pub const PIVOT_PARTICIPLE_TYPE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "participle_type",
            "Participle Type",
            <EasternArmenianParticipleType as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_participle_type,
        );
}

pub struct EasternArmenian;

impl LinguisticDefinition for EasternArmenian {
    type Morphology = EasternArmenianMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Hye;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        EasternArmenianMorphology::PIVOT_CASE,
        EasternArmenianMorphology::PIVOT_DEFINITENESS,
        EasternArmenianMorphology::PIVOT_ANIMACY,
        EasternArmenianMorphology::PIVOT_NUMBER,
        EasternArmenianMorphology::PIVOT_DEGREE,
        EasternArmenianMorphology::PIVOT_MOOD,
        EasternArmenianMorphology::PIVOT_TENSE,
        EasternArmenianMorphology::PIVOT_VERB_FORM,
        EasternArmenianMorphology::PIVOT_PARTICIPLE_TYPE,
        EasternArmenianMorphology::PIVOT_VOICE,
        EasternArmenianMorphology::PIVOT_POLARITY,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::ARMN]
    }

    fn default_script(&self) -> Script {
        Script::ARMN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            TypologicalFeature::Conjugation(&[Upos::Verb]),
            TypologicalFeature::Declension(&[Upos::Noun, Upos::ProperNoun, Upos::Pronoun]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope and lemmatization: analyze contemporary Standard Eastern Armenian of the Republic of Armenia. Use reformed orthography. Lemmatize common and proper nouns to the nominative singular indefinite form, verbs to the -ել or -ալ infinitive, adjectives to the positive form, and pronouns to their nominative citation form. The copular auxiliary has lemma 'եմ'; the lexical verb 'to be/become' has lemma 'լինել'.\n\
         2. Never introduce grammatical gender: Eastern Armenian has none. For nouns and proper nouns always report human/non-human animacy, number, syntactic case and definiteness. The article -ը/-ն and possessive suffixes are attached to the host; do not emit them as determiner tokens. A possessive suffix makes the nominal definite and supplies possessor_person and, when distinguishable, possessor_number.\n\
         3. Case is SYNTACTIC despite surface syncretism. A non-human direct object is normally nominative-shaped and a human direct object normally dative-shaped, but both are accusative in function and must be reported as accusative. Genitive and dative are often identical in form; decide from syntax. Report locative only for the productive -ում case, not merely for any location phrase.\n\
         4. Adpositions: Armenian has prepositions and postpositions. Report the case governed in this occurrence. Keep a postposition such as 'համար', 'հետ', 'մեջ' or 'վրա' as its own token; do not merge it with its complement.\n\
         5. Verbs: every verb gets verb_form, voice and polarity. Finite forms get mood, tense, person and number. Omit mood, tense and person on infinitives, participles and converbs. Add case, definiteness, possessor fields or nominal number only when a non-finite form is actually used and marked as a nominal.\n\
         6. Split every analytic construction into lexical tokens. In 'գրում եմ', analyze 'գրում' as an imperfective participle of 'գրել' and 'եմ' as a finite present indicative verb. Apply the same split to perfect, resultative, prospective and secondary compound constructions; do not assign the whole phrase as a synthetic tense to each token.\n\
         7. Finite tense values are morphological: present, imperfect and past (the synthetic aorist/simple past). The կ- series is conditional mood with present or imperfect morphology even when its contextual translation is future or would. Necessitative պետք/պիտի constructions use mood necessitative; imperatives have no tense.\n\
         8. Participle types: imperfective -ում; future -ու/-ելու; perfect -ել in an analytic perfect; resultative -ած; subject -ող; future_adjectival -իք/-ելիք; processual -իս. Set participle_type only when verb_form is participle. Treat adverbial forms such as -ելով/-ելիս used as clause modifiers as converbs.\n\
         9. Voice values are active, passive, causative, middle and reciprocal. Do not label every -վ- verb passive: many are lexical middle/intransitive or reciprocal. Polarity follows the construction, including a lexical verb governed by a negative auxiliary.\n\
         10. Adjectives do not agree with an attributive noun in case or number. If an adjective is substantivized and bears nominal morphology, analyze that occurrence as a noun. Use absolute_superlative for գեր- formations and superlative for ամենա- or equivalent ordinary superlatives.\n\
         11. Tokenization and writing: keep Armenian question, exclamation and emphasis marks attached to their lexical host while analyzing the word without the punctuation mark; never emit punctuation as a morphology token. Preserve reformed spellings such as 'Երևան' and the letter 'և'; never normalize them to traditional Western/Iranian spellings."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armenian_script_and_iso_are_exact() {
        let language = EasternArmenian;
        assert_eq!(EasternArmenian::ISO_LANG, IsoLang::Hye);
        assert_eq!(language.supported_scripts(), &[Script::ARMN]);
        assert_eq!(language.default_script(), Script::ARMN);
    }

    #[test]
    fn optional_participle_type_remains_a_closed_pivot() {
        let form = EasternArmenianMorphology::Verb {
            lemma: "գրել".to_string(),
            verb_form: EasternArmenianVerbForm::Participle,
            voice: EasternArmenianVoice::Active,
            polarity: EasternArmenianPolarity::Affirmative,
            mood: None,
            tense: None,
            person: None,
            number: None,
            participle_type: Some(EasternArmenianParticipleType::Imperfective),
            case: None,
            definiteness: None,
            possessor_person: None,
            possessor_number: None,
        };

        assert_eq!(
            EasternArmenianMorphology::PIVOT_PARTICIPLE_TYPE.value(&form),
            Some("imperfective".to_string())
        );
    }

    #[test]
    fn learner_facing_number_and_degree_are_exposed_as_pivots() {
        let pivot_keys = EasternArmenian::MORPHOLOGY_PIVOTS
            .iter()
            .map(|pivot| pivot.key)
            .collect::<Vec<_>>();

        assert!(pivot_keys.contains(&"number"));
        assert!(pivot_keys.contains(&"degree"));
    }
}
