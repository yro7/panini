//! Czech (`ces`) — the value space of *spisovná čeština*, standard written Czech.
//!
//! Three decisions shape everything below and are worth stating once rather than
//! repeating on every variant:
//!
//! - **Animacy is folded into gender, not kept beside it.** Russian keeps the two
//!   apart because there animacy cuts across all three genders (it decides the
//!   accusative plural of feminines too). In Czech it does not: it is a split of
//!   the masculine alone, and Czech grammars accordingly teach four *rody* —
//!   mužský životný, mužský neživotný, ženský, střední. Modelling it as a fifth
//!   orthogonal field would put `animacy: inanimate` on every feminine and neuter,
//!   which is a statement Czech grammar does not make.
//! - **Number is binary.** The dual is gone as a category; what survives is a
//!   handful of paired body parts (ruce, nohy, oči, uši, ramena, kolena) and the
//!   numerals dva/oba, whose endings are historically dual but whose agreement is
//!   plural throughout — *ty ruce byly*, never a third agreement class. Giving the
//!   enum a `dual` value would publish a lexicon facet that a learner could fill
//!   with six nouns; the directives instead pin those forms to plural.
//! - **The verbal paradigm hangs off `verb_form`.** Czech's past and conditional
//!   are two tokens each — an *l*-participle carrying gender and number, and an
//!   auxiliary carrying person — so which fields apply to a verb token follows
//!   entirely from which slot of the system it occupies.
//!
//! Two dimensions Czech has that are deliberately **not** modelled:
//!
//! - **The noun paradigm (*vzor*: pán, hrad, muž, stroj, předseda, soudce, žena,
//!   růže, píseň, kost, město, moře, kuře, stavení).** Fourteen values, each of
//!   them a lexical lookup rather than something readable off the token, on a
//!   path that has no dictionary to look them up in. German makes the same call
//!   about its plural classes and for the same reason.
//! - **Voice.** The short passive participle (je napsán) is already its own
//!   `verb_form`, and the reflexive passive (dělá se) is a construction spread
//!   over two tokens, so a `voice` field would be a syntactic judgement about a
//!   neighbouring word rather than a feature of the token being analysed.

use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, SlavicAspect, TypologicalFeature,
    Upos,
};

/// The seven cases of Czech, in the order Czech schools number them (1.–7. pád).
///
/// Seven, and the vocative is a full member of the set — unlike Russian, where it
/// survives only in frozen forms. Czech forms it productively on any masculine or
/// feminine noun (Petr → Petře, pan doktor → pane doktore, Jana → Jano) and
/// requires it in direct address.
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
pub enum CzechCase {
    Nominative,   // 1. pád — nominativ (kdo? co?)
    Genitive,     // 2. pád — genitiv (koho? čeho?)
    Dative,       // 3. pád — dativ (komu? čemu?)
    Accusative,   // 4. pád — akuzativ (koho? co?)
    Vocative,     // 5. pád — vokativ (oslovujeme)
    Locative,     // 6. pád — lokál (o kom? o čem?)
    Instrumental, // 7. pád — instrumentál (kým? čím?)
}

/// The four declensional genders of Czech (*rody*).
///
/// Local rather than [`panini_core::traits::TernaryGender`] because the masculine
/// is split by animacy, and that split is not decoration: it decides the
/// masculine singular accusative (vidím pána, genitive-shaped, against vidím
/// hrad, nominative-shaped), the nominative plural (páni/pánové against hrady),
/// and the written -i / -y of an agreeing adjective or *l*-participle (mladí muži
/// psali against mladé hrady stály).
///
/// Not `PolishGender` either: Polish splits the masculine three ways (personal /
/// animate / inanimate) because its plural agreement distinguishes male persons
/// from everything else. Czech splits it twice.
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
pub enum CzechGender {
    MasculineAnimate,   // rod mužský životný — pán, muž, soudce
    MasculineInanimate, // rod mužský neživotný — hrad, stroj
    Feminine,           // rod ženský — žena, růže, píseň, kost
    Neuter,             // rod střední — město, moře, kuře, stavení
}

impl CzechGender {
    /// Whether this is one of the two masculines — the split animacy makes.
    #[must_use]
    pub const fn is_masculine(&self) -> bool {
        matches!(self, Self::MasculineAnimate | Self::MasculineInanimate)
    }
}

