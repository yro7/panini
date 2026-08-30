use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryGender,
    TypologicalFeature, Upos,
};

/// The two agreement classes of contemporary Standard Dutch nouns.
///
/// `Common` is the class selected by singular `de`; it contains the historical
/// masculine and feminine genders, which no longer contrast in articles or
/// attributive adjective endings. `Neuter` is selected by singular `het`.
/// Personal pronouns retain a separate three-gender contrast, represented by
/// `pronominal_gender` rather than by widening this genuinely binary system.
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
pub enum DutchNominalGender {
    Common,
    Neuter,
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
pub enum DutchDefiniteness {
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
pub enum DutchDegree {
    Positive,
    Comparative,
    Superlative,
}

/// The two tenses expressed by a synthetic Dutch verb form.
///
/// Future, perfect and conditional readings are periphrastic and are analysed
/// token by token. `zal werken` is present `zullen` plus an infinitive, while
/// `zou werken` is past `zullen` plus an infinitive.
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
pub enum DutchTense {
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
pub enum DutchMood {
    Indicative,
    Imperative,
    /// The marginal contemporary subjunctive in formulae such as `leve de
    /// koning`, `men neme` and `het zij zo`.
    Subjunctive,
}

/// The productive finite/non-finite division of Dutch verbal morphology.
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
pub enum DutchVerbForm {
    Finite,
    Infinitive,
    PresentParticiple,
    PastParticiple,
}

/// The principal-parts class of a Dutch verb lexeme.
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
pub enum DutchVerbClass {
    /// Suffixing (traditionally weak): `werken`, `werkte`, `gewerkt`.
    Weak,
    /// Ablauting (traditionally strong): `lopen`, `liep`, `gelopen`.
    Strong,
    /// Neither productive weak inflection nor a strong ablaut paradigm, for
    /// example `zeggen`, `zei`, `gezegd` and the highly irregular auxiliaries.
    Irregular,
}

/// Whether a complex verb separates under Dutch verb-second syntax.
///
/// This is absent on a simplex verb. It is not predictable from the letters of
/// the prefix alone: stress and meaning distinguish pairs such as separable
/// `voorkomen` (occur) and inseparable `voorkomen` (prevent).
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
pub enum DutchSeparability {
    Separable,
    Inseparable,
}

/// The living case opposition of Dutch pronouns.
///
/// Nouns no longer have a productive case paradigm. Personal pronouns retain a
/// subject form against an object/oblique form; the prescriptive `hen`/`hun`
/// distinction lives inside that oblique domain rather than recreating a full
/// nominal accusative/dative system.
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
pub enum DutchPronounCase {
    Nominative,
    Oblique,
}

/// The syntactic role that selects an oblique personal-pronoun form.
///
/// This keeps the standard `hen`/`hun` opposition representable without
/// pretending that Dutch nouns still have a productive case paradigm.
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
pub enum DutchPronounRole {
    DirectObject,
    IndirectObject,
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
pub enum DutchPronounType {
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

/// Full/stressed against reduced/unstressed pronominal forms (`jij`/`je`,
/// `wij`/`we`, `mij`/`me`).
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
pub enum DutchPronounStrength {
    Strong,
    Weak,
}

/// The `jij/je` versus `u` address distinction.
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
pub enum DutchPoliteness {
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
pub enum DutchDeterminerType {
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
pub enum DutchParticleType {
    Modal,
    Negation,
    Infinitival,
    SeparatedVerbPrefix,
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
pub enum DutchMorphology {
    /// Adjectives, including ordinals and participles used attributively as
    /// adjectives.
    Adjective {
        lemma: String,
        /// Only gradable adjectives have a degree. Relational, material and
        /// absolute adjectives such as `medisch`, `houten` and `rechter` do
        /// not acquire a fictitious positive degree.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<DutchDegree>,
        /// Whether this occurrence carries an overt inflectional `-e`.
        /// Predicative forms and the ordinary indefinite singular neuter form
        /// are false; invariant adjectives are also false.
        inflected: bool,
        /// Agreement features belong only to attributive occurrences. Gender
        /// is absent in the plural, where the paradigm does not distinguish it.
        #[serde(skip_serializing_if = "Option::is_none")]
        nominal_gender: Option<DutchNominalGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        definiteness: Option<DutchDefiniteness>,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        /// Only when the adverb's own form carries comparison (`graag`,
        /// `liever`, `liefst`; `vaak`, `vaker`, `vaakst`).
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<DutchDegree>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: DutchDeterminerType,
        /// Number of the possessed or otherwise determined noun phrase, not
        /// the number of a possessor.
        number: BinaryNumber,
        /// Singular agreement only. Plural determiners do not distinguish the
        /// `de`/`het` class.
        #[serde(skip_serializing_if = "Option::is_none")]
        nominal_gender: Option<DutchNominalGender>,
        /// Present only where the determiner has a definite/indefinite value;
        /// not every quantifier or interrogative does.
        #[serde(skip_serializing_if = "Option::is_none")]
        definiteness: Option<DutchDefiniteness>,
        /// Referential gender of a third-person singular possessor, encoded
        /// by `zijn`/`z'n` versus `haar`/`d'r`. This is independent of the
        /// common/neuter agreement class of the possessed noun.
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_gender: Option<TernaryGender>,
        /// Full versus reduced possessive form (`mijn`/`m'n`, `jouw`/`je`).
        #[serde(skip_serializing_if = "Option::is_none")]
        strength: Option<DutchPronounStrength>,
        /// Familiar `jouw`/`je` versus formal `uw`; other possessives omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        politeness: Option<DutchPoliteness>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        /// The lexical `de`/`het` class, retained in the plural even though all
        /// plural definite articles surface as `de`.
        nominal_gender: DutchNominalGender,
        number: BinaryNumber,
        /// Productive diminutives are always neuter and form their plural in
        /// `-s`; the lemma remains the singular diminutive (`huisjes` ->
        /// `huisje`), not the underived noun.
        diminutive: bool,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_type: DutchParticleType,
    },
    Pronoun {
        lemma: String,
        pronoun_type: DutchPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Masculine/feminine/neuter belongs to the personal-pronoun system,
        /// not to the binary `de`/`het` noun-class system.
        #[serde(skip_serializing_if = "Option::is_none")]
        pronominal_gender: Option<TernaryGender>,
        /// Demonstrative and relative pronouns instead agree with the nominal
        /// common/neuter contrast (`die`/`dat`).
        #[serde(skip_serializing_if = "Option::is_none")]
        nominal_gender: Option<DutchNominalGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<DutchPronounCase>,
        /// Direct object, indirect object without a preposition, or complement
        /// of a preposition. Personal oblique forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        pronoun_role: Option<DutchPronounRole>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strength: Option<DutchPronounStrength>,
        /// Second-person forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        politeness: Option<DutchPoliteness>,
    },
    ProperNoun {
        lemma: String,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        /// A lexical principal-parts class, reported on every token of the
        /// lemma even where the present form itself does not reveal it.
        verb_class: DutchVerbClass,
        /// Only complex/prefixed verbs. A simplex verb has no separability.
        #[serde(skip_serializing_if = "Option::is_none")]
        separability: Option<DutchSeparability>,
        verb_form: DutchVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<DutchMood>,
        /// Finite indicative forms only; imperatives and non-finite forms have
        /// no tense in this model.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<DutchTense>,
        /// Finite indicative forms only. Imperatives do not productively mark
        /// person or number in contemporary Standard Dutch.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Other {
        lemma: String,
    },
}

impl DutchMorphology {
    fn __pivot_degree(&self) -> Option<String> {
        match self {
            Self::Adjective { degree, .. } | Self::Adverb { degree, .. } => degree
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_politeness(&self) -> Option<String> {
        match self {
            Self::Determiner { politeness, .. } | Self::Pronoun { politeness, .. } => politeness
                .as_ref()
                .map(|value| {
                    panini_core::aggregable::ClosedValues::variant_str(value).to_string()
                }),
            _ => None,
        }
    }

    fn __pivot_strength(&self) -> Option<String> {
        match self {
            Self::Determiner { strength, .. } | Self::Pronoun { strength, .. } => strength
                .as_ref()
                .map(|value| {
                    panini_core::aggregable::ClosedValues::variant_str(value).to_string()
                }),
            _ => None,
        }
    }

    fn __pivot_pronoun_role(&self) -> Option<String> {
        match self {
            Self::Pronoun { pronoun_role, .. } => pronoun_role.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    fn __pivot_separability(&self) -> Option<String> {
        match self {
            Self::Verb { separability, .. } => separability.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    pub const PIVOT_DEGREE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "degree",
            "Degree",
            <DutchDegree as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_degree,
        );

    pub const PIVOT_POLITENESS: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "politeness",
            "Politeness",
            <DutchPoliteness as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_politeness,
        );

    pub const PIVOT_STRENGTH: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "strength",
            "Form Strength",
            <DutchPronounStrength as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_strength,
        );

    pub const PIVOT_PRONOUN_ROLE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "pronoun_role",
            "Pronoun Role",
            <DutchPronounRole as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_pronoun_role,
        );

    pub const PIVOT_SEPARABILITY: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "separability",
            "Separability",
            <DutchSeparability as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_separability,
        );

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <DutchMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <DutchTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );
}

pub struct Dutch;

impl LinguisticDefinition for Dutch {
    type Morphology = DutchMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Nld;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        DutchMorphology::PIVOT_NOMINAL_GENDER,
        DutchMorphology::PIVOT_NUMBER,
        DutchMorphology::PIVOT_DIMINUTIVE,
        DutchMorphology::PIVOT_DEGREE,
        DutchMorphology::PIVOT_INFLECTED,
        DutchMorphology::PIVOT_STRENGTH,
        DutchMorphology::PIVOT_PRONOUN_ROLE,
        DutchMorphology::PIVOT_POLITENESS,
        DutchMorphology::PIVOT_VERB_CLASS,
        DutchMorphology::PIVOT_SEPARABILITY,
        DutchMorphology::PIVOT_VERB_FORM,
        DutchMorphology::PIVOT_MOOD,
        DutchMorphology::PIVOT_TENSE,
    ];

