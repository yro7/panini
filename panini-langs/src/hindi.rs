use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, BinaryVoice, IsoLang, LinguisticDefinition, Person, Script,
    TypologicalFeature, Upos,
};

/// The three cases of the Hindi nominal.
///
/// Hindi lost the inherited case system and rebuilt it as a two-layer one: a
/// three-cell inflectional core (direct / oblique / vocative) plus a set of
/// free-standing postpositions (`ने`, `को`, `से`, `में`, `पर`, `का`) that carry
/// the actual semantic roles. Every postposition governs the oblique, so the
/// oblique never appears alone — `लड़के` is oblique only because something
/// follows it, or because it is an adverbial of time or place (`सुबह`, `इस साल`).
///
/// The vocative is a real, if small, cell: `लड़के!` (singular) and `लड़को!`
/// (plural) are distinct from both direct and oblique in the plural.
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
pub enum HindiCase {
    Direct,   // सीधा / कर्ता कारक
    Oblique,  // तिरछा — required before any postposition
    Vocative, // सम्बोधन
}

/// Whether a nominal or adjectival stem belongs to the inflecting class.
///
/// One enum for nouns and adjectives because it is one phenomenon: a stem whose
/// citation form ends in `-आ` (masculine `लड़का`, `अच्छा`) or in `-ई`
/// (feminine `लड़की`) has a full paradigm, and every other stem (`घर`, `किताब`,
/// `लाल`, `सुन्दर`, and the Perso-Arabic and English loans) is invariant and
/// shows its case and number only on what follows it.
///
/// Learners meet this split on day one — it is what decides whether `बड़ा` or
/// `बड़े` goes in front of the noun — so it is worth a dimension of its own
/// rather than being left implicit in the ending.
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
pub enum HindiInflectionClass {
    /// Inflects: `-आ` masculines (`लड़का` → `लड़के` → `लड़कों`), `-ई` feminines
    /// (`लड़की` → `लड़कियाँ`), `-आ` adjectives (`अच्छा` / `अच्छे` / `अच्छी`).
    Marked,
    /// Invariant in the direct and oblique singular: `घर`, `किताब`, `लाल`,
    /// `ख़ूबसूरत`, `साफ़`. Marked plurals still surface on nouns (`घरों`), never
    /// on adjectives.
    Unmarked,
}

/// Grammatical aspect, carried by the participle of a verb phrase.
///
/// Hindi's finite verb is analytic: a participle carries aspect and agrees in
/// gender and number, while a following copula carries tense and person. The
/// three cells below are the participles the language actually builds; the
/// forms that carry no aspect at all — the future (`करेगा`), the subjunctive
/// (`करे`), the imperative (`करो`), the infinitive (`करना`) — omit the field
/// rather than being given a fourth "simple" value they do not contrast with.
///
/// `Habitual` is the `-ता` participle, which the descriptive tradition also
/// calls the *imperfective* participle: one form, one value here, so the model
/// is never asked to choose between two names for the same cell.
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
pub enum HindiAspect {
    /// The `-ता` (imperfective) participle: `करता है`, `करता था`.
    Habitual,
    /// The `-आ` participle: `किया`, `किया है`, `किया था`.
    Perfective,
    /// `रहा` + stem: `कर रहा है`, `कर रही थी`.
    Progressive,
}

/// Tense, as carried by the copula or by the synthetic future.
///
/// Only three cells, because Hindi's past/present/future distinctions beyond
/// these are aspectual: `करता था` is habitual + past, not a separate "imperfect"
/// tense.
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
pub enum HindiTense {
    Present, // है / हैं / हूँ
    Past,    // था / थी / थे
    Future,  // -गा / -गी / -गे
}

/// Mood of a finite verb.
///
/// `Presumptive` is a genuine fourth cell, not a use of the future: `होगा` in
/// `वह घर पर होगा` ("he must be at home") and `कर रहा होगा` ("he is probably
/// working") is a present inference, and it is built from the future form of
/// `होना` used as an auxiliary.
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
pub enum HindiMood {
    Indicative,  // करता है, किया, करेगा
    Subjunctive, // करे, करें — after अगर, शायद, चाहिए कि
    Imperative,  // कर / करो / कीजिए
    Presumptive, // होगा, कर रहा होगा
}