/// Tense of a finite verb or of the *l*-participle.
///
/// Three cells, distributed by aspect exactly as in the other Slavic languages:
/// an imperfective has all three (psal / píšu / budu psát), a perfective only
/// past and future (napsal / napíšu) — a perfective present-shaped form *is* a
/// future.
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
pub enum CzechTense {
    Past,    // minulý čas
    Present, // přítomný čas
    Future,  // budoucí čas
}

/// Mood of a finite verb (*způsob*).
///
/// The conditional is a mood here and not a tense: Czech builds it from a
/// dedicated auxiliary paradigm (bych, bys, by, bychom, byste) plus the same
/// *l*-participle the past uses, so it contrasts with the indicative on the same
/// axis the imperative does, not with present against future.
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
pub enum CzechMood {
    Indicative,  // oznamovací způsob
    Imperative,  // rozkazovací způsob
    Conditional, // podmiňovací způsob
}

/// Which slot of the verbal system a verb token occupies.
///
/// Required on every verb, because every other verbal field follows from it.
/// Czech's two compound tenses are compound in the literal sense — two tokens —
/// so the participle and its auxiliary are analysed separately and each gets the
/// fields it actually carries.
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
pub enum CzechVerbForm {
    /// Určitý tvar — a conjugated form: píšu, napíšeš, budeme, piš, bych.
    Finite,
    /// Infinitiv — psát, napsat, být.
    Infinitive,
    /// Příčestí činné, the *l*-participle: psal, psala, psali, psaly. The lexical
    /// half of both the past tense and the conditional; agrees in gender and
    /// number and has no person of its own.
    PastParticiple,
    /// Příčestí trpné in its short, predicative form: je napsán, byla otevřena,
    /// jsou zavřeny. The long form (napsaný) declines like a hard adjective and
    /// is analysed as an adjective, which is what Czech dictionaries make of it.
    PassiveParticiple,
    /// Přechodník — nesa, nesouc, napsav, napsavši. Alive in literary and older
    /// written Czech, and frozen into a small set of prepositions (počínaje,
    /// konče, nehledě na); a slot for it keeps those out of `Other`.
    Transgressive,
}

/// Verbal polarity (*kladný / záporný*).
///
/// Czech negates a verb by prefixing ne- to the form itself — nevím, nechtěl,
/// nebudu — so unlike Russian's separate не there is no token to carry it. The
/// lemma stays positive (nevím → vědět, so the learner keeps one mastery record
/// per verb), and this field is where the negation the prefix expresses is
/// recorded instead of being thrown away.
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
pub enum CzechPolarity {
    Affirmative, // píšu, byl, chtěj
    Negative,    // nepíšu, nebyl, nechtěj
}

/// Which paradigm an adjective's endings come from.
///
/// The hard/soft split is the first thing a Czech course teaches about
/// adjectives, because it decides the whole table: mladý has twelve distinct
/// endings, jarní has three. The possessive declension (otcův, matčin) is a third
/// paradigm, mixing nominal and adjectival endings, and `Indeclinable` is the
/// small closed class of borrowings that take no ending at all.
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
pub enum CzechAdjectiveDeclension {
    /// Tvrdá — vzor mladý: mladý, mladá, mladé, mladého, mladým…
    Hard,
    /// Měkká — vzor jarní: jarní throughout the singular and most of the plural.
    /// Every comparative and superlative declines here, whatever the positive
    /// does (mladý is hard, mladší and nejmladší are soft).
    Soft,
    /// Přivlastňovací — otcův, otcova, otcovo; matčin, matčina, matčino.
    Possessive,
    /// Nesklonná — khaki, bordó, prima, fajn, super, blond. No endings at all;
    /// the case, gender and number reported are the ones the noun assigns.
    Indeclinable,
}