    /// Contemporary Dutch is written in the Latin script. Historical blackletter
    /// is a typographic tradition, not a second script in current use.
    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            TypologicalFeature::Conjugation(&[Upos::Verb]),
            // These three produce honest root-to-form clozes with the current
            // card contract: huis -> huizen, groot -> grote, hij -> hem.
            // Determiners are deliberately absent: de/het is agreement-driven
            // lexical choice, not a useful "root of the object" transformation.
            TypologicalFeature::Declension(&[Upos::Noun, Upos::Adjective, Upos::Pronoun]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope and lemmatization: analyze the contemporary Standard Dutch actually written, including standard usage from the Netherlands, Belgium, Suriname and the Caribbean without normalizing one region into another. Lemmatize nouns to the singular dictionary form, verbs to the complete infinitive, adjectives and degree-bearing adverbs to the positive form, and compounds to the whole compound. Keep the official spelling of the lemma, including ij, diaereses and apostrophes. Never split an ordinary lexical compound into its heads.\n\
         2. Nouns: always report number, nominal_gender and diminutive. Nominal gender is ONLY common or neuter: singular de-words are common and singular het-words are neuter. Retain that lexical gender in the plural even though every definite plural takes de. Every productive diminutive is neuter and takes an -s plural; set diminutive true, but keep the diminutive itself as the lemma (huisjes -> huisje, not huis). Lexicalized diminutives likewise keep their dictionary lemma (meisjes -> meisje). A compound normally inherits the gender of its final head, but use the lexical gender of the actual compound rather than guessing from spelling alone. Dutch nouns have no productive nominative/accusative/dative paradigm, so never attach case to a noun.\n\
         3. Adjectives: for a gradable adjective, report positive, comparative or superlative; omit degree for a non-gradable relational, material or absolute adjective (medisch, houten, rechter). Set inflected true only when this occurrence carries the overt adjectival -e (grote, mooiere, grootste); set it false for a bare form (groot, mooi, groter, grootst) and for an invariant adjective whose form takes no added -e. For every ATTRIBUTIVE adjective, provide number and definiteness; provide nominal_gender in the singular and omit it in the plural. In the ordinary rule, an attributive adjective is bare only in an indefinite singular neuter phrase (een mooi huis, mooi weer) and takes -e elsewhere (de mooie dag, het mooie huis, een mooie dag, mooie huizen), but invariant forms and adjectives ending in -en such as houten remain uninflected. Predicative and adverbially used adjectives are bare and must omit nominal_gender, number and definiteness. Ordinals used attributively are Adjectives. An attributive participle functioning as a modifier is an Adjective with its adjectival lemma; an uninflected participle in a verbal construction is a Verb.\n\
         4. Determiners: classify articles, demonstratives, possessives, quantifiers, interrogatives and relatives by their function in context. Always report number for the determined noun phrase; this is NOT the possessor's number. Report nominal_gender for singular agreement and omit it in the plural, where de/deze/die no longer distinguish common from neuter. Add definiteness only where the determiner has a definite or indefinite value: de and het are definite, een is indefinite; demonstratives and possessives are definite. Lemmatize reduced possessives to their contextually correct full possessive form: m'n -> mijn, z'n -> zijn and d'r -> haar; je -> jouw for one familiar addressee but jullie for multiple familiar addressees (Jullie moeten je boek meenemen). Report strength only where the written possessive belongs to a full/reduced pair: mijn/m'n, jouw or jullie/je, zijn/z'n and haar/d'r; report strong for the full form and weak for the reduced form. Report politeness familiar on second-person familiar possessives (jouw, jullie and either use of je) and formal on uw; omit strength and politeness where those oppositions do not apply. For a third-person singular possessive, also report possessor_gender from its antecedent: zijn/z'n for masculine or neuter and haar/d'r for feminine; omit possessor_gender on all other determiners. Resolve the many syncretic forms from their noun phrase rather than from spelling alone. A substantively used form is a Pronoun, not a Determiner.\n\
         5. Pronouns: lemmatize reduced and oblique personal forms to the strong nominative paradigm base (mij/me -> ik, jou/je -> jij, hem/'m -> hij, haar/d'r -> zij, ons -> wij, hen/hun/ze -> zij); u stays u. Use case nominative for a subject personal pronoun and oblique for an object, prepositional complement or reflexive; other pronoun types receive case only when the contrast genuinely applies. On an oblique personal pronoun, report pronoun_role direct_object, indirect_object or prepositional from its syntax; in edited Standard Dutch this distinguishes hen as direct object or prepositional complement from hun as an indirect object without a preposition, while reduced ze can fill either object role. Personal third-person singular pronouns use pronominal_gender masculine/feminine/neuter and OMIT nominal_gender. Demonstrative and relative die/dat instead use nominal_gender common/neuter only in the singular and OMIT it in the plural; they always OMIT pronominal_gender. Plural personal pronouns have no gender.\n\
         6. Pronoun strength and address: when a personal-pronoun paradigm has distinct full/stressed and reduced/unstressed forms, classify the actual written form: jij/jou, mij, wij, zij, hij, hem, haar, hen and hun are strong; 'k, je, me, we, ze, -ie, 'm and d'r are weak. Personal-pronoun het occurs only in positions for reduced forms, so both the usual spelling het and the spelling 't are weak; emphatic reference instead normally uses demonstrative dat. Omit strength for forms such as u, ons and jullie whose spelling does not express that contrast. Familiar second-person forms jij/je/jou and jullie take politeness familiar. Formal u takes person second, agreement number singular and politeness formal even when it addresses more than one listener; its finite verb is singular. Recognize ge/gij as a southern/Belgian second-person form when it occurs and analyze the text as written rather than rewriting it to jij or u. Resolve je as personal Pronoun versus possessive Determiner from syntax, and resolve singular feminine ze against plural ze from agreement and context.\n\
         7. Verbs: every verb token, including zijn, hebben, worden, zullen and modal auxiliaries, gets verb_class and verb_form. Weak verbs form the past with -de/-te and the participle with -d/-t (werken, werkte, gewerkt); strong verbs use stem alternation and normally an -en participle (lopen, liep, gelopen); irregular is reserved for paradigms that fit neither, such as zeggen, zei, gezegd and the highly irregular auxiliaries. Report the lexical class on every token, including present forms where it is not visible. Where Standard Dutch permits competing weak and strong paradigms, follow the paradigm realized by the surrounding text and its regional/register context.\n\
         8. Finite features: a finite indicative gets mood, present/past tense, person and number. Dutch has no synthetic future or conditional: zal is present zullen and zou is past zullen, each followed by a separate infinitive. An imperative gets mood imperative but no tense, person or number because contemporary Standard Dutch has no productive person/number opposition in the imperative. Use subjunctive only for a genuine surviving form such as leve, neme, moge or zij, never merely for a semantic wish. Non-finite infinitives and participles get no mood, tense, person or number. With jij/je, identify the inverted present form correctly: jij werkt but werk jij/je; a following possessive je does not trigger deletion (werkt je broer?). With formal u, use second person singular analysis despite the third-singular-shaped -t agreement.\n\
         9. Analytic verb phrases: split every written verb into its own token. A perfect is finite hebben/zijn plus a past participle (heeft gewerkt, is gekomen); a passive is finite worden/zijn plus the same past participle; a future or conditional reading is finite zullen plus an infinitive. Do not invent perfect, future, conditional, aspect or voice values on a single token: this morphology records only categories expressed by that word form. The aan het + infinitive progressive is likewise a multi-token construction, not a synthetic verb form.\n\
         10. Separable verbs: the verb lemma is ALWAYS the complete infinitive. In ik bel je op, the Verb bel has lemma opbellen and separability separable, while op is a separate Particle with particle_type separated_verb_prefix. In dat ik je opbel, ik zal je opbellen and ik heb je opgebeld, the adjacent written form is one Verb token with the same lemma and separability. In om je op te bellen, emit op as the separated-prefix Particle, te as an infinitival Particle, and bellen as the Verb with lemma opbellen. Prefixed verbs that never separate (begrijpen, vertellen) get separability inseparable; simplex verbs omit separability entirely. Resolve ambiguous prefixes by syntax, stress/meaning and participle shape, not by a memorized prefix alone (voorkomen 'occur' is separable, voorkomen 'prevent' inseparable).\n\
         11. Particles and ambiguous short words: te before an infinitive is an infinitival Particle; niet is a negation Particle; unstressed words such as eens, maar, toch, even and nou are modal Particles only in their discourse-softening reading, not automatically in every occurrence. A stranded part of a separable verb is a separated_verb_prefix. Tag ja/nee used as answers as Interjections. Analyze er by its actual adverbial/pronominal function in context rather than treating it as a meaningless filler.\n\
         12. Tokenization: preserve one token per orthographic word except that a visibly separated verb prefix remains its own Particle as above. Reduced written pronouns ('k, 't, 'm, d'r) remain their own Pronoun tokens and take their full paradigm lemma; do not merge them into a neighbouring verb. Keep ordinary noun compounds and adjacent forms of separable verbs as one token. Never emit punctuation as a token."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dutch_identity_script_and_typology_are_exact() {
        let language = Dutch;

        assert_eq!(Dutch::ISO_LANG, IsoLang::Nld);
        assert_eq!(Dutch::ISO_LANG.to_639_3(), "nld");
        assert_eq!(language.supported_scripts(), &[Script::LATN]);
        assert_eq!(language.default_script(), Script::LATN);
        assert_eq!(
            language.typological_features(),
            &[
                TypologicalFeature::Conjugation(&[Upos::Verb]),
                TypologicalFeature::Declension(&[
                    Upos::Noun,
                    Upos::Adjective,
                    Upos::Pronoun,
                ]),
            ]
        );
    }

