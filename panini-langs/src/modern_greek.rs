use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryGender, TypologicalFeature,
    Upos,
};

/// The four productive cases of Standard Modern Greek.
///
/// The inherited dative survives only inside fixed learned expressions. It is
/// not a productive cell of the modern declension and therefore has no value
/// here; such frozen forms are analyzed according to their current lexical use.
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
pub enum GreekCase {
    Nominative,
    Genitive,
    Accusative,
    Vocative,
}

/// The aspect contrast expressed by the Modern Greek verb stem.
///
/// Perfect constructions are analytic (`έχω γράψει`), so "perfect" is not a
/// third token-level value: the auxiliary and invariant perfective form are
/// analyzed separately.
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
pub enum GreekAspect {
    Imperfective,
    Perfective,
}

/// Morphologically marked mood of a finite verb.
///
/// The traditional "subjunctive" is a construction headed by `να` or `ας`,
/// not a third inflectional mood. Its following verb retains indicative-form
/// morphology, while aspect supplies the contrast (`να γράφω` / `να γράψω`).
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
pub enum GreekMood {
    Indicative,
    Imperative,
}

/// Tense carried by the verb form itself.
///
/// Past imperfective and past perfective forms are distinguished by
/// [`GreekAspect`]. Future time is built with the separate particle `θα`; there
/// is no synthetic future value on the lexical verb.
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
pub enum GreekTense {
    Present,
    Past,
}

/// The morphological opposition between the two Modern Greek conjugations.
///
/// `Mediopassive` deliberately names form rather than semantic voice: the
/// `-μαι` conjugation includes passive, middle, reflexive and deponent verbs.
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
pub enum GreekVoice {
    Active,
    Mediopassive,
}

/// The four verb forms used in contemporary Standard Modern Greek.
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
pub enum GreekVerbForm {
    Finite,
    /// The invariant perfective form used only after `έχω`, as in `έχω γράψει`.
    PerfectDependent,
    /// An agreeing verbal participle, principally the `-μένος/-μένη/-μένο` type.
    Participle,
    /// The indeclinable adverbial form in `-οντας/-ώντας`.
    Converb,
}

