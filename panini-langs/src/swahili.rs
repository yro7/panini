//! Coastal / Standard Swahili (ISO 639-3 `swh`), the unified Tanzanian and
//! Kenyan standard.
//!
//! The whole definition is organised around one fact: Swahili is a Bantu
//! language whose noun-class system drives agreement across almost every part
//! of speech. A noun belongs to a class, and that class is echoed by the
//! adjective, the demonstrative, the possessive, the numeral, the associative
//! connector and — twice over, in the subject and object slots — by the verb.
//! Number is fused into the class rather than marked separately, exactly as
//! Basque fuses number into the article, so there is no `number` field on
//! nouns: `class` is the single slot.
//!
//! The verb is the other half. Its template is
//! `subject – negation – TAM – relative – object – ROOT – extension(s) – final vowel`,
//! and each of those positions is a morpheme in the inventory below.

use serde::{Deserialize, Serialize};

use panini_core::morpheme::{Agglutinative, MorphemeDefinition, WordSegmentation};
use panini_core::traits::{
    BinaryNumber, BinaryVoice, IsoLang, LinguisticDefinition, MorphologyInfo, Person, Script,
    TypologicalFeature, Upos,
};

// ─── Noun classes ─────────────────────────────────────────────────────────────

/// The Bantu noun classes of Standard Swahili, in the conventional numbering.
///
/// Classes pair singular with plural (1/2, 3/4, 5/6, 7/8, 9/10, 11/10), so the
/// class of a token already states its number. Classes 14 (abstracts), 15
/// (infinitives) and 16/17/18 (locatives) have no number partner.
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
pub enum SwahiliNounClass {
    Class1,  // m-/mw- singular persons: mtu, mwalimu, mtoto (+ the 1a loans: baba, rafiki)
    Class2,  // wa- plural of 1: watu, walimu, watoto
    Class3,  // m-/mw- singular plants, body parts, natural forces: mti, mwaka, mkono
    Class4,  // mi- plural of 3: miti, miaka, mikono
    Class5,  // ji-/Ø singular, augmentatives, paired body parts: jicho, gari, tunda
    Class6,  // ma- plural of 5, and mass nouns: macho, magari, matunda, maji
    Class7,  // ki-/ch- singular artefacts, languages, diminutives: kitabu, chakula
    Class8,  // vi-/vy- plural of 7: vitabu, vyakula
    Class9,  // N- singular animals and loans: nyumba, ndizi, kalamu, simba
    Class10, // N- plural of 9 and of 11: nyumba, ndizi, kuta
    Class11, // u-/w- singular long or extended things: ukuta, wimbo, uso (plural in class 10)
    Class14, // u- abstract nouns, no plural: uhuru, upendo, ubaya, ujana
    Class15, // ku- verbal infinitives used as nouns: kusoma, kula, kuimba
    Class16, // pa- definite location: mahali pazuri, pana
    Class17, // ku- indefinite or directional location: kuna, kule, mjini kuna
    Class18, // mu-/m- interior location: mna, humu, ndani mwa
}

/// The concord paradigm — what a subject, object or relative slot can agree
/// with. First and second person have their own markers; every third-person
/// value is a noun class (class 1 and 2 doubling as third singular and plural).
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
pub enum SwahiliConcord {
    FirstSingular,  // ni- subject, -ni- object
    SecondSingular, // u- subject, -ku- object
    FirstPlural,    // tu- subject, -tu- object
    SecondPlural,   // m-/mw- subject, -wa- object
    Class1,         // a- subject, -m-/-mw- object (also third person singular)
    Class2,         // wa- subject and object (also third person plural)
    Class3,         // u- subject, -u- object
    Class4,         // i- subject, -i- object
    Class5,         // li- subject and object
    Class6,         // ya- subject and object
    Class7,         // ki- subject and object
    Class8,         // vi- subject and object
    Class9,         // i- subject and object
    Class10,        // zi- subject and object
    Class11,        // u- subject and object
    Class14,        // u- subject
    Class15,        // ku- subject
    Class16,        // pa- subject: mahali pana watu
    Class17,        // ku- subject: kuna watu
    Class18,        // m-/mu- subject: mna watu
}

// ─── Verbal categories ────────────────────────────────────────────────────────

/// The single tense–aspect–mood slot of the Swahili verb, between the negation
/// and the object marker. Exactly one marker occupies it.
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
pub enum SwahiliTam {
    Present,            // -na- : ninasoma (also the unmarked negative present, hasomi)
    PresentIndefinite,  // -a-  : twaona, aenda — literary, tenseless present
    Past,               // -li- : nilisoma (negative -ku-, sikusoma)
    Future,             // -ta- : nitasoma
    Perfect,            // -me- : nimesoma (negative -ja-, sijasoma "not yet")
    Habitual,           // hu-  : husoma — carries no subject marker at all
    Situational,        // -ki- : nikisoma "if/when/while I read" (negative -sipo-)
    Consecutive,        // -ka- : akaenda "and then he went"
    Conditional,        // -nge- : ningesoma "I would read"
    CounterfactualPast, // -ngali-/-ngeli- : ningalisoma "I would have read"
}