/// Which slot of the verbal system a verb token occupies.
///
/// Required on every verb, because in Hindi it is the value that decides which
/// of the other fields exist at all: a participle agrees in gender and number
/// and has no person, a finite future has person and number and no gender in the
/// way the copula does, and the bare stem in a compound verb agrees with nothing.
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
pub enum HindiVerbForm {
    /// A form inflected for person or agreeing as a main predicate: `करता है`
    /// (the participle plus its copula are two tokens, both finite in the phrase
    /// sense — see the directives), `करेगा`, `करे`, `करो`, `था`.
    Finite,
    /// The bare stem, which stands alone only as the intimate imperative (`कर!`)
    /// and as the first half of a compound verb (`कर` in `कर लिया`, `खा` in
    /// `खा गया`).
    Stem,
    /// `-ना`: `करना`, `जाना`. Behaves as a masculine noun and can be declined
    /// (`करने के लिए`), which is why it may carry case.
    Infinitive,
    /// The `-ता` / `-आ` / `रहा` participles, whether predicative (`करता` in
    /// `करता है`) or attributive (`सोया हुआ बच्चा`).
    Participle,
    /// The `-कर` / `-के` conjunctive participle: `खाकर`, `देखकर` — "having
    /// eaten", "having seen". Invariant, and never a main predicate.
    Conjunctive,
}

