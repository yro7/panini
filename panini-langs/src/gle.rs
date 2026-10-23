//! Contemporary Irish, with the written standard as the pedagogical baseline.
//!
//! Reference systems: the Oireachtas' *An Caighdeán Oifigiúil* (2017),
//! https://www.teanglann.ie/en/gram/ and the Irish UD documentation:
//! https://universaldependencies.org/ga/ (especially Case, Person and VerbForm).

use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature,
    Upos,
};

/// Nominative is the common form, used for objects as well as subjects.
/// The prepositional dative is normally syncretic with it in the standard.
#[panini_macro::closed_enum]
pub enum IrishCase {
    Nominative,
    Genitive,
    Dative,
    Vocative,
}

/// Overt initial changes, rather than the mutation a preceding word could
/// trigger. Vowel n-prefixing belongs to eclipsis; h/t-prefixing is separate.
#[panini_macro::closed_enum]
pub enum IrishMutation {
    Unmutated,
    Lenition,
    Eclipsis,
    HPrefix,
    TPrefix,
}

#[panini_macro::closed_enum]
pub enum IrishNounForm {
    Simple,
    VerbalNoun,
}

#[panini_macro::closed_enum]
pub enum IrishDegree {
    Positive,
    Comparative,
    Superlative,
}

/// The habitual present of bí and habitual past have their own paradigms.
/// Conditional and imperative belong to mood and carry no tense here.
#[panini_macro::closed_enum]
pub enum IrishTense {
    Present,
    PresentHabitual,
    Past,
    PastHabitual,
    Future,
}

#[panini_macro::closed_enum]
pub enum IrishMood {
    Indicative,
    Imperative,
    Conditional,
    Subjunctive,
}

/// The autonomous paradigm leaves the agent unspecified; it is not a passive
/// agreement form. Copular is/ba is distinct from forms of the verb bí.
#[panini_macro::closed_enum]
pub enum IrishVerbForm {
    Analytic,
    Synthetic,
    Autonomous,
    Copula,
}

#[panini_macro::closed_enum]
pub enum IrishDeterminerType {
    Article,
    Possessive,
    Demonstrative,
    Quantifier,
    Interrogative,
}

#[panini_macro::closed_enum]
pub enum IrishPronounType {
    Personal,
    Reflexive,
    Demonstrative,
    Relative,
    Interrogative,
    Indefinite,
}

/// Personal-pronoun forms: sé/sí/siad versus é/í/iad; mé is syncretic.
#[panini_macro::closed_enum]
pub enum IrishPronounCase {
    Subjective,
    Objective,
}

#[panini_macro::closed_enum]
pub enum IrishEmphasis {
    Plain,
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
#[serde(tag = "pos", rename_all = "snake_case")]
pub enum IrishMorphology {
    Adjective {
        lemma: String,
        degree: IrishDegree,
        /// Agreement applies to attributive positive forms, not invariant
        /// predicative or comparative/superlative forms.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<IrishCase>,
        mutation: IrishMutation,
    },
    Adposition {
        lemma: String,
        /// Pronominal object, or possessor in a fused possessive preposition;
        /// absent on an ordinary preposition and article contractions.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Encoded only in third-person singular forms.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        possessive: bool,
        /// Present on pronominal prepositions such as agam/agam-sa, not
        /// ordinary prepositions or fused preposition + article forms.
        #[serde(skip_serializing_if = "Option::is_none")]
        emphasis: Option<IrishEmphasis>,
    },
    Adverb {
        lemma: String,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: IrishDeterminerType,
        /// Possessor's person for possessives, absent otherwise.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Article agreement, or the possessor's number for possessives;
        /// absent on invariant demonstratives/quantifiers/interrogatives.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Singular article agreement or third-singular possessor gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        /// Article agreement only: possessives and invariant determiners
        /// have no case paradigm.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<IrishCase>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
        case: IrishCase,
        noun_form: IrishNounForm,
        mutation: IrishMutation,
    },
    Numeral {
        lemma: String,
        mutation: IrishMutation,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        pronoun_type: IrishPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pronoun_case: Option<IrishPronounCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        emphasis: Option<IrishEmphasis>,
    },
    ProperNoun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
        case: IrishCase,
        mutation: IrishMutation,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Verb {
        lemma: String,
        verb_form: IrishVerbForm,
        mood: IrishMood,
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<IrishTense>,
        /// Only when encoded by the synthetic form (including the bare
        /// second-singular imperative), never borrowed from a separate subject.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        mutation: IrishMutation,
    },
    Other {
        lemma: String,
    },
}

impl IrishMorphology {
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    /// Optional because imperative/conditional forms are tenseless, but a
    /// finite-tense pivot is essential to the conjugation curriculum.
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <IrishTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );
}