/// Degree of comparison (*stupeň*), for adjectives and adverbs alike.
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
pub enum CzechDegree {
    Positive,    // 1. stupeň — mladý, rychle
    Comparative, // 2. stupeň — mladší, rychleji
    Superlative, // 3. stupeň — nejmladší, nejrychleji
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
pub enum CzechMorphology {
    /// Adjective — including ordinals (první, druhý) and long passive participles
    /// (napsaný, otevřená), both of which inflect on the adjectival pattern and
    /// are listed as adjectives by Czech dictionaries.
    ///
    /// Every field is required, and that is the difference from German: a Czech
    /// adjective agrees in the predicate as well as in the attribute (ten dům je
    /// velký, ta kniha je velká), so there is no uninflected use to make the
    /// agreement fields optional for.
    Adjective {
        lemma: String,
        degree: CzechDegree,
        /// Which paradigm this form's ending comes from; comparatives and
        /// superlatives are soft even when the positive is hard.
        declension: CzechAdjectiveDeclension,
        gender: CzechGender,
        number: BinaryNumber,
        case: CzechCase,
    },
    /// Preposition, with the case it governs **in this instance**.
    ///
    /// Required, and the reason is the same as German's two-way prepositions:
    /// na, v, o, po, za and pod each govern two or three cases and only the
    /// occurrence settles which (na stole locative, na stůl accusative).
    Adposition {
        lemma: String,
        case: CzechCase,
    },
    /// Adverb.
    ///
    /// `degree` is required rather than optional: Czech grades adverbs
    /// productively (rychle → rychleji → nejrychleji, dobře → lépe → nejlépe),
    /// and `positive` is the unmarked base form that a non-gradable adverb
    /// (tady, včera, velmi) stands in.
    Adverb {
        lemma: String,
        degree: CzechDegree,
    },
    /// Coordinating conjunction — a, i, ale, nebo, však, tedy.
    CoordinatingConjunction {
        lemma: String,
    },
    /// Determiner — demonstratives (ten, tento, onen), possessives (můj, tvůj,
    /// náš, svůj) and quantifiers (každý, všechen, žádný, nějaký, který) used
    /// adnominally. Czech has no articles, so this class is smaller than its
    /// Germanic equivalent and entirely declining.
    Determiner {
        lemma: String,
        /// Reported in the plural too: unlike German, the Czech plural
        /// distinguishes gender (ti muži, ty ženy, ta města).
        gender: CzechGender,
        number: BinaryNumber,
        case: CzechCase,
    },
    /// Interjection — ach, hele, jejda, no.
    Interjection {
        lemma: String,
    },
    /// Noun.
    Noun {
        lemma: String,
        /// Inherent and lexical, and reported for plurals too, where the ending
        /// alone no longer settles it (města could be neuter plural or feminine
        /// singular genitive).
        gender: CzechGender,
        number: BinaryNumber,
        case: CzechCase,
    },
    /// Cardinal numeral — jeden, dva, pět, sto, tisíc.
    ///
    /// Ordinals are not here: první, druhý and třetí inflect exactly like
    /// adjectives and are analysed as adjectives.
    Numeral {
        lemma: String,
        /// Only jeden (jeden/jedna/jedno) and dva (dva/dvě) distinguish gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<CzechGender>,
        /// Only jeden has a number contrast of its own.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Required: Czech cardinals decline (pět → pěti → s pěti) and the
        /// higher ones govern the genitive of what they count (pět stolů).
        case: CzechCase,
    },
    /// Particle — ať, nechť, prý, snad, asi, ne as a bare answer.
    Particle {
        lemma: String,
    },
    /// Pronoun — personal, reflexive, possessive, demonstrative, interrogative,
    /// relative, indefinite and negative, used pronominally.
    Pronoun {
        lemma: String,
        /// Personal and possessive pronouns only; kdo, co, který and the
        /// reflexive se/si stand outside person.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Third-person forms and the pronominal demonstratives; já, ty, my, vy,
        /// se and kdo do not show it.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<CzechGender>,
        /// Absent on se/si, kdo and co, which are outside number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: CzechCase,
        /// True for the short, unstressed forms that queue up in the clause's
        /// second position — mě, mi, tě, ti, ho, mu, ji, je, se, si — and false
        /// for the long stressed ones they alternate with (mne, mně, tebe, tobě,
        /// jeho, jemu, sebe, sobě) and for every pronoun that has no such
        /// alternation. Czech clitic ordering is one of the hardest things about
        /// its word order, which is why the distinction is a first-class field.
        clitic: bool,
    },
    /// Proper noun — declines exactly like a common noun (Praha → v Praze,
    /// Petr → Petra, Karel → Karle).
    ProperNoun {
        lemma: String,
        gender: CzechGender,
        number: BinaryNumber,
        case: CzechCase,
    },
    /// Subordinating conjunction — že, aby, když, kdyby, protože, jestli.
    SubordinatingConjunction {
        lemma: String,
    },
    /// Symbol.
    Symbol {
        lemma: String,
    },
    /// Verb.
    ///
    /// `aspect` and `polarity` hold of the token whatever slot it occupies;
    /// everything else follows from `verb_form`, and each `Option` below marks a
    /// cell Czech genuinely does not have rather than one the model might not
    /// know. The two that matter most, and the two that separate Czech from
    /// Russian here: the *l*-participle has **no person** (its auxiliary carries
    /// that) and it **does have gender in the plural** (psali against psaly
    /// against psala).
    Verb {
        lemma: String,
        /// Perfective or imperfective — a property of the lemma, never of the
        /// form: psát and napsat are two verbs, not two shapes of one.
        aspect: SlavicAspect,
        /// Whether the form carries the negative prefix ne-.
        polarity: CzechPolarity,
        verb_form: CzechVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<CzechMood>,
        /// Finite indicative forms, the *l*-participle (always past) and the
        /// transgressive. The infinitive, the imperative, the conditional
        /// auxiliary and the passive participle have none.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<CzechTense>,
        /// Finite forms only — the *l*-participle agrees in gender and number
        /// instead, so psal is masculine singular whether the subject is já, ty
        /// or on.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms, both participles and the transgressive; never the
        /// infinitive.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Both participles and the transgressive, in **both numbers** — the
        /// plural *l*-participle distinguishes all four genders in writing
        /// (psali / psaly / psala).
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<CzechGender>,
    },
    /// Other, for unanalyzable tokens.
    Other {
        lemma: String,
    },
}

impl CzechMorphology {
    /// Extracts the tense value for the tense pivot.
    ///
    /// `tense` is `Option` on the verb (the infinitive, the imperative, the
    /// conditional auxiliary and the passive participle have none), so the
    /// `MorphologyInfo` derive skips it for pivot generation. This hand-written
    /// handle keeps `PIVOT_TENSE` available for lexicon faceting, yielding `None`
    /// when no tense was extracted.
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
    /// conditional contrast is one of the dimensions a Czech learner most wants
    /// to drill: the conditional is a whole auxiliary paradigm of its own.
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

