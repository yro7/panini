use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryGender,
    TypologicalFeature, Upos,
};

/// Morphological definiteness on Bokmål nominals.
///
/// This names the form of the token, not the definiteness of the whole noun
/// phrase. A noun after a possessive is morphologically indefinite even though
/// the phrase is referentially definite (`min bil`).
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
pub enum NorwegianBokmalDefiniteness {
    Indefinite,
    Definite,
}

/// The two productive written forms conventionally called case on Bokmål
/// nouns and proper nouns.
///
/// `Unmarked` is deliberately not called nominative: the same form serves as
/// subject, object and prepositional complement. `Genitive` is the written
/// phrasal `-s` form.
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
pub enum NorwegianBokmalNominalCase {
    Unmarked,
    Genitive,
}

/// The living subject/object opposition of Bokmål personal pronouns.
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
pub enum NorwegianBokmalPronounCase {
    Subject,
    Object,
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
pub enum NorwegianBokmalDegree {
    Positive,
    Comparative,
    Superlative,
}

/// Participles used adjectivally, whose agreement belongs to the adjective
/// system rather than to an analytic verb phrase.
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
pub enum NorwegianBokmalParticipleKind {
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
pub enum NorwegianBokmalDeterminerType {
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
pub enum NorwegianBokmalPronounType {
    Personal,
    Possessive,
    Reflexive,
    Demonstrative,
    Relative,
    Interrogative,
    Indefinite,
    Reciprocal,
    Expletive,
}

/// Natural-gender reference in third-person personal pronouns.
///
/// `Epicene` is the personal pronoun `hen`; grammatical neuter belongs to the
/// separate nominal-gender system (`det`).
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
pub enum NorwegianBokmalReferentialGender {
    Masculine,
    Feminine,
    Epicene,
}

/// Whether a third-person possessive points back to the clause subject.
///
/// This represents the learner-critical `sin/si/sitt/sine` versus
/// `hans/hennes/hens/deres` contrast. First- and second-person possessives do
/// not participate in it.
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
pub enum NorwegianBokmalPossessiveRelation {
    Reflexive,
    NonReflexive,
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
pub enum NorwegianBokmalParticleType {
    Infinitival,
    Negation,
    VerbParticle,
}

/// The two tenses expressed by a synthetic Bokmål verb form.
///
/// Perfect, pluperfect and future constructions are analytic and are analyzed
/// token by token.
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
pub enum NorwegianBokmalTense {
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
pub enum NorwegianBokmalMood {
    Indicative,
    Imperative,
}

/// Slots in the contemporary Bokmål verb paradigm.
///
/// `Supine` is the invariant form in analytic perfect and passive
/// constructions (`har skrevet`, `blir skrevet`). An agreeing participle is
/// instead analyzed as an adjective with `participle_kind = past`.
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
pub enum NorwegianBokmalVerbForm {
    Finite,
    Infinitive,
    Supine,
    PresentParticiple,
}

/// Learner-facing principal-parts classes for Bokmål verbs.
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
pub enum NorwegianBokmalVerbClass {
    /// Weak verbs with `-et`/accepted `-a` past and supine: `snakke`.
    #[serde(rename = "weak_group_1")]
    WeakGroup1,
    /// Weak verbs with `-te` past and `-t` supine: `kjøpe`.
    #[serde(rename = "weak_group_2")]
    WeakGroup2,
    /// Weak verbs with `-de` past and `-d` supine: `prøve`.
    #[serde(rename = "weak_group_3")]
    WeakGroup3,
    /// Weak vowel-stem verbs with `-dde` past and `-dd` supine: `bo`.
    #[serde(rename = "weak_group_4")]
    WeakGroup4,
    /// Strong verbs with a suffixless, usually vowel-changing past: `skrive`.
    Strong,
    /// Suppletive and residual paradigms not described by the five classes.
    Irregular,
}

/// Morphological active/`-s` diathesis.
///
/// `SForm` names the written form, not just passive meaning: lexical deponents,
/// reciprocals and middle readings also end in `-s`.
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
pub enum NorwegianBokmalDiathesis {
    Active,
    SForm,
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
pub enum NorwegianBokmalMorphology {
    Adjective {
        lemma: String,
        /// Only for a genuinely gradable adjective whose own form carries the
        /// degree. Relational adjectives and participles omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<NorwegianBokmalDegree>,
        /// Present only when this token is a participle used adjectivally.
        #[serde(skip_serializing_if = "Option::is_none")]
        participle_kind: Option<NorwegianBokmalParticipleKind>,
        /// Agreement is omitted on invariant forms such as comparatives and
        /// present participles.
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_number: Option<BinaryNumber>,
        /// Attributive forms only; predicate adjectives omit definiteness.
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_definiteness: Option<NorwegianBokmalDefiniteness>,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        /// Only when the adverb's own form expresses comparison.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<NorwegianBokmalDegree>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: NorwegianBokmalDeterminerType,
        /// Number and gender of the determined noun, never of the possessor.
        determined_number: BinaryNumber,
        /// Singular agreement only; plural forms do not distinguish gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        determined_gender: Option<TernaryGender>,
        /// Articles and demonstratives encode this contrast; possessives and
        /// many quantifiers do not.
        #[serde(skip_serializing_if = "Option::is_none")]
        determined_definiteness: Option<NorwegianBokmalDefiniteness>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_gender: Option<NorwegianBokmalReferentialGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessive_relation: Option<NorwegianBokmalPossessiveRelation>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        /// The gender of the paradigm realized in this text. A noun that the
        /// Bokmål norm permits as feminine or masculine follows the writer's
        /// locally consistent choice (`ei bok/boka` versus `en bok/boken`).
        nominal_gender: TernaryGender,
        number: BinaryNumber,
        definiteness: NorwegianBokmalDefiniteness,
        nominal_case: NorwegianBokmalNominalCase,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_type: NorwegianBokmalParticleType,
    },
    Pronoun {
        lemma: String,
        pronoun_type: NorwegianBokmalPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pronoun_number: Option<BinaryNumber>,
        /// `han`, `hun`, and personal `hen` only.
        #[serde(skip_serializing_if = "Option::is_none")]
        referential_gender: Option<NorwegianBokmalReferentialGender>,
        /// Pronominal `den`/`det` and forms agreeing with a noun paradigm.
        #[serde(skip_serializing_if = "Option::is_none")]
        nominal_gender: Option<TernaryGender>,
        /// Personal subject/object forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        pronoun_case: Option<NorwegianBokmalPronounCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessive_relation: Option<NorwegianBokmalPossessiveRelation>,
    },
    ProperNoun {
        lemma: String,
        number: BinaryNumber,
        nominal_case: NorwegianBokmalNominalCase,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        /// Lexical principal-parts class, reported on every token of the lemma.
        verb_class: NorwegianBokmalVerbClass,
        verb_form: NorwegianBokmalVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<NorwegianBokmalMood>,
        /// Finite indicative forms only. Imperatives have no tense here.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<NorwegianBokmalTense>,
        /// Finite and infinitive forms where active and `-s` forms contrast.
        /// Supines and present participles omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        diathesis: Option<NorwegianBokmalDiathesis>,
    },
    Other {
        lemma: String,
    },
}

impl NorwegianBokmalMorphology {
    fn __pivot_degree(&self) -> Option<String> {
        match self {
            Self::Adjective { degree, .. } | Self::Adverb { degree, .. } => degree
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_participle_kind(&self) -> Option<String> {
        match self {
            Self::Adjective {
                participle_kind, ..
            } => participle_kind
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_pronoun_case(&self) -> Option<String> {
        match self {
            Self::Pronoun { pronoun_case, .. } => pronoun_case
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_possessive_relation(&self) -> Option<String> {
        match self {
            Self::Determiner {
                possessive_relation,
                ..
            }
            | Self::Pronoun {
                possessive_relation,
                ..
            } => possessive_relation
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

    fn __pivot_diathesis(&self) -> Option<String> {
        match self {
            Self::Verb { diathesis, .. } => diathesis
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    pub const PIVOT_DEGREE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "degree",
            "Degree",
            <NorwegianBokmalDegree as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_degree,
        );

    pub const PIVOT_PARTICIPLE_KIND: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "participle_kind",
            "Participle Kind",
            <NorwegianBokmalParticipleKind as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_participle_kind,
        );

    pub const PIVOT_PRONOUN_CASE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "pronoun_case",
            "Pronoun Case",
            <NorwegianBokmalPronounCase as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_pronoun_case,
        );

    pub const PIVOT_POSSESSIVE_RELATION: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "possessive_relation",
            "Possessive Relation",
            <NorwegianBokmalPossessiveRelation as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_possessive_relation,
        );

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <NorwegianBokmalMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <NorwegianBokmalTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    pub const PIVOT_DIATHESIS: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "diathesis",
            "Diathesis",
            <NorwegianBokmalDiathesis as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_diathesis,
        );
}

pub struct NorwegianBokmal;

impl LinguisticDefinition for NorwegianBokmal {
    type Morphology = NorwegianBokmalMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Nob;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        NorwegianBokmalMorphology::PIVOT_NOMINAL_GENDER,
        NorwegianBokmalMorphology::PIVOT_NUMBER,
        NorwegianBokmalMorphology::PIVOT_DEFINITENESS,
        NorwegianBokmalMorphology::PIVOT_NOMINAL_CASE,
        NorwegianBokmalMorphology::PIVOT_DEGREE,
        NorwegianBokmalMorphology::PIVOT_PARTICIPLE_KIND,
        NorwegianBokmalMorphology::PIVOT_PRONOUN_CASE,
        NorwegianBokmalMorphology::PIVOT_POSSESSIVE_RELATION,
        NorwegianBokmalMorphology::PIVOT_VERB_CLASS,
        NorwegianBokmalMorphology::PIVOT_VERB_FORM,
        NorwegianBokmalMorphology::PIVOT_MOOD,
        NorwegianBokmalMorphology::PIVOT_TENSE,
        NorwegianBokmalMorphology::PIVOT_DIATHESIS,
    ];

    fn supported_scripts(&self) -> &[Script] {
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
                Upos::Pronoun,
                Upos::Determiner,
            ]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        concat!(
            "1. Scope and lemmatization: analyze contemporary written Norwegian Bokmål, never Nynorsk and never an undifferentiated 'Norwegian' macrolanguage. Preserve the standard letters æ, ø and å and any internally consistent permitted Bokmål forms. Lemmatize common nouns to singular indefinite form, retaining a conventional plural lemma for pluralia tantum; verbs to the complete active infinitive, except that a lexical s-verb keeps its -s citation form (trives stays trives); ordinary adjectives and degree-bearing adverbs to the positive unmarked form; compounds to the whole compound. Lemmatize personal-pronoun object forms to their subject paradigm member (meg -> jeg, henne -> hun, dem -> de) and possessives to the masculine/common singular member (mi/mitt/mine -> min, si/sitt/sine -> sin).\n",
            "2. Nouns: always report nominal_gender, number, morphological definiteness and nominal_case. Bokmål permits masculine, feminine and neuter paradigms. For nouns whose gender is normatively optional, follow the paradigm consistently realized by the text: ei bok/boka is feminine, while en bok/boken is masculine; never infer that every traditional feminine noun must use the feminine paradigm, and never mix ei bok with boken as one local paradigm. Preserve lexical gender in the plural, where the surface ending may not reveal it. Bil/biler are indefinite and bilen/bilene definite; a noun after a possessive or genitive stays morphologically indefinite even though the phrase is referentially definite (min bil, Annas bil).\n",
            "3. Nominal case and compounds: ordinary subject, object and prepositional-complement nouns are unmarked; never invent accusative or dative on nouns. Use nominal_case genitive only when the written nominal token carries possessive -s (kongens, Norges). The -s has phrasal distribution, but attach it to the final written nominal token in this analysis. Never mistake a compound linking -s for genitive (arbeidsdag is one unmarked compound). Keep established compounds as one token, lemmatize them as wholes, and use the actual compound's gender rather than blindly copying the final element.\n",
            "4. Adjectives, adverbs and degree: add degree positive, comparative or superlative only where comparison genuinely applies. God is positive, bedre contrasts as comparative and best as superlative; a relational adjective such as medisinsk MUST omit degree. For adverbs, ofte is positive, oftere contrasts as comparative and oftest as superlative, while a non-gradable temporal adverb such as nå MUST omit degree. Ordinary positive adjectives agree: stor is masculine/feminine singular, stort contrasts as neuter singular, and store as plural or weak/definite. Report agreement_gender for singular agreeing forms, agreement_number where the paradigm participates, and agreement_definiteness only for attributive forms (en stor bil is indefinite versus den store bilen definite). A predicate adjective MUST omit agreement_definiteness (bilen er stor); an invariant comparative such as bedre MUST omit all three agreement fields rather than copying them from its head.\n",
            "5. Participles and adjective use: an adjectivally used present participle takes participle_kind present (en skinnende sol), while an agreeing adjectival past participle takes past (de skrevne reglene); an ordinary adjective such as stor MUST omit participle_kind. Present participles are invariant and omit agreement fields. An adjectival past participle reports the agreement its form participates in. In analytic verb phrases the invariant perfect form is a Verb with verb_form supine, not an Adjective: har skrevet and blir skrevet each contain a separate finite auxiliary plus supine skrevet.\n",
            "6. Determiners: classify articles, demonstratives, possessives, quantifiers, interrogatives and relatives by function in context; a substantively used form is a Pronoun. Determined_number and determined_gender describe the selected noun, never the possessor. In the singular, en contrasts with feminine ei and neuter et; in the plural de/disse MUST omit determined_gender because their form does not distinguish it. Articles encode definiteness (en/ei/et indefinite versus den/det/de definite), while a possessive such as min MUST omit determined_definiteness. Respect double definiteness in modified definite noun phrases (den store bilen), but do not assign the noun phrase's semantic definiteness to a morphologically indefinite noun after a possessive (den store bilen versus min store bil).\n",
            "7. Possessives: report possessor_person on min first versus din second, and omit it on a non-possessive determiner such as den. Report possessor_number on min for a singular possessor versus vår for a plural possessor, and omit it on den. Third-person non-reflexives encode referential possessor gender: hans is masculine, hennes feminine and hens epicene; vår contrasts by having no possessor_gender and MUST omit it. Sin/si/sitt/sine takes possessive_relation reflexive only when coreferential with the clause subject; hans/hennes/hens/deres is non_reflexive, while first- and second-person min/din MUST omit possessive_relation. Resolve the agreement of min/mi/mitt/mine and sin/si/sitt/sine against the possessed noun, not the possessor.\n",
            "8. Pronouns: classify the type and resolve syncretic forms from syntax. Personal pronouns report person and pronoun_number: jeg is first versus du second, and jeg singular contrasts with vi plural; an interrogative hvem MUST omit both fields. Use pronoun_case subject for jeg in subject position and object for meg after a verb or preposition; an interrogative hvem MUST omit case. Han has referential_gender masculine, hun feminine and personal hen epicene; det instead uses nominal_gender neuter, den uses the antecedent noun's masculine or feminine nominal gender, and han/hun/hen MUST omit nominal_gender. Conversely, den/det MUST omit referential_gender. A substantively used possessive is a Pronoun: in elliptical han tok sin, sin is reflexive; han tok hans contrasts as non_reflexive; jeg tok min MUST omit possessive_relation because the third-person opposition does not apply. Every non-possessive pronoun also omits possessive_relation.\n",
            "9. Verb classes: report verb_class on every verb token, including auxiliaries. weak_group_1 has -et or accepted -a past and supine (snakke, snakket/snakka, snakket/snakka); weak_group_2 has -te/-t (kjøpe, kjøpte, kjøpt); weak_group_3 has -de/-d (prøve, prøvde, prøvd); weak_group_4 has -dde/-dd on a vowel stem (bo, bodde, bodd). Strong verbs have a suffixless, usually vowel-changing past (skrive, skrev, skrevet). Use irregular only for suppletive or residual paradigms that those patterns do not describe, such as være/er/var/vært; do not label every strong verb irregular. Follow the standard variant realized by the text when Bokmål permits alternative principal parts.\n",
            "10. Verb forms and finite features: every verb has verb_form. A finite indicative has mood and present/past tense: skriver is present and skrev is past. An imperative such as skriv contrasts with indicative but MUST omit tense. Infinitives, supines and present participles MUST omit both mood and tense. Bokmål verbs do not agree with person or number, so never invent either feature. The infinitive marker å is a separate infinitival Particle, not part of the lemma or verb token.\n",
            "11. Diathesis and analytic constructions: report active on an ordinary finite or infinitive form (bruker/bruke) and s_form on the corresponding written -s form (brukes); a supine such as brukt contrasts by having no diathesis and MUST omit it, as must a present participle. s_form names morphology, not passive meaning alone: include synthetic passives, lexical s-verbs/deponents such as trives, reciprocals such as møtes and middle readings. Analyze analytic constructions token by token: har skrevet is present active ha plus supine skrive; hadde skrevet uses past active ha plus the same supine; skal skrive is present active skulle plus an infinitive; blir skrevet is present active bli plus a supine. Never invent perfect, pluperfect, future or passive as a tense value on one token.\n",
            "12. Particles, word order and tokenization: å before an infinitive is an infinitival Particle; ikke is a negation Particle. A stressed free element of a particle verb (slå av, finne ut) is a verb_particle, while the Verb keeps the complete lexical expression as its lemma only when the dictionary treats that expression as the lexeme; let the Multiword Expressions component record the combination. Use Bokmål V2 main-clause order and subordinate-clause placement of sentence adverbs only to resolve ambiguous forms, not as morphology to encode. Preserve one token per orthographic word, keep compounds intact, and never emit punctuation as a token."
        )
    }
}

#[cfg(test)]
mod tests {
    use panini_core::aggregable::ClosedValues;

    use super::*;

    #[test]
    fn bokmal_identity_script_and_typology_are_exact() {
        let language = NorwegianBokmal;

        assert_eq!(NorwegianBokmal::ISO_LANG, IsoLang::Nob);
        assert_eq!(NorwegianBokmal::ISO_LANG.to_639_3(), "nob");
        assert_eq!(language.supported_scripts(), &[Script::LATN]);
        assert_eq!(language.default_script(), Script::LATN);
        assert_eq!(
            language.typological_features(),
            &[
                TypologicalFeature::Conjugation(&[Upos::Verb]),
                TypologicalFeature::Declension(&[
                    Upos::Noun,
                    Upos::ProperNoun,
                    Upos::Adjective,
                    Upos::Pronoun,
                    Upos::Determiner,
                ]),
            ]
        );
    }

    #[test]
    fn verb_classes_publish_learner_facing_wire_values() {
        assert_eq!(
            NorwegianBokmalVerbClass::all_variants(),
            &[
                "weak_group_1",
                "weak_group_2",
                "weak_group_3",
                "weak_group_4",
                "strong",
                "irregular",
            ]
        );
    }

    #[test]
    fn feminine_and_masculine_bokmal_paradigms_remain_distinct() {
        let boka = NorwegianBokmalMorphology::Noun {
            lemma: "bok".to_string(),
            nominal_gender: TernaryGender::Feminine,
            number: BinaryNumber::Singular,
            definiteness: NorwegianBokmalDefiniteness::Definite,
            nominal_case: NorwegianBokmalNominalCase::Unmarked,
        };
        let boken = NorwegianBokmalMorphology::Noun {
            lemma: "bok".to_string(),
            nominal_gender: TernaryGender::Masculine,
            number: BinaryNumber::Singular,
            definiteness: NorwegianBokmalDefiniteness::Definite,
            nominal_case: NorwegianBokmalNominalCase::Unmarked,
        };

        assert_eq!(
            NorwegianBokmalMorphology::PIVOT_NOMINAL_GENDER.value(&boka),
            Some("feminine".to_string())
        );
        assert_eq!(
            NorwegianBokmalMorphology::PIVOT_NOMINAL_GENDER.value(&boken),
            Some("masculine".to_string())
        );
    }

    #[test]
    fn optional_verb_dimensions_remain_closed_pivots() {
        let skriver = NorwegianBokmalMorphology::Verb {
            lemma: "skrive".to_string(),
            verb_class: NorwegianBokmalVerbClass::Strong,
            verb_form: NorwegianBokmalVerbForm::Finite,
            mood: Some(NorwegianBokmalMood::Indicative),
            tense: Some(NorwegianBokmalTense::Present),
            diathesis: Some(NorwegianBokmalDiathesis::Active),
        };
        let skrevet = NorwegianBokmalMorphology::Verb {
            lemma: "skrive".to_string(),
            verb_class: NorwegianBokmalVerbClass::Strong,
            verb_form: NorwegianBokmalVerbForm::Supine,
            mood: None,
            tense: None,
            diathesis: None,
        };

        assert_eq!(
            NorwegianBokmalMorphology::PIVOT_MOOD.value(&skriver),
            Some("indicative".to_string())
        );
        assert_eq!(
            NorwegianBokmalMorphology::PIVOT_TENSE.value(&skriver),
            Some("present".to_string())
        );
        assert_eq!(NorwegianBokmalMorphology::PIVOT_MOOD.value(&skrevet), None);
        assert_eq!(
            NorwegianBokmalMorphology::PIVOT_TENSE.value(&skrevet),
            None
        );
        assert_eq!(
            NorwegianBokmalMorphology::PIVOT_DIATHESIS.value(&skrevet),
            None
        );
    }

    #[test]
    fn personal_hen_is_not_grammatical_neuter() {
        let hen = NorwegianBokmalMorphology::Pronoun {
            lemma: "hen".to_string(),
            pronoun_type: NorwegianBokmalPronounType::Personal,
            person: Some(Person::Third),
            pronoun_number: Some(BinaryNumber::Singular),
            referential_gender: Some(NorwegianBokmalReferentialGender::Epicene),
            nominal_gender: None,
            pronoun_case: Some(NorwegianBokmalPronounCase::Subject),
            possessive_relation: None,
        };

        let serialized = serde_json::to_value(hen).unwrap();
        assert_eq!(serialized["referential_gender"], "epicene");
        assert!(serialized.get("nominal_gender").is_none());
    }
}
