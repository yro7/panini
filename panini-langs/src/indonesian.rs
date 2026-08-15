use serde::{Deserialize, Serialize};

use panini_core::morpheme::{Agglutinative, MorphemeDefinition, WordSegmentation};
use panini_core::traits::{
    BinaryNumber, BinaryVoice, IsoLang, LinguisticDefinition, MorphologyInfo, Person, Script,
    TypologicalFeature,
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
pub enum IndonesianPronounType {
    Personal,
    Possessive,
    Demonstrative,
    Interrogative,
    Relative,
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
pub enum IndonesianValency {
    Causative,
    Applicative,
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
pub enum IndonesianDerivation {
    Intransitive,
    AgentNoun,
    ActionNoun,
    AbstractNoun,
    Accidental,
    Reciprocal,
    Superlative,
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
pub enum IndonesianClitic {
    Emphatic,
    Interrogative,
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
pub enum IndonesianAttachment {
    Possessive,
    Object,
}

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
pub enum IndonesianMorphemeFunction {
    Voice { value: BinaryVoice },
    Valency { value: IndonesianValency },
    Derivation { value: IndonesianDerivation },
    Clitic { value: IndonesianClitic },
    Attachment {
        value: IndonesianAttachment,
        person: Person,
        number: BinaryNumber,
    },
}

impl IndonesianMorphemeFunction {
    fn directive_label(&self) -> String {
        let json = serde_json::to_value(self).expect("morpheme function is serializable");
        let category = json["category"]
            .as_str()
            .expect("internally tagged category is present");

        match self {
            Self::Attachment { .. } => format!(
                "{category}:{} {} {}",
                json["value"].as_str().expect("value is serialized"),
                json["person"].as_str().expect("person is serialized"),
                json["number"].as_str().expect("number is serialized")
            ),
            _ => format!(
                "{category}:{}",
                json["value"].as_str().expect("value is serialized")
            ),
        }
    }
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
pub enum IndonesianMorphology {
    Adjective { lemma: String },
    Adposition { lemma: String },
    Adverb { lemma: String },
    CoordinatingConjunction { lemma: String },
    Determiner { lemma: String },
    Interjection { lemma: String },
    Noun { lemma: String },
    Numeral { lemma: String },
    Particle { lemma: String },
    Pronoun {
        lemma: String,
        pronoun_type: IndonesianPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    ProperNoun { lemma: String },
    SubordinatingConjunction { lemma: String },
    Symbol { lemma: String },
    Verb {
        lemma: String,
        /// Set only for morphologically agentive (`meN-`) and patient (`di-`) verbs.
        #[serde(skip_serializing_if = "Option::is_none")]
        voice: Option<BinaryVoice>,
    },
    Other { lemma: String },
}

impl IndonesianMorphology {
    fn __pivot_voice(&self) -> Option<String> {
        match self {
            Self::Verb { voice, .. } => voice
                .as_ref()
                .map(|voice| panini_core::aggregable::ClosedValues::variant_str(voice).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for the optional verb voice field.
    pub const PIVOT_VOICE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "voice",
            "Voice",
            <BinaryVoice as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_voice,
        );
}

type P = IndonesianMorphologyPosTag;
type F = IndonesianMorphemeFunction;

static INDONESIAN_MORPHEMES: &[MorphemeDefinition<F, P>] = &[
    MorphemeDefinition {
        base_form: "meN-",
        functions: &[F::Voice {
            value: BinaryVoice::Active,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "di-",
        functions: &[F::Voice {
            value: BinaryVoice::Passive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "ber-",
        functions: &[F::Derivation {
            value: IndonesianDerivation::Intransitive,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "ter-",
        functions: &[
            F::Derivation {
                value: IndonesianDerivation::Accidental,
            },
            F::Derivation {
                value: IndonesianDerivation::Superlative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "peN-",
        functions: &[F::Derivation {
            value: IndonesianDerivation::AgentNoun,
        }],
        applies_to: &[P::Verb, P::Noun],
    },
    MorphemeDefinition {
        base_form: "ke-",
        functions: &[F::Derivation {
            value: IndonesianDerivation::AbstractNoun,
        }],
        applies_to: &[P::Noun, P::Adjective, P::Verb],
    },
    MorphemeDefinition {
        base_form: "per-",
        functions: &[F::Valency {
            value: IndonesianValency::Causative,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "-kan",
        functions: &[
            F::Valency {
                value: IndonesianValency::Causative,
            },
            F::Valency {
                value: IndonesianValency::Applicative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective, P::Noun],
    },
    MorphemeDefinition {
        base_form: "-i",
        functions: &[F::Valency {
            value: IndonesianValency::Applicative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "-an",
        functions: &[F::Derivation {
            value: IndonesianDerivation::ActionNoun,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "-lah",
        functions: &[F::Clitic {
            value: IndonesianClitic::Emphatic,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective, P::Adverb, P::Particle],
    },
    MorphemeDefinition {
        base_form: "-kah",
        functions: &[F::Clitic {
            value: IndonesianClitic::Interrogative,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective, P::Adverb, P::Particle],
    },
    MorphemeDefinition {
        base_form: "-tah",
        functions: &[F::Clitic {
            value: IndonesianClitic::Interrogative,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective, P::Adverb, P::Particle],
    },
    MorphemeDefinition {
        base_form: "-pun",
        functions: &[F::Clitic {
            value: IndonesianClitic::Emphatic,
        }],
        applies_to: &[P::Verb, P::Noun, P::Adjective, P::Adverb, P::Particle],
    },
    MorphemeDefinition {
        base_form: "-ku",
        functions: &[
            F::Attachment {
                value: IndonesianAttachment::Possessive,
                person: Person::First,
                number: BinaryNumber::Singular,
            },
            F::Attachment {
                value: IndonesianAttachment::Object,
                person: Person::First,
                number: BinaryNumber::Singular,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "-mu",
        functions: &[
            F::Attachment {
                value: IndonesianAttachment::Possessive,
                person: Person::Second,
                number: BinaryNumber::Singular,
            },
            F::Attachment {
                value: IndonesianAttachment::Object,
                person: Person::Second,
                number: BinaryNumber::Singular,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "-nya",
        functions: &[
            F::Attachment {
                value: IndonesianAttachment::Possessive,
                person: Person::Third,
                number: BinaryNumber::Singular,
            },
            F::Attachment {
                value: IndonesianAttachment::Object,
                person: Person::Third,
                number: BinaryNumber::Singular,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Verb],
    },
];

impl Agglutinative for Indonesian {
    fn morpheme_inventory() -> &'static [MorphemeDefinition<
        IndonesianMorphemeFunction,
        <IndonesianMorphology as MorphologyInfo>::PosTag,
    >] {
        INDONESIAN_MORPHEMES
    }

    fn morpheme_directives(&self) -> String {
        let inventory_lines = INDONESIAN_MORPHEMES
            .iter()
            .map(|morpheme| {
                let functions = morpheme
                    .functions
                    .iter()
                    .map(IndonesianMorphemeFunction::directive_label)
                    .collect::<Vec<_>>()
                    .join(" / ");
                format!("  {} → {functions}", morpheme.base_form)
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "MORPHEME SEGMENTATION — fill `morpheme_segmentation` only for words carrying a listed affix or clitic.\n\
             Each entry contains the word and its affix morphemes in surface order. Do NOT include the stem: its lemma is recorded separately.\n\
             For `meN-`, map the assimilated surface allomorph (`me-`, `mem-`, `men-`, `meng-`, or `meny-`) to the one base form `meN-`.\n\
             Do not treat reduplication as an affix morpheme; it is non-concatenative word formation and not represented by this inventory.\n\
             <morpheme_inventory>\n\
             Use ONLY these base forms:\n\
             {inventory_lines}\n\
             </morpheme_inventory>"
        )
    }
}

pub struct Indonesian;

impl LinguisticDefinition for Indonesian {
    type Morphology = IndonesianMorphology;
    type MorphemeFunction = IndonesianMorphemeFunction;

    const ISO_LANG: IsoLang = IsoLang::Ind;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        IndonesianMorphology::PIVOT_PRONOUN_TYPE,
        IndonesianMorphology::PIVOT_VOICE,
    ];
    const MORPHEME_PIVOTS: &'static [panini_core::pivot::PivotField<Self::MorphemeFunction>] = &[
        IndonesianMorphemeFunction::PIVOT_VOICE,
        IndonesianMorphemeFunction::PIVOT_VALENCY,
        IndonesianMorphemeFunction::PIVOT_DERIVATION,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[TypologicalFeature::Agglutination]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization: use the uninflected dictionary headword. For derived words, remove productive affixes where the base lemma is recoverable; use the conventional lexical lemma for opaque derivatives.\n\
         2. Indonesian has no obligatory grammatical gender, noun case, or verbal person/tense inflection. Do not invent these categories.\n\
         3. Verbs: set voice to active only for the agentive `meN-` series and passive only for the patient `di-` series. Omit voice for unmarked, `ber-`, `ter-`, and other verbs whose affixation is not this active/passive opposition.\n\
         4. Pronouns: specify pronoun_type. Provide person and number only when they are lexically expressed; omit them for demonstrative, interrogative, relative, and indefinite pronouns.\n\
         5. Treat productive prefixes, suffixes, and enclitics as part of one orthographic token. Keep hyphenated reduplication as one token, but do not represent reduplication as an affix morpheme.\n\
         6. Do not infer plural solely from a bare noun: full reduplication can mark plurality, but Indonesian number marking is optional and semantically variable."
    }

    fn extra_extraction_directives(&self) -> Option<String> {
        Some(self.morpheme_directives())
    }

    fn post_process_extraction(
        &self,
        segmentation: &mut Option<Vec<WordSegmentation<IndonesianMorphemeFunction>>>,
    ) -> Result<(), String> {
        self.validate_and_enrich(segmentation)
    }
}
