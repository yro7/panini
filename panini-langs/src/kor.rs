use serde::{Deserialize, Serialize};

use panini_core::morpheme::{Agglutinative, MorphemeDefinition, WordSegmentation};
use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, MorphologyInfo, Person, Script,
    TypologicalFeature, Upos,
};

// ─── Korean grammatical enums ────────────────────────────────────────────────

/// Contextual nominal relation expressed by an attached Korean particle.
///
/// This pedagogical axis is intentionally finer-grained than the traditional
/// 격조사 classes recorded separately by [`KoreanParticleClass`]. For example,
/// one adverbial-case particle `(으)로` can express `Instrumental` or
/// `Directional`, while auxiliary particles `부터` and `까지` express the
/// boundary relations `Ablative` and `Terminative`.
///
/// `Unmarked` is the value for a bare nominal: Korean drops case particles
/// freely in speech.
#[panini_macro::closed_enum]
pub enum KoreanCase {
    Nominative,   // 주격 (이/가, 께서)
    Accusative,   // 목적격 (을/를)
    Genitive,     // 관형격 (의)
    Dative,       // 여격 (에게/한테/께)
    Locative,     // 처소격 (에, 에서)
    Ablative,     // 출발점 (에서, 부터)
    Terminative,  // 도착점 (까지)
    Instrumental, // 도구·수단 ((으)로)
    Directional,  // 방향·도착지 ((으)로)
    Essive,       // 자격·지위 ((으)로서)
    Comitative,   // 공동격 (와/과, 하고, (이)랑)
    Comparative,  // 비교격 (보다)
    Equative,     // 비유격 (처럼, 같이)
    Vocative,     // 호격 (아/야)
    Unmarked,     // 조사 없음 (bare nominal)
}

/// Information-structure marking by a 보조사 (auxiliary particle). Stacks on
/// top of, and often replaces, the case particle.
#[panini_macro::closed_enum]
pub enum KoreanMarking {
    Unmarked,     // no 보조사
    Topic,        // 은/는
    Additive,     // 도
    Inclusive,    // 조차, 마저
    Exclusive,    // 만, 밖에, 뿐
    Distributive, // 마다, 씩
}

/// Traditional Korean particle classes (조사 분류). This is deliberately
/// separate from [`KoreanCase`], which records the particle's contextual
/// nominal relation. One surface particle can have more than one class:
/// `이/가` is subject or complement case, while `와/과` is adverbial or
/// conjunctive according to use.
#[panini_macro::closed_enum]
pub enum KoreanParticleClass {
    SubjectCase,     // 주격 조사
    PredicativeCase, // 서술격 조사 (`이다`)
    ObjectCase,      // 목적격 조사
    ComplementCase,  // 보격 조사 (`이/가` with `되다`, `아니다`)
    AdnominalCase,   // 관형격 조사
    AdverbialCase,   // 부사격 조사
    VocativeCase,    // 호격 조사
    Auxiliary,       // 보조사
    Conjunctive,     // 접속 조사
}

#[panini_macro::closed_enum]
pub enum KoreanTense {
    NonPast,    // 비과거 (unmarked; present or contextually future)
    Past,       // 과거 (-았/었-)
    RemotePast, // 대과거 (-았었/었었-)
}

/// The six 상대높임법 speech levels. `Familiar` (하게체) and `SemiFormal`
/// (하오체) survive only in fiction, older speech and set phrases, but a
/// learner meets them in written dialogue, so they stay in the value space.
#[panini_macro::closed_enum]
pub enum KoreanSpeechLevel {
    Deferential, // 하십시오체 (-습니다)
    Polite,      // 해요체 (-아요/어요)
    SemiFormal,  // 하오체 (-(으)오)
    Familiar,    // 하게체 (-네)
    Plain,       // 해라체 (-ㄴ다, -니, -자)
    Intimate,    // 해체 / 반말 (-아/어)
}

/// Communicative function selected by a final ending (종결어미).
///
/// This is deliberately not called "sentence type": endings such as -네요 and
/// -군/구나 remain declaratives whose mirative force belongs to modality.
#[panini_macro::closed_enum]
pub enum KoreanFinalEndingFunction {
    Declarative,   // 평서문
    Interrogative, // 의문문
    Imperative,    // 명령문
    Propositive,   // 청유문 (-자, -(으)ㅂ시다)
}

/// 주체 높임 — respect directed at the grammatical subject.
#[panini_macro::closed_enum]
pub enum KoreanSubjectHonorification {
    NonHonorific,
    Honorific, // -(으)시-, 께서
}

/// 객체 높임 — respect directed at an object or recipient.
#[panini_macro::closed_enum]
pub enum KoreanObjectHonorification {
    NonHonorific,
    Honorific, // 께, 뵙다, 드리다, 모시다
}

/// Lexical humility of the speaker-denoting pronoun.
#[panini_macro::closed_enum]
pub enum KoreanSpeakerHumility {
    Neutral,
    Humble, // 저, 저희
}

#[panini_macro::closed_enum]
pub enum KoreanVoice {
    Active,    // 능동
    Passive,   // 피동 (-이/히/리/기-, -되다, -어지다)
    Causative, // 사동 (-이/히/리/기/우/구/추-, -게 하다)
}

#[panini_macro::closed_enum]
pub enum KoreanPolarity {
    Positive, // 긍정
    Negative, // 부정 (안, 못, -지 않다, -지 못하다)
}

/// Pedagogical value of a complete auxiliary construction. The value does not
/// belong to the auxiliary lexeme in isolation: lexical `있다` remains unset.
#[panini_macro::closed_enum]
pub enum KoreanAuxiliaryConstruction {
    Progressive,  // -고 있다
    Resultative,  // -아/어 있다
    Experiential, // -아/어 보다
    Completive,   // -아/어 버리다
}

/// Which slot the predicate's ending fills: whether the predicate ends the
/// sentence, links to another clause, modifies a noun, or becomes one.
#[panini_macro::closed_enum]
pub enum KoreanVerbForm {
    Final,      // 종결형 — ends the sentence
    Connective, // 연결형 — links clauses (-고, -아서, -면)
    Adnominal,  // 관형사형 — modifies a noun (-는, -(으)ㄴ, -(으)ㄹ)
    Nominal,    // 명사형 — nominalised (-기, -(으)ㅁ)
}