    /// Typed pivot handle for verb tense. Defined manually because `tense` is
    /// optional (see [`CzechMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <CzechTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// Typed pivot handle for verb mood. Defined manually because `mood` is
    /// optional (see [`CzechMorphology::__pivot_mood`]).
    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <CzechMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );
}

pub struct Czech;

impl LinguisticDefinition for Czech {
    type Morphology = CzechMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Ces;
    /// Ten facets, curated from the fourteen handles the derive emits.
    ///
    /// `polarity` is deliberately absent: it is a real dimension of the verb, but
    /// running text is affirmative almost throughout, so as a lexicon facet it
    /// would be one bucket holding everything and a second holding a handful.
    /// `person` is absent for the reason Russian's is — it partitions the verb
    /// paradigm, not the vocabulary.
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        CzechMorphology::PIVOT_CASE,
        CzechMorphology::PIVOT_GENDER,
        CzechMorphology::PIVOT_NUMBER,
        CzechMorphology::PIVOT_DECLENSION,
        CzechMorphology::PIVOT_DEGREE,
        CzechMorphology::PIVOT_ASPECT,
        CzechMorphology::PIVOT_VERB_FORM,
        CzechMorphology::PIVOT_MOOD,
        CzechMorphology::PIVOT_TENSE,
        CzechMorphology::PIVOT_CLITIC,
    ];

    /// Czech is written in the Latin script alone, with the háček and the
    /// kroužek that Jan Hus's orthographic reform introduced. There is no second
    /// contemporary writing system to declare.
    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
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
        "1. Lemmatization: verbs to the infinitive OF THEIR OWN ASPECT (píše → psát, napíše → \
         napsat); nouns to the nominative singular (na stole → stůl, v Praze → Praha, na ruce → \
         ruka), pluralia tantum staying plural (kalhoty, dveře, nůžky, Vánoce, játra); adjectives \
         to the masculine nominative singular POSITIVE degree (mladší → mladý, nejlepší → dobrý, \
         napsaného → napsaný); adverbs to the positive (lépe → dobře, rychleji → rychle); \
         participles and transgressives to the infinitive of the verb they are built on, same \
         aspect (napsán → napsat, píše jako přechodník → psát); pronouns and determiners to the \
         masculine nominative singular (jemu → on, mého → můj, té → ten). Lemmatize past the stem \
         alternations rather than stopping at them: ů/o (stůl → stolu), e/nothing (pes → psa, den \
         → dne), k/c (ruka → ruce), h/z (Praha → Praze), ch/š (moucha → mouše), r/ř (sestra → \
         sestře), d/ď and t/ť and n/ň before -e/-i (hrad → hradě).\n\
         2. Aspect pairs are DIFFERENT LEXEMES, not two forms of one verb. Never lemmatize a \
         perfective to its imperfective partner or the reverse: psát/napsat, dělat/udělat, \
         číst/přečíst, brát/vzít, říkat/říct, dávat/dát are twelve lemmas, not six. Report the \
         aspect the form actually carries. For the biaspectual verbs (informovat, absolvovat, \
         organizovat, věnovat, jmenovat, obětovat) report the aspect the context realizes.\n\
         3. Negation is a PREFIX, never a separate token: nevím, nechtěl, nebudu, nejsem are one \
         token each. Keep the lemma positive (nevím → vědět, nebyl → být) and record the negation \
         in 'polarity' instead. Set polarity negative for any verb form carrying ne- (and for \
         nesmím, nemusím), affirmative otherwise. Do not mistake a lexical ne- that is part of the \
         verb itself (nenávidět 'to hate') for the negative prefix: nenávidím is AFFIRMATIVE, \
         nenenávidím does not exist. The standalone ne answering a question is a particle, not a \
         verb.\n\
         4. Case is SYNTACTIC, never read off the ending. Czech syncretism is everywhere and the \
         ending is not the evidence: the masculine ANIMATE accusative singular is identical to the \
         genitive (vidím pána, vidím muže, vidím psa = accusative, NOT genitive) and the masculine \
         INANIMATE accusative singular is identical to the nominative (vidím hrad = accusative); \
         ženy is genitive singular, nominative plural and accusative plural; the feminine hard \
         dative and locative singular are both ženě, and only the preposition separates them. Use \
         this rule for that last one: the LOCATIVE never occurs without a preposition (v, na, o, \
         po, při) — a bare -ě/-i form after a verb is a dative (dal jsem to ženě), the same form \
         after v/na/o is a locative (o ženě).\n\
         5. The vocative is a full case and is REQUIRED in direct address: Petře!, pane doktore!, \
         Jano!, Evo!, kolegové!, přátelé!. Report vocative for every noun, proper noun, adjective \
         and determiner inside an address, including the ones whose vocative is identical to the \
         nominative (paní!, město!, Ivo!) — never fall back to nominative there.\n\
         6. Gender has four values, and animacy is a declension class rather than a semantic test: \
         masculine_animate is the class whose singular accusative equals its genitive, which \
         covers people and animals and a few nouns for things that decline that way (sněhulák, \
         panák, ledoborec). Report gender for plurals too, where the ending may not show it. \
         The written -i / -y of an agreeing adjective or l-participle is grammatical agreement and \
         nothing else — both are pronounced the same, so decide it from the subject's gender, not \
         from the sound: mladí muži psali (masculine animate), mladé hrady stály (masculine \
         inanimate), ženy psaly (feminine), města stála (neuter).\n\
         7. Number is singular or plural, never anything else. The dual survives only in the paired \
         body parts (ruce, nohy, oči, uši, ramena, kolena) and after dva/oba; report every one of \
         those as PLURAL — rukama, nohama, očima, ušima are instrumental PLURAL forms, not a third \
         number. Pluralia tantum (kalhoty, dveře, nůžky, brýle, Vánoce, narozeniny) are plural even \
         where they denote one thing.\n\
         8. Verbs — which fields apply follows from 'verb_form', and each omission below is a cell \
         Czech does not have:\n\
         - finite present/future indicative: mood indicative, tense, person and number; NO gender.\n\
         - imperative: mood imperative, person and number; no tense, no gender.\n\
         - conditional auxiliary (bych, bys, by, bychom, byste): lemma být, finite, mood \
         conditional, person and number; no tense, no gender.\n\
         - infinitive: aspect and polarity only; no mood, tense, person, number or gender.\n\
         - l-participle (psal, psala, psalo, psali, psaly): verb_form past_participle, tense past, \
         number, and gender in BOTH numbers. OMIT PERSON ENTIRELY — the person sits on the \
         auxiliary, so psal is masculine singular whether the subject is já, ty or on.\n\
         - short passive participle (je napsán, byla otevřena, jsou zavřeny): verb_form \
         passive_participle, number and gender; no tense, no person, no mood. The LONG form \
         (napsaný, otevřená) is an ADJECTIVE, not a verb — analyse it as one, with its degree, \
         declension, gender, number and case.\n\
         - transgressive (nesa, nesouc, napsav, napsavši): verb_form transgressive, tense, number \
         and gender; no person, no mood.\n\
         9. The compound tenses are two tokens each, and the auxiliary is a verb in its own right. \
         Past: napsal jsem → 'napsal' is the l-participle above and 'jsem' is a separate verb, \
         lemma být, finite, indicative, PRESENT, first person singular. In the third person there \
         is no auxiliary at all (napsal = he wrote). Conditional: napsal bych → 'napsal' plus \
         'bych' (lemma být, finite, conditional, first singular). Imperfective future: budu psát → \
         'budu' is a finite FUTURE form of být and 'psát' is a separate infinitive token. A \
         perfective present-shaped ending IS a future: napíšu → tense future, aspect perfective. \
         Motion verbs build the future with the prefix po-/pů- (půjdu, ponesu, povezu) → tense \
         future. Unlike Russian, Czech WRITES its present copula: analyse je in Petr je student as \
         a verb.\n\
         10. Tokenization: the reflexive se and si are separate PRONOUN tokens, never fused with \
         the verb, and the verb is lemmatized without them (myje se → 'mýt' + 'se'). Beware that se \
         is also a preposition (se mnou, se školou) — the vocalized form of s, and lemmatized to s; \
         the same goes for ve → v, ke → k, ze → z, and for the -e added before a pronoun (ke mně, \
         ve všem). aby and kdyby fuse the conjunction with the conditional auxiliary and stay ONE \
         subordinating-conjunction token (abych, abys, aby, abychom, abyste; kdybych, kdybys, \
         kdyby) — lemmatize them to aby and kdyby. Contracted preposition + pronoun forms (nač, \
         proč, oč, zač, nato) are single tokens. Hyphenated forms (česko-slovenský, ping-pong) are \
         single tokens.\n\
         11. Adjectives: report all five fields on every one. degree positive/comparative/ \
         superlative (mladý / mladší / nejmladší, dobrý / lepší / nejlepší); declension hard for \
         the mladý paradigm, soft for the jarní paradigm AND for every comparative and superlative \
         whatever the positive does (mladší is soft), possessive for otcův and matčin, indeclinable \
         for the endingless borrowings (khaki, bordó, prima, fajn, super, blond). A predicative \
         adjective still agrees in Czech (ta kniha je velká) — give it its gender, number and case, \
         which is nominative. The short nominal forms (je zdráv, jsem rád, buď tak laskav) are \
         adjectives too: hard declension, nominative.\n\
         12. Pronouns and determiners: a pronoun used adnominally before a noun is a DETERMINER \
         (ten dům, moje kniha, každý den), the same word standing alone is a PRONOUN (ten je můj, \
         každý ví). Set clitic true only for the short unstressed second-position forms — mě, mi, \
         tě, ti, ho, mu, ji, je, se, si — and false for the long stressed alternants (mne, mně, \
         tebe, tobě, jeho, jemu, sebe, sobě) and for every pronoun with no such alternation (já, \
         ty, on, kdo, co, který, ten). The possessives jeho and jejich are indeclinable but still \
         occupy a syntactic slot: report the gender, number and case the noun assigns.\n\
         13. Orthography and register: keep every diacritic exactly as written, and keep ú and ů \
         apart (úterý, dům) — they are different letters, not variants. If the input contains \
         colloquial obecná čeština forms, analyse them as the standard forms they correspond to \
         and lemmatize to the STANDARD lemma: dobrej / dobrý mléko → dobrý, vokno → okno, von → on, \
         s klukama → kluk (instrumental plural), voni dělaj → dělat. Never emit a colloquial form \
         as a lemma.\n\
         14. Value guardrails: NEVER put a gender value in the 'number' field or a number value in \
         the 'gender' field. NEVER report 'dual' — the enum has no such value. Do not report a case \
         on a verb: Czech verbs do not decline, and the long passive participle that does is an \
         adjective."
    }
}
