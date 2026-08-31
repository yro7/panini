use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature, Upos,
};

/// The two noun classes of contemporary Standard Swedish.
///
/// `Common` is *utrum* (the `en` class), historically the merger of masculine
/// and feminine. `Neuter` is *neutrum* (the `ett` class). Natural gender in
/// personal pronouns is a different system and is represented separately by
/// [`SwedishReferentialGender`].
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
pub enum SwedishNominalGender {
    Common,
    Neuter,
}

/// Morphological definiteness (*species*) on Swedish nominals.
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
pub enum SwedishDefiniteness {
    Indefinite,
    Definite,
}

/// The two forms conventionally called case on contemporary Swedish nouns.
///
/// `Nominative` is the traditional label for the unmarked *grundform*, used
/// for objects as well as subjects. The genitive `-s` is phrasal in its
/// distribution, but it is still an overt form that a learner must produce.
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
pub enum SwedishNominalCase {
    Nominative,
    Genitive,
}

/// The living case paradigm of contemporary Swedish personal pronouns.
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
pub enum SwedishPronounCase {
    Nominative,
    Accusative,
    Genitive,
}

/// A noun's productive plural pattern.
///
/// Svenska Akademiens grammatik distinguishes seven declensions by their
/// plural suffix. Naming the values after the suffix is more useful to a
/// learner than exposing the arbitrary traditional numbers.
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
pub enum SwedishNounDeclension {
    /// First declension: `gata` -> `gator`.
    PluralOr,
    /// Second declension: `bil` -> `bilar`.
    PluralAr,
    /// Third declension: `balkong` -> `balkonger`.
    PluralEr,
    /// Fourth declension: `sko` -> `skor`.
    PluralR,
    /// Fifth declension: `hjärta` -> `hjärtan`.
    PluralN,
    /// Sixth declension: no plural suffix, possibly with stem alternation
    /// (`hus` -> `hus`, `man` -> `män`).
    ZeroPlural,
    /// Seventh declension, principally loans: `slogan` -> `slogans`.
    PluralS,
    /// A minor pattern outside the seven suffix classes, such as
    /// `faktum` -> `fakta` or `öga` -> `ögon`.
    Other,
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
pub enum SwedishDegree {
    Positive,
    Comparative,
    Superlative,
}

/// Participles used adjectivally, where their agreement belongs to the
/// adjective system rather than to the finite verb paradigm.
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
pub enum SwedishParticipleKind {
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
pub enum SwedishDeterminerType {
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
pub enum SwedishPronounType {
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

/// Natural-gender reference by personal pronouns.
///
/// `Epicene` is the sex/gender-neutral personal `hen`; it must not be folded
/// into grammatical neuter, which is the separate `det` class.
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
pub enum SwedishReferentialGender {
    Masculine,
    Feminine,
    Epicene,
}

/// Whether a possessive points back to the clause subject.
///
/// This is the contrast between `sin/sitt/sina` and the non-reflexive
/// `hans/hennes/hens/deras`, not a general semantic property of possession.
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
pub enum SwedishPossessiveRelation {
    Reflexive,
    NonReflexive,
}

/// The limited singular address contrast between ordinary `du` and formal
/// `ni`. Plural `ni` has no register value because it is simply the plural
/// second-person pronoun.
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
pub enum SwedishAddressRegister {
    Familiar,
    Formal,
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
pub enum SwedishParticleType {
    Infinitival,
    Negation,
    VerbParticle,
}

/// The two synthetic tenses of contemporary Swedish. Perfect, pluperfect and
/// future constructions are periphrastic and are analysed token by token.
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
pub enum SwedishTense {
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
pub enum SwedishMood {
    Indicative,
    Imperative,
    /// Surviving conjunctive forms such as `vore` and `funnes`.
    Subjunctive,
    /// Productive only in a small set of formulae such as `leve` and
    /// `frid vare med dig`.
    Optative,
}

/// Slots in the Swedish verb paradigm.
///
/// The supine is deliberately distinct from the past participle: `skrivit` is
/// invariant after `ha`, while `skriven/skrivet/skrivna` agrees like an
/// adjective or participates in a `bli` passive.
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
pub enum SwedishVerbForm {
    Finite,
    Infinitive,
    Supine,
    PresentParticiple,
    PastParticiple,
}

/// The learner-facing conjugation class of a Swedish verb lemma.
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
pub enum SwedishVerbClass {
    /// Weak verbs in `-ar, -ade, -at`: `tala`.
    #[serde(rename = "group_1")]
    Group1,
    /// Weak consonant-stem verbs with preterite `-de`: `stänga`.
    #[serde(rename = "group_2a")]
    Group2A,
    /// Weak consonant-stem verbs with preterite `-te`: `köpa`.
    #[serde(rename = "group_2b")]
    Group2B,
    /// Weak stressed-vowel stems in `-dde, -tt`: `bo`.
    #[serde(rename = "group_3")]
    Group3,
    /// Strong verbs with a suffixless, usually ablauting preterite: `skriva`.
    #[serde(rename = "group_4")]
    Group4,
    /// Suppletive and other paradigms that do not fit the four productive
    /// classes, including the small half-weak set.
    #[serde(rename = "irregular")]
    Irregular,
}

/// Morphological diathesis of forms that participate in the active/`-s`
/// opposition.
///
/// `SForm` is intentionally not named `Passive`: Swedish `-s` also marks
/// deponent, reciprocal and middle readings (`hoppas`, `mötas`, `trivas`).
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
pub enum SwedishDiathesis {
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
pub enum SwedishMorphology {
    Adjective {
        lemma: String,
        /// Only for a genuinely gradable adjective whose own form carries the
        /// degree. Relational adjectives and adjectival participles omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<SwedishDegree>,
        /// Present only when this token is a participle used adjectivally.
        #[serde(skip_serializing_if = "Option::is_none")]
        participle_kind: Option<SwedishParticipleKind>,
        /// Agreement features are absent on invariant forms such as
        /// comparatives and present participles.
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_gender: Option<SwedishNominalGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_definiteness: Option<SwedishDefiniteness>,
        /// Only when an independently used adjective heads a genitive nominal
        /// phrase (`den gamles`). Ordinary modifiers omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        adjectival_case: Option<SwedishNominalCase>,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        /// Only where the adverb itself carries comparison.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<SwedishDegree>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: SwedishDeterminerType,
        /// Number and (in the singular) gender of the determined noun, never
        /// the number or gender of a possessor.
        determined_number: BinaryNumber,
        #[serde(skip_serializing_if = "Option::is_none")]
        determined_gender: Option<SwedishNominalGender>,
        /// Articles and demonstratives have a morphological definiteness;
        /// quantifiers and possessives may omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        determined_definiteness: Option<SwedishDefiniteness>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_gender: Option<SwedishReferentialGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessive_relation: Option<SwedishPossessiveRelation>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        /// Lexical `en`/`ett` class, retained in the plural.
        nominal_gender: SwedishNominalGender,
        number: BinaryNumber,
        definiteness: SwedishDefiniteness,
        nominal_case: SwedishNominalCase,
        noun_declension: SwedishNounDeclension,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_type: SwedishParticleType,
    },
    Pronoun {
        lemma: String,
        pronoun_type: SwedishPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pronoun_number: Option<BinaryNumber>,
        /// `han`, `hon`, and sex/gender-neutral personal `hen` only.
        #[serde(skip_serializing_if = "Option::is_none")]
        referential_gender: Option<SwedishReferentialGender>,
        /// Pronominal `den`/`det` and forms agreeing with an `en`/`ett` noun.
        #[serde(skip_serializing_if = "Option::is_none")]
        nominal_gender: Option<SwedishNominalGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pronoun_case: Option<SwedishPronounCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessive_relation: Option<SwedishPossessiveRelation>,
        /// Singular second-person address only. Ordinary plural `ni` omits it.
        #[serde(skip_serializing_if = "Option::is_none")]
        address_register: Option<SwedishAddressRegister>,
    },
    ProperNoun {
        lemma: String,
        number: BinaryNumber,
        nominal_case: SwedishNominalCase,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        /// Lexical class, reported on every token of the lemma.
        verb_class: SwedishVerbClass,
        verb_form: SwedishVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<SwedishMood>,
        /// Finite indicative and subjunctive forms only. Imperatives and
        /// formulaic optatives have no tense in this model.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<SwedishTense>,
        /// Finite, infinitive and supine forms where active and `-s` forms
        /// contrast. Participles omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        diathesis: Option<SwedishDiathesis>,
    },
    Other {
        lemma: String,
    },
}

impl SwedishMorphology {
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