    #[test]
    fn optional_verb_dimensions_remain_closed_pivots() {
        let opgebeld = DutchMorphology::Verb {
            lemma: "opbellen".to_string(),
            verb_class: DutchVerbClass::Weak,
            separability: Some(DutchSeparability::Separable),
            verb_form: DutchVerbForm::PastParticiple,
            mood: None,
            tense: None,
            person: None,
            number: None,
        };
        let werkt = DutchMorphology::Verb {
            lemma: "werken".to_string(),
            verb_class: DutchVerbClass::Weak,
            separability: None,
            verb_form: DutchVerbForm::Finite,
            mood: Some(DutchMood::Indicative),
            tense: Some(DutchTense::Present),
            person: Some(Person::Third),
            number: Some(BinaryNumber::Singular),
        };

        assert_eq!(
            DutchMorphology::PIVOT_SEPARABILITY.value(&opgebeld),
            Some("separable".to_string())
        );
        assert_eq!(DutchMorphology::PIVOT_TENSE.value(&opgebeld), None);
        assert_eq!(
            DutchMorphology::PIVOT_MOOD.value(&werkt),
            Some("indicative".to_string())
        );
        assert_eq!(
            DutchMorphology::PIVOT_TENSE.value(&werkt),
            Some("present".to_string())
        );
    }

