//! Ukrainian (`ukr`) — the value space of *сучасна українська літературна мова*,
//! the modern standard written language.
//!
//! Ukrainian sits next to Russian and Czech in this crate, and the three
//! decisions that shape the file below are best read as choices *against* those
//! two neighbours rather than as defaults:
//!
//! - **Animacy is a field of its own, beside gender, not folded into it.** Czech
//!   folds it in because there animacy splits the masculine and nothing else.
//!   Ukrainian is the Russian case: animacy decides the accusative of masculine
//!   singulars (бачу брата, genitive-shaped, against бачу стіл, nominative-shaped)
//!   **and** the accusative plural of every gender (бачу сестер, бачу коней
//!   against бачу столи, бачу книги). A category that cuts across all three
//!   genders is not a gender value, so gender stays
//!   [`panini_core::traits::TernaryGender`] and animacy stands beside it.
//! - **The past tense is a finite form, not a participle plus an auxiliary.**
//!   Czech's past is two tokens (psal + jsem) and its *l*-form is therefore a
//!   participle. Ukrainian lost the enclitic auxiliary: писав, писала, писали are
//!   whole finite verbs that happen to agree in gender and number instead of in
//!   person. So `verb_form: finite` with `person` omitted, exactly as in Russian.
//! - **Negation is a separate token.** Czech prefixes ne- to the verb and needs a
//!   `polarity` field to keep the information; Ukrainian writes не apart (не пишу,
//!   не був), so the negation is already a `Particle` token and a polarity field
//!   would duplicate it.
//!
//! Two things Ukrainian has that this file deliberately does **not** model:
//!
//! - **The hard / soft / mixed stem groups of the noun** (*тверда, м'яка, мішана
//!   групи*). They are real and they decide the exact ending, but they only split
//!   the first and second declensions — the third and fourth have no group at all
//!   — so the field would be optional on the majority of nouns while adding a
//!   sixth required judgement to every one of them. The four declensions
//!   (`declension`) carry the part of the same information a learner navigates by;
//!   the group is left to the generation side, where the endings are actually
//!   spelled. Adjectives keep their own hard/soft split (`stem_group`), because
//!   for them it is a clean binary read straight off the citation form (новий
//!   against синій) and it is the whole story of their paradigm.
//! - **The pluperfect** (*давноминулий час*: був написав, ходив був). It is two
//!   tokens, each of them already a past-tense verb in its own right, so a
//!   `pluperfect` value would be a claim about a neighbouring word rather than a
//!   feature of the token being analysed.

use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, BinaryVoice, IsoLang, LinguisticDefinition, Person, Script, SlavicAspect,
    TernaryGender, TypologicalFeature, Upos,
};

/// The seven cases of Ukrainian, in the order Ukrainian schools number them.
///
/// Seven, and the vocative is a full member of the paradigm rather than a
/// survival: unlike Russian, where Бо́же and о́тче are frozen relics reported as
/// the case their syntax calls for, Ukrainian forms the *кличний відмінок*
/// productively on any masculine or feminine noun (Петро → Петре, Ганна → Ганно,
/// пан Коваль → пане Ковалю, друг → друже) and requires it in direct address.
/// Ukrainian is Polish and Czech here, not Russian.
///
/// The sixth case is the **locative** (*місцевий*), not the "prepositional". The
/// name is not decoration: it is the case that never occurs without a
/// preposition, and that is the rule which separates it from the dative.
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
pub enum UkrainianCase {
    Nominative,   // називний (хто? що?)
    Genitive,     // родовий (кого? чого?)
    Dative,       // давальний (кому? чому?)
    Accusative,   // знахідний (кого? що?)
    Instrumental, // орудний (ким? чим?)
    Locative,     // місцевий (на кому? на чому?)
    Vocative,     // кличний (звертання)
}

/// Animacy (*істота / неістота*) — a morphosyntactic category in its own right.
///
/// Kept orthogonal to gender rather than folded into it the way Czech folds it,
/// because in Ukrainian animacy cuts across all three genders. It decides the
/// accusative of masculine singulars (бачу брата = the genitive form) and the
/// accusative plural of every gender without exception (бачу сестер, бачу коней,
/// бачу дітей), while leaving the rest of the paradigm untouched.
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
pub enum UkrainianAnimacy {
    Animate,   // істота — людина, кінь, дитина
    Inanimate, // неістота — стіл, книга, місто
}