/// Finite against non-finite, plus the mood the final vowel selects.
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
pub enum SwahiliVerbForm {
    Indicative,  // subject marker + TAM + root + final -a: anasoma, hakusoma
    Subjunctive, // subject marker + root + final -e: asome, usisome, tuende
    Imperative,  // bare stem, no subject marker: soma!, someni!, njoo!
    Infinitive,  // ku- + root, a class 15 verbal noun: kusoma, kula
}

/// Polarity is marked on both ends of the verb — `ha-`/`si-` in front and a
/// changed final vowel behind — so it is a category of the whole word.
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
pub enum SwahiliPolarity {
    Affirmative, // ninasoma, nimekula
    Negative,    // sisomi, hakusoma, sijala, asisome
}

/// The stackable derivational suffixes between the root and the final vowel.
/// Modelled as morphemes rather than a morphology field because they compose:
/// `kusomeshwa` carries the causative and the passive at once.
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
pub enum SwahiliExtension {
    Applicative, // -i-/-e-, -li-/-le- : soma → somea "read for/to"
    Causative,   // -ish-/-esh-, -iz-/-ez-, -y- : soma → somesha "teach"
    Passive,     // -w-, -iw-/-ew- : soma → somwa "be read"
    Stative,     // -ik-/-ek- : vunja → vunjika "be breakable / get broken"
    Reciprocal,  // -an- : penda → pendana "love one another"
    Reversive,   // -u-/-o- : funga → fungua "open", ficha → fichua "reveal"
}

/// Non-verbal derivation, and the two affixes that change a word's class.
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
pub enum SwahiliDerivation {
    Reflexive, // -ji- in the object slot: anajipenda "he loves himself"
    Agentive,  // -ji on a class 1/2 noun: imba → mwimbaji "singer"
    Deverbal,  // -o on a derived noun: fungua → ufunguo "key", enda → mwendo
    Locative,  // -ni on a noun: nyumba → nyumbani "at home", shule → shuleni
}

