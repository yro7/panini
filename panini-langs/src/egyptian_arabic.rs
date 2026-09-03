use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryNumber,
    TypologicalFeature, Upos,
};

/// Definiteness of an Egyptian Arabic nominal.
///
/// A construct-state noun can carry either value: its definiteness comes from
/// the following possessor rather than from the article on the head noun.
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
pub enum EgyptianArabicDefiniteness {
    Indefinite,
    Definite,
}

/// Whether a noun is free or is the head of an idafa/possessive construction.
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
pub enum EgyptianArabicNominalState {
    Free,
    Construct,
}

/// The synchronically relevant source of an adjectival form.
///
/// Egyptian active participles can serve as predicates with present, recent
/// past, or prospective force, but morphologically they still agree like
/// adjectives. The elative is one form whose comparative or superlative
/// interpretation is supplied by syntax, so those are not split here.
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
pub enum EgyptianArabicAdjectiveForm {
    Lexical,
    ActiveParticiple,
    PassiveParticiple,
    Elative,
}

/// The productive and lexically established verbal patterns of Egyptian Arabic.
///
/// `form_vii_in` is the inherited `infa3al` pattern. `form_vii_it` is the
/// Egyptian `itfa3al` passive/reflexive pattern sometimes called VIIt in
/// descriptions of the dialect. Only the two surviving quadriliteral patterns
/// are represented; obsolete Classical patterns are deliberately absent.
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
pub enum EgyptianArabicVerbPattern {
    FormI,
    FormII,
    FormIII,
    FormIV,
    FormV,
    FormVI,
    #[serde(rename = "form_vii_in")]
    FormVIIIn,
    #[serde(rename = "form_vii_it")]
    FormVIIIt,
    FormVIII,
    FormIX,
    FormX,
    QuadriliteralI,
    QuadriliteralII,
}

/// The five finite paradigms a learner must distinguish in Egyptian Arabic.
///
/// Unlike Modern Standard Arabic, Egyptian Arabic does not retain inflectional
/// case or the indicative/subjunctive/jussive ending system. Its imperfective
/// stem instead contrasts a bare form with `b-` and future `ha-/ḥa-` series.
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
pub enum EgyptianArabicVerbForm {
    Perfective,
    BareImperfective,
    BiImperfective,
    HaImperfective,
    Imperative,
}