/// The four declensions of the Ukrainian noun (*відміни*).
///
/// Modelled where Czech's fourteen *vzory* were not, and the difference is that
/// these four are readable off the citation form together with the gender rather
/// than being a lexical lookup: a noun in -а/-я is first, a masculine with a zero
/// ending or -о and a neuter in -о/-е/-я is second, a feminine with a zero ending
/// is third, and a neuter that grows -ат-/-ят-/-ен- in the oblique cases is
/// fourth. Every Ukrainian course organises nominal morphology by them, which
/// makes them the facet a learner navigates the lexicon with.
///
/// The last two values are the nouns that stand outside the system entirely,
/// and they are here so that those nouns are not silently filed under a
/// declension they do not belong to.
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
pub enum UkrainianDeclension {
    // I відміна — nouns in -а/-я of any gender: жінка, земля, Микола, суддя.
    #[serde(rename = "first_declension")]
    First,
    // II відміна — masculines with a zero ending or -о (стіл, кінь, батько,
    // Дніпро) and neuters in -о/-е/-я (село, поле, життя, обличчя).
    #[serde(rename = "second_declension")]
    Second,
    // III відміна — feminines with a zero ending: ніч, сіль, радість, любов,
    // plus мати.
    #[serde(rename = "third_declension")]
    Third,
    // IV відміна — neuters in -а/-я that grow a suffix in the oblique cases:
    // теля → теляти, курча → курчати, ім'я → імені, плем'я → племені.
    #[serde(rename = "fourth_declension")]
    Fourth,
    // Незмінювані — borrowings that take no ending at all: кіно, таксі, метро,
    // журі, Токіо, Гюго. The case and number reported are the ones their syntax
    // assigns.
    Indeclinable,
    // Множинні іменники — the plural-only nouns, which decline on a pattern of
    // their own and belong to none of the four: двері, ножиці, окуляри,
    // канікули, гроші, Карпати.
    PluraleTantum,
}

/// Which of the two adjectival paradigms a form's endings come from.
///
/// The hard/soft split is the first thing a Ukrainian course teaches about
/// adjectives because it decides the whole table, and unlike the noun groups it
/// is a clean binary read straight off the citation form: -ий is hard (новий,
/// добрий, великий), -ій is soft (синій, вечірній, ранній, безкраїй). Ordinals
/// and long participles follow the same split.
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
pub enum UkrainianStemGroup {
    // Тверда група — новий, нова, нове, нового, новим…
    Hard,
    // М'яка група — синій, синя, синє, синього, синім…
    Soft,
    // Незмінювані — the endingless borrowings that stand outside both groups:
    // бордо, хакі, максі, беж. The gender, number and case reported are the
    // ones the noun assigns.
    Indeclinable,
}

/// Tense of a finite verb or of a participle.
///
/// Three cells, distributed by aspect as everywhere in Slavic: an imperfective
/// has all three (писав / пишу / писатиму), a perfective only past and future
/// (написав / напишу) — a perfective present-shaped form *is* a future.
///
/// The pluperfect (був написав) gets no value here: see the module doc.
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
pub enum UkrainianTense {
    Past,    // минулий час
    Present, // теперішній час
    Future,  // майбутній час
}

/// Mood of a finite verb (*спосіб*).
///
/// The conditional is a mood and not a tense: Ukrainian builds it from the same
/// form the past tense uses plus the invariable particle би/б, so it contrasts
/// with the indicative on the axis the imperative does, not with present against
/// future. Unlike Czech, the particle conjugates for nothing, so the mood is
/// carried by the verb token and би/б is analysed as the particle it is.
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
pub enum UkrainianMood {
    Indicative,  // дійсний спосіб
    Imperative,  // наказовий спосіб
    Conditional, // умовний спосіб (past form + би/б)
}

