use serde::{Deserialize, Serialize};

use panini_core::morpheme::{Agglutinative, MorphemeDefinition, WordSegmentation};
use panini_core::traits::{
    IsoLang, LinguisticDefinition, MorphologyInfo, Script, TypologicalFeature, Upos,
};

// ─── Basque grammatical enums ────────────────────────────────────────────────

/// The declension (`deklinabidea`) suffix inventory. Basque alignment is
/// ergative–absolutive: the absolutive is the unmarked case shared by the
/// intransitive subject and the transitive object, and the ergative `-k` marks
/// the transitive subject. There is no accusative.
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
pub enum BasqueCase {
    Absolutive,          // NOR — ∅ (etxea, gizonak "the men")
    Ergative,            // NORK — -k (gizonak "the man", lagunek)
    Dative,              // NORI — -i (lagunari, lagunei)
    Genitive,            // NOREN — -ren (Jonen, etxearen)
    LocativeGenitive,    // NONGO — -ko / -go (Bilboko, mendiko)
    Inessive,            // NON — -n (etxean, etxeetan)
    Allative,            // NORA — -ra (etxera)
    TerminalAllative,    // NORAINO — -raino (etxeraino)
    DirectionalAllative, // NORANTZ — -rantz (etxerantz)
    DestinativeAllative, // NORAKO — -rako (etxerako)
    Ablative,            // NONDIK — -tik (etxetik)
    Comitative,          // NOREKIN — -rekin (lagunarekin)
    Instrumental,        // ZERTAZ — -z (autoz, horretaz)
    Benefactive,         // NORENTZAT — -rentzat (niretzat)
    Motivative,          // ZERGATIK — -gatik (zuregatik)
    Partitive,           // -rik (ez dut dirurik)
    Prolative,           // -tzat (irakasletzat hartu)
}

/// The `mugatzailea` — one fused slot carrying definiteness *and* number.
/// Basque marks no number at all on an indefinite noun phrase, so this is a
/// single dimension, never a definiteness field beside a number field.
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
pub enum BasqueDetermination {
    Indefinite,       // mugagabea — bare stem (etxe, liburu bat, zenbat lagun)
    DefiniteSingular, // -a (etxea, etxean)
    DefinitePlural,   // -ak, oblique -eta- (etxeak, etxeetan)
    ProximatePlural,  // -ok, hurbilekoa (gu euskaldunok, liburuok)
}

/// The form a verb token takes. `Synthetic` and `Auxiliary` are the two finite
/// options; everything else is non-finite. Only a small closed set of verbs
/// (izan, ukan/*edun, egon, joan, etorri, ibili, eduki, jakin, esan, eraman,
/// erabili, iraun) has synthetic forms — every other verb is periphrastic.
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
pub enum BasqueVerbForm {
    Synthetic,              // aditz trinkoa — the lexical verb is itself finite (dator, dakit)
    Auxiliary,              // laguntzailea — finite izan / *edun / *edin / *ezan (da, dut, dio)
    PerfectiveParticiple,   // -tu / -du / -i / -n — also the citation form (ikusi, hartu)
    ImperfectiveParticiple, // -t(z)en (ikusten, hartzen)
    FutureParticiple,       // -ko / -go (ikusiko, joango)
    Radical,                // aditzoina — the bare stem (ikus, jan, sar)
    VerbalNoun,             // -t(z)e and its declined forms (ikustea, ikusteko)
    ResultativeParticiple,  // -ta / -da / -(r)ik / -a(k) (eginda, ikusirik, apurtua)
}

/// The auxiliary paradigm (`aditz laguntzailearen jokoa`) — which argument
/// slots the finite form indexes. It is what selects the auxiliary: NOR and
/// NOR-NORI take izan, NOR-NORK and NOR-NORI-NORK take *edun.
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
pub enum BasqueParadigm {
    Nor,        // absolutive only (naiz, da, dira)
    NorNori,    // absolutive + dative (zait, zaio, zaizkit)
    NorNork,    // absolutive + ergative (dut, ditu, zuen)
    NorNoriNork, // absolutive + dative + ergative (diot, dizkigu)
}

