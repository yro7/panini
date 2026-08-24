use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, BinaryVoice, IsoLang, LinguisticDefinition, Person, Script, SlavicAspect,
    TernaryGender, TypologicalFeature, Upos,
};

/// The six productive cases of modern Russian.
///
/// Six, not seven: the vocative remnants (Бо́же, отче) and the "second locative"
/// (в лесу́, на краю́) are lexically restricted survivals, not productive cells of
/// the paradigm, so they are reported as the case their syntax calls for rather
/// than given a slot of their own.
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
pub enum RussianCase {
    Nominative,    // именительный
    Genitive,      // родительный
    Dative,        // дательный
    Accusative,    // винительный
    Instrumental,  // творительный
    Prepositional, // предложный
}

/// Animacy (одушевлённость) — a morphosyntactic category in its own right.
///
/// Kept orthogonal to gender rather than folded into it the way Polish folds it,
/// because in Russian animacy cuts across all three genders: it decides the
/// accusative of masculine singulars (вижу брата = genitive form) and of every
/// gender in the plural (вижу сестёр), while leaving the rest of the paradigm
/// untouched.
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
pub enum RussianAnimacy {
    Animate,   // одушевлённое
    Inanimate, // неодушевлённое
}

/// Tense of a finite verb or a participle.
///
/// Three cells, but they are distributed by aspect: an imperfective has all
/// three (писа́л / пишу́ / бу́ду писа́ть), a perfective only past and future
/// (написа́л / напишу́) — a perfective "present" form is a future.
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
pub enum RussianTense {
    Past,    // прошедшее
    Present, // настоящее
    Future,  // будущее
}

/// Mood of a finite verb (наклонение).
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
pub enum RussianMood {
    Indicative,  // изъявительное
    Imperative,  // повелительное
    Subjunctive, // сослагательное (past form + бы)
}

/// Which slot of the verbal system a verb token occupies.
///
/// Required on every verb: Russian's non-finite forms are not a fringe, they are
/// where the case system and the verb system meet — participles decline like
/// adjectives, verbal adverbs decline for nothing at all — and which fields
/// apply follows entirely from this value.
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
pub enum RussianVerbForm {
    Finite,       // спрягаемая форма
    Infinitive,   // инфинитив
    Participle,   // причастие
    VerbalAdverb, // деепричастие
}

/// Long vs short form — shared by adjectives and by participles, which is why
/// it is not spelled "adjective form": написан is the short form of a passive
/// participle by exactly the same mechanism that makes краси́в the short form of
/// краси́вый.
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
pub enum RussianAdjectivalForm {
    Long,  // полная форма (declines)
    Short, // краткая форма (predicative only)
}