/// Which slot of the verbal system a verb token occupies.
///
/// Required on every verb, because every other verbal field follows from it.
/// Ukrainian school grammar calls the participle and the adverbial participle
/// *особливі форми дієслова* — special forms of the verb, not separate parts of
/// speech — which is why both live here rather than under `Adjective` and
/// `Adverb`.
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
pub enum UkrainianVerbForm {
    // Дієвідмінювана форма — a conjugated form, including the past: пишу,
    // писатиму, буду, пиши, писав, писала б.
    Finite,
    // Інфінітив (неозначена форма) — писати, написати, бути.
    Infinitive,
    // Дієприкметник — a participle, which declines like an adjective: активний
    // (почорнілий, квітучий) or пасивний (написаний, розбитий, вимита).
    // Ukrainian has no short participle; the -но/-то form took over that slot.
    Participle,
    // Дієприслівник — пишучи, читаючи, написавши, прочитавши. Invariable, and
    // the aspect alone carries the simultaneous/anterior contrast.
    AdverbialParticiple,
    // Безособова форма на -но, -то — написано, зроблено, вбито, здобуто.
    // Wholly invariable and the most distinctively Ukrainian cell of the whole
    // system: *Роботу виконано* keeps its patient in the accusative, so it is a
    // predicate with no subject at all rather than a passive clause. It takes
    // no tense of its own — було зроблено and буде зроблено shift the time on
    // the auxiliary.
    Impersonal,
}

/// How a future-tense form is built (*форми майбутнього часу*).
///
/// Ukrainian grammars teach three, and the three-way contrast is the single
/// feature that most sharply separates Ukrainian verbal morphology from Russian:
/// no other Slavic standard has the synthetic future at all.
///
/// Only future-tense finite forms carry this, so the field is optional and its
/// pivot handle is written by hand.
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
pub enum UkrainianFutureFormation {
    // Проста форма — a perfective conjugated on the present pattern: напишу,
    // зроблю, прочитаєш.
    #[serde(rename = "simple_future")]
    Simple,
    // Складна форма — the infinitive fused with the descendants of яти:
    // писатиму, писатимеш, читатиме, робитимуть. Imperfective only.
    #[serde(rename = "synthetic_future")]
    Synthetic,
    // Складена форма — буду / будеш / буде + infinitive. The auxiliary carries
    // this value; the infinitive beside it is an ordinary infinitive token.
    #[serde(rename = "analytic_future")]
    Analytic,
}