/// One agreement slot's value. Basque indexes seven referents, not a clean
/// person × number grid: `zu` is historically plural but synchronically a
/// singular polite address, and `hi` is the familiar singular that the
/// allocutive paradigm is built on.
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
pub enum BasquePersonNumber {
    FirstSingular,          // ni
    SecondSingularFamiliar, // hi
    SecondSingular,         // zu
    ThirdSingular,          // hura
    FirstPlural,            // gu
    SecondPlural,           // zuek
    ThirdPlural,            // haiek
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
pub enum BasqueTense {
    Present,      // orainaldia (da, dut, dator; dadin, dezan)
    Past,         // iraganaldia (zen, zuen, zetorren; zedin, zezan)
    Hypothetical, // alegiazkoa (balitz, banu, litzateke, nuke; ledin, lezan)
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
pub enum BasqueMood {
    Indicative,    // indikatiboa (da, dut)
    Conditional,   // baldintza — the ba- protasis (banu, balitz)
    Consequential, // ondorioa — the apodosis (nuke, litzateke)
    Potential,     // ahalera (dezaket, naiteke)
    Subjunctive,   // subjuntiboa (dezadan, nadin)
    Imperative,    // agintera (ezazu, zaitez)
}

/// Hitanoa — the addressee indexed on a finite verb outside its argument
/// structure. `None` is neutral zuka speech or a non-finite form; a present
/// value is a genuine allocutive form and must never be flattened into the
/// neutral paradigm.
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
pub enum BasqueAllocutive {
    MasculineFamiliar, // toka (duk, zakiat, ziok)
    FeminineFamiliar,  // noka (dun, zakinat, zionat)
    Respectful,        // xuka — the polite hitano of Zuberoa and Nafarroa Beherea
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
pub enum BasquePolarity {
    Affirmative,
    Negative, // ez / ezin — also fronts the finite verb
}

/// The subordinating suffix or prefix a finite verb carries. Basque has no
/// standalone complementizer word: "that" is the verbal suffix -(e)la.
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
pub enum BasqueSubordination {
    Completive,    // -(e)la, -(e)nik — "that"
    Relative,      // -(e)n (datorren gizona)
    Interrogative, // -(e)n — indirect question (ea datorren)
    Temporal,      // -(e)nean, -(e)larik
    Causal,        // -(e)lako, bait-
    Conditional,   // ba- (baldin badator)
    Concessive,    // -(e)n arren
    Purposive,     // -t(z)eko
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
pub enum BasqueDegree {
    Positive,    // handi
    Comparative, // -ago (handiago)
    Superlative, // -en (handien)
    Excessive,   // -egi (handiegi)
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
pub enum BasquePronounType {
    Personal,      // ni, hi, hura, gu, zu, zuek, haiek
    Intensive,     // indartuak — neu, heu, geu, zeu
    Demonstrative, // hau, hori, hura, hauek
    Interrogative, // nor, zer, zein, non, noiz
    Indefinite,    // norbait, zerbait, inor, ezer, bakoitza
    Reciprocal,    // elkar
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
pub enum BasqueDeterminerType {
    Demonstrative, // hau, hori, hura, hauek
    Quantifier,    // asko, gutxi, batzuk, guzti, dena
    Interrogative, // zein, zenbat, zer
    Indefinite,    // beste, edozein, zenbait
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
pub enum BasqueNumeralType {
    Cardinal,     // bat, bi, hiru, hamar
    Ordinal,      // -garren (lehen, bigarren, hirugarren)
    Distributive, // -na (bana, bina, hiruna)
}

/// The preverbal particles. Basque marks evidentiality and inference with
/// clitic-like particles sitting immediately before the finite verb.
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
pub enum BasqueParticleType {
    Interrogative, // al
    Dubitative,    // ote
    Evidential,    // omen, ei — hearsay
    Inferential,   // bide
    Negative,      // ez, ezin
    Additive,      // ere
    Affirmative,   // bai, ba
}

/// Morpheme-level aspect: the four participle constructions. Distinct from
/// [`BasqueVerbForm`], which also has to name the two finite options.
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
pub enum BasqueAspect {
    Perfective,   // -tu / -du / -i / -n
    Imperfective, // -t(z)en
    Prospective,  // -ko / -go
    Resultative,  // -ta / -da / -(r)ik / -a(k)
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
pub enum BasqueDerivation {
    AbstractNoun,   // -tasun, -tza (edertasun, nekazaritza)
    ActionNoun,     // -keta, -pen, -aldi, -t(z)e (garbiketa, ikuste)
    AgentNoun,      // -le / -tzaile, -gile (irakasle, saltzaile)
    PlaceNoun,      // -tegi, -toki (liburutegi, lantoki)
    Possessional,   // -dun, -tsu (euskaldun, indartsu)
    Privative,      // -gabe (etxegabe)
    Adjectivizing,  // -garri, -kor, -ezin (ikusgarri, hauskor)
    Verbalizing,    // -tu (handitu, zuritu)
    Adverbializing, // -ki, -ro (ederki, astiro)
    Diminutive,     // -txo / -txu, -ño (etxetxo)
    Pejorative,     // -keria (zikinkeria)
    Relational,     // -tar (donostiarra, europar)
    Collective,     // -di, -eria (pinudi, jenderia)
}

// ─── MorphemeFunction wrapper enum ───────────────────────────────────────────

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    panini_macro::AggregableFields,
    panini_macro::MorphemeFunctionCatalog,
)]
#[serde(tag = "category", rename_all = "snake_case")]
pub enum BasqueMorphemeFunction {
    Case {
        value: BasqueCase,
    },
    Determination {
        value: BasqueDetermination,
    },
    Aspect {
        value: BasqueAspect,
    },
    Subordination {
        value: BasqueSubordination,
    },
    Particle {
        value: BasqueParticleType,
    },
    Degree {
        value: BasqueDegree,
    },
    NumeralType {
        value: BasqueNumeralType,
    },
    Derivation {
        value: BasqueDerivation,
    },
}

impl BasqueMorphemeFunction {
    /// `category:value` — how the inventory is rendered in the extraction
    /// prompt. Every variant is single-field, so there is no composite case.
    fn directive_label(&self) -> String {
        let json = serde_json::to_value(self).unwrap();
        let cat = json["category"].as_str().unwrap();
        let val = json["value"].as_str().unwrap();
        format!("{cat}:{val}")
    }
}

// ─── BasqueMorphology ────────────────────────────────────────────────────────

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
pub enum BasqueMorphology {
    /// Only the last element of a Basque noun phrase is inflected, so a
    /// non-final modifier carries neither case nor article.
    Adjective {
        lemma: String,
        degree: BasqueDegree,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<BasqueCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        determination: Option<BasqueDetermination>,
    },
    /// Postpositions written as their own word (gainean, azpian, buruz, arte).
    /// The case suffixes themselves are never tokens.
    Adposition {
        lemma: String,
    },
    /// Lexical adverbs include fixed forms such as `mesedez`; a final string
    /// resembling a case suffix does not turn one into an adposition.
    Adverb {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<BasqueDegree>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    /// A determiner carries the phrase ending only when it is the final
    /// element (`liburu horietan`, `leku askotan`); non-final `zer` in
    /// `zer moduz` omits it. Indefinite quantifiers remain indefinite when
    /// declined: `askotan` in `leku askotan` is not definite plural.
    Determiner {
        lemma: String,
        determiner_type: BasqueDeterminerType,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<BasqueCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        determination: Option<BasqueDetermination>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<BasqueCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        determination: Option<BasqueDetermination>,
    },
    Numeral {
        lemma: String,
        numeral_type: BasqueNumeralType,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<BasqueCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        determination: Option<BasqueDetermination>,
    },
    Particle {
        lemma: String,
        particle_type: BasqueParticleType,
    },
    Pronoun {
        lemma: String,
        pronoun_type: BasquePronounType,
        case: BasqueCase,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement: Option<BasquePersonNumber>,
    },
    ProperNoun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<BasqueCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        determination: Option<BasqueDetermination>,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    /// A finite form indexes up to three arguments at once — absolutive (NOR),
    /// dative (NORI) and ergative (NORK) — so each is its own optional slot.
    Verb {
        lemma: String,
        form: BasqueVerbForm,
        polarity: BasquePolarity,
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<BasqueTense>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<BasqueMood>,
        #[serde(skip_serializing_if = "Option::is_none")]
        paradigm: Option<BasqueParadigm>,
        #[serde(skip_serializing_if = "Option::is_none")]
        absolutive_agreement: Option<BasquePersonNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dative_agreement: Option<BasquePersonNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ergative_agreement: Option<BasquePersonNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        allocutive: Option<BasqueAllocutive>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subordination: Option<BasqueSubordination>,
        /// Verbal nouns and nominalized finite relatives can carry the
        /// phrase-final declension ending; ordinary verb forms omit this.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<BasqueCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        determination: Option<BasqueDetermination>,
    },
    Other {
        lemma: String,
    },
}

impl BasqueMorphology {
    /// Case belongs to the phrase-final word. Most hosts therefore expose an
    /// optional field, while pronouns always carry one; the derived pivot only
    /// sees the latter and would miss ordinary declined noun phrases.
    fn __pivot_declension_case(&self) -> Option<String> {
        let case = match self {
            Self::Adjective { case, .. }
            | Self::Determiner { case, .. }
            | Self::Noun { case, .. }
            | Self::Numeral { case, .. }
            | Self::ProperNoun { case, .. }
            | Self::Verb { case, .. } => case.as_ref(),
            Self::Pronoun { case, .. } => Some(case),
            _ => return None,
        };

        case.map(|c| panini_core::aggregable::ClosedValues::variant_str(c).to_string())
    }