/// Diathesis as expressed by an Egyptian Arabic verb in context.
///
/// Passive and middle/reflexive meanings are normally built with derived
/// patterns rather than the productive internal passive of Standard Arabic.
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
pub enum EgyptianArabicVoice {
    Active,
    Passive,
    MiddleReflexive,
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
pub enum EgyptianArabicPolarity {
    Affirmative,
    Negative,
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
pub enum EgyptianArabicPronounType {
    Personal,
    Demonstrative,
    Relative,
    Interrogative,
    Reflexive,
    Indefinite,
}

/// Syntactic function of an attached personal-pronoun clitic.
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
pub enum EgyptianArabicAttachmentFunction {
    Possessive,
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
pub enum EgyptianArabicDeterminerType {
    DefiniteArticle,
    Demonstrative,
    Quantifier,
    Interrogative,
}

/// Function of an Egyptian Arabic particle or morphologically split clitic.
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
pub enum EgyptianArabicParticleFunction {
    Negation,
    Interrogative,
    Vocative,
    Future,
    Imperfective,
    Focus,
    Existential,
}

/// Morphological features of contemporary Egyptian Arabic (`arz`).
///
/// This is intentionally not a reduced copy of the former Standard Arabic
/// definition. Egyptian has no productive nominal case endings or finite mood
/// endings, no dual or gender-distinct plural verb agreement, and its everyday
/// finite system is organized around perfective and three imperfective series.
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
pub enum EgyptianArabicMorphology {
    Adjective {
        lemma: String,
        /// Present only for a synchronically transparent Arabic root.
        #[serde(skip_serializing_if = "Option::is_none")]
        root: Option<String>,
        adjective_form: EgyptianArabicAdjectiveForm,
        /// Omitted when the adjective's form has no gender contrast, notably
        /// on the ordinary plural agreement form.
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_gender: Option<BinaryGender>,
        agreement_number: BinaryNumber,
        definiteness: EgyptianArabicDefiniteness,
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
        determiner_type: EgyptianArabicDeterminerType,
        /// Demonstratives encode singular/plural; articles and most
        /// non-demonstrative determiners do not.
        #[serde(skip_serializing_if = "Option::is_none")]
        referent_number: Option<BinaryNumber>,
        /// Singular demonstratives contrast masculine/feminine. The plural
        /// demonstrative and the definite article do not.
        #[serde(skip_serializing_if = "Option::is_none")]
        referent_gender: Option<BinaryGender>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        /// Present only for a synchronically transparent Arabic root.
        #[serde(skip_serializing_if = "Option::is_none")]
        root: Option<String>,
        gender: BinaryGender,
        number: TernaryNumber,
        definiteness: EgyptianArabicDefiniteness,
        state: EgyptianArabicNominalState,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_function: EgyptianArabicParticleFunction,
    },
    Pronoun {
        lemma: String,
        pronoun_type: EgyptianArabicPronounType,
        clitic: bool,
        /// Present only for an attached personal pronoun.
        #[serde(skip_serializing_if = "Option::is_none")]
        attachment_function: Option<EgyptianArabicAttachmentFunction>,
        /// Present only when the pronoun itself encodes person.
        #[serde(skip_serializing_if = "Option::is_none")]
        referent_person: Option<Person>,
        /// Present only when the pronoun itself encodes number.
        #[serde(skip_serializing_if = "Option::is_none")]
        referent_number: Option<BinaryNumber>,
        /// Present only when the pronoun itself contrasts masculine/feminine.
        #[serde(skip_serializing_if = "Option::is_none")]
        referent_gender: Option<BinaryGender>,
    },
    ProperNoun {
        lemma: String,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Verb {
        lemma: String,
        /// Arabic consonantal root, including quadriliteral roots.
        #[serde(skip_serializing_if = "Option::is_none")]
        root: Option<String>,
        pattern: EgyptianArabicVerbPattern,
        verb_form: EgyptianArabicVerbForm,
        voice: EgyptianArabicVoice,
        polarity: EgyptianArabicPolarity,
        person: Person,
        agreement_number: BinaryNumber,
        /// Egyptian distinguishes gender only in singular second- and
        /// third-person agreement; first person and plural forms omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_gender: Option<BinaryGender>,
    },
    Other {
        lemma: String,
    },
}

impl EgyptianArabicMorphology {
    fn __pivot_root(&self) -> Option<String> {
        match self {
            Self::Adjective { root, .. } | Self::Noun { root, .. } | Self::Verb { root, .. } => {
                root.clone()
            }
            _ => None,
        }
    }