/// Degree of comparison (*ступінь порівняння*), for adjectives and adverbs alike.
///
/// Ukrainian, unlike Russian, has no indeclinable simple comparative on the
/// adjective: вищий, кращий and новіший decline exactly like any other soft
/// adjective, and it is the *adverb* that has the invariable вище, краще,
/// швидше.
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
pub enum UkrainianDegree {
    Positive,    // звичайний ступінь — високий, швидко
    Comparative, // вищий ступінь — вищий, швидше
    Superlative, // найвищий ступінь — найвищий, найшвидше
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
pub enum UkrainianMorphology {
    /// Adjective — including ordinals (перший, другий), which inflect on the
    /// adjectival pattern and are listed as adjectives by Ukrainian dictionaries.
    ///
    /// Participles are **not** here: Ukrainian grammar files написаний and
    /// почорнілий under the verb (see [`UkrainianVerbForm::Participle`]).
    Adjective {
        lemma: String,
        degree: UkrainianDegree,
        /// Hard or soft — the paradigm this form's ending comes from.
        stem_group: UkrainianStemGroup,
        /// Absent in the plural, which levels all three genders into one form
        /// (нові, синіх), and absent on the indeclinables.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        number: BinaryNumber,
        case: UkrainianCase,
    },
    /// Preposition, with the case it governs **in this instance**.
    ///
    /// Required, because the commonest Ukrainian prepositions govern two or
    /// three cases and only the occurrence settles which: на столі is locative
    /// and на стіл accusative, за домом instrumental and за дім accusative,
    /// з Києва genitive and з другом instrumental.
    Adposition {
        lemma: String,
        case: UkrainianCase,
    },
    /// Adverb.
    ///
    /// `degree` is required rather than optional: Ukrainian grades adverbs
    /// productively (швидко → швидше → найшвидше, добре → краще → найкраще), and
    /// `positive` is the unmarked base a non-gradable adverb (тут, учора, дуже)
    /// stands in.
    Adverb {
        lemma: String,
        degree: UkrainianDegree,
    },
    /// Coordinating conjunction — і/й, та, а, але, або, проте, зате.
    CoordinatingConjunction {
        lemma: String,
    },
    /// Determiner — demonstratives (цей, той), possessives (мій, твій, наш,
    /// свій) and quantifiers (кожен, весь, жодний, який, котрий, деякий) used
    /// adnominally. Ukrainian has no articles, so this class is small and
    /// entirely declining.
    Determiner {
        lemma: String,
        /// Absent in the plural: the Ukrainian plural levels gender (ці, ті,
        /// мої), unlike the Czech one.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        number: BinaryNumber,
        case: UkrainianCase,
    },
    /// Interjection — ой, ех, гей, ну, отакої.
    Interjection {
        lemma: String,
    },
    /// Noun.
    Noun {
        lemma: String,
        /// Inherent and lexical, and reported in the plural too, where the
        /// ending alone no longer settles it (книги could be feminine genitive
        /// singular or feminine nominative plural).
        gender: TernaryGender,
        animacy: UkrainianAnimacy,
        /// Which of the four declensions the noun belongs to, or the bucket for
        /// the nouns that stand outside them.
        declension: UkrainianDeclension,
        number: BinaryNumber,
        case: UkrainianCase,
    },
    /// Cardinal numeral — один, два, п'ять, сто, тисяча — and the collectives
    /// (двоє, троє, четверо). Ordinals are adjectives.
    Numeral {
        lemma: String,
        /// Only один (один/одна/одне) and два (два/дві), обидва/обидві
        /// distinguish gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Only один has a number contrast of its own (один / одні).
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Required: Ukrainian cardinals decline (п'ять → п'яти / п'ятьох) and
        /// govern the case of what they count.
        case: UkrainianCase,
    },
    /// Particle — не, б/би, ж/же, хай/нехай, ось, он, лише, тільки, -но, -то.
    Particle {
        lemma: String,
    },
    /// Pronoun — personal, reflexive, possessive, demonstrative, interrogative,
    /// relative, indefinite and negative, used pronominally.
    Pronoun {
        lemma: String,
        /// Personal and possessive pronouns only; хто, що, себе, який and котрий
        /// stand outside person.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Third-person singular forms and the pronominal demonstratives; я, ти,
        /// ми, ви, себе, хто and що do not show it.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Absent on себе, хто and що, which stand outside number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: UkrainianCase,
    },
    /// Proper noun — declines exactly like a common noun (Київ → у Києві,
    /// Ганна → Ганно, Петро → Петре, Шевченко → Шевченка).
    ProperNoun {
        lemma: String,
        gender: TernaryGender,
        animacy: UkrainianAnimacy,
        declension: UkrainianDeclension,
        number: BinaryNumber,
        case: UkrainianCase,
    },
    /// Subordinating conjunction — що, щоб, коли, якщо, бо, тому що, хоча.
    SubordinatingConjunction {
        lemma: String,
    },
    /// Symbol.
    Symbol {
        lemma: String,
    },
    /// Verb.
    ///
    /// `aspect` and `voice` hold of the token whatever slot it occupies;
    /// everything else follows from `verb_form`, and each `Option` below marks a
    /// cell Ukrainian genuinely does not have rather than one the model might
    /// not know. The two that matter most: the past is **finite and has no
    /// person** (писав is masculine singular whether the subject is я, ти or
    /// він), and the impersonal -но/-то form has none of these fields at all.
    Verb {
        lemma: String,
        /// Perfective or imperfective — a property of the lemma, never of the
        /// form: писати and написати are two verbs, not two shapes of one.
        aspect: SlavicAspect,
        voice: BinaryVoice,
        verb_form: UkrainianVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<UkrainianMood>,
        /// Finite indicative forms and participles. The infinitive, the
        /// imperative, the conditional, the adverbial participle and the
        /// impersonal -но/-то form have none.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<UkrainianTense>,
        /// Future finite forms only — which of the three Ukrainian futures this
        /// token realizes.
        #[serde(skip_serializing_if = "Option::is_none")]
        future_formation: Option<UkrainianFutureFormation>,
        /// Present and future finite forms, and the imperative. The past and the
        /// conditional agree in gender and number instead and have no person at
        /// all.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms and participles; never the infinitive, the adverbial
        /// participle or the impersonal form.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Past and conditional singulars, and singular participles — exactly
        /// where the form agrees in gender instead of person. The plural levels
        /// gender (писали, написані).
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Participles only: they decline like adjectives.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<UkrainianCase>,
    },
    /// Other, for unanalyzable tokens.
    Other {
        lemma: String,
    },
}

impl UkrainianMorphology {
    /// Extracts the tense value for the tense pivot.
    ///
    /// `tense` is `Option` on the verb (the infinitive, the imperative, the
    /// conditional, the adverbial participle and the impersonal form have none),
    /// so the `MorphologyInfo` derive skips it for pivot generation. This
    /// hand-written handle keeps `PIVOT_TENSE` available for lexicon faceting,
    /// yielding `None` when no tense was extracted.
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

