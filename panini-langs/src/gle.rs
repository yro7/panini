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

/// Polarity encoded by the verb token itself, not by a separate particle.
/// In particular, níl and the negative copula retain their positive lemma.
#[panini_macro::closed_enum]
pub enum IrishPolarity {
    Affirmative,
    Negative,
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
        polarity: IrishPolarity,
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
        IrishMorphology::PIVOT_POLARITY,
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
        "1. Lemmas, tokens and mutation: preserve fadas; undo contextual initial changes in lemmas. \
         Nouns use nominative singular (mná → bean); verbs the second-singular imperative \
         (rinne → déan, chonaic → feic, tá/bhí/beidh/raibh/níl → bí); copulas use is, never bí; \
         adjectives use positive singular (fearr → maith). Personal/place names remain proper_noun \
         with capitalization, including Muire in Dia is Muire duit. Keep orthographic tokens \
         (d'ith, n-úll, agam-sa); elided possessives m'/d' may split from their noun (lemma mo/do). \
         Mutation records only overt change: bhád lenition, mbád/n-úll eclipsis, háit h_prefix, \
         t-uisce/tseachtain t_prefix; unchanged initials, lexical h/clusters and past d' are unmutated.\n\
         2. Nominals: retain lexical gender in plurals. Nominative covers subjects AND objects; \
         use genitive for possession and appropriate verbal-noun/compound-preposition complements, \
         dative after dative-governing simple prepositions even without a distinct ending, vocative \
         for address (a Sheáin). Follow government: chun normally takes genitive. Attributive \
         adjectives normally agree with the head; counted forms keep their OWN number: trí bhád \
         mhóra has singular bhád but plural mhóra. Cardinals 2–19 take plural adjectives; 1 and \
         full tens normally singular; higher numbers follow the actual traditional/simplified \
         construction. Omit adjective gender/number/case in invariant predicative and comparative/ \
         superlative uses; distinguish níos fearr comparative from is fearr superlative.\n\
         3. Deverbatives: verbal nouns are NOUNS with noun_form verbal_noun and their own nominal \
         lemma/gender/number/case (a dhéanamh → déanamh); ordinary nouns have noun_form simple. \
         a + verbal noun is particle + noun, not a morphological infinitive; bí + ag + verbal noun \
         keeps separate tokens with tense only on bí. Verbal adjectives (dúnta/déanta) stay \
         ADJECTIVES with their adjective lemma, not past-tense verbs.\n\
         4. Verbs: only synthetic forms encode person/number (ólaim first singular; bare imperative \
         ól second singular). Analytic ólann/d'ól/beidh, autonomous óltar/óladh/ólfar and copulas \
         omit person/number; autonomous is not passive agreement. Questions remain indicative. \
         Give tense on indicative/subjunctive, omit it on imperative/conditional. tá is present, \
         bím/bíonn present_habitual, bhí/d'ól past, bhíodh/d'óladh past_habitual; context resolves \
         autonomous past versus dependent habitual óladh. Copular ba/b' is past in assertions, \
         conditional without tense in hypotheticals. Polarity is TOKEN-LOCAL: negative on níl/nílim \
         and fused negative copulas (ní/níor/níorbh/nach/nár/nárbh), affirmative otherwise, including \
         interrogative copular an. Separate ní/níor stay particles: raibh/ól in ní raibh/níor ól \
         retain affirmative token polarity.\n\
         5. Prepositions and determiners: inflected prepositions stay ADPOSITIONS with the base \
         preposition as lemma (agam → ag; dó/dí → do). Person/number describe the pronominal object \
         or fused possessor; gender only third singular. Omit them on simple/article contractions \
         (sa/san → i, den → de, don → do). possessive is true only with a fused possessive \
         (ina/óna/lena, traits from context): ina chónaí differs from pronominal ann. Emphasis \
         plain/emphatic applies only to pronominal forms (agam/agam-sa). Articles an/na use lemma an, \
         article number/case and singular head gender; na also marks feminine genitive singular. \
         Possessives carry POSSESSOR person/number, gender only third singular, no case \
         (a chóta his, a cóta her, a gcóta their). Invariant determiners omit agreement: seo/sin/úd \
         demonstrative, gach/aon quantifier, cén interrogative; standalone demonstratives are pronouns.\n\
         6. Pronouns and homographs: personal pronouns carry person/number, gender only third singular, \
         subjective case for subjects, objective for objects and copular é/í/iad. Emphatic \
         mise/tusa/seisean/sise/muidne/sibhse/siadsan use lemmas mé/tú/sé/sí/muid/sibh/siad; ordinary \
         forms have emphasis plain. Reflexive féin stays separate from the personal token. Other \
         pronouns omit these features unless encoded. Resolve an as article versus question particle; \
         a as possessive, relative, vocative or infinitival marker. Fused copulas ní/an/nach/gur/ar \
         are VERBS with lemma is when supplying the copula; coordinating is has lemma agus. \
         Never invent unwritten copulas/subjects. Adverbial go (go maith/go fóill) is a PARTICLE, \
         not to/until; the following adjective may stay adjective."
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
    use panini_core::aggregable::Aggregable;
    use panini_core::traits::MorphologyInfo;
    use serde_json::json;