    /// Typed pivot handle for phrase-final case.
    pub const PIVOT_DECLENSION_CASE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "case",
            "Case",
            <BasqueCase as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_declension_case,
        );

    /// Determination is carried only by the phrase-final inflected word, so
    /// every POS that can host it exposes an optional slot.
    fn __pivot_determination(&self) -> Option<String> {
        let determination = match self {
            Self::Adjective { determination, .. }
            | Self::Determiner { determination, .. }
            | Self::Noun { determination, .. }
            | Self::Numeral { determination, .. }
            | Self::ProperNoun { determination, .. }
            | Self::Verb { determination, .. } => determination,
            _ => return None,
        };

        determination
            .as_ref()
            .map(|d| panini_core::aggregable::ClosedValues::variant_str(d).to_string())
    }

    /// Typed pivot handle for phrase-final determination.
    pub const PIVOT_DETERMINATION: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "determination",
            "Determination",
            <BasqueDetermination as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_determination,
        );

    /// `tense` is `Option` — a participle carries none — so the derive skips it
    /// for pivot generation. Written by hand to keep the facet available.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|t| panini_core::aggregable::ClosedValues::variant_str(t).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for tense (see [`BasqueMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <BasqueTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// `mood` is defined only on a finite form, hence `Option`.
    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood
                .as_ref()
                .map(|m| panini_core::aggregable::ClosedValues::variant_str(m).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for mood (see [`BasqueMorphology::__pivot_mood`]).
    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <BasqueMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    /// The paradigm is what selects izan against *edun, so it is the facet a
    /// Basque learner drills hardest — worth the hand-written handle.
    fn __pivot_paradigm(&self) -> Option<String> {
        match self {
            Self::Verb { paradigm, .. } => paradigm
                .as_ref()
                .map(|p| panini_core::aggregable::ClosedValues::variant_str(p).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for the auxiliary paradigm (see
    /// [`BasqueMorphology::__pivot_paradigm`]).
    pub const PIVOT_PARADIGM: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "paradigm",
            "Auxiliary paradigm",
            <BasqueParadigm as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_paradigm,
        );

    /// NORK is the slot the ergative alignment turns on.
    fn __pivot_ergative_agreement(&self) -> Option<String> {
        match self {
            Self::Verb {
                ergative_agreement, ..
            } => ergative_agreement
                .as_ref()
                .map(|a| panini_core::aggregable::ClosedValues::variant_str(a).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for ergative agreement (see
    /// [`BasqueMorphology::__pivot_ergative_agreement`]).
    pub const PIVOT_ERGATIVE_AGREEMENT: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "ergative_agreement",
            "Ergative agreement",
            <BasquePersonNumber as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_ergative_agreement,
        );

    /// Allocutive is absent from every neutral and non-finite form, hence
    /// `Option` — but it is a whole register a learner can slice on.
    fn __pivot_allocutive(&self) -> Option<String> {
        match self {
            Self::Verb { allocutive, .. } => allocutive
                .as_ref()
                .map(|a| panini_core::aggregable::ClosedValues::variant_str(a).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for the allocutive register (see
    /// [`BasqueMorphology::__pivot_allocutive`]).
    pub const PIVOT_ALLOCUTIVE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "allocutive",
            "Allocutive",
            <BasqueAllocutive as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_allocutive,
        );
}

// ─── Static morpheme inventory ───────────────────────────────────────────────

type P = BasqueMorphologyPosTag;
type F = BasqueMorphemeFunction;

/// A Basque declension ending belongs to the whole phrase and surfaces on its
/// last word. Besides ordinary nominals, that last word can be a numeral or a
/// nominalized verbal form/relative clause.
const DECLINABLE_POS: &[P] = &[
    P::Noun,
    P::ProperNoun,
    P::Pronoun,
    P::Adjective,
    P::Determiner,
    P::Numeral,
    P::Verb,
];

/// Base forms use the notation of the Euskaltzaindia declension tables: the
/// linking segment in parentheses (`-(r)en`), and the singular and plural
/// shapes separated by a slash where they diverge (`-(e)an/-etan`). Every
/// entry is unique as a string, which is what `validate_inventory` checks.
static BASQUE_MORPHEMES: &[MorphemeDefinition<F, P>] = &[
    // === Deklinabidea — case suffixes ===
    // The absolutive has no morpheme: it is the unmarked case.
    MorphemeDefinition {
        base_form: "-(e)k",
        functions: &[F::Case {
            value: BasqueCase::Ergative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)i",
        functions: &[F::Case {
            value: BasqueCase::Dative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)en",
        functions: &[F::Case {
            value: BasqueCase::Genitive,
        }],
        applies_to: DECLINABLE_POS,
    },
    // Homophonous with the future participle — the host's part of speech is
    // what separates Bilboko from ikusiko.
    MorphemeDefinition {
        base_form: "-ko/-go",
        functions: &[
            F::Case {
                value: BasqueCase::LocativeGenitive,
            },
            F::Aspect {
                value: BasqueAspect::Prospective,
            },
        ],
        applies_to: &[
            P::Noun,
            P::ProperNoun,
            P::Pronoun,
            P::Adjective,
            P::Adverb,
            P::Determiner,
            P::Numeral,
            P::Verb,
        ],
    },
    MorphemeDefinition {
        base_form: "-(e)an/-etan",
        functions: &[F::Case {
            value: BasqueCase::Inessive,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(e)ra/-etara",
        functions: &[F::Case {
            value: BasqueCase::Allative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(e)raino",
        functions: &[F::Case {
            value: BasqueCase::TerminalAllative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(e)rantz",
        functions: &[F::Case {
            value: BasqueCase::DirectionalAllative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(e)rako",
        functions: &[F::Case {
            value: BasqueCase::DestinativeAllative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(e)tik/-etatik",
        functions: &[F::Case {
            value: BasqueCase::Ablative,
        }],
        applies_to: DECLINABLE_POS,
    },
    // Animate local cases use the -gan- series instead of attaching the
    // inanimate local endings directly (lagunarengan, zuregandik).
    MorphemeDefinition {
        base_form: "-(r)engan",
        functions: &[F::Case {
            value: BasqueCase::Inessive,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)engana",
        functions: &[F::Case {
            value: BasqueCase::Allative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)enganaino",
        functions: &[F::Case {
            value: BasqueCase::TerminalAllative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)enganantz",
        functions: &[F::Case {
            value: BasqueCase::DirectionalAllative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)enganako",
        functions: &[F::Case {
            value: BasqueCase::DestinativeAllative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)engandik",
        functions: &[F::Case {
            value: BasqueCase::Ablative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)ekin",
        functions: &[F::Case {
            value: BasqueCase::Comitative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(e)z",
        functions: &[F::Case {
            value: BasqueCase::Instrumental,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)entzat",
        functions: &[F::Case {
            value: BasqueCase::Benefactive,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(en)gatik",
        functions: &[F::Case {
            value: BasqueCase::Motivative,
        }],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-(r)ik",
        functions: &[
            F::Case {
                value: BasqueCase::Partitive,
            },
            F::Aspect {
                value: BasqueAspect::Resultative,
            },
        ],
        applies_to: DECLINABLE_POS,
    },
    MorphemeDefinition {
        base_form: "-tzat",
        functions: &[F::Case {
            value: BasqueCase::Prolative,
        }],
        applies_to: DECLINABLE_POS,
    },
    // === Mugatzailea — the article ===
    MorphemeDefinition {
        base_form: "-a",
        functions: &[F::Determination {
            value: BasqueDetermination::DefiniteSingular,
        }],
        applies_to: &[P::Noun, P::ProperNoun, P::Adjective, P::Numeral, P::Verb],
    },
    MorphemeDefinition {
        base_form: "-ak",
        functions: &[F::Determination {
            value: BasqueDetermination::DefinitePlural,
        }],
        applies_to: &[
            P::Noun,
            P::ProperNoun,
            P::Adjective,
            P::Numeral,
            P::Verb,
        ],
    },
    MorphemeDefinition {
        base_form: "-ok",
        functions: &[F::Determination {
            value: BasqueDetermination::ProximatePlural,
        }],
        applies_to: &[P::Noun, P::ProperNoun, P::Adjective, P::Pronoun],
    },
    // === Aditz-atzizkiak — participles and the verbal noun ===
    MorphemeDefinition {
        base_form: "-tu/-du",
        functions: &[
            F::Aspect {
                value: BasqueAspect::Perfective,
            },
            F::Derivation {
                value: BasqueDerivation::Verbalizing,
            },
        ],
        applies_to: &[P::Verb, P::Adjective, P::Noun],
    },
    MorphemeDefinition {
        base_form: "-i",
        functions: &[F::Aspect {
            value: BasqueAspect::Perfective,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-n",
        functions: &[F::Aspect {
            value: BasqueAspect::Perfective,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-t(z)en",
        functions: &[F::Aspect {
            value: BasqueAspect::Imperfective,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-t(z)e",
        functions: &[F::Derivation {
            value: BasqueDerivation::ActionNoun,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-ta/-da",
        functions: &[F::Aspect {
            value: BasqueAspect::Resultative,
        }],
        applies_to: &[P::Verb],
    },
    // === Menderagailuak — subordination ===
    MorphemeDefinition {
        base_form: "ba-",
        functions: &[
            F::Subordination {
                value: BasqueSubordination::Conditional,
            },
            F::Particle {
                value: BasqueParticleType::Affirmative,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "bait-",
        functions: &[F::Subordination {
            value: BasqueSubordination::Causal,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)la",
        functions: &[F::Subordination {
            value: BasqueSubordination::Completive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)nik",
        functions: &[F::Subordination {
            value: BasqueSubordination::Completive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)n",
        functions: &[
            F::Subordination {
                value: BasqueSubordination::Relative,
            },
            F::Subordination {
                value: BasqueSubordination::Interrogative,
            },
            F::Subordination {
                value: BasqueSubordination::Concessive,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)nean",
        functions: &[F::Subordination {
            value: BasqueSubordination::Temporal,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)netik",
        functions: &[F::Subordination {
            value: BasqueSubordination::Temporal,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)larik",
        functions: &[F::Subordination {
            value: BasqueSubordination::Temporal,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)lako",
        functions: &[F::Subordination {
            value: BasqueSubordination::Causal,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-(e)nez",
        functions: &[F::Subordination {
            value: BasqueSubordination::Causal,
        }],
        applies_to: &[P::Verb],
    },
    // Lexicalised as one purposive suffix, not -t(z)e followed by -ko.
    MorphemeDefinition {
        base_form: "-t(z)eko",
        functions: &[F::Subordination {
            value: BasqueSubordination::Purposive,
        }],
        applies_to: &[P::Verb],
    },
    // === Graduatzaileak — degree ===
    MorphemeDefinition {
        base_form: "-ago",
        functions: &[F::Degree {
            value: BasqueDegree::Comparative,
        }],
        applies_to: &[P::Adjective, P::Adverb],
    },
    MorphemeDefinition {
        base_form: "-en",
        functions: &[F::Degree {
            value: BasqueDegree::Superlative,
        }],
        applies_to: &[P::Adjective, P::Adverb],
    },
    MorphemeDefinition {
        base_form: "-egi",
        functions: &[F::Degree {
            value: BasqueDegree::Excessive,
        }],
        applies_to: &[P::Adjective, P::Adverb],
    },
    // === Zenbatzaileak — numerals ===
    MorphemeDefinition {
        base_form: "-garren",
        functions: &[F::NumeralType {
            value: BasqueNumeralType::Ordinal,
        }],
        applies_to: &[P::Numeral],
    },
    MorphemeDefinition {
        base_form: "-na",
        functions: &[F::NumeralType {
            value: BasqueNumeralType::Distributive,
        }],
        applies_to: &[P::Numeral],
    },
    // === Eratorpena — derivation ===
    MorphemeDefinition {
        base_form: "-tasun",
        functions: &[F::Derivation {
            value: BasqueDerivation::AbstractNoun,
        }],
        applies_to: &[P::Adjective, P::Noun],
    },
    MorphemeDefinition {
        base_form: "-tza",
        functions: &[F::Derivation {
            value: BasqueDerivation::AbstractNoun,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "-keta",
        functions: &[F::Derivation {
            value: BasqueDerivation::ActionNoun,
        }],
        applies_to: &[P::Verb, P::Noun],
    },
    MorphemeDefinition {
        base_form: "-pen",
        functions: &[F::Derivation {
            value: BasqueDerivation::ActionNoun,
        }],
        applies_to: &[P::Verb, P::Noun],
    },
    MorphemeDefinition {
        base_form: "-aldi",
        functions: &[F::Derivation {
            value: BasqueDerivation::ActionNoun,
        }],
        applies_to: &[P::Noun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "-le/-tzaile",
        functions: &[F::Derivation {
            value: BasqueDerivation::AgentNoun,
        }],
        applies_to: &[P::Verb, P::Noun],
    },
    MorphemeDefinition {
        base_form: "-gile",
        functions: &[F::Derivation {
            value: BasqueDerivation::AgentNoun,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "-tegi",
        functions: &[F::Derivation {
            value: BasqueDerivation::PlaceNoun,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "-toki",
        functions: &[F::Derivation {
            value: BasqueDerivation::PlaceNoun,
        }],
        applies_to: &[P::Noun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "-dun",
        functions: &[F::Derivation {
            value: BasqueDerivation::Possessional,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "-tsu",
        functions: &[F::Derivation {
            value: BasqueDerivation::Possessional,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "-gabe",
        functions: &[F::Derivation {
            value: BasqueDerivation::Privative,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "-garri",
        functions: &[F::Derivation {
            value: BasqueDerivation::Adjectivizing,
        }],
        applies_to: &[P::Verb, P::Noun],
    },
    MorphemeDefinition {
        base_form: "-kor",
        functions: &[F::Derivation {
            value: BasqueDerivation::Adjectivizing,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "-ezin",
        functions: &[F::Derivation {
            value: BasqueDerivation::Adjectivizing,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-tar",
        functions: &[F::Derivation {
            value: BasqueDerivation::Relational,
        }],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "-ki",
        functions: &[F::Derivation {
            value: BasqueDerivation::Adverbializing,
        }],
        applies_to: &[P::Adjective, P::Adverb],
    },
    MorphemeDefinition {
        base_form: "-ro",
        functions: &[F::Derivation {
            value: BasqueDerivation::Adverbializing,
        }],
        applies_to: &[P::Adjective, P::Adverb],
    },
    MorphemeDefinition {
        base_form: "-txo/-txu",
        functions: &[F::Derivation {
            value: BasqueDerivation::Diminutive,
        }],
        applies_to: &[P::Noun, P::Adjective, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "-ño",
        functions: &[F::Derivation {
            value: BasqueDerivation::Diminutive,
        }],
        applies_to: &[P::Noun, P::Adjective, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "-keria",
        functions: &[F::Derivation {
            value: BasqueDerivation::Pejorative,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "-di",
        functions: &[F::Derivation {
            value: BasqueDerivation::Collective,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "-eria",
        functions: &[F::Derivation {
            value: BasqueDerivation::Collective,
        }],
        applies_to: &[P::Noun],
    },
];

// ─── Agglutinative implementation ────────────────────────────────────────────

impl Agglutinative for Basque {
    fn morpheme_inventory() -> &'static [MorphemeDefinition<
        BasqueMorphemeFunction,
        <BasqueMorphology as MorphologyInfo>::PosTag,
    >] {
        BASQUE_MORPHEMES
    }

    fn morpheme_directives(&self) -> String {
        let inventory_lines: String = BASQUE_MORPHEMES
            .iter()
            .map(|m| {
                let funcs: Vec<String> = m
                    .functions
                    .iter()
                    .map(BasqueMorphemeFunction::directive_label)
                    .collect();
                format!("  {} → {}", m.base_form, funcs.join(" / "))
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "MORPHEME SEGMENTATION — fill `morpheme_segmentation` as an array of objects, \
             one per word that carries a suffix or a subordinating prefix.\n\
             Each object has:\n\
             - `word`: the surface form of the word\n\
             - `morphemes`: one entry per affix (NOT the stem — its dictionary form is the word's `lemma`):\n\
               - `surface`: the allomorph as it actually appears (e.g. \"an\", \"etan\", \"ari\", \"ko\")\n\
               - `base_form`: the identifier from the inventory below, copied verbatim\n\
               - `function`: {{\"category\": \"<type>\", \"value\": \"<value>\"}}\n\
             \n\
             <morpheme_inventory>\n\
             Use ONLY base_forms from this list:\n\
             {inventory_lines}\n\
             </morpheme_inventory>\n\
             \n\
             ORDER OF SLOTS: a Basque nominal ending represents stem + determination/number + \
             case, in that order, but the surface often fuses those slots. Segment each overt \
             substring exactly once: etxean has the single case allomorph \"an\" and etxeetan \
             the single case allomorph \"etan\"; their `determination` remains explicit in the \
             word-level morphology. By contrast, singular ergative gizonak transparently has \
             article \"a\" plus case \"k\", while absolutive-plural gizonak has article \"ak\" and \
             no case morpheme. Never emit overlapping surfaces for one word.\n\
             THE ABSOLUTIVE HAS NO MORPHEME: never invent a case entry for it. An absolutive \
             word carries only its article.\n\
             ALLOMORPHY: the linking vowel and -r- appear after a consonant-final or \
             vowel-final stem respectively (lagun-ari vs neska-ri; lagun-ekin vs neska-rekin). \
             Report the surface allomorph in `surface` and the citation form in `base_form`.\n\
             DO NOT SEGMENT FINITE VERBS: synthetic forms (dator, dakit) and auxiliaries \
             (da, dut, dio, zaizkigu) are paradigm cells, not transparently affixed stems. \
             Their nor / nori / nork content belongs in the morphology fields, not here. \
             Only segment a finite verb for an outer affix it carries (datorrela → -(e)la; \
             conditional badator or emphatic badakit → ba-). Resolve ba- from syntax: a real \
             condition is subordination:conditional, while assertion/emphasis is \
             particle:affirmative.\n\
             FIXED EXPRESSIONS: mesedez is synchronically a lexical adverb with lemma mesedez, \
             not a declined occurrence of mesede; do not segment its final z. In eskerrik asko, \
             segment only the partitive -rik on eskerrik; asko has no suffix. By contrast, in \
             leku askotan, askotan is declined asko: segment its surface -tan as the inessive \
             `-(e)an/-etan`.\n\
             Segment only words that have at least one affix worth annotating."
        )
    }
}

// ─── LinguisticDefinition implementation ─────────────────────────────────────

pub struct Basque;

impl LinguisticDefinition for Basque {
    type Morphology = BasqueMorphology;
    type MorphemeFunction = BasqueMorphemeFunction;

    const ISO_LANG: IsoLang = IsoLang::Eus;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        BasqueMorphology::PIVOT_DECLENSION_CASE,
        BasqueMorphology::PIVOT_DETERMINATION,
        BasqueMorphology::PIVOT_FORM,
        BasqueMorphology::PIVOT_PARADIGM,
        BasqueMorphology::PIVOT_TENSE,
        BasqueMorphology::PIVOT_MOOD,
        BasqueMorphology::PIVOT_ERGATIVE_AGREEMENT,
        BasqueMorphology::PIVOT_ALLOCUTIVE,
    ];
    const MORPHEME_PIVOTS: &'static [panini_core::pivot::PivotField<Self::MorphemeFunction>] = &[
        BasqueMorphemeFunction::PIVOT_CASE,
        BasqueMorphemeFunction::PIVOT_DETERMINATION,
        BasqueMorphemeFunction::PIVOT_ASPECT,
        BasqueMorphemeFunction::PIVOT_SUBORDINATION,
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
            // Only the last element of a noun phrase is inflected, but any of
            // these can be that element.
            TypologicalFeature::Declension(&[
                Upos::Noun,
                Upos::ProperNoun,
                Upos::Pronoun,
                Upos::Adjective,
                Upos::Determiner,
                Upos::Numeral,
            ]),
            TypologicalFeature::Agglutination,
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization: lexical verbs take the PERFECTIVE PARTICIPLE, which is the Basque dictionary form (ikusi, hartu, jan, etorri, izan, egon, jakin) — never the verbal noun (ikustea) and never a finite form. Finite auxiliaries have four distinct lemmas: indicative forms use izan for nor/nor_nori and *edun for nor_nork/nor_nori_nork; potential, subjunctive and imperative forms use *edin for nor/nor_nori and *ezan for nor_nork/nor_nori_nork (da → izan, dut → *edun, nadin/naiteke/zaitez → *edin, dezan/dezaket/ezazu → *ezan). A synthetic lexical form lemmatizes to its own participle (dator → etorri, dakit → jakin, nago → egon, doa → joan). Nouns, proper nouns, adjectives, determiners and numerals take the bare stem with the article and every case suffix stripped (etxeetan → etxe, mendiaren → mendi, handiagoak → handi, honetan → hau).\n\
         2. Tokenization: Basque case suffixes and the article are SUFFIXES, not words. Never split them into separate tokens — etxean is ONE noun token (case inessive, determination definite_singular), lagunarekin is ONE noun token (comitative), Bilbokoa is ONE proper-noun token. Tag `adposition` only for a genuinely separate postposition word (gainean, azpian, ondoan, aurrean, buruz, arte, bidez, kontra); record the case its complement carries on that complement, not on the postposition. The subordinating -(e)la / -(e)n / -(e)lako and the prefixes ba- / bait- likewise attach to the finite verb: record them in the verb's `subordination` field, never as their own tokens.\n\
         3. Nominal phrase endings: on nouns, proper nouns, adjectives, determiners and numerals, give `case` and `determination` ONLY when that token closes its noun phrase and carries the phrase ending. In `etxe handian`, omit both from the bare non-final noun etxe and give inessive + definite_singular to adjective handi. In `egun on`, likewise omit both from egun and give absolutive + indefinite to phrase-final on. In `hiru liburu horietan`, omit both from liburu and give inessive + definite_plural to demonstrative hori. An independently declined numeral such as hirurekin carries comitative + indefinite. A noun or proper noun that is itself phrase-final still receives both fields: etxe = absolutive + indefinite, etxea = absolutive + definite_singular, Jon = absolutive + indefinite, Jonek = ergative + indefinite.\n\
         4. `determination` is a single fused slot — `indefinite` for a bare or articleless paradigm (etxe, liburu bat, zenbat lagun, Jon, ez dut dirurik), `definite_singular` for -a, `definite_plural` for -ak and plural oblique endings (etxeetan, gizonei), `proximate_plural` for -ok (gu euskaldunok). Do NOT report number as a separate feature: Basque fuses number into the determiner paradigm and an indefinite noun phrase carries no number at all. Give `degree` on every adjective and `numeral_type` on every numeral.\n\
         5. Verbs — decide finite against non-finite first. `form` is `synthetic` for a finite form of the lexical verb itself (dator, dakit, nago, dabil, dakar); `auxiliary` for a finite izan / *edun / *edin / *ezan (naiz, da, dira, dut, ditu, zen, zuen, zaio, diot, dezaket, nadin); `perfective_participle` for -tu / -du / -i / -n (ikusi, hartu, jakin, esan); `imperfective_participle` for -t(z)en (ikusten, hartzen); `future_participle` for -ko / -go (ikusiko, joango); `radical` for the bare aditzoina used before subjunctive, potential and imperative auxiliaries (ikus, jan, sar); `verbal_noun` for -t(z)e and its declined forms (ikustea, ikusteko, ikustera); `resultative_participle` for -ta/-da, -(r)ik or agreeing -a(k) forms (eginda, ikusirik, apurtua, behartuak).\n\
         6. Give `mood`, `paradigm` and the agreement slots ONLY on a finite form (`synthetic` or `auxiliary`), and give them all there. Give `tense` on finite indicative, conditional, consequential, potential and subjunctive forms: `present`, `past`, or the distinct `hypothetical` series (banu, balitz, nuke, litzateke, ledin, lezan). Omit `tense` on an imperative. Omit all four finite dimensions on a participle, radical or verbal noun. `case` and `determination` are a separate option: give them only when a verbal noun or a nominalized finite relative carries the phrase ending (ikustea = verbal_noun + absolutive + definite_singular; etorri denak = auxiliary + relative + ergative + definite_singular). `polarity` is required on every verb: `negative` whenever the predicate is under ez or ezin, `affirmative` otherwise. ez is its own particle token, but the verb it scopes still carries `polarity: negative`.\n\
         7. Polypersonal agreement — this is the core of the language. Fill `absolutive_agreement` (NOR), `dative_agreement` (NORI) and `ergative_agreement` (NORK) independently, each only when the form actually indexes that argument. da = absolutive third_singular. dut = absolutive third_singular + ergative first_singular. ditut = absolutive third_plural + ergative first_singular. zaizkit = absolutive third_plural + dative first_singular. diot = absolutive third_singular + dative third_singular + ergative first_singular. gaituzte = absolutive first_plural + ergative third_plural.\n\
         8. `paradigm` names exactly which slots the form has: `nor`, `nor_nori`, `nor_nork`, `nor_nori_nork`; it must agree with the agreement slots. Paradigm and mood together select the auxiliary. Indicative and hypothetical conditional/consequential forms use izan for nor/nor_nori and *edun for nor_nork/nor_nori_nork. Potential, subjunctive and imperative forms instead use *edin for nor/nor_nori and *ezan for nor_nork/nor_nori_nork. Thus naiteke is *edin + nor + potential, while dezaket is *ezan + nor_nork + potential. In an indicative clause, an ergative subject still requires *edun even when the meaning looks intransitive (dirua behar dut, euskaraz dakit).\n\
         9. `allocutive` (hitanoa) is the ADDRESSEE indexed on a finite verb outside its argument structure: `masculine_familiar` for the toka forms (duk, diat, zakiat, ziok), `feminine_familiar` for the noka forms (dun, dinat, zakinat, zionat), `respectful` for the xuka forms of Zuberoa and Nafarroa Beherea. OMIT the field entirely for ordinary zuka speech and for every non-finite form. Read it off the verb form itself — never infer it from hi appearing in the sentence, and never from a second-person argument: hi as an argument fills an agreement slot, allocutive marking does not.\n\
         10. Pronouns: give `pronoun_type` and `case`, plus `agreement` for a personal or intensive pronoun (ni → first_singular, hi → second_singular_familiar, zu → second_singular, hura → third_singular, gu → first_plural, zuek → second_plural, haiek → third_plural). Note zu is a SINGULAR polite address despite its historic plural origin, and hi is the familiar singular.\n\
         11. Particles: give `particle_type`. al is `interrogative`, ote is `dubitative`, omen and ei are `evidential`, bide is `inferential`, ez and ezin are `negative`, ere is `additive`, bai is `affirmative`. Attached emphatic ba- is not a separate token: keep the whole form as a verb and record ba- only in morpheme segmentation as particle:affirmative. Conditional ba- is instead subordination:conditional, and the finite verb under it keeps its ordinary indicative mood unless its own form is hypothetical (badator = indicative; balitz = conditional).\n\
         12. Guardrails for the confusions this language actually provokes:\n\
         - Final -ak is TWO different things. gizonak is the ergative singular \"the man (as agent)\" (-a + -k) and the absolutive plural \"the men\". Decide from the verb: if the finite form indexes a NORK, the -ak phrase is ergative singular; if the clause is a nor form, it is absolutive plural. Never decide from the suffix alone.\n\
         - Final -k on an indefinite or on a numeral phrase is the ergative, not a plural marker (lagun batek, zenbat lagunek, hiru neskak). Basque has NO plural -k outside the article.\n\
         - -ko is the locative genitive on a nominal (Bilboko, mendiko, etxerako) and the FUTURE participle on a verb (ikusiko, emango, joango). Decide by the host: nominal → case, verb → form future_participle.\n\
         - -(r)en is the genitive on a nominal (Jonen, etxearen), -en is the superlative on an adjective (handien, ederren), and -(e)n is the relative or subordinating ending on a finite verb (datorren, dakien). Three different suffixes with the same shape.\n\
         - The absolutive is the ZERO case and it covers both the subject of an intransitive verb and the direct object of a transitive one. Basque has no accusative: never tag a direct object `accusative`, tag it `absolutive`.\n\
         - Do not strip a lexical final -a. gizona lemmatizes to gizon, but euskara, gauza, eliza, arrosa, denbora and neska end in -a lexically and lemmatize to themselves.\n\
         - The NOR of a transitive clause is the OBJECT. In `nik liburua irakurri dut` the absolutive is liburua (third_singular) and the ergative is nik (first_singular), never the reverse.\n\
         - A demonstrative used without a following noun is a `pronoun`, including its declined forms: horregatik is pronoun lemma hori + motivative, never a proper noun. Mesedez is an ungradable lexical `adverb` with lemma mesedez: omit `degree`. In `zer moduz`, zer is determiner_type `interrogative` with no case or determination, while moduz is noun lemma modu + instrumental + indefinite. In `eskerrik asko`, asko is determiner_type `quantifier`, not `indefinite`, and has no case or determination. In `leku askotan`, however, askotan is the phrase-final determiner lemma asko, determiner_type `quantifier`, case `inessive`, determination `indefinite`; it is not the lexical adverb askotan and not definite plural.\n\
         13. Basque has no grammatical gender and no gender agreement anywhere in the noun phrase. The only masculine/feminine distinction in the language is `allocutive`."
    }

    fn extra_extraction_directives(&self) -> Option<String> {
        Some(self.morpheme_directives())
    }

    fn post_process_extraction(
        &self,
        segmentation: &mut Option<Vec<WordSegmentation<BasqueMorphemeFunction>>>,
    ) -> Result<(), String> {
        self.validate_and_enrich(segmentation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basque_identity_script_and_typology_are_exact() {
        let language = Basque;

        assert_eq!(Basque::ISO_LANG, IsoLang::Eus);
        assert_eq!(Basque::ISO_LANG.to_639_3(), "eus");
        assert_eq!(language.supported_scripts(), &[Script::LATN]);
        assert_eq!(language.default_script(), Script::LATN);
    }

    /// Only the last element of a Basque noun phrase is inflected, and it can
    /// be an adjective or a determiner — a declension cloze scoped to nouns
    /// alone would never exercise `etxe handian`.
    #[test]
    fn modifiers_are_in_the_declension_payload() {
        let declines: &[Upos] = Basque
            .typological_features()
            .iter()
            .find_map(|feature| match feature {
                TypologicalFeature::Declension(pos) => Some(*pos),
                TypologicalFeature::Conjugation(_) | TypologicalFeature::Agglutination => None,
            })
            .expect("Basque declares declension");

        assert!(declines.contains(&Upos::Noun));
        assert!(declines.contains(&Upos::Adjective));
        assert!(declines.contains(&Upos::Determiner));
        assert!(declines.contains(&Upos::Numeral));
    }

    /// The three agreement slots are `Option`, so the derive skips them — and
    /// a form indexing only some of them must leave the rest empty rather than
    /// collapsing into a single "person" field.
    #[test]
    fn polypersonal_slots_are_independent() {
        let diot = BasqueMorphology::Verb {
            lemma: "*edun".to_string(),
            form: BasqueVerbForm::Auxiliary,
            polarity: BasquePolarity::Affirmative,
            tense: Some(BasqueTense::Present),
            mood: Some(BasqueMood::Indicative),
            paradigm: Some(BasqueParadigm::NorNoriNork),
            absolutive_agreement: Some(BasquePersonNumber::ThirdSingular),
            dative_agreement: Some(BasquePersonNumber::ThirdSingular),
            ergative_agreement: Some(BasquePersonNumber::FirstSingular),
            allocutive: None,
            subordination: None,
            case: None,
            determination: None,
        };
        let da = BasqueMorphology::Verb {
            lemma: "izan".to_string(),
            form: BasqueVerbForm::Auxiliary,
            polarity: BasquePolarity::Affirmative,
            tense: Some(BasqueTense::Present),
            mood: Some(BasqueMood::Indicative),
            paradigm: Some(BasqueParadigm::Nor),
            absolutive_agreement: Some(BasquePersonNumber::ThirdSingular),
            dative_agreement: None,
            ergative_agreement: None,
            allocutive: None,
            subordination: None,
            case: None,
            determination: None,
        };

        assert_eq!(
            BasqueMorphology::PIVOT_ERGATIVE_AGREEMENT.value(&diot),
            Some("first_singular".to_string())
        );
        assert_eq!(BasqueMorphology::PIVOT_ERGATIVE_AGREEMENT.value(&da), None);
        assert_eq!(
            BasqueMorphology::PIVOT_PARADIGM.value(&diot),
            Some("nor_nori_nork".to_string())
        );
        assert_eq!(
            BasqueMorphology::PIVOT_PARADIGM.value(&da),
            Some("nor".to_string())
        );
    }

    /// A neutral zuka form and a toka form are the same paradigm cell with the
    /// same arguments — without the allocutive dimension `duk` would be
    /// analysed as `du` and the register would vanish.
    #[test]
    fn allocutive_survives_as_its_own_dimension() {
        let neutral = BasqueMorphology::Verb {
            lemma: "*edun".to_string(),
            form: BasqueVerbForm::Auxiliary,
            polarity: BasquePolarity::Affirmative,
            tense: Some(BasqueTense::Present),
            mood: Some(BasqueMood::Indicative),
            paradigm: Some(BasqueParadigm::NorNork),
            absolutive_agreement: Some(BasquePersonNumber::ThirdSingular),
            dative_agreement: None,
            ergative_agreement: Some(BasquePersonNumber::ThirdSingular),
            allocutive: None,
            subordination: None,
            case: None,
            determination: None,
        };
        let toka = BasqueMorphology::Verb {
            lemma: "*edun".to_string(),
            form: BasqueVerbForm::Auxiliary,
            polarity: BasquePolarity::Affirmative,
            tense: Some(BasqueTense::Present),
            mood: Some(BasqueMood::Indicative),
            paradigm: Some(BasqueParadigm::NorNork),
            absolutive_agreement: Some(BasquePersonNumber::ThirdSingular),
            dative_agreement: None,
            ergative_agreement: Some(BasquePersonNumber::ThirdSingular),
            allocutive: Some(BasqueAllocutive::MasculineFamiliar),
            subordination: None,
            case: None,
            determination: None,
        };

        assert_eq!(BasqueMorphology::PIVOT_ALLOCUTIVE.value(&neutral), None);
        assert_eq!(
            BasqueMorphology::PIVOT_ALLOCUTIVE.value(&toka),
            Some("masculine_familiar".to_string())
        );
        assert_ne!(neutral, toka);
    }

    /// Definiteness and number are one fused slot on the article, so an
    /// indefinite noun phrase has no number to report at all.
    #[test]
    fn determination_fuses_definiteness_and_number() {
        let indefinite = BasqueMorphology::Noun {
            lemma: "etxe".to_string(),
            case: Some(BasqueCase::Absolutive),
            determination: Some(BasqueDetermination::Indefinite),
        };

        assert_eq!(
            BasqueMorphology::PIVOT_DETERMINATION.value(&indefinite),
            Some("indefinite".to_string())
        );
        assert!(
            BasqueMorphology::PIVOT_DETERMINATION
                .values()
                .contains(&"proximate_plural")
        );
    }

    #[test]
    fn hypothetical_and_resultative_forms_are_closed_values() {
        assert!(
            BasqueMorphology::PIVOT_TENSE
                .values()
                .contains(&"hypothetical")
        );
        assert!(
            BasqueMorphology::PIVOT_FORM
                .values()
                .contains(&"resultative_participle")
        );
        assert!(
            BasqueMorphemeFunction::PIVOT_ASPECT
                .values()
                .contains(&"resultative")
        );
    }

    #[test]
    fn phrase_final_determiners_numerals_and_verbs_can_decline() {
        let honetan = BasqueMorphology::Determiner {
            lemma: "hau".to_string(),
            determiner_type: BasqueDeterminerType::Demonstrative,
            case: Some(BasqueCase::Inessive),
            determination: Some(BasqueDetermination::DefiniteSingular),
        };
        let hirurekin = BasqueMorphology::Numeral {
            lemma: "hiru".to_string(),
            numeral_type: BasqueNumeralType::Cardinal,
            case: Some(BasqueCase::Comitative),
            determination: Some(BasqueDetermination::Indefinite),
        };
        let ikustea = BasqueMorphology::Verb {
            lemma: "ikusi".to_string(),
            form: BasqueVerbForm::VerbalNoun,
            polarity: BasquePolarity::Affirmative,
            tense: None,
            mood: None,
            paradigm: None,
            absolutive_agreement: None,
            dative_agreement: None,
            ergative_agreement: None,
            allocutive: None,
            subordination: None,
            case: Some(BasqueCase::Absolutive),
            determination: Some(BasqueDetermination::DefiniteSingular),
        };

        for morphology in [honetan, hirurekin, ikustea] {
            let json = serde_json::to_value(morphology).unwrap();
            assert!(json.get("case").is_some());
            assert!(json.get("determination").is_some());
        }

        let zer = BasqueMorphology::Determiner {
            lemma: "zer".to_string(),
            determiner_type: BasqueDeterminerType::Interrogative,
            case: None,
            determination: None,
        };
        assert_eq!(BasqueMorphology::PIVOT_DECLENSION_CASE.value(&zer), None);
        assert_eq!(BasqueMorphology::PIVOT_DETERMINATION.value(&zer), None);
    }

    #[test]
    fn non_final_nouns_carry_no_phrase_ending() {
        let egun = BasqueMorphology::Noun {
            lemma: "egun".to_string(),
            case: None,
            determination: None,
        };

        assert_eq!(
            BasqueMorphology::PIVOT_DECLENSION_CASE.value(&egun),
            None
        );
        assert_eq!(BasqueMorphology::PIVOT_DETERMINATION.value(&egun), None);
    }

    #[test]
    fn declined_indefinite_quantifier_remains_indefinite() {
        let askotan = BasqueMorphology::Determiner {
            lemma: "asko".to_string(),
            determiner_type: BasqueDeterminerType::Quantifier,
            case: Some(BasqueCase::Inessive),
            determination: Some(BasqueDetermination::Indefinite),
        };

        assert_eq!(
            BasqueMorphology::PIVOT_DECLENSION_CASE.value(&askotan),
            Some("inessive".to_string())
        );
        assert_eq!(
            BasqueMorphology::PIVOT_DETERMINATION.value(&askotan),
            Some("indefinite".to_string())
        );
    }

    #[test]
    fn inventory_covers_resultatives_animate_locatives_and_bound_ba() {
        Basque::validate_inventory().unwrap();

        let inventory = Basque::morpheme_inventory();
        assert!(inventory.iter().any(|definition| {
            definition.base_form == "-ta/-da"
                && definition.functions.contains(&F::Aspect {
                    value: BasqueAspect::Resultative,
                })
        }));
        assert!(inventory.iter().any(|definition| {
            definition.base_form == "-(r)engandik"
                && definition.functions.contains(&F::Case {
                    value: BasqueCase::Ablative,
                })
        }));
        assert!(inventory.iter().any(|definition| {
            definition.base_form == "ba-"
                && definition.functions.contains(&F::Particle {
                    value: BasqueParticleType::Affirmative,
                })
                && definition.functions.contains(&F::Subordination {
                    value: BasqueSubordination::Conditional,
                })
        }));
    }
}