    /// Extracts the mood value for the mood pivot.
    ///
    /// `mood` is `Option` on the verb (only finite forms have one), so the derive
    /// skips it. Written by hand because the indicative / imperative /
    /// conditional contrast is one of the dimensions a learner most wants to
    /// drill.
    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood
                .as_ref()
                .map(|m| panini_core::aggregable::ClosedValues::variant_str(m).to_string()),
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

    /// Extracts which of the three futures a form realizes.
    ///
    /// `future_formation` is `Option` (only a future-tense finite form has one),
    /// so the derive skips it. Written by hand because the simple / synthetic /
    /// analytic contrast is the most distinctively Ukrainian thing in the verb
    /// and the facet a learner would reach for first — писатиму against буду
    /// писати is the choice the language is recognised by.
    fn __pivot_future_formation(&self) -> Option<String> {
        match self {
            Self::Verb {
                future_formation, ..
            } => future_formation
                .as_ref()
                .map(|f| panini_core::aggregable::ClosedValues::variant_str(f).to_string()),
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

    /// Typed pivot handle for verb tense. Defined manually because `tense` is
    /// optional (see [`UkrainianMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <UkrainianTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// Typed pivot handle for verb mood. Defined manually because `mood` is
    /// optional (see [`UkrainianMorphology::__pivot_mood`]).
    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <UkrainianMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    /// Typed pivot handle for the three futures. Defined manually because
    /// `future_formation` is optional (see
    /// [`UkrainianMorphology::__pivot_future_formation`]).
    pub const PIVOT_FUTURE_FORMATION: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "future_formation",
            "Future Formation",
            <UkrainianFutureFormation as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_future_formation,
        );
}

pub struct Ukrainian;

impl LinguisticDefinition for Ukrainian {
    type Morphology = UkrainianMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Ukr;
    /// Twelve facets, curated from the thirteen handles available.
    ///
    /// `voice` is deliberately absent: it is a real dimension of the verb, but
    /// running text is active almost throughout, so as a lexicon facet it would
    /// be one bucket holding everything and a second holding the participles.
    /// `person` is absent for the reason Russian's and Czech's are — it
    /// partitions the verb paradigm, not the vocabulary.
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        UkrainianMorphology::PIVOT_CASE,
        UkrainianMorphology::PIVOT_GENDER,
        UkrainianMorphology::PIVOT_ANIMACY,
        UkrainianMorphology::PIVOT_NUMBER,
        UkrainianMorphology::PIVOT_DECLENSION,
        UkrainianMorphology::PIVOT_STEM_GROUP,
        UkrainianMorphology::PIVOT_DEGREE,
        UkrainianMorphology::PIVOT_ASPECT,
        UkrainianMorphology::PIVOT_VERB_FORM,
        UkrainianMorphology::PIVOT_MOOD,
        UkrainianMorphology::PIVOT_TENSE,
        UkrainianMorphology::PIVOT_FUTURE_FORMATION,
    ];