/// Degrees distinguished in the contemporary adjective/adverb system.
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
pub enum GreekDegree {
    Positive,
    Comparative,
    /// Relative superlative, normally article plus comparative form.
    Superlative,
    /// Absolute superlative in `-ότατος/-ότατα` and related formations.
    AbsoluteSuperlative,
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
pub enum GreekDefiniteness {
    Definite,
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
pub enum GreekDeterminerType {
    Article,
    Possessive,
    Demonstrative,
    Interrogative,
    Relative,
    Indefinite,
    Quantifier,
    Emphatic,
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
pub enum GreekPronounType {
    Personal,
    Possessive,
    Demonstrative,
    Reflexive,
    Reciprocal,
    Interrogative,
    Relative,
    Indefinite,
    Emphatic,
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
pub enum ModernGreekMorphology {
    /// Adjectives agree with their head in gender, number and case.
    Adjective {
        lemma: String,
        gender: TernaryGender,
        number: BinaryNumber,
        case: GreekCase,
        /// Absent for adjectives to which comparison genuinely does not apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<GreekDegree>,
    },
    /// Preposition, with the case governed in this occurrence.
    Adposition {
        lemma: String,
        case: GreekCase,
    },
    Adverb {
        lemma: String,
        /// Absent for non-gradable adverbs such as temporal `σήμερα`.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<GreekDegree>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: GreekDeterminerType,
        /// Articles only; other determiner types do not encode definiteness.
        #[serde(skip_serializing_if = "Option::is_none")]
        definiteness: Option<GreekDefiniteness>,
        /// Invariant determiners such as `κάθε` omit all three agreement fields.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<GreekCase>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        gender: TernaryGender,
        number: BinaryNumber,
        case: GreekCase,
    },
    /// Numerals which inflect expose their marked agreement features; invariant
    /// cardinals do not acquire invented values merely from the noun they count.
    Numeral {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<GreekCase>,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        pronoun_type: GreekPronounType,
        /// Personal pronouns only.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Omit when the pronoun does not distinguish gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Omit for invariant pronouns which stand outside the number contrast.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: GreekCase,
        /// True only for an unstressed weak personal-pronoun form.
        clitic: bool,
    },
    ProperNoun {
        lemma: String,
        gender: TernaryGender,
        number: BinaryNumber,
        case: GreekCase,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    /// Finite forms, the invariant perfect form, participles and converbs.
    Verb {
        lemma: String,
        aspect: GreekAspect,
        /// Morphological conjugation, independent of semantic voice.
        voice: GreekVoice,
        verb_form: GreekVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<GreekMood>,
        /// Finite indicative forms only; imperatives and non-finite forms omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<GreekTense>,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms and agreeing participles only.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Agreeing participles only.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Agreeing participles only.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<GreekCase>,
    },
    Other {
        lemma: String,
    },
}

impl ModernGreekMorphology {
    fn __pivot_degree(&self) -> Option<String> {
        match self {
            Self::Adjective { degree, .. } | Self::Adverb { degree, .. } => degree
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_definiteness(&self) -> Option<String> {
        match self {
            Self::Determiner { definiteness, .. } => definiteness
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

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

    pub const PIVOT_DEGREE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "degree",
            "Degree",
            <GreekDegree as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_degree,
        );

    pub const PIVOT_DEFINITENESS: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "definiteness",
            "Definiteness",
            <GreekDefiniteness as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_definiteness,
        );

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <GreekMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <GreekTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );
}

pub struct ModernGreek;

impl LinguisticDefinition for ModernGreek {
    type Morphology = ModernGreekMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Ell;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        ModernGreekMorphology::PIVOT_CASE,
        ModernGreekMorphology::PIVOT_GENDER,
        ModernGreekMorphology::PIVOT_NUMBER,
        ModernGreekMorphology::PIVOT_DEGREE,
        ModernGreekMorphology::PIVOT_DEFINITENESS,
        ModernGreekMorphology::PIVOT_CLITIC,
        ModernGreekMorphology::PIVOT_ASPECT,
        ModernGreekMorphology::PIVOT_VOICE,
        ModernGreekMorphology::PIVOT_VERB_FORM,
        ModernGreekMorphology::PIVOT_MOOD,
        ModernGreekMorphology::PIVOT_TENSE,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::GREK]
    }