// ─── Nominal-modifier categories ──────────────────────────────────────────────

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
pub enum SwahiliDeterminerType {
    Demonstrative, // huyu, hiki, kile, hicho
    Possessive,    // -angu, -ako, -ake, -etu, -enu, -ao
    Quantifier,    // -ote "all", -o -ote "any", -ingi "many"
    Interrogative, // -pi "which", gani "what kind"
    Indefinite,    // -ingine "other, another"
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
pub enum SwahiliPronounType {
    Personal,      // mimi, wewe, yeye, sisi, ninyi, wao
    Possessive,    // wangu, changu, yake used on their own
    Demonstrative, // huyu, hiki, kile standing without a head noun
    Interrogative, // nani, nini, lipi
    Relative,      // ambaye, ambacho, ambao
    Indefinite,    // kila mtu, mtu yeyote, kitu chochote
}

/// The three-way Swahili deixis: near the speaker, already mentioned, far.
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
pub enum SwahiliDemonstrative {
    Proximal,    // huyu, hiki, hii — near the speaker
    Referential, // huyo, hicho, hiyo — the one already mentioned
    Distal,      // yule, kile, ile — far from both
}

// ─── MorphemeFunction wrapper enum ────────────────────────────────────────────

/// What a Swahili affix does in the word. Every variant is single-field, so
/// every generated pivot is a clean closed pivot over its own dimension.
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
pub enum SwahiliMorphemeFunction {
    /// The class prefix on a noun, and the same shape echoed as concord on an
    /// adjective, numeral, possessive or demonstrative.
    ClassPrefix { value: SwahiliNounClass },
    /// The word-initial agreement slot of a verb.
    SubjectConcord { value: SwahiliConcord },
    /// The agreement slot immediately before the root.
    ObjectConcord { value: SwahiliConcord },
    /// The infixed or postfinal relative marker (-ye-, -cho-, -po- …).
    RelativeConcord { value: SwahiliConcord },
    Tense { value: SwahiliTam },
    Polarity { value: SwahiliPolarity },
    Mood { value: SwahiliVerbForm },
    Extension { value: SwahiliExtension },
    Derivation { value: SwahiliDerivation },
}

impl SwahiliMorphemeFunction {
    /// `category:value` — how one function is rendered in the inventory block of
    /// the extraction prompt. Every variant is single-field, so there is no
    /// composite case.
    fn directive_label(&self) -> String {
        let json = serde_json::to_value(self).expect("morpheme function is serializable");
        let category = json["category"]
            .as_str()
            .expect("internally tagged category is present");
        let value = json["value"].as_str().expect("value is serialized");
        format!("{category}:{value}")
    }
}

// ─── SwahiliMorphology ────────────────────────────────────────────────────────

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
pub enum SwahiliMorphology {
    /// Bantu adjectives take the concord of their head noun; Arabic loans
    /// (safi, rahisi, ghali, bora, hodari, tayari) are invariable and carry none.
    Adjective {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_class: Option<SwahiliNounClass>,
    },
    /// Plain prepositions (na, kwa, katika, bila, hadi) carry no concord; the
    /// associative connector -a (wa, ya, cha, vya, la, za, kwa, pa, mwa) does.
    Adposition {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_class: Option<SwahiliNounClass>,
    },
    Adverb {
        lemma: String,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: SwahiliDeterminerType,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_class: Option<SwahiliNounClass>,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        distance: Option<SwahiliDemonstrative>,
    },
    Interjection {
        lemma: String,
    },
    /// `class` is the fused class-and-number slot — class 7 *is* singular and
    /// class 8 *is* its plural — so there is no separate number field.
    /// `locative` records the derived -ni form, which keeps its own class but
    /// governs class 16/17/18 concord.
    Noun {
        lemma: String,
        class: SwahiliNounClass,
        locative: bool,
    },
    Numeral {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_class: Option<SwahiliNounClass>,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        pronoun_type: SwahiliPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_class: Option<SwahiliNounClass>,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        distance: Option<SwahiliDemonstrative>,
    },
    ProperNoun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        class: Option<SwahiliNounClass>,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    /// One orthographic word holds the whole template: subject marker,
    /// negation, TAM, relative marker, object marker, root, extensions and
    /// final vowel. Each agreement slot is filled independently and only when
    /// the form actually indexes that argument.
    Verb {
        lemma: String,
        form: SwahiliVerbForm,
        polarity: SwahiliPolarity,
        voice: BinaryVoice,
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<SwahiliTam>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject_agreement: Option<SwahiliConcord>,
        #[serde(skip_serializing_if = "Option::is_none")]
        object_agreement: Option<SwahiliConcord>,
        #[serde(skip_serializing_if = "Option::is_none")]
        relative_agreement: Option<SwahiliConcord>,
    },
    Other {
        lemma: String,
    },
}

impl SwahiliMorphology {
    /// `tense` is `Option` — an infinitive, an imperative and a plain
    /// subjunctive carry none — so the derive skips it for pivot generation.
    /// Written by hand to keep the facet available.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|t| panini_core::aggregable::ClosedValues::variant_str(t).to_string()),
            Self::Adjective { .. }
            | Self::Adposition { .. }
            | Self::Adverb { .. }
            | Self::CoordinatingConjunction { .. }
            | Self::Determiner { .. }
            | Self::Interjection { .. }
            | Self::Noun { .. }
            | Self::Numeral { .. }
            | Self::Particle { .. }
            | Self::Pronoun { .. }
            | Self::ProperNoun { .. }
            | Self::SubordinatingConjunction { .. }
            | Self::Symbol { .. }
            | Self::Other { .. } => None,
        }
    }

    /// Typed pivot handle for tense (see [`SwahiliMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <SwahiliTam as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// `agreement_class` is `Option` on every part of speech that carries it —
    /// an invariable adjective or a plain preposition has none — so the derive
    /// generates no handle. Written by hand: concord is the dimension a Swahili
    /// learner most wants to slice their lexicon by, and the inherent
    /// `PIVOT_CLASS` covers only nouns.
    fn __pivot_agreement_class(&self) -> Option<String> {
        let class = match self {
            Self::Adjective {
                agreement_class, ..
            }
            | Self::Adposition {
                agreement_class, ..
            }
            | Self::Determiner {
                agreement_class, ..
            }
            | Self::Numeral {
                agreement_class, ..
            }
            | Self::Pronoun {
                agreement_class, ..
            } => agreement_class.as_ref(),
            Self::Adverb { .. }
            | Self::CoordinatingConjunction { .. }
            | Self::Interjection { .. }
            | Self::Noun { .. }
            | Self::Particle { .. }
            | Self::ProperNoun { .. }
            | Self::SubordinatingConjunction { .. }
            | Self::Symbol { .. }
            | Self::Verb { .. }
            | Self::Other { .. } => None,
        };
        class.map(|c| panini_core::aggregable::ClosedValues::variant_str(c).to_string())
    }

    /// Typed pivot handle for concord
    /// (see [`SwahiliMorphology::__pivot_agreement_class`]).
    pub const PIVOT_AGREEMENT_CLASS: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "agreement_class",
            "Agreement Class",
            <SwahiliNounClass as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_agreement_class,
        );
}

// ─── Static morpheme inventory ────────────────────────────────────────────────

type P = SwahiliMorphologyPosTag;
type F = SwahiliMorphemeFunction;

