use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature,
};

/// The learner-relevant grammatical function of a Vietnamese adverb.
///
/// Vietnamese temporal reference, aspect, negation and modality are expressed
/// by independent words rather than by inflecting the lexical verb. Keeping
/// those markers on the adverb token prevents an analytic construction from
/// being misreported as a synthetic verb paradigm.
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
pub enum VietnameseAdverbType {
    /// An ordinary lexical adverb, including manner and location expressions.
    Lexical,
    /// A free marker such as `đã`, `đang`, `sẽ`, `từng`, `vừa` or `mới`.
    TemporalAspect,
    /// A free negator such as `không`, `chưa`, `chẳng` or prohibitive `đừng`.
    Negation,
    /// A degree expression such as `rất`, `khá`, `quá` or `nhất`.
    Degree,
    /// A modal expression such as `hãy` or `có lẽ` when used adverbially.
    Modal,
}

/// The discourse function of a Vietnamese grammatical particle.
///
/// The inventory is functional rather than positional: sentence-final
/// particles do not all mean the same thing, and several items have different
/// functions in different contexts.
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
pub enum VietnameseParticleType {
    /// A polar-question or response-seeking particle, for example final `à`.
    Interrogative,
    /// A politeness particle such as `ạ`.
    Politeness,
    /// A speaker-stance or illocutionary particle such as `nhé` or `nhỉ`.
    Modal,
    /// A constituent-focus particle such as `chính`.
    Focus,
    /// A topic or contrast marker such as `thì` in its particle use.
    Topic,
    /// A turn-management or connective discourse particle such as `mà` or `chứ`.
    Discourse,
    /// A genuine particle whose contextual function does not fit the classes above.
    Other,
}

/// How an expression functioning as a Vietnamese pronoun establishes reference.
///
/// `KinshipAddress` and `TitleAddress` are kept distinct from dedicated personal
/// pronouns because person reference in Vietnamese is relational: the same term
/// can denote the speaker, addressee or a third person according to context.
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
pub enum VietnamesePronounType {
    Personal,
    KinshipAddress,
    TitleAddress,
    Demonstrative,
    Interrogative,
    Relative,
    Reflexive,
    Reciprocal,
    IndefiniteGeneric,
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
pub enum VietnameseMorphology {
    Adjective {
        lemma: String,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        adverb_type: VietnameseAdverbType,
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
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_type: VietnameseParticleType,
    },
    Pronoun {
        lemma: String,
        pronoun_type: VietnamesePronounType,
        /// Present only when this occurrence refers to a speech participant or
        /// a third person; relational address terms take their person from context.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Present only where the pronominal expression itself establishes number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
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
    },
    Other {
        lemma: String,
    },
}

pub struct Vietnamese;

impl LinguisticDefinition for Vietnamese {
    type Morphology = VietnameseMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Vie;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        VietnameseMorphology::PIVOT_ADVERB_TYPE,
        VietnameseMorphology::PIVOT_PARTICLE_TYPE,
        VietnameseMorphology::PIVOT_PRONOUN_TYPE,
    ];