    fn default_script(&self) -> Script {
        Script::GREK
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            TypologicalFeature::Conjugation(&[Upos::Verb]),
            TypologicalFeature::Declension(&[
                Upos::Noun,
                Upos::ProperNoun,
                Upos::Adjective,
                Upos::Pronoun,
                Upos::Numeral,
                Upos::Determiner,
            ]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope, orthography and lemmas: analyze contemporary Standard Modern Greek in monotonic Greek script. Keep the written tonos and dialytika, normalize final sigma correctly, and never add polytonic breathings. Lemmatize nouns and proper nouns to nominative singular (keep a conventional plural lemma for pluralia tantum such as 'διακοπές'), adjectives to masculine nominative singular positive, and verbs to the dictionary first-person singular present ('έγραψα' -> 'γράφω', 'ήρθα' -> 'έρχομαι'). A genuine verbal participle keeps the parent verb lemma; a lexicalized participle used as an adjective gets its adjectival lemma.\n\
         2. Nominals: nouns and proper nouns always get lexical gender plus number and SYNTACTIC case, even when surface forms are syncretic or indeclinable. Adjectives always get agreement gender, number and case. Modern Greek has nominative, genitive, accusative and vocative only; do not invent dative for fixed learned expressions such as 'εν τω μεταξύ'.\n\
         3. Case is decided from syntax, not endings. 'η μητέρα' is nominative as a subject and 'τη μητέρα' accusative as an object; 'της μητέρας' is genitive; direct address 'μητέρα!' is vocative. Genitive weak pronouns mark ordinary Standard Greek indirect objects ('του έδωσα'), while preposition complements are normally accusative. For an adposition report the case it governs in this occurrence.\n\
         4. Degree is genuinely optional. Encode comparative 'καλύτερος' and contrast absolute_superlative 'ωραιότατος'; encode superlative for contextual 'ο καλύτερος'. Omit degree from a non-gradable relational adjective such as 'ιατρικός' and a non-gradable adverb such as 'σήμερα'. In periphrastic 'πιο καλός', keep 'πιο' as its own comparative adverb and analyze 'καλός' as positive; the definite article plus comparative supplies a relative superlative construction.\n\
         5. Determiners: classify their type. Articles encode definiteness: 'ο' is definite and 'ένας' is indefinite; a demonstrative such as 'αυτός' contrasts with both and MUST omit definiteness. Inflecting determiners encode agreement: 'ο' is masculine singular nominative and 'τις' is feminine plural accusative. The invariant quantifier 'κάθε' is the omission case: omit gender, number and case rather than copying them from its noun.\n\
         6. Numerals: add gender, number and case only where the numeral form itself participates in the contrast. 'μία' is feminine versus neuter 'ένα'; 'ένας' is singular versus plural 'χίλιοι'; 'ενός' is genitive versus accusative 'έναν'. Omit all three fields on invariant 'πέντε' — do not copy plural, gender or case from the counted noun.\n\
         7. Pronouns: classify the type, assign syntactic case, and set clitic true only on unstressed weak personal forms ('μου', 'σου', 'τον', 'τη', 'το', 'μας', 'σας', 'τους') — false on strong forms. Person: 'εγώ' is first versus 'εσύ' second; omit person from interrogative 'ποιος'. Gender: 'αυτός' is masculine versus 'αυτή' feminine; omit gender from 'εγώ'. Number: 'εγώ' is singular versus 'εμείς' plural; omit number from invariant relative 'ό,τι'. Lemmatize weak personal forms to the corresponding strong nominative citation form.\n\
         8. Every verb gets aspect, morphological voice and verb_form. Aspect is imperfective versus perfective: 'γράφω/έγραφα' contrasts with 'γράψω/έγραψα'. Voice is FORM, not meaning: -ω/-ώ paradigms are active and -μαι paradigms are mediopassive; deponents and middle/reflexive uses such as 'έρχομαι' and 'πλένομαι' remain mediopassive even though they are not semantically passive.\n\
         9. Finite forms encode mood, person and number. 'γράφω' is indicative first singular versus 'γράφεις' indicative second singular and 'γράφουμε' first plural; 'γράψε' contrasts as imperative second singular. Finite indicative forms encode present or past ('γράφω' present versus 'έγραψα' past), while imperative 'γράψε' MUST omit tense. Forms following 'να', 'ας' or 'θα' retain indicative-form mood; the particle creates the construction, not a synthetic subjunctive or future morphology on the verb.\n\
         10. Non-finite omission rules are strict. The invariant perfect-dependent form in 'έχω γράψει' contrasts with finite 'γράφει': tag 'γράψει' perfect_dependent, perfective, active and omit mood, tense, person, number, gender and case. It is not a productive infinitive and must never be generated or analyzed as a standalone complement. The converb 'γράφοντας' is imperfective active and omits those same six fields. An agreeing participle encodes number, gender and case: nominative masculine singular 'γραμμένος' contrasts with genitive feminine singular 'γραμμένης'; both omit mood, tense and person.\n\
         11. Split analytic constructions into lexical tokens. 'έχω γράψει' is finite 'έχω' plus perfect-dependent 'γράψει'; 'θα γράψω' is particle 'θα' plus finite perfective 'γράψω'; 'να γράφω' is particle 'να' plus finite imperfective 'γράφω'. The forms after 'θα' and 'να' remain finite and inflect for person and number — do not confuse them with the invariant perfect-dependent form. Never give every token the whole construction's tense or mood. Treat 'θα', 'να', 'ας', 'δεν' and 'μη(ν)' as particles, not verbs.\n\
         12. Tokenization: split fused σε + article forms into both underlying tokens ('στον' -> 'σε' + 'τον', 'στη' -> 'σε' + 'τη', 'στα' -> 'σε' + 'τα') and restore ordinary elisions ('απ\''' -> 'από'). Keep weak pronouns as separate tokens. Strip punctuation from analyzed tokens and never emit punctuation as morphology."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modern_greek_identity_and_script_are_exact() {
        let language = ModernGreek;
        assert_eq!(ModernGreek::ISO_LANG, IsoLang::Ell);
        assert_eq!(language.supported_scripts(), &[Script::GREK]);
        assert_eq!(language.default_script(), Script::GREK);
    }

    #[test]
    fn optional_learner_dimensions_remain_closed_pivots() {
        let adjective = ModernGreekMorphology::Adjective {
            lemma: "καλός".to_string(),
            gender: TernaryGender::Masculine,
            number: BinaryNumber::Singular,
            case: GreekCase::Nominative,
            degree: Some(GreekDegree::Comparative),
        };
        let article = ModernGreekMorphology::Determiner {
            lemma: "ο".to_string(),
            determiner_type: GreekDeterminerType::Article,
            definiteness: Some(GreekDefiniteness::Definite),
            gender: Some(TernaryGender::Masculine),
            number: Some(BinaryNumber::Singular),
            case: Some(GreekCase::Nominative),
        };

        assert_eq!(
            ModernGreekMorphology::PIVOT_DEGREE.value(&adjective),
            Some("comparative".to_string())
        );
        assert_eq!(
            ModernGreekMorphology::PIVOT_DEFINITENESS.value(&article),
            Some("definite".to_string())
        );
    }

    #[test]
    fn imperative_and_non_finite_forms_do_not_acquire_tense() {
        let imperative = ModernGreekMorphology::Verb {
            lemma: "γράφω".to_string(),
            aspect: GreekAspect::Perfective,
            voice: GreekVoice::Active,
            verb_form: GreekVerbForm::Finite,
            mood: Some(GreekMood::Imperative),
            tense: None,
            person: Some(Person::Second),
            number: Some(BinaryNumber::Singular),
            gender: None,
            case: None,
        };
        let perfect_dependent = ModernGreekMorphology::Verb {
            lemma: "γράφω".to_string(),
            aspect: GreekAspect::Perfective,
            voice: GreekVoice::Active,
            verb_form: GreekVerbForm::PerfectDependent,
            mood: None,
            tense: None,
            person: None,
            number: None,
            gender: None,
            case: None,
        };

        assert_eq!(
            ModernGreekMorphology::PIVOT_MOOD.value(&imperative),
            Some("imperative".to_string())
        );
        assert_eq!(ModernGreekMorphology::PIVOT_TENSE.value(&imperative), None);
        assert_eq!(
            ModernGreekMorphology::PIVOT_MOOD.value(&perfect_dependent),
            None
        );
        assert_eq!(
            ModernGreekMorphology::PIVOT_TENSE.value(&perfect_dependent),
            None
        );
    }

    #[test]
    fn cloze_features_cover_the_real_inflecting_pos() {
        let features = ModernGreek.typological_features();

        assert_eq!(
            features,
            &[
                TypologicalFeature::Conjugation(&[Upos::Verb]),
                TypologicalFeature::Declension(&[
                    Upos::Noun,
                    Upos::ProperNoun,
                    Upos::Adjective,
                    Upos::Pronoun,
                    Upos::Numeral,
                    Upos::Determiner,
                ]),
            ]
        );
    }
}
