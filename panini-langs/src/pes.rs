//! Iranian Persian. Morphological reference: Sagot & Walther, PerLex (LREC 2010),
//! https://pauillac.inria.fr/~sagot/pub/lrec10perlex.pdf; clitic tokenization:
//! https://universaldependencies.org/fa/. Persian has neither gender nor nominal case.

use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature, Upos,
};

#[panini_macro::closed_enum]
pub enum PersianDegree {
    Positive,
    Comparative,
    Superlative,
}

/// Token tense, not the tense of the entire auxiliary construction.
#[panini_macro::closed_enum]
pub enum PersianTense {
    Present,
    Past,
}

#[panini_macro::closed_enum]
pub enum PersianMood {
    Indicative,
    Subjunctive,
    Imperative,
}

#[panini_macro::closed_enum]
pub enum PersianAspect {
    Perfective,
    Imperfective,
}

#[panini_macro::closed_enum]
pub enum PersianVerbForm {
    Finite,
    Infinitive,
    Participle,
}

#[panini_macro::closed_enum]
pub enum PersianPolarity {
    Affirmative,
    Negative,
}

#[panini_macro::closed_enum]
pub enum PersianPronounType {
    Personal,
    Demonstrative,
    Interrogative,
    Reflexive,
    Relative,
    Indefinite,
    Reciprocal,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema,
    panini_macro::MorphologyInfo,
)]
#[serde(tag = "pos", rename_all = "snake_case")]
pub enum PersianMorphology {
    Adjective {
        lemma: String,
        degree: PersianDegree,
        ezafe: bool,
        /// The indefinite -i on this host, not inferred semantic indefiniteness.
        indefinite: bool,
    },
    Adposition {
        lemma: String,
        ezafe: bool,
    },
    Adverb { lemma: String },
    CoordinatingConjunction { lemma: String },
    Determiner { lemma: String },
    Interjection { lemma: String },
    Noun {
        lemma: String,
        number: BinaryNumber,
        ezafe: bool,
        indefinite: bool,
    },
    Numeral { lemma: String },
    Particle { lemma: String },
    Pronoun {
        lemma: String,
        pronoun_type: PersianPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    ProperNoun {
        lemma: String,
        number: BinaryNumber,
        ezafe: bool,
    },
    SubordinatingConjunction { lemma: String },
    Symbol { lemma: String },
    Verb {
        lemma: String,
        verb_form: PersianVerbForm,
        polarity: PersianPolarity,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<PersianMood>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<PersianTense>,
        /// mi- forms and the simple past contrast aspect; nonfinite forms and
        /// unmarked presents/subjunctives/imperatives do not encode that contrast.
        #[serde(skip_serializing_if = "Option::is_none")]
        aspect: Option<PersianAspect>,
    },
    Other { lemma: String },
}

impl PersianMorphology {
    fn pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense: Some(value), .. } => Some(
                panini_core::aggregable::ClosedValues::variant_str(value).to_string(),
            ),
            _ => None,
        }
    }

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense", "Tense",
            <PersianTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::pivot_tense,
        );

    fn pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood: Some(value), .. } => Some(
                panini_core::aggregable::ClosedValues::variant_str(value).to_string(),
            ),
            _ => None,
        }
    }

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood", "Mood",
            <PersianMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::pivot_mood,
        );

    fn pivot_aspect(&self) -> Option<String> {
        match self {
            Self::Verb { aspect: Some(value), .. } => Some(
                panini_core::aggregable::ClosedValues::variant_str(value).to_string(),
            ),
            _ => None,
        }
    }

    pub const PIVOT_ASPECT: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "aspect", "Aspect",
            <PersianAspect as panini_core::aggregable::ClosedValues>::all_variants,
            Self::pivot_aspect,
        );
}

pub struct IranianPersian;