/// The three-way politeness scale of the second person.
///
/// This is a closed grammatical dimension in Hindi, not a stylistic preference:
/// each value takes its own verb agreement (`तू करता है` / `तुम करते हो` /
/// `आप करते हैं`), its own imperative (`कर` / `करो` / `कीजिए`), and its own
/// oblique and possessive forms. Choosing the wrong one is a social error a
/// learner needs drilled, which is why it is published as a pivot.
///
/// It reaches the third person too: `आप` agreement is what `वे`/`ये` do when a
/// single respected person is referred to (`पिताजी आए हैं`).
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
pub enum HindiHonorificity {
    /// `तू` — intimate/inferior. Rare in neutral modern speech; marked when used.
    Intimate,
    /// `तुम` — familiar, the default among peers, friends and to children.
    Familiar,
    /// `आप` — respectful, and the safe default with strangers and elders.
    Honorific,
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
pub enum HindiMorphology {
    /// Adjective — the `-आ` class agrees with its head noun in gender, number
    /// and case; every other adjective is invariant.
    Adjective {
        lemma: String,
        inflection_class: HindiInflectionClass,
        /// Agreement gender. Absent on an invariant adjective, which shows none.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        /// Agreement number. Absent on an invariant adjective.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Agreement case — the oblique `अच्छे` of `अच्छे लड़के को`. Absent on an
        /// invariant adjective.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<HindiCase>,
    },
    /// Postposition — `ने`, `को`, `से`, `में`, `पर`, `तक`, `का/की/के`.
    ///
    /// Hindi has essentially no prepositions; the UD tag is still `ADP`.
    Adposition {
        lemma: String,
        /// The case this postposition governs on the noun phrase to its left.
        /// In practice always oblique — that government is the entire reason the
        /// oblique exists — and it is reported per instance so the link between
        /// the two is visible in the analysis rather than merely implied.
        case: HindiCase,
        /// The genitive `का` alone agrees with the *possessed* noun, so it has a
        /// gender, a number and a case of its own (`राम की किताब`,
        /// `राम के घर में`). Absent on every other postposition.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    /// Adverb — `जल्दी`, `यहाँ`, `बहुत`, `कल`.
    Adverb {
        lemma: String,
    },
    /// Coordinating conjunction — `और`, `या`, `लेकिन`, `पर`.
    CoordinatingConjunction {
        lemma: String,
    },
    /// Determiner — the demonstratives `यह` / `वह` / `ये` / `वे` in attributive
    /// use, plus `कुछ`, `सब`, `हर`, `कोई`, `कौन सा`.
    Determiner {
        lemma: String,
        /// Demonstratives only; `हर`, `कुछ` and `सब` stand outside number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Demonstratives only — `इस`/`उस`/`इन`/`उन` are their oblique forms.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<HindiCase>,
    },
    /// Interjection — `अरे`, `वाह`, `हाय`.
    Interjection {
        lemma: String,
    },
    /// Noun.
    Noun {
        lemma: String,
        /// Hindi has two genders. There is no neuter: the Sanskrit neuter was
        /// redistributed, mostly into the masculine.
        gender: BinaryGender,
        number: BinaryNumber,
        case: HindiCase,
        inflection_class: HindiInflectionClass,
    },
    /// Numeral — cardinals (`एक`, `दो`, `सौ`) are invariant.
    ///
    /// Ordinals (`पहला`, `दूसरा`) inflect on the `-आ` adjective pattern and are
    /// tagged `Adjective`, not here.
    Numeral {
        lemma: String,
    },
    /// Particle — `नहीं`, `न`, `मत`, `ही`, `भी`, `तो`, `क्या` (the polar
    /// question marker).
    Particle {
        lemma: String,
    },
    /// Pronoun — personal, demonstrative in nominal use, relative, interrogative
    /// and reflexive.
    ///
    /// No gender field: no Hindi pronoun distinguishes gender. `वह` covers "he",
    /// "she" and "it" alike; the gender surfaces only on the verb.
    Pronoun {
        lemma: String,
        /// Personal and demonstrative pronouns. `जो`, `कौन`, `क्या` and the
        /// reflexive `अपना`/`ख़ुद` have no person.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// `कौन`, `क्या` and `ख़ुद` stand outside number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: HindiCase,
        /// The `तू` / `तुम` / `आप` scale. Second-person pronouns always; third
        /// person only when an honorific plural refers to one person.
        #[serde(skip_serializing_if = "Option::is_none")]
        honorificity: Option<HindiHonorificity>,
    },
    /// Proper noun — declines exactly like a common noun (`दिल्ली में`,
    /// `राम को`).
    ProperNoun {
        lemma: String,
        /// Absent when the name has no settled grammatical gender in Hindi —
        /// typically a foreign name with no established usage.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        number: BinaryNumber,
        case: HindiCase,
    },
    /// Subordinating conjunction — `कि`, `अगर`, `क्योंकि`, `जब`, `ताकि`.
    SubordinatingConjunction {
        lemma: String,
    },
    /// Symbol.
    Symbol {
        lemma: String,
    },
    /// Verb — participles, copulas, synthetic futures, subjunctives,
    /// imperatives, infinitives, conjunctive participles and bare stems.
    ///
    /// Which fields apply follows almost entirely from `verb_form`, and each
    /// `Option` below marks a cell Hindi genuinely lacks rather than one the
    /// model might not know. The agreement fields carry the language's most
    /// distinctive syntax: with the ergative postposition `ने`, the verb agrees
    /// with the *object*, so `gender` and `number` here are the features of
    /// whatever the verb actually agrees with in this clause, not of the subject.
    Verb {
        lemma: String,
        verb_form: HindiVerbForm,
        voice: BinaryVoice,
        /// Participles only, and the copula that heads their phrase carries none:
        /// omit on futures, subjunctives, imperatives, infinitives, conjunctive
        /// participles and bare stems.
        #[serde(skip_serializing_if = "Option::is_none")]
        aspect: Option<HindiAspect>,
        /// Copulas (`है`, `था`, `होगा`) and the synthetic future. A participle
        /// standing alone in a perfective past (`उसने किया`) carries no tense of
        /// its own — omit it there.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<HindiMood>,
        /// Copulas and the synthetic future only.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<HindiTense>,
        /// Forms that inflect for person: the copula, the future, the
        /// subjunctive and the imperative. Participles have none — they agree in
        /// gender and number instead.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// As above. `तुम हो` is grammatically plural even for one addressee.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Participles and the future, which agree in gender; never the copula
        /// `है`/`था`… wait — `था`/`थी`/`थे` does agree, and is included. Absent
        /// on `है`/`हैं`, on subjunctives, imperatives, conjunctive participles
        /// and bare stems.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        /// Which addressee level an imperative selects: `कर` / `करो` / `कीजिए`.
        /// Absent on every non-imperative form.
        #[serde(skip_serializing_if = "Option::is_none")]
        honorificity: Option<HindiHonorificity>,
        /// Infinitives decline like masculine `-आ` nouns (`करने के लिए`,
        /// `जाने से`). Absent on every other verb form.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<HindiCase>,
    },
    /// Other, for unanalyzable tokens.
    Other {
        lemma: String,
    },
}

impl HindiMorphology {
    /// Extracts the aspect value for the aspect pivot.
    ///
    /// `aspect` is `Option` on the verb — the future, the subjunctive, the
    /// imperative and the infinitive carry none — so the `MorphologyInfo` derive
    /// skips it. Written by hand because aspect is the axis the whole Hindi verb
    /// phrase turns on and a learner will want to drill it.
    fn __pivot_aspect(&self) -> Option<String> {
        match self {
            Self::Verb { aspect, .. } => aspect
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    /// Extracts the tense value for the tense pivot. `tense` is `Option`
    /// (only copulas and the synthetic future carry one), so the derive skips it.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    /// Extracts the mood value for the mood pivot. `mood` is `Option` (the
    /// non-finite forms have none), so the derive skips it.
    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    /// Extracts the politeness level from pronouns and imperatives alike.
    ///
    /// `honorificity` is `Option` in both variants that carry it, so the derive
    /// skips it. Written by hand because the `तू`/`तुम`/`आप` scale is exactly
    /// the kind of closed social dimension a learner needs to practise, and
    /// because a derived handle would have seen only one of the two parts of
    /// speech that express it.
    fn __pivot_honorificity(&self) -> Option<String> {
        match self {
            Self::Pronoun { honorificity, .. } | Self::Verb { honorificity, .. } => honorificity
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for aspect. Defined manually because `aspect` is
    /// optional (see [`HindiMorphology::__pivot_aspect`]).
    pub const PIVOT_ASPECT: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "aspect",
            "Aspect",
            <HindiAspect as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_aspect,
        );

    /// Typed pivot handle for tense. Defined manually because `tense` is
    /// optional (see [`HindiMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <HindiTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// Typed pivot handle for mood. Defined manually because `mood` is optional
    /// (see [`HindiMorphology::__pivot_mood`]).
    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <HindiMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    /// Typed pivot handle for the politeness scale. Defined manually because
    /// `honorificity` is optional (see
    /// [`HindiMorphology::__pivot_honorificity`]).
    pub const PIVOT_HONORIFICITY: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "honorificity",
            "Honorificity",
            <HindiHonorificity as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_honorificity,
        );
}

pub struct Hindi;

impl LinguisticDefinition for Hindi {
    type Morphology = HindiMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Hin;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        HindiMorphology::PIVOT_CASE,
        HindiMorphology::PIVOT_GENDER,
        HindiMorphology::PIVOT_NUMBER,
        HindiMorphology::PIVOT_INFLECTION_CLASS,
        HindiMorphology::PIVOT_ASPECT,
        HindiMorphology::PIVOT_TENSE,
        HindiMorphology::PIVOT_MOOD,
        HindiMorphology::PIVOT_VERB_FORM,
        HindiMorphology::PIVOT_HONORIFICITY,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::DEVA]
    }