    #[test]
    fn fused_negation_survives_lemma_normalization_and_aggregation() {
        // tá/níl and copular is/ní must remain distinct after normalization.
        for (lemma, verb_form) in [("bí", "analytic"), ("is", "copula")] {
            let affirmative: IrishMorphology = serde_json::from_value(json!({
                "pos": "verb", "lemma": lemma, "verb_form": verb_form,
                "mood": "indicative", "tense": "present", "polarity": "affirmative",
                "mutation": "unmutated"
            }))
            .unwrap();
            let mut negative_json = serde_json::to_value(&affirmative).unwrap();
            negative_json["polarity"] = json!("negative");
            let negative: IrishMorphology = serde_json::from_value(negative_json).unwrap();

            assert_eq!(affirmative.lemma(), negative.lemma());
            assert_ne!(affirmative.observations(), negative.observations());
            for (form, expected) in [(&affirmative, "affirmative"), (&negative, "negative")] {
                assert_eq!(
                    IrishMorphology::PIVOT_POLARITY.value(form).as_deref(),
                    Some(expected)
                );
                assert!(form.observations()[0].contains(&("polarity".into(), expected.into())));
            }
        }
        assert!(Irish::MORPHOLOGY_PIVOTS.iter().any(|pivot| pivot.key == "polarity"));
    }

    #[test]
    fn verb_polarity_is_required_and_closed_in_the_extraction_schema() {
        let schema = serde_json::to_value(schemars::schema_for!(IrishMorphology)).unwrap();
        let verb = schema["oneOf"]
            .as_array()
            .unwrap()
            .iter()
            .find(|variant| variant["properties"]["pos"]["const"] == "verb")
            .unwrap();
        assert!(verb["required"].as_array().unwrap().contains(&json!("polarity")));

        let mut form = json!({
            "pos": "verb", "lemma": "bí", "verb_form": "analytic",
            "mood": "indicative", "tense": "present", "mutation": "unmutated"
        });
        assert!(serde_json::from_value::<IrishMorphology>(form.clone()).is_err());
        form["polarity"] = json!("unknown");
        assert!(serde_json::from_value::<IrishMorphology>(form).is_err());
    }

    #[test]
    fn autonomous_verb_has_no_agreement_and_reaches_the_tense_pivot() {
        let form: IrishMorphology = serde_json::from_value(json!({
            "pos": "verb", "lemma": "ól", "verb_form": "autonomous",
            "mood": "indicative", "tense": "present", "polarity": "affirmative",
            "mutation": "unmutated"
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