    /// Ukrainian is written in Cyrillic alone. The Latin *latynka* is a live
    /// proposal and an old philological tradition, but it has never been an
    /// official or in-use writing system, so it is not declared here — the same
    /// call Turkish's Ottoman past gets.
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
        "1. Lemmatization: verbs to the infinitive OF THEIR OWN ASPECT (пише → писати, напише → \
         написати); nouns to the nominative singular (на столі → стіл, у Києві → Київ, на нозі → \
         нога), plural-only nouns staying plural (двері, ножиці, окуляри, канікули, гроші); \
         adjectives to the masculine nominative singular POSITIVE degree (найкращий → добрий, \
         вищої → високий, синіми → синій); adverbs to the positive (краще → добре, швидше → \
         швидко); participles and adverbial participles to the infinitive of the verb they are \
         built on, same aspect (написаного → написати, пишучи → писати, прочитавши → прочитати); \
         pronouns and determiners to the masculine nominative singular (йому → він, моєї → мій, \
         тією → той).\n\
         2. Lemmatize THROUGH the Ukrainian vowel alternations rather than stopping at the oblique \
         stem. This is the single commonest way to produce a wrong lemma, because the oblique stem \
         often looks Russian. In a closed syllable Ukrainian writes і where the open syllable has \
         о or е: стола → СТІЛ (never стол), коня → КІНЬ (never конь), печі → ПІЧ (never печь), \
         ночі → НІЧ, семи → СІМ, Києва → КИЇВ, Львова → ЛЬВІВ. Also lemmatize past the consonant \
         alternations before -і — руці → РУКА (к/ц), нозі → НОГА (г/з), мусі → МУХА (х/с) — and \
         past the vowel that drops out of the oblique stem: дня → ДЕНЬ, вітру → ВІТЕР, сну → СОН.\n\
         3. Aspect pairs are DIFFERENT LEXEMES, not two forms of one verb. Never lemmatize a \
         perfective to its imperfective partner or the reverse: писати/написати, робити/зробити, \
         читати/прочитати, брати/взяти, казати/сказати, давати/дати are twelve lemmas, not six. \
         Report the aspect the form actually carries. For the biaspectual verbs (телефонувати, \
         веліти, атакувати, організувати, гарантувати) report the aspect the context realizes.\n\
         4. Case is SYNTACTIC, never read off the ending. Ukrainian syncretism is everywhere: the \
         ANIMATE accusative is identical to the genitive in the singular masculine and in the \
         plural of EVERY gender (бачу брата, бачу коня, бачу сестер, бачу дітей = accusative, NOT \
         genitive), and the INANIMATE accusative is identical to the nominative (бачу стіл, бачу \
         столи, бачу книги = accusative). Report accusative whenever the word is a direct object, \
         whatever the ending looks like. Separate the dative from the locative by the preposition: \
         the LOCATIVE never occurs without one (на столі, у книзі, при школі), so a bare -і/-ю form \
         after a verb is a DATIVE (дав сестрі, допоміг братові) and the same form after на/у/в/по/при \
         is a locative. книги is genitive singular, nominative plural and accusative plural, \
         and only the syntax settles which.\n\
         5. The vocative is a full, live case and is REQUIRED in direct address — this is not \
         Russian, where it survives only in frozen forms. Report vocative for every noun, proper \
         noun, adjective and determiner inside an address: Петре!, Ганно!, друже!, пане Ковалю!, \
         добродію!, мамо!, Оксано Іванівно!, дорогі друзі!. Include the ones whose vocative is \
         identical to the nominative (пані!, місто!, ноче!) — never fall back to nominative inside \
         an address. In a name plus title or patronymic, EVERY part goes to the vocative (пане \
         директоре, Іване Петровичу).\n\
         6. Nouns: always give gender, animacy, declension, number and case. Gender is inherent and \
         lexical — report it for plurals too (книги → feminine). Pick the declension from the \
         NOMINATIVE SINGULAR plus the gender: first_declension for any noun in -а/-я whatever its gender \
         (жінка, земля, Микола, суддя, сирота); second_declension for a masculine with a zero ending or -о \
         (стіл, кінь, батько, Дніпро) and for a neuter in -о/-е/-я (село, поле, життя, обличчя); \
         third_declension for a feminine with a zero ending (ніч, сіль, радість, любов) and for мати; fourth_declension \
         ONLY for the neuters that grow -ат-/-ят-/-ен- in the oblique cases (теля → теляти, курча \
         → курчати, ім'я → імені, плем'я → племені). Use indeclinable for the endingless borrowings \
         (кіно, таксі, метро, журі, Токіо) and plurale_tantum for the plural-only nouns (двері, \
         ножиці, окуляри, канікули, гроші, Карпати) — those two still take the case and number \
         their syntax assigns. Common-gender nouns (сирота, староста, листоноша) take the gender the \
         context gives them.\n\
         7. Prepositions: report the case governed IN THIS INSTANCE, not the preposition's whole \
         range. на столі is locative but на стіл accusative; за домом instrumental but за дім \
         accusative; з Києва genitive but з другом instrumental; про, через, під, над, перед all \
         behave the same way.\n\
         8. Verbs — which fields apply follows from 'verb_form', and each omission below is a cell \
         Ukrainian does not have:\n\
         - finite present/future indicative: mood indicative, tense, person and number; NO gender. \
         A future form also takes future_formation (rule 9).\n\
         - finite past: mood indicative, tense past, number, and gender in the SINGULAR only. OMIT \
         PERSON ENTIRELY — the Ukrainian past has no person at all, so писав is masculine singular \
         whether the subject is я, ти or він, and писали is plural whether it is ми, ви or вони. \
         The plural levels gender: omit gender there.\n\
         - imperative: mood imperative, person and number (пиши 2sg, пишімо 1pl, пишіть 2pl); no \
         tense, no gender. The analytic imperative хай/нехай пише is a particle plus an ordinary \
         third-person PRESENT indicative — do not tag that verb imperative.\n\
         - conditional (писав би, зробила б, пішли б): mood conditional, number, gender in the \
         singular; NO tense and NO person. The particle б/би is a separate PARTICLE token.\n\
         - infinitive: aspect and voice only; no mood, tense, person, number, gender or case.\n\
         - participle (дієприкметник): verb_form participle, tense, voice, number, gender in the \
         singular, and CASE, because it declines like an adjective. Passive -ний/-тий forms \
         (написаний, розбите, вимиті) are voice passive and tense past; active -лий forms \
         (почорнілий, збіднілий) are voice active and tense past; active -учий/-ючий/-ачий forms \
         (квітучий, працюючий) are voice active and tense present. A participle is a VERB here, \
         not an adjective, even when it stands before a noun — analyse an adjective only where the \
         form has lexicalized away from its verb and a dictionary lists it separately (учений \
         'learned, scholar', вихований 'well-mannered', відомий 'well-known').\n\
         - adverbial participle (дієприслівник: пишучи, читаючи, написавши, прочитавши): aspect and \
         voice only. The imperfective/perfective contrast is what carries simultaneous against \
         anterior — omit tense, person, number, gender and case.\n\
         - impersonal -но/-то form (написано, зроблено, вбито, здобуто, прочитано): verb_form \
         impersonal, aspect, voice passive, and NOTHING ELSE — no mood, tense, person, number, \
         gender or case. It is completely invariable. Its patient stays in the ACCUSATIVE (Роботу \
         виконано, Книжку прочитано), so tag that noun accusative, not nominative. When було or \
         буде stands beside it (Було зроблено, Буде здобуто) that auxiliary is a separate finite \
         verb token, lemma бути, carrying the tense.\n\
         9. The three futures, and each is a different value of future_formation:\n\
         - simple_future (проста): a PERFECTIVE conjugated on the present pattern — напишу, зробиш, \
         прочитає. tense future, aspect perfective, future_formation simple_future. A perfective \
         present-shaped ending is ALWAYS a future, never a present.\n\
         - synthetic_future (складна): the imperfective infinitive fused with -му/-меш/-ме/-мемо/-мете/-муть \
         — писатиму, читатимеш, робитиме, ходитимуть. ONE token, tense future, aspect \
         imperfective, future_formation synthetic_future, with its own person and number. This form has no \
         Russian counterpart; do not mistake it for an infinitive.\n\
         - analytic_future (складена): буду / будеш / буде / будемо / будете / будуть + infinitive. TWO \
         tokens: the auxiliary is lemma бути, finite, tense future, future_formation analytic_future, with \
         person and number; the following infinitive is a separate verb token with no tense and no \
         future_formation.\n\
         Set future_formation ONLY on a finite future form. Omit it everywhere else.\n\
         10. Voice: verbs in -ся are reflexive, reciprocal or middle, NOT passive. умиватися, \
         вчитися, зустрічатися, здаватися, знаходитися are ACTIVE. Report passive only for a \
         genuine passive: a passive participle (написаний, розбитий), the impersonal -но/-то form, \
         or an imperfective -ся verb with an instrumental agent (Будинок будується робітниками). \
         Everything else, the infinitive and the adverbial participle included, is active.\n\
         11. Tokenization: -ся and -сь are FUSED to the verb and are part of it — умивається is ONE \
         token lemmatized to умиватися, never a verb plus a reflexive pronoun (that is Czech and \
         Polish, not Ukrainian). не, б, би, ж, же, хай, нехай, ось, он are separate PARTICLE tokens, \
         never fused with the word beside them. Hyphenated forms are single tokens (будь-хто, \
         хто-небудь, по-українськи, з-під, де-небудь, скажи-но, синьо-жовтий). The euphonic \
         alternants are the same word: lemmatize у → В, із/зі/зо → З, й → І, під/піді → ПІД, \
         над/наді → НАД, від/віді → ВІД, so that one word never gets two mastery records.\n\
         12. Numerals govern what they count, and the result is a real trap: after два, три, \
         чотири, обидва and обидві the counted noun is NOMINATIVE PLURAL (два столи, три книги, \
         чотири вікна), not genitive singular, even though the form can look like one; after п'ять and \
         every higher cardinal it is GENITIVE PLURAL (п'ять столів, десять книг). Report the \
         numeral's own case as its syntax assigns it.\n\
         13. Orthography — Ukrainian is not Russian, and the letters are where that bites:\n\
         - и is a full vowel letter of its own, distinct from і and from ї, and it is NOT the \
         Russian ы. і and и distinguish words: кіт (cat) and кит (whale), лік and лик are \
         different lemmas — never merge them.\n\
         - ї spells /ji/ and is a letter (Україна, їжа, їхати, її, з'їзд). Never write it as йі or \
         reduce it to і.\n\
         - г and ґ are different letters. Keep ґ in the words that have it (ґанок, ґрунт, ґудзик, \
         ґава, аґрус, ґрати) and never substitute it for г anywhere else (гора, нога, дорога).\n\
         - the apostrophe is part of the writing system, not punctuation: keep it in every lemma \
         that has one (м'ясо, п'ять, об'єкт, здоров'я, з'їзд, ім'я, Дем'ян). Reproduce whichever \
         character the input uses, never drop it, never replace it with ь, and never treat it as a \
         token boundary or a quotation mark.\n\
         - Ukrainian has NO ы, э, ё or ъ. If one appears, the token is not Ukrainian; do not \
         'restore' one into a lemma.\n\
         - never carry stress marks into any field: if the input is textbook text with acute \
         accents (кни́га, добре́), strip them before lemmatizing, and never add them yourself.\n\
         - the 2019 orthography permits doublets (авдиторія/аудиторія, етер/ефір, ирій/ірій, and \
         the genitives радості/радости, крові/крови). Lemmatize to the spelling the input actually uses and never \
         rewrite one variant into the other.\n\
         14. Register and interference — the guardrail that matters most here. If the input \
         contains a russianism or a surzhyk form, analyse it as the STANDARD Ukrainian word it \
         corresponds to and lemmatize to that standard lemma: получається → виходити, тоже → також, \
         больниця → лікарня, часи (as clock) → годинник, слідуючий → наступний, приймати участь → \
         брати участь. Never emit a Russian lemma for a Ukrainian token — стола is стіл and not \
         стол, коней is кінь and not конь, у Києві is Київ and not Киев, дівчата is дівчина and not \
         девушка. Never emit a surzhyk form as a lemma.\n\
         15. Value guardrails: NEVER put a gender value in the 'number' field or a number value in \
         the 'gender' field. There is no 'prepositional' case in this schema — Ukrainian's sixth \
         case is the LOCATIVE, and the locative is the only value to report after на/у/в/по/при. \
         There is no 'dual' number: report the paired body parts (руки, ноги, очі, вуха, плечі) and \
         everything after два/три/чотири as PLURAL. Do not report a case on a finite verb, an \
         infinitive, an adverbial participle or an impersonal -но/-то form — only a participle \
         declines. Do not report a person on a past-tense or conditional verb. The present copula \
         is normally unwritten (Він студент): never invent a token for it, but when є is actually \
         written (Він є студентом), analyse it as a finite present form of бути."
    }
}