/// Degree of comparison (степень сравнения), for adjectives and adverbs.
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
pub enum RussianDegree {
    Positive,    // положительная
    Comparative, // сравнительная
    Superlative, // превосходная
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
pub enum RussianMorphology {
    /// Adjective — agrees in gender, number and case in the long form.
    Adjective {
        lemma: String,
        /// Gender; the plural does not distinguish it, and neither does the
        /// simple comparative.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Number; absent on the indeclinable simple comparative.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: RussianCase,
        degree: RussianDegree,
        /// Long or short; absent on the simple comparative, which is neither.
        #[serde(skip_serializing_if = "Option::is_none")]
        form: Option<RussianAdjectivalForm>,
    },
    /// Preposition, with the case it governs in this instance.
    Adposition {
        lemma: String,
        /// The grammatical case this preposition governs here — в and на govern
        /// two, and only the instance settles which.
        case: RussianCase,
    },
    /// Adverb
    Adverb {
        lemma: String,
        degree: RussianDegree,
    },
    /// Coordinating conjunction
    CoordinatingConjunction {
        lemma: String,
    },
    /// Determiner — demonstratives, possessives and quantifiers, which decline
    /// on the adjectival pattern.
    Determiner {
        lemma: String,
        /// Gender; the plural does not distinguish it.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        number: BinaryNumber,
        case: RussianCase,
    },
    /// Interjection
    Interjection {
        lemma: String,
    },
    /// Noun
    Noun {
        lemma: String,
        /// Inherent and lexical — reported in the plural too, where the ending
        /// no longer shows it.
        gender: TernaryGender,
        animacy: RussianAnimacy,
        number: BinaryNumber,
        case: RussianCase,
    },
    /// Numeral — declines, and governs the case of what it counts.
    Numeral {
        lemma: String,
        /// Only оди́н, два/две and the ordinals distinguish gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Only оди́н and the ordinals distinguish number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: RussianCase,
    },
    /// Particle — не, бы, ли, же, -то, вот.
    Particle {
        lemma: String,
    },
    /// Pronoun
    Pronoun {
        lemma: String,
        /// Personal pronouns only; кто, что, себя and the relatives have none.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Third person singular only; я, ты, мы, вы, они do not show gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Absent on себя, кто and что, which stand outside number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: RussianCase,
    },
    /// Proper noun — declines exactly like a common noun (Москва → Москвы).
    ProperNoun {
        lemma: String,
        gender: TernaryGender,
        animacy: RussianAnimacy,
        number: BinaryNumber,
        case: RussianCase,
    },
    /// Subordinating conjunction
    SubordinatingConjunction {
        lemma: String,
    },
    /// Symbol
    Symbol {
        lemma: String,
    },
    /// Verb — finite forms, infinitives, participles and verbal adverbs.
    ///
    /// Aspect and voice hold for every one of them; everything else is decided
    /// by `verb_form`, and each `Option` below marks a cell Russian genuinely
    /// does not have rather than one the model might not know.
    Verb {
        lemma: String,
        /// Perfective or imperfective — the axis the whole verbal system turns
        /// on, and never a property of the token alone: писать and написать are
        /// two lemmas, not two forms of one.
        aspect: SlavicAspect,
        voice: BinaryVoice,
        verb_form: RussianVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<RussianMood>,
        /// Finite indicative forms and participles; the infinitive, the
        /// imperative, the subjunctive and the verbal adverb have no tense.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<RussianTense>,
        /// Present and future finite forms, and the imperative. The Russian past
        /// has no person at all — it is an old participle and agrees in gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms and participles; never the infinitive or verbal adverb.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Past-tense and subjunctive singulars, and singular participles —
        /// exactly where the past agrees in gender instead of person.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Long-form participles only.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<RussianCase>,
        /// Participles only: long (declining) or short (predicative).
        #[serde(skip_serializing_if = "Option::is_none")]
        form: Option<RussianAdjectivalForm>,
    },
    /// Other, for unanalyzable tokens
    Other {
        lemma: String,
    },
}

impl RussianMorphology {
    /// Extracts the tense value for the tense pivot.
    ///
    /// `tense` is `Option` on the verb (the infinitive, the imperative, the
    /// subjunctive and the verbal adverb have none), so the `MorphologyInfo`
    /// derive skips it for pivot generation. This hand-written handle keeps
    /// `PIVOT_TENSE` available for lexicon faceting, yielding `None` when no
    /// tense was extracted.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|t| panini_core::aggregable::ClosedValues::variant_str(t).to_string()),
            _ => None,
        }
    }

    /// Extracts the long/short value for the form pivot, from adjectives and
    /// participles alike.
    ///
    /// `form` is `Option` in both variants that carry it, so the derive skips
    /// it. Written by hand because the long/short split is one of the dimensions
    /// a learner most wants to drill on, and because a derived handle would have
    /// seen only one of the two parts of speech that share it.
    fn __pivot_form(&self) -> Option<String> {
        match self {
            Self::Adjective { form, .. } | Self::Verb { form, .. } => form
                .as_ref()
                .map(|f| panini_core::aggregable::ClosedValues::variant_str(f).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for verb tense. Defined manually because `tense` is
    /// optional (see [`RussianMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <RussianTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// Typed pivot handle for the long/short form. Defined manually because
    /// `form` is optional (see [`RussianMorphology::__pivot_form`]).
    pub const PIVOT_FORM: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "form",
            "Form",
            <RussianAdjectivalForm as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_form,
        );
}

pub struct Russian;

impl LinguisticDefinition for Russian {
    type Morphology = RussianMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Rus;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        RussianMorphology::PIVOT_CASE,
        RussianMorphology::PIVOT_ASPECT,
        RussianMorphology::PIVOT_GENDER,
        RussianMorphology::PIVOT_ANIMACY,
        RussianMorphology::PIVOT_NUMBER,
        RussianMorphology::PIVOT_TENSE,
        RussianMorphology::PIVOT_VERB_FORM,
        RussianMorphology::PIVOT_FORM,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::CYRL]
    }

    fn default_script(&self) -> Script {
        Script::CYRL
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
        "1. Lemmatization: verbs to the infinitive OF THEIR OWN ASPECT (пишет → писать, напишет → написать); \
         nouns to the nominative singular (столе → стол), pluralia tantum staying plural (ножницы, деньги); \
         adjectives to the masculine nominative singular LONG form (красива → красивый, лучше → хороший); \
         participles and verbal adverbs to the infinitive of the verb they are built on, same aspect \
         (прочитанный → прочитать, читая → читать); pronouns and determiners to the nominative \
         (ему → он, моей → мой).\n\
         2. Aspect pairs are DIFFERENT LEXEMES, not forms of one verb. Never lemmatize a perfective to \
         its imperfective partner or the reverse: писать/написать, читать/прочитать, \
         говорить/сказать, брать/взять are eight lemmas, not four. Report the aspect the form actually \
         carries. For the handful of biaspectual verbs (использовать, велеть, ранить, женить), report the \
         aspect the context realizes.\n\
         3. Case is SYNTACTIC, never inferred from the ending. Animate accusatives are identical to \
         genitives (вижу брата, вижу студентов, вижу сестёр = accusative, NOT genitive), and inanimate \
         masculine accusatives are identical to nominatives (вижу стол = accusative). Report accusative \
         whenever the word is a direct object, whatever the ending looks like. The same discipline applies \
         to the other syncretisms: книги is genitive singular after нет and nominative plural as a subject; \
         feminine -е is dative after the verb (сестре) and prepositional after в/о (о сестре).\n\
         4. Nouns: always give gender, animacy, number and case. Gender is inherent and lexical — report \
         it for plurals too, where the ending no longer shows it (книги → feminine). Indeclinable nouns \
         (кофе, метро, пальто, Тбилиси) still take the case and number their syntax assigns. Proper nouns \
         decline: report their case exactly as for common nouns (в Москве → prepositional).\n\
         5. Prepositions: report the case governed IN THIS INSTANCE, not the preposition's whole range. \
         в столе is prepositional but в стол is accusative; за домом is instrumental but за дом is \
         accusative.\n\
         6. Verbs — which fields apply follows from 'verb_form', and each omission below is a cell \
         Russian does not have:\n\
         - finite present/future: mood indicative, tense, person and number; NO gender.\n\
         - finite past: tense past, number, and gender in the singular only. OMIT PERSON ENTIRELY — the \
         Russian past agrees in gender and number and has no person, so писал is masculine singular \
         whether the subject is я, ты or он.\n\
         - imperative: mood imperative, person and number; no tense.\n\
         - subjunctive (past form + бы): mood subjunctive, number, gender in the singular; no tense, no \
         person.\n\
         - infinitive: aspect and voice only; no tense, person, number, gender or case.\n\
         - participle: tense (present or past), voice, number, gender in the singular, plus form; long \
         participles also take case, short passive participles (написан, прочитана) take form short and no \
         case.\n\
         - verbal adverb (деепричастие): aspect and voice only. читая vs прочитав is an aspect contrast, \
         not a tense one — omit tense, person, number, gender and case.\n\
         7. The compound future is two tokens: буду / будешь is a finite imperfective future form of быть, \
         and the following infinitive is a separate verb token. A perfective present-form ending is a \
         FUTURE (напишу → tense future, aspect perfective). The present-tense copula быть is unwritten in \
         Russian — never emit a token for it.\n\
         8. Voice: -ся verbs are reflexive or middle, not passive. смеяться, учиться, находиться and \
         казаться are ACTIVE. Report passive only for a genuine passive: an imperfective -ся verb with an \
         instrumental agent (дом строится рабочими) or a passive participle (написан, построенный).\n\
         9. Adjectives: the long form declines and takes case, number, and gender in the singular. The \
         short form (красив, готова, рады) is predicative and survives only as the nominative — report \
         form short, its gender and number, and case nominative. The simple comparative (быстрее, лучше) \
         is indeclinable: report degree comparative, omit form, omit gender and number, and report case \
         nominative. The compound comparative (более быстрый) and the superlative (самый быстрый, \
         красивейший) are ordinary long forms and decline.\n\
         10. Orthography: lemmas use the conventional dictionary spelling with ё where the dictionary has \
         it (ёж, пёс, идёт → идти), even when the source text prints е for it. все (all) and всё \
         (everything) are DIFFERENT lemmas — never merge them. Never carry stress marks into any field: \
         if the input is textbook text with acute accents (ко́шка, хорошо́), strip them before lemmatizing, \
         and never add them yourself.\n\
         11. Tokenization: hyphenated forms are single tokens (кто-то, что-нибудь, по-русски, из-за, \
         кое-как). не and бы are separate particle tokens, never fused with the verb they attach to."
    }
}
