use serde::{Deserialize, Serialize};

use panini_core::morpheme::{Agglutinative, MorphemeDefinition, WordSegmentation};
use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, MorphologyInfo, Person, Script,
    TypologicalFeature, Upos,
};

// ─── Existing Turkish grammatical enums ──────────────────────────────────────

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
pub enum TurkishPossessive {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
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
pub enum TurkishCase {
    Nominative,   // Yalın hâl
    Accusative,   // Belirtme hâli
    Dative,       // Yönelme hâli
    Locative,     // Bulunma hâli
    Ablative,     // Ayrılma hâli
    Genitive,     // Tamlayan hâli
    Instrumental, // Vasıta hâli (-ile / -la / -le)
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
pub enum TurkishTense {
    PastDefinite,
    Evidential,
    PresentProgressive,
    Future, // Gelecek zaman
    Aorist, // Geniş zaman
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
pub enum TurkishMood {
    Indicative,    // Bildirme kipi
    Imperative,    // Emir kipi
    Necessitative, // Gereklilik kipi
    Optative,      // İstek kipi
    Conditional,   // Şart kipi
    Potential,     // Yeterlilik kipi
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
pub enum TurkishVoice {
    Active,     // Etken çatı
    Passive,    // Edilgen çatı
    Reflexive,  // Dönüşlü çatı
    Reciprocal, // İşteş çatı
    Causative,  // Ettirgen çatı
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
pub enum TurkishPolarity {
    Positive, // Olumlu
    Negative, // Olumsuz
}

// ─── New enums for morpheme-level functions ───────────────────────────────────

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
pub enum TurkishDerivation {
    Nominalization,
    ActionNominalization,
    FactNominalization,
    AgentSuffix,
    AbstractSuffix,
    Privative,
    Possessional,
    Verbalization,
    Adverbial,
    Relational,
    Diminutive,
    Participle,
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
pub enum TurkishCopula {
    Epistemic,
    Interrogative,
}

// ─── MorphemeFunction wrapper enum ────────────────────────────────────────

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
pub enum TurkishMorphemeFunction {
    Case {
        value: TurkishCase,
    },
    Tense {
        value: TurkishTense,
    },
    Mood {
        value: TurkishMood,
    },
    Voice {
        value: TurkishVoice,
    },
    Polarity {
        value: TurkishPolarity,
    },
    Number {
        value: BinaryNumber,
    },
    Agreement {
        person: Person,
        number: BinaryNumber,
    },
    Possessive {
        person: Person,
        number: BinaryNumber,
    },
    Derivation {
        value: TurkishDerivation,
    },
    Copula {
        value: TurkishCopula,
    },
}

impl TurkishMorphemeFunction {
    fn directive_label(&self) -> String {
        let json = serde_json::to_value(self).unwrap();
        let cat = json["category"].as_str().unwrap();
        match self {
            Self::Agreement { .. } | Self::Possessive { .. } => {
                let p = json["person"].as_str().unwrap();
                let n = json["number"].as_str().unwrap();
                format!("{cat}:{p} {n}")
            }
            _ => {
                let val = json["value"].as_str().unwrap();
                format!("{cat}:{val}")
            }
        }
    }
}

// ─── TurkishMorphology ────────────────────────────────────────────────────────

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
#[serde(rename_all = "lowercase")]
pub enum TurkishMorphology {
    Adjective {
        lemma: String,
        person: Option<Person>,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
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
        case: TurkishCase,
        number: BinaryNumber,
        possessive: Option<TurkishPossessive>,
        person: Option<Person>,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        case: TurkishCase,
        number: BinaryNumber,
        person: Person,
    },
    ProperNoun {
        lemma: String,
        case: TurkishCase,
        number: BinaryNumber,
        possessive: Option<TurkishPossessive>,
        person: Option<Person>,
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
        tense: Option<TurkishTense>,
        compound_tense: Option<TurkishTense>,
        mood: Option<TurkishMood>,
        voice: Option<TurkishVoice>,
        person: Option<Person>,
        number: Option<BinaryNumber>,
        polarity: TurkishPolarity,
    },
    Other {
        lemma: String,
    },
}

// ─── Static morpheme inventory ────────────────────────────────────────────────

type P = TurkishMorphologyPosTag;
type F = TurkishMorphemeFunction;

static TURKISH_MORPHEMES: &[MorphemeDefinition<F, P>] = &[
    // === Cases (nominal) ===
    MorphemeDefinition {
        base_form: "(y)I",
        functions: &[F::Case {
            value: TurkishCase::Accusative,
        }],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "DA",
        functions: &[F::Case {
            value: TurkishCase::Locative,
        }],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "DAn",
        functions: &[F::Case {
            value: TurkishCase::Ablative,
        }],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "(y)A",
        functions: &[F::Case {
            value: TurkishCase::Dative,
        }],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "(n)In",
        functions: &[F::Case {
            value: TurkishCase::Genitive,
        }],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "(y)lA",
        functions: &[F::Case {
            value: TurkishCase::Instrumental,
        }],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    // === Plural ===
    MorphemeDefinition {
        base_form: "lAr",
        functions: &[
            F::Number {
                value: BinaryNumber::Plural,
            },
            F::Agreement {
                person: Person::Third,
                number: BinaryNumber::Plural,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::Verb, P::ProperNoun, P::Adjective],
    },
    // === Possessive & Type II Agreement ===
    MorphemeDefinition {
        base_form: "(I)m",
        functions: &[
            F::Possessive {
                person: Person::First,
                number: BinaryNumber::Singular,
            },
            F::Agreement {
                person: Person::First,
                number: BinaryNumber::Singular,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "(I)n",
        functions: &[
            F::Possessive {
                person: Person::Second,
                number: BinaryNumber::Singular,
            },
            F::Agreement {
                person: Person::Second,
                number: BinaryNumber::Singular,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "(s)I",
        functions: &[F::Possessive {
            person: Person::Third,
            number: BinaryNumber::Singular,
        }],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "(I)mIz",
        functions: &[F::Possessive {
            person: Person::First,
            number: BinaryNumber::Plural,
        }],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "(I)nIz",
        functions: &[
            F::Possessive {
                person: Person::Second,
                number: BinaryNumber::Plural,
            },
            F::Agreement {
                person: Person::Second,
                number: BinaryNumber::Plural,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "lArI",
        functions: &[F::Possessive {
            person: Person::Third,
            number: BinaryNumber::Plural,
        }],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    // === Polarity (negation) ===
    MorphemeDefinition {
        base_form: "mA",
        functions: &[
            F::Polarity {
                value: TurkishPolarity::Negative,
            },
            F::Derivation {
                value: TurkishDerivation::Nominalization,
            },
        ],
        applies_to: &[P::Verb],
    },
    // === Voice ===
    MorphemeDefinition {
        base_form: "(I)l",
        functions: &[F::Voice {
            value: TurkishVoice::Passive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(I)n",
        functions: &[
            F::Voice {
                value: TurkishVoice::Reflexive,
            },
            F::Voice {
                value: TurkishVoice::Passive,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(I)ş",
        functions: &[
            F::Voice {
                value: TurkishVoice::Reciprocal,
            },
            F::Derivation {
                value: TurkishDerivation::ActionNominalization,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "DIr",
        functions: &[
            F::Voice {
                value: TurkishVoice::Causative,
            },
            F::Copula {
                value: TurkishCopula::Epistemic,
            },
        ],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "t",
        functions: &[F::Voice {
            value: TurkishVoice::Causative,
        }],
        applies_to: &[P::Verb],
    },
    // === Tense / Aspect ===
    MorphemeDefinition {
        base_form: "(y)DI",
        functions: &[F::Tense {
            value: TurkishTense::PastDefinite,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(y)mIş",
        functions: &[F::Tense {
            value: TurkishTense::Evidential,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(I)yor",
        functions: &[F::Tense {
            value: TurkishTense::PresentProgressive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(y)AcAk",
        functions: &[F::Tense {
            value: TurkishTense::Future,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(A/I)r",
        functions: &[F::Tense {
            value: TurkishTense::Aorist,
        }],
        applies_to: &[P::Verb],
    },
    // === Mood ===
    MorphemeDefinition {
        base_form: "(y)sA",
        functions: &[F::Mood {
            value: TurkishMood::Conditional,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "mAlI",
        functions: &[F::Mood {
            value: TurkishMood::Necessitative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(y)A",
        functions: &[F::Mood {
            value: TurkishMood::Optative,
        }],
        applies_to: &[P::Verb],
    },
    // === Agreement ===
    MorphemeDefinition {
        base_form: "(y)Im",
        functions: &[F::Agreement {
            person: Person::First,
            number: BinaryNumber::Singular,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "sIn",
        functions: &[F::Agreement {
            person: Person::Second,
            number: BinaryNumber::Singular,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(y)Iz",
        functions: &[F::Agreement {
            person: Person::First,
            number: BinaryNumber::Plural,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(I)k",
        functions: &[F::Agreement {
            person: Person::First,
            number: BinaryNumber::Plural,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "sInIz",
        functions: &[F::Agreement {
            person: Person::Second,
            number: BinaryNumber::Plural,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    // === Question Particle / Interrogative Copula ===
    MorphemeDefinition {
        base_form: "mI",
        functions: &[F::Copula {
            value: TurkishCopula::Interrogative,
        }],
        applies_to: &[P::Particle, P::Verb, P::Noun, P::Adjective],
    },
    // === Derivation ===
    MorphemeDefinition {
        base_form: "CI",
        functions: &[F::Derivation {
            value: TurkishDerivation::AgentSuffix,
        }],
        applies_to: &[P::Noun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "lIk",
        functions: &[F::Derivation {
            value: TurkishDerivation::AbstractSuffix,
        }],
        applies_to: &[P::Noun, P::Adjective, P::Verb],
    },
    MorphemeDefinition {
        base_form: "sIz",
        functions: &[F::Derivation {
            value: TurkishDerivation::Privative,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "lI",
        functions: &[F::Derivation {
            value: TurkishDerivation::Possessional,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "DIk",
        functions: &[F::Derivation {
            value: TurkishDerivation::FactNominalization,
        }],
        applies_to: &[P::Verb],
    },
    // === Ability ===
    MorphemeDefinition {
        base_form: "(y)Abil",
        functions: &[F::Mood {
            value: TurkishMood::Potential,
        }],
        applies_to: &[P::Verb],
    },
    // === New Additions ===
    MorphemeDefinition {
        base_form: "(y)ken",
        functions: &[F::Derivation {
            value: TurkishDerivation::Adverbial,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "lAş",
        functions: &[F::Derivation {
            value: TurkishDerivation::Verbalization,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "mAk",
        functions: &[F::Derivation {
            value: TurkishDerivation::Nominalization,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "ki",
        functions: &[F::Derivation {
            value: TurkishDerivation::Relational,
        }],
        applies_to: &[P::Noun, P::Adjective, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "(y)ArAk",
        functions: &[F::Derivation {
            value: TurkishDerivation::Adverbial,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "CIk",
        functions: &[F::Derivation {
            value: TurkishDerivation::Diminutive,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(y)An",
        functions: &[F::Derivation {
            value: TurkishDerivation::Participle,
        }],
        applies_to: &[P::Verb],
    },
];

// ─── Agglutinative implementation ────────────────────────────────────────────

impl Agglutinative for Turkish {
    fn morpheme_inventory() -> &'static [MorphemeDefinition<
        TurkishMorphemeFunction,
        <TurkishMorphology as MorphologyInfo>::PosTag,
    >] {
        TURKISH_MORPHEMES
    }

    fn morpheme_directives(&self) -> String {
        let inventory_lines: String = TURKISH_MORPHEMES
            .iter()
            .map(|m| {
                let funcs: Vec<String> = m
                    .functions
                    .iter()
                    .map(TurkishMorphemeFunction::directive_label)
                    .collect();
                format!("  {} → {}", m.base_form, funcs.join(" / "))
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "MORPHEME SEGMENTATION — fill `morpheme_segmentation` as an array of objects, \
             one per word that carries derivational or inflectional suffixes.\n\
             Each object has:\n\
             - `word`: the surface form of the word\n\
             - `morphemes`: one entry per suffix (NOT the stem — its dictionary form is the word's `lemma`):\n\
               - `surface`: the actual allomorph as it appears (e.g. \"de\", \"yor\", \"lar\")\n\
               - `base_form`: the archiphonemic identifier from the inventory below\n\
               - `function`: {{\"category\": \"<type>\", ...value fields...}}\n\
             \n\
             <morpheme_inventory>\n\
             Use ONLY base_forms from this list:\n\
             {inventory_lines}\n\
             </morpheme_inventory>\n\
             \n\
             VOWEL HARMONY: Turkish suffixes harmonize with the preceding vowel. \
             Map surface allomorphs to the correct base_form.\n\
             ORDERING: list morphemes in the order they appear in the word (left to right).\n\
             STEM: do NOT include the stem — its dictionary form is already captured in `lemma`.\n\
             Only segment words that have at least one suffix worth annotating."
        )
    }
}

// ─── LinguisticDefinition implementation ─────────────────────────────────────

pub struct Turkish;

impl LinguisticDefinition for Turkish {
    type Morphology = TurkishMorphology;
    type MorphemeFunction = TurkishMorphemeFunction;

    const ISO_LANG: IsoLang = IsoLang::Tur;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        TurkishMorphology::PIVOT_CASE,
        TurkishMorphology::PIVOT_POLARITY,
    ];
    const MORPHEME_PIVOTS: &'static [panini_core::pivot::PivotField<Self::MorphemeFunction>] = &[
        TurkishMorphemeFunction::PIVOT_POLARITY,
        TurkishMorphemeFunction::PIVOT_TENSE,
        TurkishMorphemeFunction::PIVOT_CASE,
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
            TypologicalFeature::Agglutination,
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization: All extracted words must be in their dictionary form (e.g., nouns in nominative singular, verbs in infinitive form).\n\
         2. For nouns and proper nouns: provide the grammatical case, number, possessive if present, and person if copula applies.\n\
         3. For verbs: provide the tense, compound_tense, mood, voice, person, number, and polarity.\n\
         4. For pronouns: provide the grammatical case, number, and person.\n\
         5. Question Particle 'mi': Extract the question particle 'mi' (and its vowel-harmonized variants) as a separate particle."
    }

    fn extra_extraction_directives(&self) -> Option<String> {
        Some(self.morpheme_directives())
    }

    fn post_process_extraction(
        &self,
        segmentation: &mut Option<Vec<WordSegmentation<TurkishMorphemeFunction>>>,
    ) -> Result<(), String> {
        self.validate_and_enrich(segmentation)
    }
}
