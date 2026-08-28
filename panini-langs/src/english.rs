use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature, Upos,
};

/// Whether a common-noun occurrence denotes countable units or undifferentiated
/// substance. English frequently shifts the same lemma between the two by
/// sense (`coffee` / `two coffees`), so this describes the occurrence rather
/// than an immutable dictionary property.
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
pub enum EnglishCountability {
    Count,
    Mass,
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
pub enum EnglishDegree {
    Positive,
    Comparative,
    Superlative,
}

/// The only two synthetic tenses of contemporary English. Future, perfect and
/// progressive constructions are periphrastic and are analysed token by token.
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
pub enum EnglishTense {
    Present,
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
pub enum EnglishMood {
    Indicative,
    Imperative,
    Subjunctive,
}

/// Modern descriptions treat the verbal uses of the English `-ing` form as a
/// single gerund-participial inflection; its syntactic function does not create
/// two different word forms.
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
pub enum EnglishVerbForm {
    Finite,
    Infinitive,
    GerundParticiple,
    PastParticiple,
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
pub enum EnglishPronounCase {
    Nominative,
    Accusative,
    Genitive,
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
pub enum EnglishPronounType {
    Personal,
    Possessive,
    Reflexive,
    Demonstrative,
    Relative,
    Interrogative,
    Indefinite,
    Expletive,
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
pub enum EnglishDeterminerType {
    Article,
    Demonstrative,
    Possessive,
    Quantifier,
    Interrogative,
    Relative,
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
pub enum EnglishMorphology {
    Adjective {
        lemma: String,
        degree: EnglishDegree,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        /// Present only for an adverb whose form itself encodes degree (`fast`,
        /// `faster`, `fastest`, `well`, `better`, `best`, `more`, `most`).
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<EnglishDegree>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: EnglishDeterminerType,
        /// Only when the determiner itself marks number (`this` / `these`), not
        /// merely because its noun has a number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        number: BinaryNumber,
        countability: EnglishCountability,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        pronoun_type: EnglishPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Personal/reflexive form contrast and possessive forms only. Omit it
        /// where the pronoun has no case contrast.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<EnglishPronounCase>,
    },
    ProperNoun {
        lemma: String,
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
        verb_form: EnglishVerbForm,
        /// Finite indicative/subjunctive forms only; imperative and non-finite
        /// forms have no tense in this model.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<EnglishTense>,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<EnglishMood>,
        /// Finite forms only, recovered from their subject when the surface form
        /// is syncretic.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms only; omit when an imperative's number is genuinely
        /// indeterminate from context.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Other {
        lemma: String,
    },
}

impl EnglishMorphology {
    fn __pivot_case(&self) -> Option<String> {
        match self {
            Self::Pronoun { case, .. } => case
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

    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    pub const PIVOT_CASE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "case",
            "Case",
            <EnglishPronounCase as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_case,
        );

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <EnglishTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <EnglishMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );
}

pub struct English;

impl LinguisticDefinition for English {
    type Morphology = EnglishMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Eng;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        EnglishMorphology::PIVOT_COUNTABILITY,
        EnglishMorphology::PIVOT_NUMBER,
        EnglishMorphology::PIVOT_DEGREE,
        EnglishMorphology::PIVOT_CASE,
        EnglishMorphology::PIVOT_VERB_FORM,
        EnglishMorphology::PIVOT_TENSE,
        EnglishMorphology::PIVOT_MOOD,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        // English verbs conjugate, although the regular paradigm is small.
        // The current declension cloze asks for a noun/adjective root and an
        // inflected object, which does not model English pronoun case or the
        // possessive clitic faithfully, so Declension is deliberately absent.
        &[TypologicalFeature::Conjugation(&[Upos::Verb])]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope and lemmatization: analyze contemporary standard English of either the American or British track without normalizing one variety into the other. Lemmatize common and proper nouns to their singular citation form (but keep the dictionary form of plural-only nouns), verbs including auxiliaries and modals to the bare dictionary form, and adjectives/adverbs to the positive form. Lemmatize personal-pronoun case forms to their nominative paradigm base (me -> I, him -> he, them -> they); keep the citation form for other pronouns and determiners. Preserve the input variety's legitimate forms such as got/gotten and learned/learnt.\n\
         2. Nouns: always report grammatical number and contextual countability. Countability belongs to the sense in this occurrence, not permanently to the spelling: 'coffee' is mass in 'some coffee' but count in 'two coffees'; collective nouns such as 'team' are count nouns regardless of singular or plural agreement. Do not assign grammatical gender anywhere in English.\n\
         3. Adjectives and adverbs: report positive/comparative/superlative for adjective forms. For adverbs, report degree only where the adverb itself carries it. In periphrastic comparison ('more carefully', 'most interesting'), 'more'/'most' carries the comparative/superlative degree while 'carefully' has no degree and 'interesting' remains positive.\n\
         4. Determiners and pronouns: classify each by its function in context; 'that', 'what' and 'whose' can belong to different types or parts of speech. Add person and number only where they apply. Case is nominative for subject personal forms, accusative for object/oblique/reflexive forms, and genitive for possessive pronouns; omit case for forms with no case contrast. Treat singular 'they' as the plural agreement form and never infer grammatical gender from a referent.\n\
         5. Verbs: every verb, including BE/HAVE/DO auxiliaries and modal auxiliaries, is a Verb token with its own lemma and verb_form. A contextually finite form is finite even when identical to the bare form; a bare form governed by 'to', a modal or another auxiliary is infinitive. Treat every verbal -ing form as gerund_participle and every verbal -ed/-en form in a perfect or passive construction as past_participle; lexicalized deverbal nouns/adjectives keep their actual Noun/Adjective part of speech.\n\
         6. Finite features: finite indicative and subjunctive verbs get present/past tense, mood, person and number. Imperatives get imperative mood and second person, but no tense; omit number if context does not resolve it. Non-finite forms get no tense, mood, person or number. Classify modal forms morphologically, not by time reference: will/can/may/shall/must are present forms and would/could/might/should are past forms. English has no synthetic future: 'will leave' is finite present 'will' plus infinitive 'leave'. Perfect, progressive and passive constructions are likewise separate verb tokens, never a single tense/aspect/voice value.\n\
         7. Contractions: split transparent contractions into their grammatical constituents while retaining each written constituent in 'word': I'm -> I + 'm (lemma be), can't -> can + n't (lemma not), she'll -> she + 'll (lemma will), and we've -> we + 've (lemma have). Resolve ambiguous 's from context as contracted be, contracted have, or the possessive clitic. Analyze possessive 's (and the plural possessive apostrophe) as a separate Particle, not as noun declension.\n\
         8. Context-sensitive tokens: infinitival 'to' is a Particle and prepositional 'to' is an Adposition; 'not' is a Particle; noun modifiers such as 'coffee' in 'coffee shop' remain Nouns. Keep phrasal-verb particles as separate tokens and keep the verb's simplex lemma ('gave up' -> give + up); the Multiword Expressions component records any idiomatic unit. Never emit punctuation as a token."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_identity_script_and_typology_are_exact() {
        let language = English;

