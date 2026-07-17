use serde::{Deserialize, Serialize};

use panini_core::traits::{
    IsoLang, LinguisticDefinition, Person, Script, SlavicAspect, TypologicalFeature, Upos,
};

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
pub enum PolishCase {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Instrumental,
    Locative,
    Vocative,
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
pub enum PolishGender {
    MasculinePersonal,
    MasculineAnimate,
    MasculineInanimate,
    Feminine,
    Neuter,
}

impl PolishGender {
    #[must_use]
    pub const fn is_masculine(&self) -> bool {
        matches!(
            self,
            Self::MasculinePersonal | Self::MasculineAnimate | Self::MasculineInanimate
        )
    }
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
pub enum PolishTense {
    Past,
    Present,
    Future,
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
pub enum PolishNumber {
    Singular,
    Plural,
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
pub enum PolishMorphology {
    /// Adjective
    Adjective {
        lemma: String,
        gender: PolishGender,
        number: PolishNumber,
        case: PolishCase,
    },
    /// Adposition
    Adposition {
        lemma: String,
        /// The grammatical case this adposition governs.
        case: PolishCase,
    },
    /// Adverb
    Adverb { lemma: String },
    /// Coordinating conjunction
    CoordinatingConjunction { lemma: String },
    /// Determiner
    Determiner { lemma: String },
    /// Interjection
    Interjection { lemma: String },
    /// Noun
    Noun {
        lemma: String,
        gender: PolishGender,
        number: PolishNumber,
        case: PolishCase,
    },
    /// Numeral
    Numeral { lemma: String },
    /// Particle
    Particle { lemma: String },
    /// Pronoun
    Pronoun {
        lemma: String,
        number: PolishNumber,
        case: PolishCase,
    },
    /// Proper noun
    ProperNoun { lemma: String },
    /// Subordinating conjunction
    SubordinatingConjunction { lemma: String },
    /// Verb
    Verb {
        lemma: String,
        /// Grammatical person; omit for infinitives.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Grammatical number; omit for infinitives.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<PolishNumber>,
        /// Tense; omit for infinitives and imperatives.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<PolishTense>,
        aspect: SlavicAspect,
    },
    /// Other, for unanalyzable tokens
    Other { lemma: String },
}

impl PolishMorphology {
    /// Extracts the tense value for the tense pivot.
    ///
    /// `tense` is `Option` on the verb (absent for infinitives/imperatives), so the
    /// `MorphologyInfo` derive skips it for pivot generation. This hand-written
    /// handle keeps `PIVOT_TENSE` available for lexicon faceting, yielding `None`
    /// when no tense was extracted.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|t| panini_core::aggregable::ClosedValues::variant_str(t).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for verb tense. Defined manually because `tense` is
    /// optional (see [`PolishMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <PolishTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );
}

pub struct Polish;

impl LinguisticDefinition for Polish {
    type Morphology = PolishMorphology;
    type MorphemeFunction = ();
    const ISO_LANG: IsoLang = IsoLang::Pol;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        PolishMorphology::PIVOT_CASE,
        PolishMorphology::PIVOT_ASPECT,
        PolishMorphology::PIVOT_GENDER,
        PolishMorphology::PIVOT_TENSE,
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
                Upos::Numeral,
                Upos::Determiner,
            ]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "Always specify 'case' and 'number' for nouns, adjectives and pronouns.\n\
         For verbs, provide 'person', 'number' and 'tense' only for finite forms; \
         omit all three for infinitives, and omit 'tense' for imperatives."
    }
}