/// Notation: a trailing hyphen marks a prefix or an infix (`ki-`, `-cho-`), a
/// leading hyphen alone marks a suffix (`-ni`, `-e`). Where a morpheme has
/// well-known allomorphs the base form spells both (`-ish-/-esh-`), and the
/// `surface` field records which one actually appeared.
///
/// One entry per phonological shape, listing every function that shape can
/// serve: `ki-` is the class 7 prefix, the class 7 subject and object concord,
/// and the situational TAM, and they are historically one element.
static SWAHILI_MORPHEMES: &[MorphemeDefinition<F, P>] = &[
    // === Class prefixes and the concords that echo them ===
    MorphemeDefinition {
        base_form: "m-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class1,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class3,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class18,
            },
            F::SubjectConcord {
                value: SwahiliConcord::SecondPlural,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class18,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class1,
            },
        ],
        applies_to: &[P::Noun, P::Adjective, P::Numeral, P::Determiner, P::Verb],
    },
    MorphemeDefinition {
        base_form: "wa-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class2,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class2,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class2,
            },
            F::ObjectConcord {
                value: SwahiliConcord::SecondPlural,
            },
        ],
        applies_to: &[
            P::Noun,
            P::Adjective,
            P::Numeral,
            P::Determiner,
            P::Adposition,
            P::Pronoun,
            P::Verb,
        ],
    },
    MorphemeDefinition {
        base_form: "mi-",
        functions: &[F::ClassPrefix {
            value: SwahiliNounClass::Class4,
        }],
        applies_to: &[P::Noun, P::Adjective, P::Numeral, P::Determiner],
    },
    MorphemeDefinition {
        base_form: "ji-",
        functions: &[F::ClassPrefix {
            value: SwahiliNounClass::Class5,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "ma-",
        functions: &[F::ClassPrefix {
            value: SwahiliNounClass::Class6,
        }],
        applies_to: &[P::Noun, P::Adjective, P::Numeral, P::Determiner],
    },
    MorphemeDefinition {
        base_form: "ki-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class7,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class7,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class7,
            },
            F::Tense {
                value: SwahiliTam::Situational,
            },
        ],
        applies_to: &[
            P::Noun,
            P::Adjective,
            P::Numeral,
            P::Determiner,
            P::Adverb,
            P::Verb,
        ],
    },
    MorphemeDefinition {
        base_form: "vi-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class8,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class8,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class8,
            },
        ],
        applies_to: &[
            P::Noun,
            P::Adjective,
            P::Numeral,
            P::Determiner,
            P::Adverb,
            P::Verb,
        ],
    },
    MorphemeDefinition {
        base_form: "n-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class9,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class10,
            },
        ],
        applies_to: &[P::Noun, P::Adjective, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "u-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class11,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class14,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class3,
            },
            F::SubjectConcord {
                value: SwahiliConcord::SecondSingular,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class3,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class11,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class14,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class3,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class11,
            },
        ],
        applies_to: &[
            P::Noun,
            P::Adjective,
            P::Determiner,
            P::Adposition,
            P::Verb,
        ],
    },
    MorphemeDefinition {
        base_form: "ku-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class15,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class17,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class15,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class17,
            },
            F::ObjectConcord {
                value: SwahiliConcord::SecondSingular,
            },
            F::Tense {
                value: SwahiliTam::Past,
            },
            F::Mood {
                value: SwahiliVerbForm::Infinitive,
            },
        ],
        applies_to: &[P::Verb, P::Noun, P::Determiner, P::Adposition],
    },
    MorphemeDefinition {
        base_form: "pa-",
        functions: &[
            F::ClassPrefix {
                value: SwahiliNounClass::Class16,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class16,
            },
        ],
        applies_to: &[P::Noun, P::Adjective, P::Determiner, P::Adposition, P::Verb],
    },
    // === Verbal concords with no nominal counterpart ===
    MorphemeDefinition {
        base_form: "ni-",
        functions: &[
            F::SubjectConcord {
                value: SwahiliConcord::FirstSingular,
            },
            F::ObjectConcord {
                value: SwahiliConcord::FirstSingular,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "tu-",
        functions: &[
            F::SubjectConcord {
                value: SwahiliConcord::FirstPlural,
            },
            F::ObjectConcord {
                value: SwahiliConcord::FirstPlural,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "a-",
        functions: &[
            F::SubjectConcord {
                value: SwahiliConcord::Class1,
            },
            F::Tense {
                value: SwahiliTam::PresentIndefinite,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "i-",
        functions: &[
            F::SubjectConcord {
                value: SwahiliConcord::Class4,
            },
            F::SubjectConcord {
                value: SwahiliConcord::Class9,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class4,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class9,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "li-",
        functions: &[
            F::SubjectConcord {
                value: SwahiliConcord::Class5,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class5,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class5,
            },
            F::Tense {
                value: SwahiliTam::Past,
            },
        ],
        applies_to: &[P::Verb, P::Determiner, P::Adposition],
    },
    MorphemeDefinition {
        base_form: "ya-",
        functions: &[
            F::SubjectConcord {
                value: SwahiliConcord::Class6,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class6,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class6,
            },
        ],
        applies_to: &[P::Verb, P::Determiner, P::Adposition],
    },
    MorphemeDefinition {
        base_form: "zi-",
        functions: &[
            F::SubjectConcord {
                value: SwahiliConcord::Class10,
            },
            F::ObjectConcord {
                value: SwahiliConcord::Class10,
            },
            F::ClassPrefix {
                value: SwahiliNounClass::Class10,
            },
        ],
        applies_to: &[P::Verb, P::Determiner, P::Adposition],
    },
    // === Negation ===
    MorphemeDefinition {
        base_form: "ha-",
        functions: &[F::Polarity {
            value: SwahiliPolarity::Negative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "si-",
        functions: &[
            F::Polarity {
                value: SwahiliPolarity::Negative,
            },
            F::SubjectConcord {
                value: SwahiliConcord::FirstSingular,
            },
        ],
        applies_to: &[P::Verb, P::Particle],
    },
    MorphemeDefinition {
        base_form: "-i",
        functions: &[F::Polarity {
            value: SwahiliPolarity::Negative,
        }],
        applies_to: &[P::Verb],
    },
    // === Tense, aspect and mood markers ===
    MorphemeDefinition {
        base_form: "na-",
        functions: &[F::Tense {
            value: SwahiliTam::Present,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "ta-",
        functions: &[F::Tense {
            value: SwahiliTam::Future,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "me-",
        functions: &[F::Tense {
            value: SwahiliTam::Perfect,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "ja-",
        functions: &[
            F::Tense {
                value: SwahiliTam::Perfect,
            },
            F::Polarity {
                value: SwahiliPolarity::Negative,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "hu-",
        functions: &[F::Tense {
            value: SwahiliTam::Habitual,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "ka-",
        functions: &[F::Tense {
            value: SwahiliTam::Consecutive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "nge-",
        functions: &[F::Tense {
            value: SwahiliTam::Conditional,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "ngali-",
        functions: &[F::Tense {
            value: SwahiliTam::CounterfactualPast,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "sipo-",
        functions: &[
            F::Tense {
                value: SwahiliTam::Situational,
            },
            F::Polarity {
                value: SwahiliPolarity::Negative,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-e",
        functions: &[F::Mood {
            value: SwahiliVerbForm::Subjunctive,
        }],
        applies_to: &[P::Verb],
    },
    // === Relative markers ===
    MorphemeDefinition {
        base_form: "-ye-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class1,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-o-",
        functions: &[
            F::RelativeConcord {
                value: SwahiliConcord::Class2,
            },
            F::RelativeConcord {
                value: SwahiliConcord::Class3,
            },
            F::RelativeConcord {
                value: SwahiliConcord::Class11,
            },
            F::RelativeConcord {
                value: SwahiliConcord::Class14,
            },
        ],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-cho-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class7,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-vyo-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class8,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-lo-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class5,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-yo-",
        functions: &[
            F::RelativeConcord {
                value: SwahiliConcord::Class4,
            },
            F::RelativeConcord {
                value: SwahiliConcord::Class6,
            },
            F::RelativeConcord {
                value: SwahiliConcord::Class9,
            },
        ],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-zo-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class10,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-po-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class16,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-ko-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class17,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "-mo-",
        functions: &[F::RelativeConcord {
            value: SwahiliConcord::Class18,
        }],
        applies_to: &[P::Verb, P::Pronoun],
    },
    // === Verbal extensions ===
    MorphemeDefinition {
        base_form: "-i-/-e-",
        functions: &[F::Extension {
            value: SwahiliExtension::Applicative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-ish-/-esh-",
        functions: &[F::Extension {
            value: SwahiliExtension::Causative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-w-",
        functions: &[F::Extension {
            value: SwahiliExtension::Passive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-ik-/-ek-",
        functions: &[F::Extension {
            value: SwahiliExtension::Stative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-an-",
        functions: &[F::Extension {
            value: SwahiliExtension::Reciprocal,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-u-/-o-",
        functions: &[F::Extension {
            value: SwahiliExtension::Reversive,
        }],
        applies_to: &[P::Verb],
    },
    // === Derivation ===
    MorphemeDefinition {
        base_form: "-ji-",
        functions: &[F::Derivation {
            value: SwahiliDerivation::Reflexive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-ji",
        functions: &[F::Derivation {
            value: SwahiliDerivation::Agentive,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "-o",
        functions: &[F::Derivation {
            value: SwahiliDerivation::Deverbal,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "-ni",
        functions: &[
            F::Derivation {
                value: SwahiliDerivation::Locative,
            },
            F::Mood {
                value: SwahiliVerbForm::Imperative,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Verb],
    },
];

// ─── Agglutinative implementation ────────────────────────────────────────────

impl Agglutinative for Swahili {
    fn morpheme_inventory() -> &'static [MorphemeDefinition<
        SwahiliMorphemeFunction,
        <SwahiliMorphology as MorphologyInfo>::PosTag,
    >] {
        SWAHILI_MORPHEMES
    }

    fn morpheme_directives(&self) -> String {
        let inventory_lines = SWAHILI_MORPHEMES
            .iter()
            .map(|morpheme| {
                let functions = morpheme
                    .functions
                    .iter()
                    .map(SwahiliMorphemeFunction::directive_label)
                    .collect::<Vec<_>>()
                    .join(" / ");
                format!("  {} → {functions}", morpheme.base_form)
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "MORPHEME SEGMENTATION — fill `morpheme_segmentation` as an array of objects, one per \
             word that carries at least one affix from the inventory below.\n\
             Each object has:\n\
             - `word`: the surface form of the whole word\n\
             - `morphemes`: one entry per affix, in the order they appear left to right — NOT the \
             root, whose dictionary form is already the word's `lemma`:\n\
               - `surface`: the allomorph as it actually appears (\"ch\", \"mw\", \"esh\", \"li\")\n\
               - `base_form`: the identifier from the inventory below, copied verbatim\n\
               - `function`: {{\"category\": \"<category>\", \"value\": \"<value>\"}}\n\
             \n\
             <morpheme_inventory>\n\
             Use ONLY these base forms:\n\
             {inventory_lines}\n\
             </morpheme_inventory>\n\
             \n\
             NOTATION: a trailing hyphen marks a prefix or infix (`ki-`, `-cho-`); a leading hyphen \
             alone marks a suffix (`-ni`, `-e`). `-ji-` is the reflexive infix inside a verb and \
             `-ji` is the agentive suffix on a noun — they are different entries, decide by \
             position. A base form written with a slash (`-ish-/-esh-`) is one morpheme with two \
             allomorphs; copy the base form whole and record which allomorph you saw in `surface`.\n\
             ALLOMORPHY: map the surface shape onto the one base form. `ch-`/`vy-`/`mw-`/`ny-`/`nd-`\
             /`mb-`/`w-` are allomorphs of `ki-`/`vi-`/`m-`/`n-`/`n-`/`n-`/`u-`. The causative also \
             surfaces as `-iz-`, `-ez-`, `-y-` and `-s-`; the applicative as `-li-` and `-le-` after \
             a vowel stem; the passive as `-iw-` and `-ew-`. `-ngeli-` is an allomorph of `ngali-`.\n\
             VERB TEMPLATE: segment a verb in template order — subject concord, negation, TAM, \
             relative concord, object concord, ROOT, extension(s), final vowel. The default \
             indicative final vowel `-a` is NOT in the inventory and must not be segmented; only \
             the subjunctive `-e` and the negative present `-i` are.\n\
             NOUN PREFIX: segment the class prefix of a noun and the concord prefix of an \
             adjective, numeral, possessive, demonstrative or associative connector. A class 5 or \
             class 9/10 noun with no visible prefix (gari, nyumba, kalamu) has nothing to segment — \
             omit it from `morpheme_segmentation` entirely.\n\
             Never invent a base form, never use `-` or the root as a base form, and omit any word \
             that carries no listed affix."
        )
    }
}

// ─── LinguisticDefinition implementation ─────────────────────────────────────

pub struct Swahili;

impl LinguisticDefinition for Swahili {
    type Morphology = SwahiliMorphology;
    type MorphemeFunction = SwahiliMorphemeFunction;

    const ISO_LANG: IsoLang = IsoLang::Swh;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        SwahiliMorphology::PIVOT_CLASS,
        SwahiliMorphology::PIVOT_AGREEMENT_CLASS,
        SwahiliMorphology::PIVOT_TENSE,
        SwahiliMorphology::PIVOT_POLARITY,
        SwahiliMorphology::PIVOT_VOICE,
    ];
    const MORPHEME_PIVOTS: &'static [panini_core::pivot::PivotField<Self::MorphemeFunction>] = &[
        SwahiliMorphemeFunction::PIVOT_CLASS_PREFIX,
        SwahiliMorphemeFunction::PIVOT_SUBJECT_CONCORD,
        SwahiliMorphemeFunction::PIVOT_TENSE,
        SwahiliMorphemeFunction::PIVOT_EXTENSION,
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
                Upos::Adjective,
                Upos::Determiner,
                Upos::Numeral,
                Upos::Pronoun,
            ]),
            TypologicalFeature::Agglutination,
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization — nouns take the SINGULAR member of their class pair with any locative -ni stripped: vitabu → kitabu, watu → mtu, miti → mti, macho → jicho, nyumbani → nyumba, kutani → ukuta. A noun that has no singular partner (maji, mazingira, marufuku, fedha) lemmatizes to itself. Verbs take the INFINITIVE WITH ku-: anasoma → kusoma, tulikula → kula, hawajaenda → kwenda, nimekuwa → kuwa. Adjectives, agreeing numerals, possessives and quantifiers take the BARE STEM with the concord prefix removed and NO hyphen: wazuri → zuri, kikubwa → kubwa, mrefu → refu, wawili → wili, vitatu → tatu, changu → angu, wote → ote, kingine → ingine. Every other word — demonstratives, personal and relative pronouns, adpositions including the associative connector, conjunctions, particles, adverbs and invariable Arabic-loan adjectives (safi, rahisi, ghali, bora, hodari, tayari) — lemmatizes to its own surface form.\n\
         2. Tokenization — a Swahili verb is ONE token no matter how much it carries. ninakupenda, hatutakwenda, alichokisoma, aliyeniambia, kusomeshwa and hakuna are each a single verb token; never split off a subject marker, a tense marker, an object marker, a relative marker or an extension. The locative -ni is part of its noun (nyumbani, shuleni, mjini = one noun token each). Reduplications written solid (polepole, harakaharaka, mbalimbali) are one token. The associative connector (wa, ya, cha, vya, la, za, kwa, pa, mwa) IS its own token, as are na, kwa, katika, bila, hadi, kama and the amba- relatives (ambaye, ambacho, ambao).\n\
         3. Nouns: always give `class` and `locative`. `class` is the class of the TOKEN as it stands, not of its lemma — vitabu is class8 even though its lemma kitabu is class7, and watu is class2. Do NOT report number separately: Swahili fuses number into the class, class 7 IS the singular and class 8 IS its plural. Set `locative` to true only for a noun actually carrying the locative -ni (nyumbani, shuleni, kazini, mikononi); keep the noun's own class there and set `locative` false everywhere else, including for words that merely end in -ni lexically (jini, nyani, karani, sini).\n\
         4. Class 1a and 2a — kinship terms, titles and Arabic loans denoting people (baba, mama, dada, kaka, rafiki, daktari, mwalimu, bibi, mtoto, jirani) take class 1 and class 2 concord although most carry no m-/wa- prefix. Tag them class1 in the singular and class2 in the plural, decided by the concord they trigger (baba yangu ANAKUJA → class1; baba zangu WANAKUJA → class2). NEVER tag a person as class9/class10 just because the noun has no visible prefix.\n\
         5. Classes 9 and 10 are formally identical (nyumba, ndizi, kalamu, simba are both singular and plural). Decide from the concord in the sentence: nyumba NZURI INAJENGWA → class9, nyumba NZURI ZINAJENGWA → class10. When nothing in the sentence disambiguates, use class9 for a singular reading and class10 for a plural one. The same choice applies to class 11 nouns, whose plural is class 10: ukuta → kuta (class10), wimbo → nyimbo (class10).\n\
         6. Verbs: always give `form`, `polarity` and `voice`. `form` is `infinitive` for a ku- verbal noun (kusoma, kula, kusomeshwa), `imperative` for a bare-stem command with no subject marker (soma!, someni!, njoo!, lete!), `subjunctive` for a subject marker + root + final -e (nisome, tuende, asisome — the negative imperative usisome is a NEGATIVE SUBJUNCTIVE, not an imperative), and `indicative` everywhere else. `voice` is `passive` whenever the passive extension -w-/-iw-/-ew- is present (anasomwa, kupendwa, kimeandikwa) and `active` otherwise. `polarity` is `negative` for any ha-, si-, -ja-, -si- or -sipo- form and for the negative present in -i (hasomi), `affirmative` otherwise.\n\
         7. Verb `tense` names the semantic TAM slot even when the negative paradigm leaves it empty. -na- → present, -a- → present_indefinite, -li- → past, -ta- → future, -me- → perfect, hu- → habitual, -ki- → situational, -ka- → consecutive, -nge- → conditional, -ngali-/-ngeli- → counterfactual_past. In the negative: the bare ha-…-i form (hasomi) is tense present, -ku- (hakusoma) is past, -ja- (hajasoma) is perfect, -sipo- (nisipoenda) is situational. OMIT `tense` entirely on an infinitive, on an imperative and on a plain subjunctive — none of them fills the TAM slot.\n\
         8. Agreement — this is the core of the language, and each slot is filled independently. `subject_agreement` is the word-initial marker: ni- first_singular, u- second_singular, tu- first_plural, m-/mw- second_plural, a- class1, wa- class2, u- class3/class11/class14, i- class4/class9, li- class5, ya- class6, ki- class7, vi- class8, zi- class10, ku- class15/class17, pa- class16, m-/mu- class18. `object_agreement` is the marker immediately before the root and is present only when the verb actually carries one — ninakuona has object_agreement second_singular, ninaona has none. `relative_agreement` is the infixed or postfinal relative marker (-ye-, -o-, -cho-, -vyo-, -lo-, -yo-, -zo-, -po-, -ko-, -mo-, and postfinal -ye/-vyo/-po as in asomaye, alipokuja) and is present only when the verb carries one. OMIT every slot the form does not fill.\n\
         9. Two verb forms take no subject marker at all: the habitual hu- form (husoma, hutembea) and the affirmative imperative (soma!, someni!). Omit `subject_agreement` for both. The infinitive likewise has none — its ku- is a class 15 prefix, not a subject concord.\n\
         10. Adjectives, numerals, determiners, pronouns and the associative connector: give `agreement_class` whenever the word actually carries a concord prefix, and OMIT it when it does not. mzuri → class1, wazuri → class2, kizuri → class7, vizuri → class8, nzuri → class9 or class10, mizuri → class4, mazuri → class6; wawili → class2, vitatu → class8; kitabu CHA mwalimu → the connector cha is class7, watoto WA shule → wa is class2. Invariable adjectives (safi, rahisi, ghali, bora, hodari, tayari, kila) and the non-agreeing numerals sita, saba, tisa, kumi, ishirini, mia and elfu carry NO concord: omit the field.\n\
         11. Determiners: give `determiner_type`. Demonstratives also take `distance` — `proximal` for the hu-/hi- series near the speaker (huyu, hawa, huu, hii, hiki, hivi, hizi), `referential` for the -o series pointing back at something already mentioned (huyo, hao, hicho, hivyo, hiyo, hizo), `distal` for the -le series (yule, wale, ule, ile, kile, vile, zile). Possessives also take `person` and `number` for the POSSESSOR read off the stem: -angu first singular, -ako second singular, -ake third singular, -etu first plural, -enu second plural, -ao third plural — so kitabu changu is agreement_class class7 with person first and number singular. Omit `person`, `number` and `distance` on every non-possessive, non-demonstrative determiner.\n\
         12. Pronouns: give `pronoun_type`. Personal pronouns take `person` and `number` (mimi first singular, wewe second singular, yeye third singular, sisi first plural, ninyi/nyinyi second plural, wao third plural) and no concord. Demonstrative and relative pronouns take `agreement_class` (ambaye class1, ambao class2, ambacho class7, ambavyo class8, ambayo class4/class6/class9) and, for demonstratives, `distance`. Interrogative pronouns (nani, nini) take neither.\n\
         13. The invariable copula `ni` (\"is/are\") and its negative `si` are verbs: tag them `verb` with form indicative, the matching polarity, voice active, and no tense and no agreement slots at all. Do not confuse either with the homophonous verbal prefixes — ni- inside a word is the first-person-singular subject marker, si- inside a word is the negative.\n\
         14. Existentials are ordinary verbs whose subject agreement is a locative class: kuna / hakuna → class17, pana / hapana → class16, mna / hamna → class18. Tag them verb, indicative, with `subject_agreement` set accordingly.\n\
         15. Guardrails for the confusions this language actually provokes:\n\
         - ki- is the class 7 prefix on a NOUN or an adjective (kitabu, kizuri) and the situational TAM INSIDE a verb (nikisoma \"if/when I read\"). Decide by the host word, never by the shape.\n\
         - ku- is four different things: the infinitive/class 15 prefix (kusoma), the class 17 locative (kuna, kule), the negative past marker (sikusoma) and the second-person-singular object marker (ninakuona). Decide by position inside the verb.\n\
         - na is the present tense marker only INSIDE a verb (ninasoma). Standing alone it is the adposition/conjunction \"and, with, by\" and must be tagged adposition or coordinating_conjunction, never a tense.\n\
         - wa is the class 2 prefix (watu), the class 2 associative connector (watoto wa shule) and the root of kuwa \"to be\". Decide by the token boundaries.\n\
         - -a as a final vowel is the plain indicative and carries no information; do not report it as a tense, a mood or a morpheme.\n\
         - Swahili has NO grammatical gender and no gender agreement anywhere. Never emit masculine, feminine or neuter in any field; the noun class is not a gender and must be reported in `class` or `agreement_class`, never in `person` or `number`.\n\
         - Never put a noun-class value in `person` or `number`, and never put a person or number value in `class` or `agreement_class`. The two vocabularies do not overlap.\n\
         - Swahili has no case. Never emit nominative, accusative, genitive, dative or locative as a case value; grammatical relations are shown by word order and by the concord slots on the verb.\n\
         16. Swahili is NOT a tonal language and its orthography is fully transparent: five vowels, no vowel-length contrast, no tone marks, no diacritics. Read every word exactly as written; never add an accent, a macron or a tone mark, and never report a phonological value in a morphological field."
    }

    fn extra_extraction_directives(&self) -> Option<String> {
        Some(self.morpheme_directives())
    }

    fn post_process_extraction(
        &self,
        segmentation: &mut Option<Vec<WordSegmentation<SwahiliMorphemeFunction>>>,
    ) -> Result<(), String> {
        self.validate_and_enrich(segmentation)
    }
}
