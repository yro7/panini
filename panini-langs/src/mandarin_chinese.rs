use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature,
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
pub enum ChineseAspect {
    Perfective,   // 了 (le)
    Progressive,  // 着 (zhe) / 在 (zai)
    Experiential, // 过 (guo)
    Delimitative, // reduplicated verbs (e.g., 看看)
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
pub enum ChineseParticleType {
    Structural, // 的, 地, 得
    Aspect,     // 了, 着, 过
    Modal,      // 吗, 吧, 呢, 啊
    Temporal,   // 以前, 以后
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
pub enum ChinesePronounType {
    Personal,
    Demonstrative,
    Interrogative,
    Reflexive,
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
pub enum MandarinChineseMorphology {
    Adjective {
        lemma: String,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
    },
    Classifier {
        lemma: String,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_type: ChineseParticleType,
    },
    Pronoun {
        lemma: String,
        pronoun_type: ChinesePronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    ProperNoun {
        lemma: String,
    },
    Punctuation {
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
        #[serde(skip_serializing_if = "Option::is_none")]
        aspect: Option<ChineseAspect>,
    },
    Other {
        lemma: String,
    },
}

pub struct MandarinChinese;

impl LinguisticDefinition for MandarinChinese {
    type Morphology = MandarinChineseMorphology;
    type GrammaticalFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Cmn;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        MandarinChineseMorphology::PIVOT_PARTICLE_TYPE,
        MandarinChineseMorphology::PIVOT_PRONOUN_TYPE,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::HANI]
    }

    fn default_script(&self) -> Script {
        Script::HANI
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Tokenization and Lemmatization: Mandarin Chinese does not use spaces to separate words. Segment words accurately (e.g. '我们' as one token, not '我' and '们'). The lemma should be the standard written form (Simplified or Traditional depending on the input script).\n\
         2. Classifiers (量词): Identify classifiers (e.g., '个', '只', '本') separately from numerals. Tag them as Classifier (classifier).\n\
         3. Aspect Particles (助词): Identify grammatical aspect markers like '了' (perfective/change of state), '着' (progressive), and '过' (experiential). If attached to a verb, annotate the verb's aspect accordingly, or represent the particle itself with Particle and particle_type 'aspect'.\n\
         4. Structural Particles: Structural particles such as '的', '地', and '得' should be marked as Particle with particle_type 'structural'.\n\
         5. Modal/Sentence-final Particles: Sentence-final particles like '吗', '吧', '呢', '啊' should be marked as Particle with particle_type 'modal'.\n\
         6. Pronouns: Distinguish personal pronouns ('我', '你', '他') and plural forms with '们' (e.g. '我们' -> pronoun with person: first, number: plural)."
    }
}