    fn __pivot_address_register(&self) -> Option<String> {
        match self {
            Self::Pronoun {
                address_register, ..
            } => address_register
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
            <SwedishDegree as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_degree,
        );

    pub const PIVOT_PARTICIPLE_KIND: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "participle_kind",
            "Participle Kind",
            <SwedishParticipleKind as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_participle_kind,
        );

    pub const PIVOT_PRONOUN_CASE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "pronoun_case",
            "Pronoun Case",
            <SwedishPronounCase as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_pronoun_case,
        );

    pub const PIVOT_POSSESSIVE_RELATION: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "possessive_relation",
            "Possessive Relation",
            <SwedishPossessiveRelation as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_possessive_relation,
        );

    pub const PIVOT_ADDRESS_REGISTER: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "address_register",
            "Address Register",
            <SwedishAddressRegister as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_address_register,
        );

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <SwedishMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <SwedishTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    pub const PIVOT_DIATHESIS: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "diathesis",
            "Diathesis",
            <SwedishDiathesis as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_diathesis,
        );
}

pub struct Swedish;

impl LinguisticDefinition for Swedish {
    type Morphology = SwedishMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Swe;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        SwedishMorphology::PIVOT_NOMINAL_GENDER,
        SwedishMorphology::PIVOT_NUMBER,
        SwedishMorphology::PIVOT_DEFINITENESS,
        SwedishMorphology::PIVOT_NOMINAL_CASE,
        SwedishMorphology::PIVOT_NOUN_DECLENSION,
        SwedishMorphology::PIVOT_DEGREE,
        SwedishMorphology::PIVOT_PARTICIPLE_KIND,
        SwedishMorphology::PIVOT_PRONOUN_CASE,
        SwedishMorphology::PIVOT_POSSESSIVE_RELATION,
        SwedishMorphology::PIVOT_ADDRESS_REGISTER,
        SwedishMorphology::PIVOT_VERB_CLASS,
        SwedishMorphology::PIVOT_VERB_FORM,
        SwedishMorphology::PIVOT_MOOD,
        SwedishMorphology::PIVOT_TENSE,
        SwedishMorphology::PIVOT_DIATHESIS,
    ];