pub struct Irish;

impl LinguisticDefinition for Irish {
    type Morphology = IrishMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Gle;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        IrishMorphology::PIVOT_CASE,
        IrishMorphology::PIVOT_GENDER,
        IrishMorphology::PIVOT_NUMBER,
        IrishMorphology::PIVOT_MUTATION,
        IrishMorphology::PIVOT_NOUN_FORM,
        IrishMorphology::PIVOT_VERB_FORM,
        IrishMorphology::PIVOT_MOOD,
        IrishMorphology::PIVOT_TENSE,
        IrishMorphology::PIVOT_DEGREE,
    ];

    fn supported_scripts(&self) -> &[Script] {
        // Gaelic type is a typographic variety of Latin; historical Ogham is
        // not a contemporary writing system for the standard course.
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            TypologicalFeature::Conjugation(&[Upos::Verb]),
            TypologicalFeature::Declension(&[
                Upos::Noun,
                Upos::ProperNoun,
                Upos::Adjective,
                Upos::Adposition,
                Upos::Pronoun,
                Upos::Determiner,
            ]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmas: remove contextual lenition/eclipsis and h-/t-/n-prefixing, preserving the fada. \
         Nouns take nominative singular (fhir → fear, mná → bean, leabhair → leabhar); proper nouns \
         retain capitalization (a Sheáin → Seán). Verbs use the second-singular imperative citation \
         form (rinne → déan, chonaic → feic, d'ith → ith, tá/bhí/beidh/raibh/níl → bí). The copula \
         has lemma is, never bí. Adjectives use the unmutated positive singular (fearr → maith, \
         mó → mór); verbal adjectives keep their adjective citation form (dúnta, déanta).\n\
         2. Nominals: give lexical gender even in the plural, number, and case. Nominative is \
         the common case for subjects AND direct objects; do not invent an accusative noun case. \
         Genitive marks possession and appropriate complements of verbal nouns and compound \
         prepositions; dative is the case after ordinary dative-governing simple prepositions, \
         even when identical to nominative. Follow actual government: chun normally takes genitive. \
         Use vocative in direct address, including a Sheáin and a chara. Case is not mutation. \
         Give a counted noun its own grammatical number: trí bhád has singular bád despite three \
         referents. Lexical plurals and higher-number constructions follow their actual syntax.\n\
         3. Mutation reports the OVERT change relative to the unmutated form: bhád lenition; \
         mbád and n-úll eclipsis; háit h_prefix; t-uisce and tseachtain t_prefix. An unchanged \
         initial, including a letter that cannot mutate, is unmutated. Never classify lexical h, \
         lexical initial consonant clusters, or a past d' prefix as initial mutation.\n\
         4. The verbal noun is a NOUN with noun_form verbal_noun, its own nominal lemma, gender, \
         number and case: ag ithe → ithe, ag déanamh → déanamh, a dhéanamh → déanamh. Other nouns \
         have noun_form simple. Irish has no morphological infinitive: a + verbal noun is a \
         particle plus noun, and bí + ag + verbal noun is analysed token by token, with tense on \
         bí only. Verbal adjectives such as dúnta/déanta are ADJECTIVES, not finite verbs or a \
         fictitious past tense. Positive attributive adjectives take their head's gender, number \
         and case; omit all three for invariant predicative and comparative/superlative forms. \
         Distinguish comparative níos fearr from superlative is fearr by construction.\n\
         5. Finite verbs: synthetic forms encode person/number (ólaim first singular, ólaimid \
         first plural); the bare singular imperative ól is also synthetic, second singular. \
         Analytic forms such as ólann, d'ól and beidh take NO person/number from their separate \
         subject. Autonomous óltar/óladh/ólfar have verb_form autonomous, no person/number and \
         their actual mood/tense; they are not passive agreement. Forms of is/ba are copula, with \
         no person/number. Ordinary questions keep indicative mood. Provide tense on indicative \
         and subjunctive forms; omit tense on imperative and conditional. Use present_habitual \
         for bím/bíonn, present for tá, past_habitual for bhíodh and d'óladh (dependent óladh) \
         in a habitual-past context, and past for bhí/d'ól. Context distinguishes autonomous past \
         óladh from dependent habitual-past óladh. \
         Copular ba/b' is past in a past assertion and conditional (without tense) in a hypothetical \
         context. Do not transfer a compound construction's tense to a nominal or adjectival token.\n\
         6. Inflected prepositions stay one ADPOSITION: agam → ag first singular; liom → le \
         first singular; dó/dí → do third singular masculine/feminine; acu → ag third plural. \
         Report person and number only for a pronominal object or fused possessor; gender only \
         for third singular; omit these on simple prepositions and preposition+article contractions \
         (sa/san → i, den → de, don → do). possessive is true for ina/óna/lena when a possessive \
         determiner is fused, with the possessor's features from context; false otherwise. Do not \
         confuse ina chónaí (possessive) with ann (pronominal). Set emphasis plain/emphatic only \
         on pronominal prepositions (agam/agam-sa); omit it on simple/possessive/article forms.\n\
         7. Determiners: an/na are article, with lemma an, article number and case; singular \
         gender comes from the noun, plural gender is omitted. na can be feminine genitive SINGULAR \
         as well as plural. Possessive mo/do/a/ár/bhur has the POSSESSOR'S person/number and gender \
         only for third singular: a chóta his, a cóta her, a gcóta their. Omit possessive case. \
         seo/sin/úd modifying a noun are demonstrative determiners; gach/aon are quantifiers; \
         cén is interrogative. Omit person/number/gender/case from invariant nonpossessive \
         determiners. Demonstratives standing alone are pronouns, not determiners.\n\
         8. Personal pronouns encode person/number; gender is present only in third singular. \
         Give pronoun_case subjective for subjects and objective for objects and the é/í/iad \
         series in copular identification. mise/tusa/seisean/sise/muidne/sibhse/siadsan are emphatic \
         personal forms: lemmatize to mé/tú/sé/sí/muid/sibh/siad, with emphasis emphatic. Ordinary \
         personal forms have emphasis plain. Reflexives with féin take the personal token's \
         features; féin remains a separate token. For invariant demonstrative, relative, \
         interrogative and indefinite pronouns omit person/number/gender/pronoun_case/emphasis \
         unless that particular form genuinely encodes the feature.\n\
         9. Keep orthographic tokens with their actual apostrophe or hyphen: d'ith, n-úll, \
         t-uisce and inflected prepositions are not strings of invented words. Elided possessives \
         m'/d' before a noun may be separate determiner tokens (lemma mo/do), retaining the \
         following noun as its own token. Separate written ag, a, ní, níor, an and nach. Classify \
         homographs by syntax: an article is a determiner, an before a finite verb a question \
         particle; a may be possessive determiner, relative marker, vocative or infinitival \
         particle. A fused negative/question/relative COPULA (ní, an, nach, gur, ar etc.) is a \
         verb with lemma is when it itself supplies the copula; is coordinating 'and' is a \
         conjunction with lemma agus. Never invent an unwritten copula or a subject pronoun. \
         Names of specific people and places are proper_noun even inside a fixed expression \
         (Muire in Dia is Muire duit), with their actual gender, number, case and mutation; \
         do not add noun_form. Adverbial-forming go in go maith/go fóill is a particle, not \
         an adverb or the preposition meaning to/until. The following lexical adjective may \
         remain adjective even when the whole phrase functions adverbially."
    }

    fn alignment_directives(&self) -> Option<&'static str> {
        Some(
            "1. Initial lenition/eclipsis and h-/t-/n-prefixing stay with the lexical token: \
             bhád, mbád, n-úll, t-uisce. Do not manufacture an unmutated spelling as a segment.\n\
             2. A transparent synthetic person ending may be segmented when another sentence \
             expresses the subject separately: [\"cuir\", \"im\"], [\"cuir\", \"imid\"]. \
             Opaque irregular forms and pronominal prepositions (liom, agam, dó) may instead \
             align as a whole to several words; never invent an unattested stem segment.\n\
             3. An elided possessive and past particle can be surface segments \
             ([\"m'\", \"athair\"], [\"d'\", \"ith\"]), retaining the apostrophe. \
             Preposition+article contractions (sa, den, don) can align as a whole to several \
             words. ag and a before a verbal noun are separate words, as are bí and the noun.\n\
             4. Respect the contextual sense of grammatical particles in literal glosses. \
             In adverbial go phrases, go has no independent to/until meaning: align go fóill \
             as one unit meaning yet/still/for now rather than inventing until yet.",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use panini_core::traits::MorphologyInfo;
    use serde_json::json;

    #[test]
    fn autonomous_verb_has_no_agreement_and_reaches_the_tense_pivot() {
        let form: IrishMorphology = serde_json::from_value(json!({
            "pos": "verb", "lemma": "ól", "verb_form": "autonomous",
            "mood": "indicative", "tense": "present", "mutation": "unmutated"
        }))
        .unwrap();
        let value = serde_json::to_value(&form).unwrap();
        assert!(value.get("person").is_none());
        assert!(value.get("number").is_none());
        assert_eq!(form.pos(), Upos::Verb);
        assert_eq!(IrishMorphology::PIVOT_TENSE.value(&form).as_deref(), Some("present"));

        let mut invalid = value;
        invalid["verb_form"] = json!("passive");
        assert!(serde_json::from_value::<IrishMorphology>(invalid).is_err());
    }

    #[test]
    fn mutation_and_nominal_case_remain_independent() {
        let form: IrishMorphology = serde_json::from_value(json!({
            "pos": "noun", "lemma": "bád", "gender": "masculine",
            "number": "singular", "case": "nominative", "noun_form": "simple",
            "mutation": "eclipsis"
        }))
        .unwrap();
        assert_eq!(IrishMorphology::PIVOT_CASE.value(&form).as_deref(), Some("nominative"));
        assert_eq!(IrishMorphology::PIVOT_MUTATION.value(&form).as_deref(), Some("eclipsis"));
        assert_eq!(MorphologyInfo::lemma(&form), "bád");
    }

    #[test]
    fn verbal_noun_and_pronominal_preposition_keep_their_own_parts_of_speech() {
        let noun: IrishMorphology = serde_json::from_value(json!({
            "pos": "noun", "lemma": "ithe", "gender": "masculine",
            "number": "singular", "case": "dative", "noun_form": "verbal_noun",
            "mutation": "unmutated"
        }))
        .unwrap();
        let preposition: IrishMorphology = serde_json::from_value(json!({
            "pos": "adposition", "lemma": "le", "person": "first",
            "number": "singular", "possessive": false, "emphasis": "plain"
        }))
        .unwrap();
        assert_eq!(noun.pos(), Upos::Noun);
        assert_eq!(IrishMorphology::PIVOT_NOUN_FORM.value(&noun).as_deref(), Some("verbal_noun"));
        assert_eq!(preposition.pos(), Upos::Adposition);
        assert!(serde_json::to_value(preposition).unwrap().get("gender").is_none());
    }
}