        assert_eq!(English::ISO_LANG, IsoLang::Eng);
        assert_eq!(language.supported_scripts(), &[Script::LATN]);
        assert_eq!(language.default_script(), Script::LATN);
        assert_eq!(
            language.typological_features(),
            &[TypologicalFeature::Conjugation(&[Upos::Verb])]
        );
    }

    #[test]
    fn optional_finite_features_remain_closed_pivots() {
        let walked = EnglishMorphology::Verb {
            lemma: "walk".to_string(),
            verb_form: EnglishVerbForm::Finite,
            tense: Some(EnglishTense::Past),
            mood: Some(EnglishMood::Indicative),
            person: Some(Person::Third),
            number: Some(BinaryNumber::Singular),
        };

        assert_eq!(
            EnglishMorphology::PIVOT_TENSE.value(&walked),
            Some("past".to_string())
        );
        assert_eq!(
            EnglishMorphology::PIVOT_MOOD.value(&walked),
            Some("indicative".to_string())
        );
    }

    #[test]
    fn noun_countability_is_contextual_and_pivotable() {
        let coffee = EnglishMorphology::Noun {
            lemma: "coffee".to_string(),
            number: BinaryNumber::Plural,
            countability: EnglishCountability::Count,
        };

        assert_eq!(
            EnglishMorphology::PIVOT_COUNTABILITY.value(&coffee),
            Some("count".to_string())
        );
    }
}