    /// Contemporary Standard Swedish is written in the Latin script. Historic
    /// runes are not a present-day alternative writing system for this course.
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
            "1. Scope and lemmatization: analyze contemporary Standard Swedish as written in both Sweden and Swedish-speaking Finland without rewriting a legitimate standard regional form into the other variety. Lemmatize common nouns to singular indefinite form, retaining a plural dictionary form for pluralia tantum; verbs to the complete active infinitive, except that a lexical deponent keeps its -s citation form (hoppas stays hoppas); ordinary adjectives and degree-bearing adverbs to the positive unmarked form; adjectival participles to the unmarked common-singular participle (skrivna -> skriven), not to the verb; compounds to the whole compound. Lemmatize personal-pronoun forms to the nominative paradigm member (mig -> jag, dem -> de) and inflected possessives to the common-singular member (mitt/mina -> min, sitt/sina -> sin).\n",
            "2. Nouns: always report nominal_gender, number, morphological definiteness, nominal_case and noun_declension. Nominal gender is ONLY common (utrum/en-word) or neuter (ett-word); preserve this lexical gender in the plural, where no article reveals it. Definiteness names the written noun form: bil/bilar are indefinite and bilen/bilarna definite. A noun after a possessive or genitive stays morphologically indefinite even though the noun phrase is semantically definite (min stora bil, Annas stora bil). A compound normally inherits gender and plural class from its final lexical head, but use the established paradigm of the actual compound rather than guessing from spelling alone.\n",
            "3. Noun declensions: classify the lemma by its real plural paradigm, not merely by the ending visible on this token. plural_or is gata -> gator; plural_ar is bil -> bilar; plural_er is balkong -> balkonger; plural_r is sko -> skor; plural_n is hjärta -> hjärtan; zero_plural has no plural suffix, including both hus -> hus and stem-changing man -> män; plural_s is the restricted loan pattern slogan -> slogans; other is reserved for minor paradigms outside those seven suffix classes, such as faktum -> fakta and öga -> ögon. A plural-only or mass noun still receives the dictionary paradigm it belongs to when that paradigm is established.\n",
            "4. Nominal case and compounds: ordinary subject AND object nouns are nominative/unmarked; Swedish nouns do not acquire accusative or dative case from syntax or a preposition. Use nominal_case genitive only when the written nominal carries the possessive -s. The -s has phrasal distribution and attaches at the right edge, but it belongs to the final written nominal token for this analysis (kungens, Sveriges). Never interpret a compound linking -s as genitive: arbetsdag and stadshus are single nominative compound nouns. Keep every ordinary written compound as one token and lemmatize it as a whole.\n",
            "5. Adjectives: report degree only when the adjective's own form expresses positive, comparative or superlative; omit it on non-gradable relational adjectives and on participles. For forms that participate in agreement, report agreement_gender in the singular, agreement_number and agreement_definiteness. The unmarked common singular, neuter -t, plural/definite -a and optional definite masculine -e forms must be resolved from syntax: en stor bil, ett stort hus, stora bilar, den stora bilen, den gamle mannen; predicate adjectives also agree (bilen är stor, huset är stort, bilarna är stora) but are morphologically indefinite. Comparatives are invariant and omit all agreement fields. An uninflected predicative superlative (hon är störst) omits agreement; an attributive definite superlative (den största staden) reports the agreement it carries. An independently used adjective gets adjectival_case genitive only when the adjective itself bears final -s (den gamles); ordinary attributive adjectives omit adjectival_case. Ordinal numerals used attributively are Adjectives, not Numerals.\n",
            "6. Participles: participles used as attributes or ordinary predicates are Adjectives with participle_kind present or past. Present participles in -ande/-ende are invariant and omit agreement fields. Past participles agree like adjectives (en skriven text, ett skrivet brev, skrivna texter) and report the applicable agreement features. The supine is NEVER a participle: har skrivit is a Verb with verb_form supine and cannot agree. Only a past participle serving as the lexical verb in a periphrastic bli-passive is a Verb with verb_form past_participle; bli is a separate finite Verb token.\n",
            "7. Determiners and pronouns: classify articles, demonstratives, possessives, quantifiers, interrogatives and relatives by their function in context. A form modifying an overt noun is a Determiner; a form heading the noun phrase is a Pronoun. Determiner determined_number and determined_gender describe the noun selected, never the possessor. Articles and demonstratives report their morphological definiteness; possessives and quantifiers omit it when the category is not encoded. Personal pronouns use pronoun_case nominative as subjects, accusative as objects/obliques and genitive for possessive forms; other pronoun types omit case unless their form genuinely contrasts. Resolve de/dem (and colloquial dom) from syntax, never pronunciation or spelling alone.\n",
            "8. Pronoun gender, possession and address: han and hon take referential_gender masculine/feminine; personal hen is epicene, NEVER neuter. Pronominal den/det instead take nominal_gender common/neuter. Third-person reflexive possessive sin/sitt/sina is possessive_relation reflexive only when it is coreferential with the clause subject; hans, hennes, hens and deras are non_reflexive. First- and second-person min/din/vår/er do not participate in that third-person reflexive choice and omit possessive_relation. Report a possessive determiner's possessor_person, possessor_number and possessor_gender only where the form and context support them. Ordinary singular du has address_register familiar. Singular formal ni has formal; plural ni is simply second-person plural and omits address_register. Do not infer formal register from capitalization at sentence start.\n",
            "9. Verb classes: report verb_class on EVERY verb token, including auxiliaries and forms where the class is not visible. group_1 has -ar/-ade/-at (tala, talar, talade, talat); group_2a is a weak consonant-stem verb with preterite -de (stänga, stänger, stängde, stängt); group_2b has -te after a voiceless stem ending (köpa, köper, köpte, köpt); group_3 has a stressed-vowel stem and -dde/-tt (bo, bor, bodde, bott); group_4 is strong, with a suffixless and usually ablauting past plus normally -it in the supine (skriva, skriver, skrev, skrivit). Use irregular only for genuinely suppletive or residual paradigms that those four classes do not describe; do not label every stem-changing group_4 verb irregular.\n",
            "10. Verb forms and finite features: every verb has verb_form, including vara, ha, bli, ska and modal verbs. A finite indicative or surviving subjunctive gets present/past tense and mood. An imperative gets mood imperative and no tense. Use subjunctive only for a genuine surviving form such as vore or funnes, not for an ordinary past indicative used conditionally. Use optative only for the restricted formulaic paradigm (leve, vare, bevare), with no tense. Infinitives, supines and participles get no mood or tense. Modern Swedish verbs NEVER agree with the subject, so do not invent person or number features.\n",
            "11. Diathesis and analytic verb phrases: report diathesis active or s_form on finite, infinitive and supine forms where the opposition applies; omit it on participles. s_form describes the morphological -s form, not passive meaning alone: include synthetic passives (boken läses), deponents (hoppas, minnas, trivas), reciprocals (mötas) and middle readings. Analyze each token of an analytic construction separately: har skrivit is finite present active ha plus active supine skriva; hade skrivits uses finite past active ha plus s_form supine skriva; ska skriva and kommer att skriva contain separate finite and infinitive verbs; blir skriven is finite active bli plus a past_participle Verb. Never invent perfect, future or pluperfect as a tense value on one token.\n",
            "12. Word order, particles and tokenization: V2 syntax and the BIFF rule are syntactic context for resolving forms, not morphology to encode: a main-clause finite verb follows the first constituent, while subordinate inte normally precedes the finite verb. Infinitival att is a Particle with particle_type infinitival; subordinating att is a SubordinatingConjunction. Inte, icke and ej are negation Particles. A stressed free particle in a particle verb (tycka om, slå på) is a verb_particle token; keep the Verb lemma simplex and let the Multiword Expressions component record the lexical combination. Preserve one token per orthographic word, keep compounds intact, and never emit punctuation as a token."
        )
    }
}

