use serde::{Deserialize, Serialize};

use panini_core::traits::{
    IsoLang, LinguisticDefinition, Script, SlavicAspect, TypologicalFeature,
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
    /// Adjective (ADJ)
    Adjective {
        lemma: String,
        gender: PolishGender,
        case: PolishCase,
    },
    /// Adposition (ADP)
    Adposition {
        lemma: String,
        /// The grammatical case this adposition governs.
        case: PolishCase,
    },
    /// Adverb (ADV)
    Adverb { lemma: String },
    /// Auxiliary (AUX)
    Auxiliary { lemma: String },
    /// Coordinating conjunction (CCONJ)
    CoordinatingConjunction { lemma: String },
    /// Determiner (DET)
    Determiner { lemma: String },
    /// Interjection (INTJ)
    Interjection { lemma: String },
    /// Noun (NOUN)
    Noun {
        lemma: String,
        gender: PolishGender,
        case: PolishCase,
    },
    /// Numeral (NUM)
    Numeral { lemma: String },
    /// Particle (PART)
    Particle { lemma: String },
    /// Pronoun (PRON)
    Pronoun { lemma: String, case: PolishCase },
    /// Proper noun (PROPN)
    ProperNoun { lemma: String },
    /// Punctuation (PUNCT)
    Punctuation { lemma: String },
    /// Subordinating conjunction (SCONJ)
    SubordinatingConjunction { lemma: String },
    /// Symbol (SYM)
    Symbol { lemma: String },
    /// Verb (VERB)
    Verb {
        lemma: String,
        tense: PolishTense,
        aspect: SlavicAspect,
    },
    /// Other (X) for unanalyzable tokens
    Other { lemma: String },
}

pub struct Polish;

impl LinguisticDefinition for Polish {
    type Morphology = PolishMorphology;
    type GrammaticalFunction = ();
    // TODO : add macro to generate pivots with big brain
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
        &[TypologicalFeature::Conjugation, TypologicalFeature::Declension]
    }

    fn extraction_directives(&self) -> &'static str {
        "Do not forget to specify 'cases' when extracting the features."
    }
}