    fn __pivot_agreement_gender(&self) -> Option<String> {
        match self {
            Self::Adjective {
                agreement_gender,
                ..
            }
            | Self::Verb {
                agreement_gender,
                ..
            } => agreement_gender.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    fn __pivot_attachment_function(&self) -> Option<String> {
        match self {
            Self::Pronoun {
                attachment_function,
                ..
            } => attachment_function.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    fn __pivot_referent_number(&self) -> Option<String> {
        match self {
            Self::Determiner {
                referent_number, ..
            }
            | Self::Pronoun {
                referent_number, ..
            } => referent_number.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    fn __pivot_referent_gender(&self) -> Option<String> {
        match self {
            Self::Determiner {
                referent_gender, ..
            }
            | Self::Pronoun {
                referent_gender, ..
            } => referent_gender.as_ref().map(|value| {
                panini_core::aggregable::ClosedValues::variant_str(value).to_string()
            }),
            _ => None,
        }
    }

    /// Open root pivot, written by hand because `root` is genuinely absent on
    /// opaque loans and the derive therefore cannot generate a handle for it.
    pub const PIVOT_ROOT: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::open("root", "Root", Self::__pivot_root);

    pub const PIVOT_AGREEMENT_GENDER: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "agreement_gender",
            "Agreement Gender",
            <BinaryGender as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_agreement_gender,
        );

    pub const PIVOT_ATTACHMENT_FUNCTION: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "attachment_function",
            "Attachment Function",
            <EgyptianArabicAttachmentFunction as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_attachment_function,
        );

    pub const PIVOT_REFERENT_NUMBER: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "referent_number",
            "Referent Number",
            <BinaryNumber as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_referent_number,
        );

    pub const PIVOT_REFERENT_GENDER: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "referent_gender",
            "Referent Gender",
            <BinaryGender as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_referent_gender,
        );
}

pub struct EgyptianArabic;

impl LinguisticDefinition for EgyptianArabic {
    type Morphology = EgyptianArabicMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Arz;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        EgyptianArabicMorphology::PIVOT_ROOT,
        EgyptianArabicMorphology::PIVOT_PATTERN,
        EgyptianArabicMorphology::PIVOT_VERB_FORM,
        EgyptianArabicMorphology::PIVOT_VOICE,
        EgyptianArabicMorphology::PIVOT_POLARITY,
        EgyptianArabicMorphology::PIVOT_GENDER,
        EgyptianArabicMorphology::PIVOT_AGREEMENT_GENDER,
        EgyptianArabicMorphology::PIVOT_NUMBER,
        EgyptianArabicMorphology::PIVOT_AGREEMENT_NUMBER,
        EgyptianArabicMorphology::PIVOT_DEFINITENESS,
        EgyptianArabicMorphology::PIVOT_STATE,
        EgyptianArabicMorphology::PIVOT_ADJECTIVE_FORM,
        EgyptianArabicMorphology::PIVOT_PRONOUN_TYPE,
        EgyptianArabicMorphology::PIVOT_CLITIC,
        EgyptianArabicMorphology::PIVOT_ATTACHMENT_FUNCTION,
        EgyptianArabicMorphology::PIVOT_REFERENT_NUMBER,
        EgyptianArabicMorphology::PIVOT_REFERENT_GENDER,
    ];

    fn supported_scripts(&self) -> &[Script] {
        // Arabic script is the ordinary written medium. Latin-script Arabizi is
        // also in contemporary use, especially in informal digital writing.
        &[Script::ARAB, Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::ARAB
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            TypologicalFeature::Conjugation(&[Upos::Verb]),
            TypologicalFeature::Declension(&[Upos::Noun, Upos::Adjective]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope and lemmatization: analyze contemporary Egyptian Arabic (`arz`), with neutral Cairene usage as the default, and NEVER silently normalize it into Modern Standard Arabic. Lemmatize verbs to the third-person masculine singular perfective (بيكتب/حيكتب/اكتب -> كتب; بيتكلم -> اتكلم), nouns to the indefinite singular, adjectives to the masculine singular lexical or participial form, and pronouns/determiners to their independent citation form. There is no infinitive citation form.\n\
         2. Canonical script: keep `word` exactly in the source script. Write every lemma and root in CODA-style Arabic script so Arabic and Latin/Arabizi input aggregate to one lexicon. Write a root as hyphen-separated radicals (`ك-ت-ب`, `ك-ل-م`, including four radicals when real). Preserve Egyptian spellings and NEVER replace an Egyptian lemma with an MSA cognate.\n\
         3. Roots: include `root` only when a synchronically transparent Arabic root exists. `كتاب` has `ك-ت-ب`; by contrast `مكالمة` has `ك-ل-م`. OMIT root for an opaque loan such as `ترابيزة` and for a loan adjective such as `شيك`; never manufacture a root from every consonant in a loan. Every ordinary patterned verb must have a root, including a real quadriliteral such as `ترجم` (`ت-ر-ج-م`).\n\
         4. Nouns: always report lexical gender, singular/dual/plural number, definiteness and state. Egyptian has a productive nominal dual (`كتابين`) but NO nominal case endings. `كتاب الطالب` is construct and definite, while `كتاب طالب` is construct and indefinite; a possessive suffix also puts its host in construct state and makes it definite (`كتابه`). Free nouns are `state: free`.\n\
         5. Adjectives and participles: tag active and passive participles as Adjective even when predicative (`أنا عارف`, `هي جاية`, `هم مبسوطين`), choosing `active_participle` or `passive_participle`; use `elative` for the single أفعل-type form whether context makes it comparative or superlative. Report agreement_number. Report agreement_gender on a contrasting singular (`كبير` masculine versus `كبيرة` feminine), but OMIT it on an unmarked plural form such as `كبار`. A dual noun takes plural adjective agreement; an inanimate plural normally takes feminine singular agreement. Attributive adjectives copy definiteness; predicate adjectives are indefinite.\n\
         6. Verbs: use exactly `perfective` (`كتب`), `bare_imperfective` (`يكتب` after عايز/لازم/ممكن or another governor), `bi_imperfective` (`بيكتب` for the ordinary present/habitual), `ha_imperfective` (`حيكتب` for future), or `imperative` (`اكتب`). Do not invent MSA tense or indicative/subjunctive/jussive ending values: Egyptian lost those endings. Always give pattern, person, agreement_number, voice and polarity; verb agreement is singular/plural only, never dual and never a distinct feminine plural.\n\
         7. Verb gender and voice: set agreement_gender where the form contrasts it — `هو كتب` is masculine and `هي كتبت` feminine — but OMIT it for first person (`أنا كتبت`) and for plural forms (`هم كتبوا`), which do not encode a masculine/feminine contrast. Determine voice from the construction: `كتب` active, `اتكتب` passive, `اتحرك` middle_reflexive. Do not label every Form V, VII or VIII verb passive merely from its pattern, and do not import an MSA internal-vowel passive reading that Egyptian does not productively express.\n\
         8. Determiners: classify the attached article `الـ`, attributive demonstratives, quantifiers and interrogatives. Demonstratives encode referent_number (`ده` singular versus `دول` plural), while the article `الـ` encodes none and MUST omit it. Singular demonstratives encode referent_gender (`ده` masculine versus `دي` feminine); OMIT it on gender-neutral plural `دول`, on `الـ`, and on any determiner whose form carries no gender contrast.\n\
         9. Pronouns: `clitic` is true only for an attached pronoun. For `كتابه`, attachment_function is `possessive`; for `شفته`, it is `direct_object`; contrast `كتب له`/an attached indirect-object sequence with a preposition-hosted suffix such as `معاه`. OMIT attachment_function on independent `هو`. Set referent_person for personal forms (`أنا` first versus `إنت` second) and OMIT it for `اللي`; set referent_number for `أنا` singular versus `إحنا` plural and OMIT it for `مين`; set referent_gender for `هو` masculine versus `هي` feminine and OMIT it for `أنا`, `إحنا`, `هم`, `اللي` and forms without a gender contrast. Egyptian has no dual pronoun.\n\
         10. Negation and clitics: verbal negation is commonly the circumfix `ما ... ش` (`ماكتبش`, `مابيكتبش`); non-verbal predicates, participles and many future clauses use `مش`. Mark the governed verb negative even when the negative pieces are split. Do not rewrite either construction as MSA `لم/لن/ليس`. Distinguish aspectual `بـ` from prepositional `بـ`, and future `حـ/هـ` from an ordinary consonant.\n\
         11. Tokenization: split productive clitics into their own analyses — conjunctions `و-/ف-`, prepositions `ب-/ل-`, the article `ال-`, imperfective `ب-`, future `ح-/هـ-`, negative `ما-/-ش`, and attached pronouns — while retaining their syntactic effect on the host. Do NOT split person/number/gender inflection (`يـ/تـ/أ-/نـ`, `-ت`, `-ي`, `-وا`). Omit punctuation.\n\
         12. Pattern guardrails: classify the actual Egyptian pattern, not an MSA spelling. `form_vii_in` is inherited `انفعل`; `form_vii_it` is Egyptian `اتفعل`; Forms IV and the less productive patterns are used only for established Egyptian lexemes, never because an MSA cognate has that form. Quadriliteral verbs use only `quadriliteral_i` or `quadriliteral_ii`.\n\
         13. VALUE RULES: noun `number` is exactly singular/dual/plural; `agreement_number` and referent_number are only singular/plural. Gender values are only masculine/feminine and must never appear in a number field. Never emit `case`, `mood`, MSA nunation, dual verb agreement, or feminine-plural verb agreement."
    }
}

#[cfg(test)]
mod tests {
    use panini_core::pivot::PivotValueKind;

    use super::*;

    #[test]
    fn egyptian_identity_and_scripts_are_exact() {
        let language = EgyptianArabic;

        assert_eq!(EgyptianArabic::ISO_LANG, IsoLang::Arz);
        assert_eq!(
            language.supported_scripts(),
            &[Script::ARAB, Script::LATN]
        );
        assert_eq!(language.default_script(), Script::ARAB);
    }

    #[test]
    fn optional_root_remains_an_open_pivot() {
        let verb = EgyptianArabicMorphology::Verb {
            lemma: "كتب".to_string(),
            root: Some("ك-ت-ب".to_string()),
            pattern: EgyptianArabicVerbPattern::FormI,
            verb_form: EgyptianArabicVerbForm::BiImperfective,
            voice: EgyptianArabicVoice::Active,
            polarity: EgyptianArabicPolarity::Affirmative,
            person: Person::Third,
            agreement_number: BinaryNumber::Singular,
            agreement_gender: Some(BinaryGender::Masculine),
        };

        assert_eq!(
            EgyptianArabicMorphology::PIVOT_ROOT.value_kind,
            PivotValueKind::Open
        );
        assert_eq!(
            EgyptianArabicMorphology::PIVOT_ROOT.value(&verb),
            Some("ك-ت-ب".to_string())
        );
    }

    #[test]
    fn verb_form_inventory_is_dialect_specific() {
        assert_eq!(
            EgyptianArabicMorphology::PIVOT_VERB_FORM.values(),
            &[
                "perfective",
                "bare_imperfective",
                "bi_imperfective",
                "ha_imperfective",
                "imperative",
            ]
        );
    }

    #[test]
    fn conjugation_and_nominal_inflection_are_enabled() {
        assert_eq!(
            EgyptianArabic.typological_features(),
            &[
                TypologicalFeature::Conjugation(&[Upos::Verb]),
                TypologicalFeature::Declension(&[Upos::Noun, Upos::Adjective]),
            ]
        );
    }

    #[test]
    fn demonstrative_features_are_extracted_without_forcing_them_on_the_article() {
        let plural_demonstrative = EgyptianArabicMorphology::Determiner {
            lemma: "دول".to_string(),
            determiner_type: EgyptianArabicDeterminerType::Demonstrative,
            referent_number: Some(BinaryNumber::Plural),
            referent_gender: None,
        };

        assert_eq!(
            EgyptianArabicMorphology::PIVOT_REFERENT_NUMBER.value(&plural_demonstrative),
            Some("plural".to_string())
        );
        assert_eq!(
            EgyptianArabicMorphology::PIVOT_REFERENT_GENDER.value(&plural_demonstrative),
            None
        );
    }
}