/// Korean runs two full numeral series; which one a phrase takes is fixed by
/// the counter.
#[panini_macro::closed_enum]
pub enum KoreanNumeralSystem {
    Native,     // 고유어 수사 (하나, 둘, 셋)
    SinoKorean, // 한자어 수사 (일, 이, 삼)
}

// ─── Enums used only at morpheme level ───────────────────────────────────────

/// The semantic relation a 연결어미 (connective ending) establishes.
#[panini_macro::closed_enum]
pub enum KoreanConnective {
    Sequential,   // -고
    Causal,       // -아서/어서, -(으)니까
    Contrastive,  // -지만, -는데
    Conditional,  // -(으)면
    Concessive,   // -아도/어도
    Purposive,    // -(으)려고, -(으)러
    Simultaneous, // -(으)면서
    Alternative,  // -거나
    Quotative,    // -다고, -냐고, -(으)라고, -자고
    Obligative,   // -아야/어야
    Immediate,    // -자마자
    Proportional, // -(으)ㄹ수록
    Auxiliary,    // 보조적 연결어미 -지, -게, -고, -아/어
}

#[panini_macro::closed_enum]
pub enum KoreanNominalizer {
    Gerundive, // -기 (activity, complement of 좋아하다/시작하다)
    Nominal,   // -(으)ㅁ (fact, written register)
}

/// 관형사형 어미 — the relative-clause ending, whose value is tense on a verb
/// and largely aspectual on a descriptive verb.
#[panini_macro::closed_enum]
pub enum KoreanAdnominal {
    Present,       // -는
    Past,          // -(으)ㄴ
    Prospective,   // -(으)ㄹ
    Retrospective, // -던
}

#[panini_macro::closed_enum]
pub enum KoreanDerivation {
    Agentive,       // -자, -가, -사, -꾼
    Adverbializing, // -이, -히
    Adjectivizing,  // -스럽-, -답-, -적
    Verbalizing,    // -하-, -되-
    HonorificTitle, // -님
}