    fn default_script(&self) -> Script {
        Script::DEVA
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            TypologicalFeature::Conjugation(&[Upos::Verb]),
            TypologicalFeature::Declension(&[
                Upos::Noun,
                Upos::ProperNoun,
                Upos::Adjective,
                Upos::Pronoun,
                Upos::Determiner,
            ]),
        ]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization: verbs to the `-ना` infinitive (करता है → करना, गए → जाना, कीजिए → करना, \
         खाकर → खाना, है/हैं/था/थी/होगा → होना); nouns to the direct singular (लड़कों → लड़का, \
         किताबें → किताब), with pluralia tantum staying plural (दर्शन, प्राण); adjectives to the \
         masculine direct singular (अच्छी → अच्छा, बड़े → बड़ा), invariant adjectives to themselves \
         (लाल → लाल); pronouns and determiners to the direct form (मुझे → मैं, उसने → वह, इस → यह, \
         तुम्हें → तुम, आपको → आप). Causatives are SEPARATE LEXEMES, never lemmatized to their base: \
         करना / कराना / करवाना, पढ़ना / पढ़ाना, बनना / बनाना are distinct verbs.\n\
         2. Tokenization: postpositions are separate tokens, never fused with the noun. उसने is TWO \
         tokens (उस + ने), मुझको is मुझ + को, लड़के का is three tokens. Compound (vector) verbs are \
         also two tokens: in कर लिया, खा गया, बोल उठा, दे दिया the FIRST token is the main verb as a \
         bare stem (verb_form stem) and the SECOND is the vector verb, analysed as a normal verb with \
         its own lemma (लेना, जाना, उठना, देना) — the vector contributes aspectual and attitudinal \
         colour, so do NOT merge the pair into one token and do NOT lemmatize the vector away. \
         Conjunct verbs behave the opposite way: in काम करना, इंतज़ार करना, शुरू होना the nominal is a \
         Noun token and करना/होना a Verb token, each analysed on its own. The future suffix is NOT a \
         token: करेगा is a single verb.\n\
         3. Case is SYNTACTIC and follows government, not the ending. Report oblique on every noun, \
         adjective, determiner and pronoun that stands before a postposition, even when the form is \
         identical to the direct (घर में → घर is oblique; किताब पर → किताब is oblique). Report \
         oblique also on a bare adverbial of time or place (इस साल, सुबह, अगले हफ़्ते). Report \
         vocative only under actual address (लड़के!, दोस्तो!, भाइयो!). Otherwise direct. Beware the \
         syncretism: लड़के is masculine direct plural, oblique singular AND vocative singular — let \
         the syntax decide.\n\
         4. Nouns: always give gender, number, case and inflection_class. Gender is inherent and \
         lexical, so report it in the plural too, where the ending may not show it (किताबें → \
         feminine). inflection_class is `marked` for masculines whose direct singular ends in -आ \
         (लड़का, कमरा, but NOT the invariant loans राजा, पिता, चाचा — those are unmarked) and for \
         feminines ending in -ई (लड़की, कुर्सी); everything else is `unmarked` (घर, आदमी, किताब, \
         मेज़, स्कूल). The oblique plural ALWAYS ends in -ओं regardless of class (घरों, किताबों, \
         लड़कियों) — a noun in -ओं is oblique plural, never direct.\n\
         5. Adjectives: `-आ` adjectives are inflection_class marked and take gender, number and case \
         agreeing with their head noun (अच्छे लड़के को → अच्छे is masculine singular OBLIQUE, not \
         plural). Every other adjective — लाल, सुन्दर, ख़ूबसूरत, साफ़, ठीक, and all the Perso-Arabic \
         and English loans — is inflection_class unmarked: OMIT gender, number and case entirely \
         rather than guessing them from the noun. Comparison is periphrastic (X से बड़ा, सबसे बड़ा, \
         ज़्यादा): analyse से and सबसे as their own tokens and the adjective as a plain positive form.\n\
         6. Verbs — which fields apply follows from verb_form:\n\
         - participle (करता, किया, कर रहा, गई): aspect and voice, plus gender and number for the \
         agreement it shows. NO person, NO tense, NO mood of its own.\n\
         - finite copula (है, हैं, हूँ, हो, था, थी, थे, थीं): tense and mood indicative, person and \
         number; gender only on the past copula था/थी/थे/थीं, never on है/हैं/हूँ/हो. Omit aspect.\n\
         - synthetic future (करेगा, करेंगी, जाऊँगा): tense future, mood indicative, person, number \
         AND gender. Omit aspect.\n\
         - subjunctive (करे, करें, जाऊँ, हो): mood subjunctive, person and number. No tense, no \
         gender, no aspect.\n\
         - imperative (कर, करो, कीजिए, कीजिएगा): mood imperative, person second, and honorificity \
         (कर → intimate, करो → familiar, कीजिए/कीजिएगा → honorific). No tense, no aspect, no gender.\n\
         - presumptive (होगा, कर रहा होगा, करता होगा): mood presumptive on the होगा auxiliary, with \
         its person, number and gender; the preceding participle keeps its own aspect.\n\
         - infinitive (करना, करने, करने के लिए): voice only, plus case when a postposition follows \
         (करने से → oblique). No person, number, gender, tense or aspect.\n\
         - conjunctive participle (करके, खाकर, देखकर): voice only. It is invariant — omit gender, \
         number, person, tense, mood and aspect.\n\
         - stem (the first half of a compound verb, कर in कर लिया): voice only; it agrees with \
         nothing.\n\
         7. ERGATIVE AGREEMENT — the single most common analysis error. With the postposition ने \
         (perfective transitive clauses: उसने किताब पढ़ी), the verb agrees with the DIRECT OBJECT, not \
         with the subject. In उसने किताब पढ़ी, पढ़ी is FEMININE SINGULAR because किताब is feminine, \
         even though उस is the same pronoun as in वह पढ़ा. Report the gender and number the form \
         actually carries. Two refinements: if the object itself carries को (उसने लड़की को देखा), the \
         verb agrees with nothing and stands in the DEFAULT masculine singular — report masculine \
         singular; and ने appears only in perfective clauses of transitive verbs, so करता है and \
         करेगा never take it.\n\
         8. Negation is always a separate Particle token and is never fused with the verb: नहीं \
         (indicative), न (subjunctive and conditional), मत (imperative, मत जाओ). In a negated present \
         habitual the copula is frequently DROPPED (वह नहीं जाता, not वह नहीं जाता है) — do not \
         hallucinate a है token that is not in the text, and do not move नहीं: it normally stands \
         immediately before the verb, and after it only for emphasis.\n\
         9. The copula: है is third person SINGULAR, हैं is third person PLURAL (and also the \
         honorific singular — report number plural, which is what the grammar carries), हूँ is first \
         singular, हो is second person with तुम, हैं with आप. Do not confuse है with हैं or with the \
         particle ही. The present copula is often omitted in verbless equational clauses — again, \
         never emit a token the text does not contain.\n\
         10. Pronouns: person, number, case, and honorificity for the second person (तू intimate, तुम \
         familiar, आप honorific) and for an honorific third-person plural referring to one person. \
         तुम and आप are grammatically PLURAL — report number plural even for one addressee. वह covers \
         he/she/it: never invent a gender field for a pronoun, the gender lives on the verb. The \
         oblique stems मुझ, तुझ, इस, उस, इन, उन are oblique forms of मैं, तू, यह, वह, ये, वे — \
         lemmatize accordingly. मुझे, तुझे, उसे, इन्हें are the oblique stem fused with को: analyse \
         them as ONE pronoun token in the oblique case (the को is not separable there).\n\
         11. Voice: report passive only for the genuine जाना passive (किताब पढ़ी जाती है, काम किया \
         गया) — a perfective participle plus a form of जाना. जाना as a lexical verb of motion (वह गया) \
         is ACTIVE, and so is the capabilitative/inability construction (मुझसे नहीं खाया जाता) if you \
         cannot tell them apart, default to active. Causatives (कराना, पढ़ाना) are active lexemes, not \
         a voice.\n\
         12. Orthography: write Devanagari only, in standard modern spelling. Preserve nukta \
         distinctions where the text has them (ज़, फ़, ग़, क़, ख़: ज़रूर, फ़ोन) and preserve candrabindu \
         vs anusvara as written (हूँ vs हैं). Never carry a Latin transliteration into any field. The \
         danda ॥ and । are punctuation — omit them entirely, and do not tag them.\n\
         13. NEVER put a gender value (masculine/feminine) in the number field or a number value in \
         the gender field. NEVER use `neuter`: Hindi has two genders only. NEVER report a case other \
         than direct, oblique or vocative — ने, को, से, में are postposition tokens, not case values."
    }
}