    #[test]
    fn adjective_e_and_noun_class_are_distinct_pivots() {
        let grote = DutchMorphology::Adjective {
            lemma: "groot".to_string(),
            degree: Some(DutchDegree::Positive),
            inflected: true,
            nominal_gender: Some(DutchNominalGender::Common),
            number: Some(BinaryNumber::Singular),
            definiteness: Some(DutchDefiniteness::Definite),
        };
        let huis = DutchMorphology::Noun {
            lemma: "huis".to_string(),
            nominal_gender: DutchNominalGender::Neuter,
            number: BinaryNumber::Singular,
            diminutive: false,
        };

        assert_eq!(
            DutchMorphology::PIVOT_INFLECTED.value(&grote),
            Some("true".to_string())
        );
        assert_eq!(DutchMorphology::PIVOT_NOMINAL_GENDER.value(&grote), None);
        assert_eq!(
            DutchMorphology::PIVOT_DEGREE.value(&grote),
            Some("positive".to_string())
        );
        assert_eq!(
            DutchMorphology::PIVOT_NOMINAL_GENDER.value(&huis),
            Some("neuter".to_string())
        );
    }

    #[test]
    fn non_gradable_adjectives_and_possessor_gender_are_not_conflated() {
        let houten = DutchMorphology::Adjective {
            lemma: "houten".to_string(),
            degree: None,
            inflected: false,
            nominal_gender: Some(DutchNominalGender::Common),
            number: Some(BinaryNumber::Singular),
            definiteness: Some(DutchDefiniteness::Definite),
        };
        let zijn_tafel = DutchMorphology::Determiner {
            lemma: "zijn".to_string(),
            determiner_type: DutchDeterminerType::Possessive,
            number: BinaryNumber::Singular,
            nominal_gender: Some(DutchNominalGender::Common),
            definiteness: Some(DutchDefiniteness::Definite),
            possessor_gender: Some(TernaryGender::Masculine),
            strength: Some(DutchPronounStrength::Strong),
            politeness: None,
        };
        let je_boek = DutchMorphology::Determiner {
            lemma: "jouw".to_string(),
            determiner_type: DutchDeterminerType::Possessive,
            number: BinaryNumber::Singular,
            nominal_gender: Some(DutchNominalGender::Neuter),
            definiteness: Some(DutchDefiniteness::Definite),
            possessor_gender: None,
            strength: Some(DutchPronounStrength::Weak),
            politeness: Some(DutchPoliteness::Familiar),
        };
        let jullie_je_boek = DutchMorphology::Determiner {
            lemma: "jullie".to_string(),
            determiner_type: DutchDeterminerType::Possessive,
            // The possessed noun is singular even though the possessor is plural.
            number: BinaryNumber::Singular,
            nominal_gender: Some(DutchNominalGender::Neuter),
            definiteness: Some(DutchDefiniteness::Definite),
            possessor_gender: None,
            strength: Some(DutchPronounStrength::Weak),
            politeness: Some(DutchPoliteness::Familiar),
        };

        assert_eq!(DutchMorphology::PIVOT_DEGREE.value(&houten), None);
        let serialized = serde_json::to_value(zijn_tafel).unwrap();
        assert_eq!(serialized["nominal_gender"], "common");
        assert_eq!(serialized["possessor_gender"], "masculine");
        assert_eq!(
            DutchMorphology::PIVOT_STRENGTH.value(&je_boek),
            Some("weak".to_string())
        );
        assert_eq!(
            DutchMorphology::PIVOT_POLITENESS.value(&je_boek),
            Some("familiar".to_string())
        );
        assert_eq!(jullie_je_boek.lemma(), Some("jullie".to_string()));
        assert_eq!(
            DutchMorphology::PIVOT_STRENGTH.value(&jullie_je_boek),
            Some("weak".to_string())
        );
    }