/// 선어말어미 modality that is not reducible to tense.
#[panini_macro::closed_enum]
pub enum KoreanModality {
    Conjecture,    // -겠-, -(으)ㄹ까
    Volition,      // -겠- (first person)
    Retrospective, // -더-
    Mirative,      // -네(요): newly perceived information
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
pub enum KoreanMorphemeFunction {
    ParticleClass {
        value: KoreanParticleClass,
    },
    Case {
        value: KoreanCase,
    },
    Marking {
        value: KoreanMarking,
    },
    Number {
        value: BinaryNumber,
    },
    Tense {
        value: KoreanTense,
    },
    SpeechLevel {
        value: KoreanSpeechLevel,
    },
    FinalEndingFunction {
        value: KoreanFinalEndingFunction,
    },
    SubjectHonorification {
        value: KoreanSubjectHonorification,
    },
    ObjectHonorification {
        value: KoreanObjectHonorification,
    },
    Voice {
        value: KoreanVoice,
    },
    Connective {
        value: KoreanConnective,
    },
    Nominalization {
        value: KoreanNominalizer,
    },
    Adnominal {
        value: KoreanAdnominal,
    },
    Derivation {
        value: KoreanDerivation,
    },
    Modality {
        value: KoreanModality,
    },
}

impl KoreanMorphemeFunction {
    /// `category:value` — the label the morpheme inventory is rendered with in
    /// the extraction prompt. Every variant is single-field, so unlike Turkish
    /// there is no composite case to spell out.
    fn directive_label(&self) -> String {
        let json = serde_json::to_value(self).unwrap();
        let cat = json["category"].as_str().unwrap();
        let val = json["value"].as_str().unwrap();
        format!("{cat}:{val}")
    }
}

// ─── KoreanMorphology ────────────────────────────────────────────────────────

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
pub enum KoreanMorphology {
    /// 형용사 — a *descriptive verb*. It conjugates exactly like a verb and
    /// never takes a copula, so it carries the full predicate paradigm minus
    /// voice.
    Adjective {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<KoreanTense>,
        #[serde(skip_serializing_if = "Option::is_none")]
        speech_level: Option<KoreanSpeechLevel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        final_ending_function: Option<KoreanFinalEndingFunction>,
        form: KoreanVerbForm,
        subject_honorification: KoreanSubjectHonorification,
        polarity: KoreanPolarity,
        #[serde(skip_serializing_if = "Option::is_none")]
        modality: Option<KoreanModality>,
    },
    /// Postpositions written detached from their host (대해서, 관하여) and the
    /// case particles when the model does emit one as its own token.
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
    },
    /// 단위 명사 — a counter. Korean counts with 수사 + 단위명사, and which
    /// numeral series the counter selects is the fact worth recording.
    Classifier {
        lemma: String,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    /// 관형사 — an uninflecting prenominal (이, 그, 저, 새, 모든).
    Determiner {
        lemma: String,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        case: KoreanCase,
        marking: KoreanMarking,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Numeral {
        lemma: String,
        numeral_system: KoreanNumeralSystem,
    },
    /// 보조사 and sentence-final particles emitted as standalone tokens.
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        case: KoreanCase,
        marking: KoreanMarking,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        speaker_humility: KoreanSpeakerHumility,
    },
    ProperNoun {
        lemma: String,
        case: KoreanCase,
        marking: KoreanMarking,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
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
        tense: Option<KoreanTense>,
        #[serde(skip_serializing_if = "Option::is_none")]
        speech_level: Option<KoreanSpeechLevel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        final_ending_function: Option<KoreanFinalEndingFunction>,
        form: KoreanVerbForm,
        subject_honorification: KoreanSubjectHonorification,
        object_honorification: KoreanObjectHonorification,
        voice: KoreanVoice,
        polarity: KoreanPolarity,
        #[serde(skip_serializing_if = "Option::is_none")]
        auxiliary_construction: Option<KoreanAuxiliaryConstruction>,
        #[serde(skip_serializing_if = "Option::is_none")]
        modality: Option<KoreanModality>,
    },
    Other {
        lemma: String,
    },
}

impl KoreanMorphology {
    /// `tense` is optional because non-finite endings and pre-final tense are
    /// independent choices, so the derive skips it for pivot generation.
    /// Written by hand to keep the learner-facing facet available.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } | Self::Adjective { tense, .. } => tense
                .as_ref()
                .map(|t| panini_core::aggregable::ClosedValues::variant_str(t).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for tense. Defined manually because `tense` is
    /// optional (see [`KoreanMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <KoreanTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// Speech level is only defined on a final ending, hence `Option` — but it
    /// is the dimension a Korean learner most wants to slice their lexicon by,
    /// so it gets the same hand-written treatment as tense.
    fn __pivot_speech_level(&self) -> Option<String> {
        match self {
            Self::Verb { speech_level, .. } | Self::Adjective { speech_level, .. } => speech_level
                .as_ref()
                .map(|s| panini_core::aggregable::ClosedValues::variant_str(s).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for the speech level (see
    /// [`KoreanMorphology::__pivot_speech_level`]).
    pub const PIVOT_SPEECH_LEVEL: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "speech_level",
            "Speech level",
            <KoreanSpeechLevel as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_speech_level,
        );

    fn __pivot_auxiliary_construction(&self) -> Option<String> {
        match self {
            Self::Verb {
                auxiliary_construction,
                ..
            } => auxiliary_construction
                .as_ref()
                .map(|a| panini_core::aggregable::ClosedValues::variant_str(a).to_string()),
            _ => None,
        }
    }

    pub const PIVOT_AUXILIARY_CONSTRUCTION: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "auxiliary_construction",
            "Auxiliary construction",
            <KoreanAuxiliaryConstruction as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_auxiliary_construction,
        );

    fn __pivot_modality(&self) -> Option<String> {
        match self {
            Self::Verb { modality, .. } | Self::Adjective { modality, .. } => modality
                .as_ref()
                .map(|m| panini_core::aggregable::ClosedValues::variant_str(m).to_string()),
            _ => None,
        }
    }

    pub const PIVOT_MODALITY: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "modality",
            "Modality",
            <KoreanModality as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_modality,
        );
}

// ─── Static morpheme inventory ───────────────────────────────────────────────

type P = KoreanMorphologyPosTag;
type F = KoreanMorphemeFunction;

const PC_SUBJECT: F = F::ParticleClass {
    value: KoreanParticleClass::SubjectCase,
};
const PC_PREDICATIVE: F = F::ParticleClass {
    value: KoreanParticleClass::PredicativeCase,
};
const PC_OBJECT: F = F::ParticleClass {
    value: KoreanParticleClass::ObjectCase,
};
const PC_COMPLEMENT: F = F::ParticleClass {
    value: KoreanParticleClass::ComplementCase,
};
const PC_ADNOMINAL: F = F::ParticleClass {
    value: KoreanParticleClass::AdnominalCase,
};
const PC_ADVERBIAL: F = F::ParticleClass {
    value: KoreanParticleClass::AdverbialCase,
};
const PC_VOCATIVE: F = F::ParticleClass {
    value: KoreanParticleClass::VocativeCase,
};
const PC_AUXILIARY: F = F::ParticleClass {
    value: KoreanParticleClass::Auxiliary,
};
const PC_CONJUNCTIVE: F = F::ParticleClass {
    value: KoreanParticleClass::Conjunctive,
};

/// Base forms use the standard Korean pedagogical notation: the two allomorphs
/// separated by a slash (`이/가`), or the optional linking vowel in parentheses
/// (`(으)로`). Every entry is unique as a string, which is what
/// `validate_inventory` checks.
static KOREAN_MORPHEMES: &[MorphemeDefinition<F, P>] = &[
    // === Nominal relational particles (classed independently below) ===
    MorphemeDefinition {
        base_form: "이다",
        functions: &[PC_PREDICATIVE],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "이/가",
        functions: &[
            PC_SUBJECT,
            PC_COMPLEMENT,
            F::Case {
                value: KoreanCase::Nominative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "께서",
        functions: &[
            PC_SUBJECT,
            F::Case {
                value: KoreanCase::Nominative,
            },
            F::SubjectHonorification {
                value: KoreanSubjectHonorification::Honorific,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "을/를",
        functions: &[
            PC_OBJECT,
            F::Case {
                value: KoreanCase::Accusative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "의",
        functions: &[
            PC_ADNOMINAL,
            F::Case {
                value: KoreanCase::Genitive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "에",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Locative,
            },
            F::Case {
                value: KoreanCase::Dative,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "에서",
        functions: &[
            PC_SUBJECT,
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Locative,
            },
            F::Case {
                value: KoreanCase::Ablative,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "에게",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Dative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "에게서",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Ablative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "한테",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Dative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "한테서",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Ablative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "께",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Dative,
            },
            F::ObjectHonorification {
                value: KoreanObjectHonorification::Honorific,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "(으)로",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Instrumental,
            },
            F::Case {
                value: KoreanCase::Directional,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "(으)로서",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Essive,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Pronoun],
    },
    MorphemeDefinition {
        base_form: "와/과",
        functions: &[
            PC_ADVERBIAL,
            PC_CONJUNCTIVE,
            F::Case {
                value: KoreanCase::Comitative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "하고",
        functions: &[
            PC_ADVERBIAL,
            PC_CONJUNCTIVE,
            F::Case {
                value: KoreanCase::Comitative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "(이)랑",
        functions: &[
            PC_ADVERBIAL,
            PC_CONJUNCTIVE,
            F::Case {
                value: KoreanCase::Comitative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "부터",
        functions: &[
            PC_AUXILIARY,
            F::Case {
                value: KoreanCase::Ablative,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "까지",
        functions: &[
            PC_AUXILIARY,
            F::Case {
                value: KoreanCase::Terminative,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "보다",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Comparative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "처럼",
        functions: &[
            PC_ADVERBIAL,
            F::Case {
                value: KoreanCase::Equative,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "아/야",
        functions: &[
            PC_VOCATIVE,
            F::Case {
                value: KoreanCase::Vocative,
            },
        ],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    // === 보조사 — information-structure particles ===
    MorphemeDefinition {
        base_form: "은/는",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Topic,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Adverb, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "도",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Additive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Adverb, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "만",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Exclusive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Adverb, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "밖에",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Exclusive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "뿐",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Exclusive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "조차",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Inclusive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "마저",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Inclusive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Numeral],
    },
    MorphemeDefinition {
        base_form: "마다",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Distributive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Classifier],
    },
    MorphemeDefinition {
        base_form: "씩",
        functions: &[
            PC_AUXILIARY,
            F::Marking {
                value: KoreanMarking::Distributive,
            },
        ],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun, P::Numeral, P::Classifier],
    },
    // === Number ===
    MorphemeDefinition {
        base_form: "들",
        functions: &[F::Number {
            value: BinaryNumber::Plural,
        }],
        applies_to: &[P::Noun, P::Pronoun, P::ProperNoun],
    },
    // === 선어말어미 — pre-final endings ===
    MorphemeDefinition {
        base_form: "(으)시",
        functions: &[F::SubjectHonorification {
            value: KoreanSubjectHonorification::Honorific,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "았/었",
        functions: &[F::Tense {
            value: KoreanTense::Past,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "았었/었었",
        functions: &[F::Tense {
            value: KoreanTense::RemotePast,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "겠",
        functions: &[
            F::Modality {
                value: KoreanModality::Conjecture,
            },
            F::Modality {
                value: KoreanModality::Volition,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "더",
        functions: &[F::Modality {
            value: KoreanModality::Retrospective,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    // === 종결어미 — final endings ===
    MorphemeDefinition {
        base_form: "습니다/ㅂ니다",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Deferential,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "습니까/ㅂ니까",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Deferential,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Interrogative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)십시오",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Deferential,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Imperative,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(으)ㅂ시다",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Deferential,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Propositive,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "아요/어요",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Polite,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Interrogative,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Imperative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "아/어",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Intimate,
            },
            F::Connective {
                value: KoreanConnective::Auxiliary,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "ㄴ다/는다",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Plain,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
        ],
        applies_to: &[P::Verb],
    },
    // Bare -다: the citation form, and the 해라체 declarative of a descriptive
    // verb or of any past-tense predicate (좋다, 갔다).
    MorphemeDefinition {
        base_form: "다",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Plain,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "니",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Plain,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Interrogative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "아라/어라",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Plain,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Imperative,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "자",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Plain,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Propositive,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "네",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Familiar,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
            F::Modality {
                value: KoreanModality::Mirative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "네요",
        functions: &[
            F::SpeechLevel {
                value: KoreanSpeechLevel::Polite,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
            F::Modality {
                value: KoreanModality::Mirative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)오",
        functions: &[F::SpeechLevel {
            value: KoreanSpeechLevel::SemiFormal,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "군/구나",
        functions: &[
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
            F::Modality {
                value: KoreanModality::Mirative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)ㄹ까",
        functions: &[
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Interrogative,
            },
            F::Modality {
                value: KoreanModality::Conjecture,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "지",
        functions: &[
            F::Connective {
                value: KoreanConnective::Auxiliary,
            },
            F::SpeechLevel {
                value: KoreanSpeechLevel::Intimate,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "더라",
        functions: &[
            F::Modality {
                value: KoreanModality::Retrospective,
            },
            F::SpeechLevel {
                value: KoreanSpeechLevel::Plain,
            },
            F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    // === 연결어미 — connective endings ===
    MorphemeDefinition {
        base_form: "고",
        functions: &[
            F::Connective {
                value: KoreanConnective::Sequential,
            },
            F::Connective {
                value: KoreanConnective::Auxiliary,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "아서/어서",
        functions: &[
            F::Connective {
                value: KoreanConnective::Causal,
            },
            F::Connective {
                value: KoreanConnective::Sequential,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)니까",
        functions: &[F::Connective {
            value: KoreanConnective::Causal,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "느라고",
        functions: &[F::Connective {
            value: KoreanConnective::Causal,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(으)므로",
        functions: &[F::Connective {
            value: KoreanConnective::Causal,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "지만",
        functions: &[F::Connective {
            value: KoreanConnective::Contrastive,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "는데/(으)ㄴ데",
        functions: &[F::Connective {
            value: KoreanConnective::Contrastive,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)면",
        functions: &[F::Connective {
            value: KoreanConnective::Conditional,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "아도/어도",
        functions: &[F::Connective {
            value: KoreanConnective::Concessive,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)려고",
        functions: &[F::Connective {
            value: KoreanConnective::Purposive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(으)러",
        functions: &[F::Connective {
            value: KoreanConnective::Purposive,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "도록",
        functions: &[F::Connective {
            value: KoreanConnective::Purposive,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)면서",
        functions: &[F::Connective {
            value: KoreanConnective::Simultaneous,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "거나",
        functions: &[F::Connective {
            value: KoreanConnective::Alternative,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "자마자",
        functions: &[F::Connective {
            value: KoreanConnective::Immediate,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(으)ㄹ수록",
        functions: &[F::Connective {
            value: KoreanConnective::Proportional,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)ㄴ가/나",
        functions: &[F::Modality {
            value: KoreanModality::Conjecture,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "다고",
        functions: &[F::Connective {
            value: KoreanConnective::Quotative,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "냐고",
        functions: &[F::Connective {
            value: KoreanConnective::Quotative,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)라고",
        functions: &[F::Connective {
            value: KoreanConnective::Quotative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "자고",
        functions: &[F::Connective {
            value: KoreanConnective::Quotative,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "아야/어야",
        functions: &[F::Connective {
            value: KoreanConnective::Obligative,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "게",
        functions: &[F::Connective {
            value: KoreanConnective::Auxiliary,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    // === 관형사형 어미 — adnominal endings ===
    MorphemeDefinition {
        base_form: "는",
        functions: &[F::Adnominal {
            value: KoreanAdnominal::Present,
        }],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "(으)ㄴ",
        functions: &[
            F::Adnominal {
                value: KoreanAdnominal::Past,
            },
            F::Adnominal {
                value: KoreanAdnominal::Present,
            },
        ],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)ㄹ",
        functions: &[F::Adnominal {
            value: KoreanAdnominal::Prospective,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "던",
        functions: &[F::Adnominal {
            value: KoreanAdnominal::Retrospective,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    // === 명사형 어미 — nominalisers ===
    MorphemeDefinition {
        base_form: "기",
        functions: &[F::Nominalization {
            value: KoreanNominalizer::Gerundive,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "(으)ㅁ",
        functions: &[F::Nominalization {
            value: KoreanNominalizer::Nominal,
        }],
        applies_to: &[P::Verb, P::Adjective],
    },
    // === 피동·사동 — voice ===
    MorphemeDefinition {
        base_form: "이/히/리/기",
        functions: &[
            F::Voice {
                value: KoreanVoice::Passive,
            },
            F::Voice {
                value: KoreanVoice::Causative,
            },
        ],
        applies_to: &[P::Verb],
    },
    MorphemeDefinition {
        base_form: "우/구/추",
        functions: &[F::Voice {
            value: KoreanVoice::Causative,
        }],
        applies_to: &[P::Verb],
    },
    // === 파생 — derivation ===
    MorphemeDefinition {
        base_form: "님",
        functions: &[F::Derivation {
            value: KoreanDerivation::HonorificTitle,
        }],
        applies_to: &[P::Noun, P::ProperNoun],
    },
    MorphemeDefinition {
        base_form: "하",
        functions: &[F::Derivation {
            value: KoreanDerivation::Verbalizing,
        }],
        applies_to: &[P::Noun, P::Verb, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "되",
        functions: &[
            F::Derivation {
                value: KoreanDerivation::Verbalizing,
            },
            F::Voice {
                value: KoreanVoice::Passive,
            },
        ],
        applies_to: &[P::Noun, P::Verb],
    },
    MorphemeDefinition {
        base_form: "스럽",
        functions: &[F::Derivation {
            value: KoreanDerivation::Adjectivizing,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "답",
        functions: &[F::Derivation {
            value: KoreanDerivation::Adjectivizing,
        }],
        applies_to: &[P::Noun, P::Adjective],
    },
    MorphemeDefinition {
        base_form: "적",
        functions: &[F::Derivation {
            value: KoreanDerivation::Adjectivizing,
        }],
        applies_to: &[P::Noun],
    },
    MorphemeDefinition {
        base_form: "이/히",
        functions: &[F::Derivation {
            value: KoreanDerivation::Adverbializing,
        }],
        applies_to: &[P::Adjective, P::Adverb, P::Noun],
    },
];

// ─── Agglutinative implementation ────────────────────────────────────────────

impl Agglutinative for Korean {
    fn morpheme_inventory() -> &'static [MorphemeDefinition<
        KoreanMorphemeFunction,
        <KoreanMorphology as MorphologyInfo>::PosTag,
    >] {
        KOREAN_MORPHEMES
    }

    fn morpheme_directives(&self) -> String {
        let inventory_lines: String = KOREAN_MORPHEMES
            .iter()
            .map(|m| {
                let funcs: Vec<String> = m
                    .functions
                    .iter()
                    .map(KoreanMorphemeFunction::directive_label)
                    .collect();
                format!("  {} → {}", m.base_form, funcs.join(" / "))
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "MORPHEME SEGMENTATION — fill `morpheme_segmentation` as an array of objects, \
             one per 어절 (spacing unit) that carries particles or endings.\n\
             Each object has:\n\
             - `word`: the surface form of the 어절\n\
             - `morphemes`: one entry per particle or ending (NOT the stem — its dictionary form is the word's `lemma`):\n\
               - `surface`: the allomorph as it actually appears (e.g. \"는\", \"에서\", \"었\", \"습니다\")\n\
               - `base_form`: the identifier from the inventory below, copied verbatim\n\
               - `function`: {{\"category\": \"<type>\", \"value\": \"<value>\"}}\n\
             \n\
             <morpheme_inventory>\n\
             Use ONLY base_forms from this list:\n\
             {inventory_lines}\n\
             </morpheme_inventory>\n\
             \n\
             FUNCTIONS: emit every applicable function category as a separate entry for the \
             same surface morpheme. In particular, a nominal particle emits its `particle_class` \
             plus its contextual `case` or `marking`. Multiple values within one category are \
             alternatives: choose the value licensed by context (`이/가` subject vs complement, \
             `(으)로` instrument vs direction, `와/과` adverbial vs conjunctive).\n\
             ALLOMORPHY: a Korean particle or ending selects its shape from the preceding \
             syllable — consonant vs vowel final (이/가, 을/를, 은/는, 으로/로), or the stem's \
             last vowel (았/었, 아서/어서). Always report the surface allomorph in `surface` and \
             the slashed citation form in `base_form`.\n\
             CONTRACTIONS: undo them. 해요 is 하 + 아요/어요; 이에요/예요 and 아니에요 use 아요/어요 once, \
             never an intimate 아/어 plus a second polite ending; 갔어요 is 가 + 았/었 + 아요/어요; \
             하세요 is 하 + (으)시 + 아요/어요; 뭘 is 무엇 + 을/를.\n\
             ORDER: list morphemes left to right, exactly as they appear.\n\
             STEM: never list the stem — it is already captured in `lemma`. The sole exception is the \
             predicative particle `이다` in a noun + copula predicate: list its surface `이` with \
             base_form `이다`; in contracted `예요`, the fused surface `예` may realize both `이다` \
             and `아요/어요`.\n\
             Segment only 어절 that carry at least one particle or ending."
        )
    }
}

// ─── LinguisticDefinition implementation ─────────────────────────────────────

pub struct Korean;

impl LinguisticDefinition for Korean {
    type Morphology = KoreanMorphology;
    type MorphemeFunction = KoreanMorphemeFunction;

    const ISO_LANG: IsoLang = IsoLang::Kor;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        KoreanMorphology::PIVOT_CASE,
        KoreanMorphology::PIVOT_MARKING,
        KoreanMorphology::PIVOT_SPEECH_LEVEL,
        KoreanMorphology::PIVOT_TENSE,
        KoreanMorphology::PIVOT_AUXILIARY_CONSTRUCTION,
        KoreanMorphology::PIVOT_MODALITY,
        KoreanMorphology::PIVOT_SUBJECT_HONORIFICATION,
        KoreanMorphology::PIVOT_OBJECT_HONORIFICATION,
        KoreanMorphology::PIVOT_SPEAKER_HUMILITY,
        KoreanMorphology::PIVOT_FORM,
    ];
    const MORPHEME_PIVOTS: &'static [panini_core::pivot::PivotField<Self::MorphemeFunction>] = &[
        KoreanMorphemeFunction::PIVOT_SPEECH_LEVEL,
        KoreanMorphemeFunction::PIVOT_CASE,
        KoreanMorphemeFunction::PIVOT_TENSE,
        KoreanMorphemeFunction::PIVOT_CONNECTIVE,
        KoreanMorphemeFunction::PIVOT_PARTICLE_CLASS,
    ];

    /// Hangul is the working script; Hanja still appears in legal, academic and
    /// newspaper text and in disambiguating glosses, so it is genuinely part of
    /// what the language is written in.
    fn supported_scripts(&self) -> &[Script] {
        &[Script::HANG, Script::HANI]
    }

    fn default_script(&self) -> Script {
        Script::HANG
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            // Descriptive verbs (형용사) conjugate exactly like action verbs,
            // so they belong in the conjugation payload.
            TypologicalFeature::Conjugation(&[Upos::Verb, Upos::Adjective]),
            // Case is marked by cliticised 격조사, which is what a declension
            // cloze blanks out.
            TypologicalFeature::Declension(&[Upos::Noun, Upos::ProperNoun, Upos::Pronoun]),
            TypologicalFeature::Agglutination,
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization: verbs and descriptive verbs (adjectives) take the dictionary form ending in -다 (가다, 좋다, 공부하다). Nouns, pronouns and proper nouns take the bare stem with every particle stripped (학교에서 → 학교, 저는 → 저).\n\
         2. Tokenization: keep each 어절 (spacing unit) as ONE token. A noun and its particles (학교에서는) is a single noun token, not a noun plus an adposition; record the particles through `case` and `marking` and segment them in `morpheme_segmentation`. Only tag a particle as `particle` or `adposition` when it is genuinely written detached (에 대해서, 을 위해).\n\
         3. Nouns, proper nouns and pronouns: always give `case` and `marking`. Use case `unmarked` when no 격조사 is present, and marking `unmarked` when no 보조사 is present — 은/는 is `marking: topic`, NOT a case. Distinguish instrumental ((으)로 as a means), directional ((으)로 as a path or destination), and essive ((으)로서 'as/in the capacity of'). For stacked 보조사, put the semantically narrower non-topic value in `marking`; segmentation preserves every particle. Give `number` only when -들 is actually present; Korean nouns are otherwise number-neutral, so omit it rather than guessing singular.\n\
         4. Predicates: adjectives always give `form`, `subject_honorification` and `polarity`; verbs additionally give `object_honorification` and `voice`. Use `tense: non_past` when no past pre-final ending is present, `past` for -았/었-, and `remote_past` for -았었/었었-. Do NOT call -겠- future tense: encode its contextual meaning as `modality: conjecture` or `volition`. Give `speech_level` and `final_ending_function` only to final (종결형) predicates; omit them for connective, adnominal and nominal forms. In an auxiliary chain, the lexical predicate is `connective` and the final auxiliary is `final`. Give `auxiliary_construction` only to that auxiliary when the full construction licenses it: 있다 in -고 있다 is `progressive`, 있다 in -아/어 있다 is `resultative`, 보다 in -아/어 보다 is `experiential`, and 버리다 in -아/어 버리다 is `completive`; omit it for lexical uses. Give `modality` only when overtly encoded: `conjecture`, `volition`, `retrospective`, or `mirative`.\n\
         5. Verb vs adjective: 형용사 (좋다, 크다, 예쁘다, 아프다) are descriptive verbs — tag them `adjective`, never `verb`, even though they conjugate. 있다/없다 are existential verbs; tag them `verb`. A noun + 하다 compound (공부하다) is one `verb` token with lemma 공부하다.\n\
         6. Honorification uses independent axes because they can co-occur. Set `subject_honorification: honorific` for -(으)시- and subject-respect lexemes; set `object_honorification: honorific` for object/recipient-respect verbs such as 드리다, 뵙다 and 모시다. Otherwise use `non_honorific`. On pronouns, set `speaker_humility: humble` for 저/저희 and `neutral` otherwise. 께서 and 께 are recorded on their host noun in morpheme segmentation; never collapse these axes into speech level.\n\
         7. Negation: 안 and 못 are separate adverb tokens, but the predicate they scope carries `polarity: negative`. In -지 않다 / -지 못하다, tag 않다/못하다 as a separate verb token and propagate negative polarity across the lexical predicate and the negative auxiliary; `form` still follows each token's own ending.\n\
         8. Numerals: give `numeral_system` — `native` for 하나/둘/셋/스물 and their prenominal forms 한/두/세, `sino_korean` for 일/이/삼 and every number above 99. Counters (개, 명, 권, 시, 살) are `classifier`, not `noun`.\n\
         9. Copula: 이다 is a `verb`; when it is attached to its nominal complement in one 어절, preserve that complement in the compound lemma (학생이에요 → 학생이다). 아니다 is a `verb` with lemma 아니다 and `polarity: negative`.\n\
         10. Script: keep the surface script the input uses. Do not transliterate Hanja into Hangul or romanize anything — the lemma is written in the same script as the token."
    }

    fn extra_extraction_directives(&self) -> Option<String> {
        Some(self.morpheme_directives())
    }


    fn alignment_directives(&self) -> Option<&'static str> {
        Some(
            "1. Each 어절 (spacing unit) is one word. Particles written attached to it are separable segments, in order: plural -들, case 이/가, 을/를, 의, 에, 에서, 에게/한테, (으)로, 와/과/하고, 부터, 까지, then 은/는, 도, 만, 요 — [\"학교\", \"에서\", \"는\"], [\"친구\", \"들\", \"도\"], [\"저\", \"는\"].\n\
             2. A predicate's ending chain is separable: stem, subject honorific -(으)시-, past -았/었- or modal -겠-, and the final or connective ending (-습니다, -어요, -고, -지만, -(으)면, -(으)ㄴ/-는/-(으)ㄹ) — [\"먹\", \"었\", \"어요\"], [\"가\", \"고\"]. A syllable that fuses two morphemes (갔, 셨, 해요) stays one segment linked to everything it fuses: [\"가\", \"셨\", \"어요\"].\n\
             3. A noun + 하다 verb and a noun + copula split at the noun: [\"공부\", \"해요\"], [\"학생\", \"이에요\"].\n\
             4. Long negation spans two words in one link: [\"먹\", \"지\"] + [\"않\", \"아요\"]; 안, 못 and 아니다 are separate words.\n\
             5. Sino-Korean numerals and counters written attached split at the counter: [\"삼십\", \"분\"].",
        )
    }

    fn post_process_extraction(
        &self,
        segmentation: &mut Option<Vec<WordSegmentation<KoreanMorphemeFunction>>>,
    ) -> Result<(), String> {
        self.validate_and_enrich(segmentation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn korean_identity_scripts_and_typology_are_exact() {
        let language = Korean;

        assert_eq!(Korean::ISO_LANG, IsoLang::Kor);
        assert_eq!(Korean::ISO_LANG.to_639_3(), "kor");
        // Hangul first — `default_script` is what the API turns into a text
        // direction, and Hanja must never be the one Panglotive teaches in.
        assert_eq!(
            language.supported_scripts(),
            &[Script::HANG, Script::HANI]
        );
        assert_eq!(language.default_script(), Script::HANG);
    }

    /// Descriptive verbs conjugate in Korean, so a conjugation cloze that drew
    /// only from `Upos::Verb` would never exercise 형용사 — half the predicates
    /// a learner meets.
    #[test]
    fn adjectives_are_in_the_conjugation_payload() {
        let features = Korean.typological_features();

        let conjugates: &[Upos] = features
            .iter()
            .find_map(|feature| match feature {
                TypologicalFeature::Conjugation(pos) => Some(*pos),
                TypologicalFeature::Declension(_) | TypologicalFeature::Agglutination => None,
            })
            .expect("Korean declares conjugation");

        assert!(conjugates.contains(&Upos::Adjective));
        assert!(conjugates.contains(&Upos::Verb));
    }

    /// Both hand-written pivots exist because the derive skips optional fields,
    /// so they are the two with no generated coverage — and speech level is the
    /// headline Korean facet.
    #[test]
    fn hand_written_pivots_extract_from_optional_fields() {
        let finite = KoreanMorphology::Verb {
            lemma: "가다".to_string(),
            tense: Some(KoreanTense::Past),
            speech_level: Some(KoreanSpeechLevel::Deferential),
            final_ending_function: Some(KoreanFinalEndingFunction::Declarative),
            form: KoreanVerbForm::Final,
            subject_honorification: KoreanSubjectHonorification::NonHonorific,
            object_honorification: KoreanObjectHonorification::NonHonorific,
            voice: KoreanVoice::Active,
            polarity: KoreanPolarity::Positive,
            auxiliary_construction: None,
            modality: None,
        };
        let adnominal = KoreanMorphology::Adjective {
            lemma: "좋다".to_string(),
            tense: None,
            speech_level: None,
            final_ending_function: None,
            form: KoreanVerbForm::Adnominal,
            subject_honorification: KoreanSubjectHonorification::NonHonorific,
            polarity: KoreanPolarity::Positive,
            modality: None,
        };

        assert_eq!(
            KoreanMorphology::PIVOT_SPEECH_LEVEL.value(&finite),
            Some("deferential".to_string())
        );
        assert_eq!(KoreanMorphology::PIVOT_SPEECH_LEVEL.value(&adnominal), None);
        assert_eq!(
            KoreanMorphology::PIVOT_TENSE.value(&finite),
            Some("past".to_string())
        );
        assert_eq!(KoreanMorphology::PIVOT_TENSE.value(&adnominal), None);
    }

    /// 은/는 is a 보조사, not a case particle: it lives in its own slot and can
    /// stack on top of one (학교에서는). Collapsing it into `case` would make
    /// the topic/subject contrast — the hardest thing in beginner Korean —
    /// unrepresentable.
    #[test]
    fn topic_marking_is_independent_of_case() {
        let topic_on_a_locative = KoreanMorphology::Noun {
            lemma: "학교".to_string(),
            case: KoreanCase::Locative,
            marking: KoreanMarking::Topic,
            number: None,
        };

        assert_eq!(
            KoreanMorphology::PIVOT_CASE.value(&topic_on_a_locative),
            Some("locative".to_string())
        );
        assert_eq!(
            KoreanMorphology::PIVOT_MARKING.value(&topic_on_a_locative),
            Some("topic".to_string())
        );
    }

    #[test]
    fn homographic_ro_distinctions_are_all_representable() {
        let ro = KOREAN_MORPHEMES
            .iter()
            .find(|morpheme| morpheme.base_form == "(으)로")
            .expect("the instrumental/directional particle is inventoried");
        let roseo = KOREAN_MORPHEMES
            .iter()
            .find(|morpheme| morpheme.base_form == "(으)로서")
            .expect("the essive particle is inventoried");

        assert!(ro.functions.contains(&F::Case {
            value: KoreanCase::Instrumental,
        }));
        assert!(ro.functions.contains(&F::Case {
            value: KoreanCase::Directional,
        }));
        assert!(roseo.functions.contains(&PC_ADVERBIAL));
        assert!(roseo.functions.contains(&F::Case {
            value: KoreanCase::Essive,
        }));
    }

    #[test]
    fn observed_missing_morphemes_have_dedicated_functions() {
        let expected = [
            ("에게서", "case:ablative"),
            ("마저", "marking:inclusive"),
            ("마다", "marking:distributive"),
            ("씩", "marking:distributive"),
            ("다고", "connective:quotative"),
            ("냐고", "connective:quotative"),
            ("(으)라고", "connective:quotative"),
            ("자고", "connective:quotative"),
            ("아야/어야", "connective:obligative"),
            ("더라", "modality:retrospective"),
            ("네요", "modality:mirative"),
            ("도록", "connective:purposive"),
            ("자마자", "connective:immediate"),
            ("(으)ㄹ수록", "connective:proportional"),
            ("(으)ㄴ가/나", "modality:conjecture"),
            ("느라고", "connective:causal"),
            ("(으)므로", "connective:causal"),
        ];

        for (base_form, function) in expected {
            let morpheme = KOREAN_MORPHEMES
                .iter()
                .find(|morpheme| morpheme.base_form == base_form)
                .unwrap_or_else(|| panic!("missing Korean morpheme {base_form}"));
            assert!(
                morpheme
                    .functions
                    .iter()
                    .any(|candidate| candidate.directive_label() == function),
                "{base_form} does not expose {function}"
            );
        }
    }

    #[test]
    fn nominal_particles_expose_a_traditional_particle_class() {
        let classified = [
            "이다",
            "이/가",
            "께서",
            "을/를",
            "의",
            "에",
            "에서",
            "에게",
            "에게서",
            "한테",
            "한테서",
            "께",
            "(으)로",
            "(으)로서",
            "와/과",
            "하고",
            "(이)랑",
            "부터",
            "까지",
            "보다",
            "처럼",
            "아/야",
            "은/는",
            "도",
            "만",
            "밖에",
            "뿐",
            "조차",
            "마저",
            "마다",
            "씩",
        ];

        for base_form in classified {
            let morpheme = KOREAN_MORPHEMES
                .iter()
                .find(|morpheme| morpheme.base_form == base_form)
                .unwrap_or_else(|| panic!("missing Korean particle {base_form}"));
            assert!(
                morpheme
                    .functions
                    .iter()
                    .any(|function| matches!(function, F::ParticleClass { .. })),
                "{base_form} has no traditional particle class"
            );
        }
    }

    #[test]
    fn homographic_particles_keep_their_class_alternatives() {
        let expected = [
            ("이다", "particle_class:predicative_case"),
            ("이/가", "particle_class:subject_case"),
            ("이/가", "particle_class:complement_case"),
            ("에서", "particle_class:subject_case"),
            ("에서", "particle_class:adverbial_case"),
            ("와/과", "particle_class:adverbial_case"),
            ("와/과", "particle_class:conjunctive"),
            ("부터", "particle_class:auxiliary"),
        ];

        for (base_form, function) in expected {
            let morpheme = KOREAN_MORPHEMES
                .iter()
                .find(|morpheme| morpheme.base_form == base_form)
                .unwrap_or_else(|| panic!("missing Korean particle {base_form}"));
            assert!(
                morpheme
                    .functions
                    .iter()
                    .any(|candidate| candidate.directive_label() == function),
                "{base_form} does not expose {function}"
            );
        }
    }

    #[test]
    fn auxiliary_construction_and_modality_are_pivotable() {
        let progressive = KoreanMorphology::Verb {
            lemma: "있다".to_string(),
            tense: Some(KoreanTense::NonPast),
            speech_level: Some(KoreanSpeechLevel::Plain),
            final_ending_function: Some(KoreanFinalEndingFunction::Declarative),
            form: KoreanVerbForm::Final,
            subject_honorification: KoreanSubjectHonorification::NonHonorific,
            object_honorification: KoreanObjectHonorification::NonHonorific,
            voice: KoreanVoice::Active,
            polarity: KoreanPolarity::Positive,
            auxiliary_construction: Some(KoreanAuxiliaryConstruction::Progressive),
            modality: Some(KoreanModality::Mirative),
        };

        assert_eq!(
            KoreanMorphology::PIVOT_AUXILIARY_CONSTRUCTION.value(&progressive),
            Some("progressive".to_string())
        );
        assert_eq!(
            KoreanMorphology::PIVOT_MODALITY.value(&progressive),
            Some("mirative".to_string())
        );
    }

    #[test]
    fn subject_and_object_honorification_can_cooccur() {
        let honorific = KoreanMorphology::Verb {
            lemma: "드리다".to_string(),
            tense: Some(KoreanTense::NonPast),
            speech_level: Some(KoreanSpeechLevel::Deferential),
            final_ending_function: Some(KoreanFinalEndingFunction::Declarative),
            form: KoreanVerbForm::Final,
            subject_honorification: KoreanSubjectHonorification::Honorific,
            object_honorification: KoreanObjectHonorification::Honorific,
            voice: KoreanVoice::Active,
            polarity: KoreanPolarity::Positive,
            auxiliary_construction: None,
            modality: None,
        };

        assert_eq!(
            KoreanMorphology::PIVOT_SUBJECT_HONORIFICATION.value(&honorific),
            Some("honorific".to_string())
        );
        assert_eq!(
            KoreanMorphology::PIVOT_OBJECT_HONORIFICATION.value(&honorific),
            Some("honorific".to_string())
        );
    }

    #[test]
    fn gess_is_modal_and_miratives_stay_declarative() {
        let gess = KOREAN_MORPHEMES
            .iter()
            .find(|morpheme| morpheme.base_form == "겠")
            .expect("-겠- is inventoried");
        assert!(!gess
            .functions
            .iter()
            .any(|function| function.directive_label().starts_with("tense:")));
        assert!(gess.functions.contains(&F::Modality {
            value: KoreanModality::Conjecture,
        }));
        assert!(gess.functions.contains(&F::Modality {
            value: KoreanModality::Volition,
        }));

        for base_form in ["네", "네요", "군/구나"] {
            let ending = KOREAN_MORPHEMES
                .iter()
                .find(|morpheme| morpheme.base_form == base_form)
                .unwrap_or_else(|| panic!("missing Korean final ending {base_form}"));
            assert!(ending.functions.contains(&F::FinalEndingFunction {
                value: KoreanFinalEndingFunction::Declarative,
            }));
            assert!(ending.functions.contains(&F::Modality {
                value: KoreanModality::Mirative,
            }));
        }
    }
}