impl LinguisticDefinition for IranianPersian {
    type Morphology = PersianMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Pes;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        PersianMorphology::PIVOT_NUMBER,
        PersianMorphology::PIVOT_DEGREE,
        PersianMorphology::PIVOT_EZAFE,
        PersianMorphology::PIVOT_INDEFINITE,
        PersianMorphology::PIVOT_VERB_FORM,
        PersianMorphology::PIVOT_POLARITY,
        PersianMorphology::PIVOT_TENSE,
        PersianMorphology::PIVOT_MOOD,
        PersianMorphology::PIVOT_ASPECT,
    ];

    fn supported_scripts(&self) -> &[Script] {
        // Arabic is the standard; Latin Fingilish is also used in digital writing.
        // Empirical evidence: https://pmc.ncbi.nlm.nih.gov/articles/PMC5174010/.
        &[Script::ARAB, Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::ARAB
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[TypologicalFeature::Conjugation(&[Upos::Verb])]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmas: use standard Persian ی/ک, singular nouns (کتاب‌ها → کتاب, زنان → زن), \
         positive adjectives (بهتر/بهترین → خوب) and full infinitives for verbs \
         (می‌روم/رفت/رفته → رفتن; است/هستم/نیست → بودن). Preserve the input token's spelling \
         including ZWNJ; normalize Arabic ي/ك only in lemmas. Fingilish input keeps its surface \
         but uses the corresponding Persian-script lemma when unambiguous.\n\
         2. Tokenization: keep plural, degree, indefinite -i, verbal prefixes and agreement \
         endings on their hosts. Split attached personal clitics and copulas: کتابم → کتاب + م \
         (pronoun من), خوبم → خوب + م (verb بودن). Split رفته‌ام into participle رفته + copula ام. \
         Resolve ambiguous -am from context; a finite verb's subject ending is NOT a pronoun. \
         Keep light-verb predicates separate: کار noun + می‌کنم verb کردن. Auxiliaries are verbs.\n\
         3. Nominals: number follows the noun's morphology, so سه کتاب has singular کتاب; \
         plural ها/ان and established broken plurals are plural. Adjectives do not agree in \
         gender or number. ezafe is true on each noun/adjective/nominal preposition that bears \
         the linker, including unwritten -e: کتاب خوب من has ezafe on کتاب and خوب. \
         Simple prepositions به/از/در and object-marking را are adpositions with ezafe false. \
         indefinite is true only for the host bearing indefinite -i (کتابی or خوبی in کتاب خوبی), \
         false otherwise; it is not a definite/indefinite semantic judgment. Relative -i \
         before که is not the indefinite suffix. Do not invent an ezafe token or any case/gender.\n\
         4. Verbs: finite forms carry person/number and mood; imperatives omit tense. \
         Infinitives (including short رفت in خواهم رفت) and participles omit person, number, \
         mood, tense and aspect. Finite indicative/subjunctive tokens carry present or past \
         by their own paradigm; خواهم is present, not future. mi-/nemi- forms are imperfective; \
         indicative simple past is perfective; otherwise omit aspect. Negation on the token \
         gives negative polarity (نمی‌روم/نرفت/نیست/نرو), affirmative otherwise. \
         Analyse perfect, progressive and passive constructions token by token: رفته بودم \
         is participle + past بودن, دارم می‌روم two present verbs, نوشته شد participle + past شدن. \
         Do not transfer auxiliary agreement, tense or negation onto a separate participle.\n\
         5. Pronouns: personal forms and detached pronominal clitics carry person/number; \
         شما remains second plural for one polite addressee. Reflexive خود has person/number \
         only when morphologically expressed; its attached personal clitic is a separate token. \
         Other pronouns carry number only when encoded (این‌ها plural), person omitted. \
         Demonstratives before nouns are determiners; standalone ones are pronouns. \
         Greeting سلام is an interjection; distinguish its nominal use meaning a greeting. \
         Distinguish indefinite -i, ezafe -ye, relational adjective -i and second-singular -i."
    }

    fn alignment_directives(&self) -> Option<&'static str> {
        Some(
            "1. Preserve Persian letters and ZWNJ exactly. Attached personal clitics/copulas \
             may be surface segments (کتابم: [\"کتاب\", \"م\"]). Do not write a segment for \
             an unwritten ezafe vowel or a zero third-singular past ending.\n\
             2. Keep a light-verb predicate's written words separate within one alignment link \
             when needed (کار می‌کنم). Auxiliary constructions retain all written words. \
             را may align with an object phrase without an independent lexical gloss.\n\
             3. Transparent plural/degree/verbal affixes may be segmented when needed for \
             alignment; retain any joining ZWNJ in a segment and never substitute the lemma \
             for the actual surface stem.",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use panini_core::aggregable::Aggregable;
    use serde_json::json;

    #[test]
    fn negation_stays_distinct_after_lemmatizing_the_finite_verb() {
        let affirmative: PersianMorphology = serde_json::from_value(json!({
            "pos": "verb", "lemma": "رفتن", "verb_form": "finite",
            "polarity": "affirmative", "person": "first", "number": "singular",
            "mood": "indicative", "tense": "present", "aspect": "imperfective"
        })).unwrap();
        let mut negative_json = serde_json::to_value(&affirmative).unwrap();
        negative_json["polarity"] = json!("negative");
        let negative: PersianMorphology = serde_json::from_value(negative_json).unwrap();
        assert_eq!(affirmative.lemma(), negative.lemma());
        assert_ne!(affirmative.observations(), negative.observations());
        assert_eq!(PersianMorphology::PIVOT_POLARITY.value(&negative).as_deref(), Some("negative"));
        assert_eq!(PersianMorphology::PIVOT_ASPECT.value(&negative).as_deref(), Some("imperfective"));
    }

    #[test]
    fn participles_and_infinitives_do_not_acquire_auxiliary_agreement() {
        for verb_form in ["participle", "infinitive"] {
            let form: PersianMorphology = serde_json::from_value(json!({
                "pos": "verb", "lemma": "رفتن", "verb_form": verb_form, "polarity": "affirmative"
            })).unwrap();
            let value = serde_json::to_value(&form).unwrap();
            for absent in ["person", "number", "tense", "mood", "aspect"] {
                assert!(value.get(absent).is_none());
            }
            assert_eq!(PersianMorphology::PIVOT_TENSE.value(&form), None);
            assert_eq!(PersianMorphology::PIVOT_MOOD.value(&form), None);
        }
    }

    #[test]
    fn nominal_linker_and_indefinite_suffix_are_independent_observations() {
        let linked: PersianMorphology = serde_json::from_value(json!({
            "pos": "noun", "lemma": "کتاب", "number": "singular", "ezafe": true,
            "indefinite": false
        })).unwrap();
        let indefinite: PersianMorphology = serde_json::from_value(json!({
            "pos": "noun", "lemma": "کتاب", "number": "singular", "ezafe": false,
            "indefinite": true
        })).unwrap();
        assert_ne!(linked.observations(), indefinite.observations());
        assert_eq!(PersianMorphology::PIVOT_EZAFE.value(&linked).as_deref(), Some("true"));
        assert_eq!(PersianMorphology::PIVOT_INDEFINITE.value(&indefinite).as_deref(), Some("true"));
        let schema = serde_json::to_string(&schemars::schema_for!(PersianMorphology)).unwrap();
        assert!(!schema.contains("\"gender\""));
        assert!(!schema.contains("\"case\""));
    }

    #[test]
    fn standard_script_and_product_gates_match_persian() {
        assert_eq!(IranianPersian::ISO_LANG.to_639_3(), "pes");
        assert_eq!(IranianPersian.default_script(), Script::ARAB);
        assert!(IranianPersian.supported_scripts().contains(&Script::LATN));
        assert_eq!(IranianPersian.typological_features(), &[TypologicalFeature::Conjugation(&[Upos::Verb])]);
    }
}