    /// Contemporary standard Vietnamese is written in Quốc ngữ. Chữ Hán and
    /// chữ Nôm remain heritage systems for historical texts and calligraphy,
    /// not alternative contemporary standard orthographies for this course.
    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope and lemmatization: analyze contemporary standard Vietnamese in Quốc ngữ. Preserve every vowel-quality and tone diacritic exactly. Vietnamese words do not inflect for case, gender, number, person or tense: use the unchanged dictionary headword as lemma and never manufacture an inflected paradigm. Keep an established reduplicative or compound dictionary form intact rather than stripping a supposed affix.\n\
         2. Word segmentation: spaces in Vietnamese separate syllables, not reliably words. Group the syllables of one established lexical word into one TokenAnalysis with its exact spaced surface and lemma (for example `sinh viên`, `điện thoại`, `bắt đầu`), but never merge a productive phrase. Keep classifiers, determiners, temporal/aspect words, negators, auxiliaries and sentence particles as separate lexical tokens.\n\
         3. Verbs and analytic predicates: verbs never receive tense, aspect, mood, voice, person or number fields. Analyze each independent marker separately. Tag `là`, modal/auxiliary expressions such as `có thể` and `phải`, and passive auxiliaries `bị`/`được` as separate Verbs under Panini's AUX-to-Verb policy; do not transfer their function onto the lexical verb. Vietnamese property words can head predicates without a copula: tag conventional property lexemes as Adjectives, not as inflected verbs.\n\
         4. Adverbs: every Adverb gets adverb_type. Use temporal_aspect for free markers such as `đã`, `đang`, `sẽ`, `từng`, `vừa` and `mới`; these locate or structure an event but do not conjugate the verb, and their precise temporal/aspectual reading comes from context. Use negation for `không`, `chưa`, `chẳng` and prohibitive `đừng`; degree for items such as `rất`, `khá`, `quá` and `nhất`; modal for adverbial expressions such as `hãy` and `có lẽ`; otherwise use lexical.\n\
         5. Classifiers and noun phrases: tag a classifier or measure/class word accompanying a noun as Classifier (`con`, `cái`, `chiếc`, `quyển/cuốn`, `tờ`, and contextually classifier-like nouns). Keep the numeral, classifier and noun as separate tokens. Tag `các` and `những` as Determiners rather than as plural morphology on the noun, and do not infer singular or plural from a bare noun. Distinguish a classifier use from the same form's ordinary Noun or Particle use by syntax.\n\
         6. Pronouns and person reference: every Pronoun gets pronoun_type. Use kinship_address only when a kinship term such as `anh`, `chị`, `em`, `cô`, `chú`, `bác`, `ông` or `bà` functions as person reference; use title_address for an occupational or status term used the same way. Assign person from the referent in this occurrence, never permanently from the lemma: a kinship/address term can be first, second or third person. Add number only when the whole pronominal expression establishes it. Omit person and number for demonstrative, interrogative, relative, reflexive, reciprocal and indefinite/generic forms when those categories are not grammatically expressed. Do not collapse this relational system into a single formal/informal scale.\n\
         7. Pronouns versus determiners: classify forms such as `này`, `đó`, `kia`, `ai`, `gì` and `nào` by their actual syntactic function. A form modifying an overt nominal is a Determiner; a form heading the noun phrase is a Pronoun. Keep lexicalized multi-syllable or multiword pronouns such as `chúng tôi`, `chúng ta`, `anh ấy`, `cô ấy`, `bản thân` and `người ta` as one TokenAnalysis when they function as one reference expression.\n\
         8. Particles: every Particle gets particle_type according to its function in context, not merely its position. Use interrogative for question particles, politeness for `ạ`, modal for stance/illocutionary softeners such as `nhé` or `nhỉ`, focus for focus markers such as `chính`, topic for particle `thì`, and discourse for connective or turn-management uses such as `mà` and `chứ`; use other only for a genuine particle outside these functions. The same spelling may instead be an Adverb, Determiner, Pronoun, Verb or Interjection in another context. Never emit punctuation as a token."
    }
}

#[cfg(test)]
mod tests {
    use panini_core::aggregable::ClosedValues;

    use super::*;

    #[test]
    fn vietnamese_identity_script_and_typology_are_exact() {
        let language = Vietnamese;

        assert_eq!(Vietnamese::ISO_LANG, IsoLang::Vie);
        assert_eq!(Vietnamese::ISO_LANG.to_639_3(), "vie");
        assert_eq!(language.supported_scripts(), &[Script::LATN]);
        assert_eq!(language.default_script(), Script::LATN);
        assert!(language.typological_features().is_empty());
    }

    #[test]
    fn analytic_grammar_pivots_are_deliberately_limited() {
        let keys = Vietnamese::MORPHOLOGY_PIVOTS
            .iter()
            .map(|pivot| pivot.key)
            .collect::<Vec<_>>();

        assert_eq!(keys, ["adverb_type", "particle_type", "pronoun_type"]);
    }

    #[test]
    fn person_reference_types_have_stable_wire_values() {
        assert_eq!(
            VietnamesePronounType::all_variants(),
            &[
                "personal",
                "kinship_address",
                "title_address",
                "demonstrative",
                "interrogative",
                "relative",
                "reflexive",
                "reciprocal",
                "indefinite_generic",
            ]
        );
    }

    #[test]
    fn relational_address_person_is_contextual() {
        let addressee = VietnameseMorphology::Pronoun {
            lemma: "chị".to_string(),
            pronoun_type: VietnamesePronounType::KinshipAddress,
            person: Some(Person::Second),
            number: Some(BinaryNumber::Singular),
        };

        let serialized = serde_json::to_value(addressee).expect("pronoun is serializable");
        assert_eq!(serialized["person"], "second");
        assert_eq!(serialized["number"], "singular");
    }
}