    #[test]
    fn personal_het_is_weak_even_when_fully_spelled() {
        let het = DutchMorphology::Pronoun {
            lemma: "het".to_string(),
            pronoun_type: DutchPronounType::Personal,
            person: Some(Person::Third),
            number: Some(BinaryNumber::Singular),
            pronominal_gender: Some(TernaryGender::Neuter),
            nominal_gender: None,
            case: Some(DutchPronounCase::Oblique),
            pronoun_role: Some(DutchPronounRole::DirectObject),
            strength: Some(DutchPronounStrength::Weak),
            politeness: None,
        };

        let serialized = serde_json::to_value(het).unwrap();
        assert_eq!(serialized["strength"], "weak");
        assert_eq!(serialized["pronominal_gender"], "neuter");
    }

    #[test]
    fn third_person_plural_roles_keep_hen_and_hun_distinct() {
        let make = |pronoun_role| DutchMorphology::Pronoun {
            lemma: "zij".to_string(),
            pronoun_type: DutchPronounType::Personal,
            person: Some(Person::Third),
            number: Some(BinaryNumber::Plural),
            pronominal_gender: None,
            nominal_gender: None,
            case: Some(DutchPronounCase::Oblique),
            pronoun_role: Some(pronoun_role),
            strength: Some(DutchPronounStrength::Strong),
            politeness: None,
        };
        let hen = make(DutchPronounRole::DirectObject);
        let hun = make(DutchPronounRole::IndirectObject);

        assert_ne!(
            serde_json::to_string(&hen).unwrap(),
            serde_json::to_string(&hun).unwrap()
        );
        assert_eq!(
            DutchMorphology::PIVOT_PRONOUN_ROLE.value(&hen),
            Some("direct_object".to_string())
        );
        assert_eq!(
            DutchMorphology::PIVOT_PRONOUN_ROLE.value(&hun),
            Some("indirect_object".to_string())
        );
    }
}