#[cfg(test)]
mod tests {
    use panini_core::aggregable::ClosedValues;

    use super::*;

    #[test]
    fn swedish_identity_script_and_typology_are_exact() {
        let language = Swedish;

        assert_eq!(Swedish::ISO_LANG, IsoLang::Swe);
        assert_eq!(Swedish::ISO_LANG.to_639_3(), "swe");
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
    fn noun_declensions_publish_suffix_based_wire_values() {
        assert_eq!(
            SwedishNounDeclension::all_variants(),
            &[
                "plural_or",
                "plural_ar",
                "plural_er",
                "plural_r",
                "plural_n",
                "zero_plural",
                "plural_s",
                "other",
            ]
        );
    }

    #[test]
    fn verb_groups_have_unambiguous_wire_values() {
        assert_eq!(
            SwedishVerbClass::all_variants(),
            &[
                "group_1",
                "group_2a",
                "group_2b",
                "group_3",
                "group_4",
                "irregular",
            ]
        );
    }

    #[test]
    fn supine_and_past_participle_are_distinct_forms() {
        assert_ne!(
            SwedishVerbForm::Supine.variant_str(),
            SwedishVerbForm::PastParticiple.variant_str()
        );
        assert_eq!(SwedishVerbForm::Supine.variant_str(), "supine");
        assert_eq!(
            SwedishVerbForm::PastParticiple.variant_str(),
            "past_participle"
        );
    }

    #[test]
    fn optional_dimensions_remain_closed_pivots() {
        let skrivit = SwedishMorphology::Verb {
            lemma: "skriva".to_string(),
            verb_class: SwedishVerbClass::Group4,
            verb_form: SwedishVerbForm::Supine,
            mood: None,
            tense: None,
            diathesis: Some(SwedishDiathesis::Active),
        };
        let vore = SwedishMorphology::Verb {
            lemma: "vara".to_string(),
            verb_class: SwedishVerbClass::Irregular,
            verb_form: SwedishVerbForm::Finite,
            mood: Some(SwedishMood::Subjunctive),
            tense: Some(SwedishTense::Past),
            diathesis: Some(SwedishDiathesis::Active),
        };

        assert_eq!(SwedishMorphology::PIVOT_MOOD.value(&skrivit), None);
        assert_eq!(SwedishMorphology::PIVOT_TENSE.value(&skrivit), None);
        assert_eq!(
            SwedishMorphology::PIVOT_DIATHESIS.value(&skrivit),
            Some("active".to_string())
        );
        assert_eq!(
            SwedishMorphology::PIVOT_MOOD.value(&vore),
            Some("subjunctive".to_string())
        );
        assert_eq!(
            SwedishMorphology::PIVOT_TENSE.value(&vore),
            Some("past".to_string())
        );
    }

    #[test]
    fn epicene_personal_gender_does_not_collapse_into_neuter_noun_gender() {
        let hen = SwedishMorphology::Pronoun {
            lemma: "hen".to_string(),
            pronoun_type: SwedishPronounType::Personal,
            person: Some(Person::Third),
            pronoun_number: Some(BinaryNumber::Singular),
            referential_gender: Some(SwedishReferentialGender::Epicene),
            nominal_gender: None,
            pronoun_case: Some(SwedishPronounCase::Nominative),
            possessive_relation: None,
            address_register: None,
        };

        let serialized = serde_json::to_value(hen).unwrap();
        assert_eq!(serialized["referential_gender"], "epicene");
        assert!(serialized.get("nominal_gender").is_none());
    }
}
