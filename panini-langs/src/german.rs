use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryGender, TypologicalFeature,
    Upos,
};

/// The four cases of modern German.
///
/// Four, not five: the vocative has no forms of its own (direct address takes
/// the nominative), and the instrumental survives only in frozen adverbs
/// (deswegen, meinetwegen) that are lexicalised rather than paradigm cells.
///
/// Declaration order follows the order German paradigm tables are printed and
/// learned in — nominative, accusative, dative, genitive — not the alphabet.
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
pub enum GermanCase {
    Nominative, // Nominativ (Wer-Fall)
    Accusative, // Akkusativ (Wen-Fall)
    Dative,     // Dativ (Wem-Fall)
    Genitive,   // Genitiv (Wes-Fall)
}

/// Which of the three adjective paradigms an attributive adjective inflects by.
///
/// The single most distinctive fact about German nominal morphology, and the
/// one no Romance or Slavic language has: the ending on an attributive
/// adjective is not decided by its own gender, number and case alone, but by
/// how much of that information the *preceding determiner* has already
/// spelled out.
///
/// - `Strong` — no determiner, or one with no ending of its own (guter Wein,
///   kaltes Wasser, viel frisches Obst): the adjective carries the full
///   pronominal ending.
/// - `Weak` — after a der-word, which already marks case, gender and number
///   (der gute Wein, dieses kalte Wasser): the adjective reduces to -e / -en.
/// - `Mixed` — after an ein-word (ein guter Wein, mein kaltes Wasser), whose
///   paradigm is endingless in exactly three cells and der-like everywhere
///   else, so the adjective is strong in those three and weak in the rest.
///
/// The class is a property of the whole determiner phrase, so `Mixed` is
/// reported across the entire ein-word paradigm — not only in the three cells
/// where the strong ending surfaces.
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
pub enum GermanAdjectiveDeclension {
    Strong, // starke Deklination
    Weak,   // schwache Deklination
    Mixed,  // gemischte Deklination
}

/// Degree of comparison, for adjectives and for the handful of adverbs that
/// carry it in their own form (oft / öfter, gern / lieber, bald / eher).
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
pub enum GermanDegree {
    Positive,    // Positiv
    Comparative, // Komparativ
    Superlative, // Superlativ
}

/// The two synthetic tenses of German.
///
/// Perfekt, Plusquamperfekt, Futur I and Futur II are periphrastic — an
/// inflected form of haben / sein / werden plus a participle or an infinitive —
/// so they are analysed as the two verb tokens they are written as, exactly as
/// English does with its perfect and progressive. Nothing in the German verb
/// inflects for them.
///
/// The value is morphological, not semantic: it names the stem the finite form
/// is built on. That is why Konjunktiv I, built on the present stem, is
/// `Present` and Konjunktiv II, built on the preterite stem, is `Past`, whatever
/// time either one refers to.
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
pub enum GermanTense {
    Present, // Präsens
    Past,    // Präteritum
}

/// Mood of a finite verb.
///
/// The two Konjunktive are kept apart because they are two paradigms with two
/// jobs, not two uses of one. Konjunktiv I is built on the present stem and is
/// the mood of reported speech (der Minister sagte, er habe nichts gewusst);
/// Konjunktiv II is built on the preterite stem, usually with umlaut, and is
/// the mood of the counterfactual, the wish and the polite request (wenn ich
/// Zeit hätte; ich könnte). Collapsing them into one `subjunctive` would erase
/// the only formal contrast a learner has to work from, and would make the
/// indirect-speech register unlearnable.
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
pub enum GermanMood {
    Indicative, // Indikativ
    Imperative, // Imperativ
    /// Konjunktiv I — present stem, reported speech.
    ///
    /// Renamed explicitly: serde's snake_case rule inserts a separator before
    /// every uppercase letter, so `SubjunctiveII` would serialise as
    /// `subjunctive_i_i`.
    #[serde(rename = "subjunctive_i")]
    SubjunctiveI,
    /// Konjunktiv II — preterite stem, counterfactual and polite.
    #[serde(rename = "subjunctive_ii")]
    SubjunctiveII,
}

/// Which slot of the verbal system a verb token occupies.
///
/// Required on every verb, because which of the remaining fields apply follows
/// entirely from it. Only the *uninflected* participles are verbs here: an
/// attributive participle (der lachende Mann, das gebaute Haus) takes adjective
/// endings by the very paradigm above and is analysed as an adjective, which is
/// also what Universal Dependencies does with it.
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
pub enum GermanVerbForm {
    Finite,            // finite Form
    Infinitive,        // Infinitiv
    PresentParticiple, // Partizip I (Partizip Präsens)
    PastParticiple,    // Partizip II (Partizip Perfekt)
}

/// The inflection class of a verb lexeme.
///
/// A lexical property of the lemma rather than a feature of the token, on the
/// same footing as Slavic aspect: it is invisible in the present tense
/// (ich singe, ich mache) and decides the whole preterite and participle, which
/// is precisely why German courses hand out a list of it to memorise. Reported
/// on every token of the lemma so the learner can facet on it.
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
pub enum GermanVerbClass {
    /// Schwach — dental suffix, no stem-vowel change: machen, machte, gemacht.
    Weak,
    /// Stark — ablaut in the preterite, participle in -en: singen, sang,
    /// gesungen.
    Strong,
    /// Gemischt — dental suffix *and* a stem-vowel change. A closed list:
    /// brennen, bringen, denken, kennen, nennen, rennen, senden, wenden,
    /// wissen, and their prefixed derivatives.
    Mixed,
    /// The six modal verbs, whose present tense inflects like a strong
    /// preterite (ich kann, er kann) and whose preterite is weak.
    Modal,
    /// sein, haben, werden and tun, whose paradigms none of the classes above
    /// describes.
    Irregular,
}

/// Whether a prefixed verb strands its prefix or keeps it attached.
///
/// Genuinely optional: a verb with no prefix (gehen, machen, singen) has no
/// separability at all, which is why this is an `Option` rather than a
/// three-valued enum with a `none` member.
///
/// The variable prefixes — durch-, hinter-, über-, um-, unter-, voll-, wider-,
/// wieder- — are separable or inseparable by *sense*, so the value describes the
/// reading realised in this occurrence: ǘbersetzen (to ferry across, separable)
/// and übersétzen (to translate, inseparable) are the same eight letters.
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
pub enum GermanSeparability {
    Separable,   // trennbar
    Inseparable, // untrennbar
}

/// The address register a second-person form belongs to.
///
/// German's formal Sie borrows third-person-plural morphology to address a
/// second person, which makes sie / sie / Sie the one genuinely ambiguous
/// pronoun of the language. Marking the register lets the formal form be
/// reported as what it *is* — second person with plural agreement — instead of
/// forcing a choice between losing its address value and losing its agreement.
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
pub enum GermanPoliteness {
    Familiar, // du / ihr
    Formal,   // Sie
}

/// What a particle token is doing.
///
/// Required, and it earns its place twice over. It gives the stranded half of a
/// separable verb somewhere to live — without it the `auf` of steht ... auf is
/// indistinguishable from the preposition auf — and it names the
/// Modalpartikeln, the unstressed flavouring words (doch, mal, ja, halt, eben)
/// that are a hallmark of spoken German and that every one of which doubles as
/// an adverb, a conjunction or an interjection with a wholly different meaning.
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
pub enum GermanParticleType {
    /// Modalpartikel / Abtönungspartikel — doch, mal, ja, halt, eben, wohl.
    Modal,
    /// nicht.
    Negation,
    /// The zu of a zu-infinitive, written as its own word (ohne zu fragen).
    Infinitival,
    /// A separable prefix stranded away from its verb (Ich stehe früh auf).
    SeparatedVerbPrefix,
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
pub enum GermanMorphology {
    /// Adjective — attributive, predicative, or an inflected participle.
    ///
    /// Everything but `degree` is optional together: an attributive adjective
    /// inflects and carries all four fields, while a predicative or adverbially
    /// used one (der Wein ist gut, er läuft schnell) is bare and carries none
    /// of them.
    Adjective {
        lemma: String,
        degree: GermanDegree,
        /// Which of the three paradigms the ending comes from. Absent on
        /// uninflected forms.
        #[serde(skip_serializing_if = "Option::is_none")]
        declension: Option<GermanAdjectiveDeclension>,
        /// Agreement gender; the plural does not distinguish it.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<GermanCase>,
    },
    /// Preposition or postposition, with the case it governs in this instance.
    ///
    /// Required, and the reason it is: the nine two-way prepositions govern the
    /// accusative for a change of place and the dative for a location, so only
    /// the occurrence settles which (in die Stadt vs in der Stadt).
    Adposition {
        lemma: String,
        case: GermanCase,
    },
    /// Adverb.
    Adverb {
        lemma: String,
        /// Only for an adverb whose own form encodes degree (oft / öfter, gern
        /// / lieber, bald / eher, gut / besser).
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<GermanDegree>,
    },
    /// Coordinating conjunction — und, oder, aber, denn, sondern.
    CoordinatingConjunction {
        lemma: String,
    },
    /// Determiner — articles, demonstratives, possessives, quantifiers.
    Determiner {
        lemma: String,
        /// The plural article and the plural possessive do not distinguish it.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        number: BinaryNumber,
        case: GermanCase,
    },
    /// Interjection.
    Interjection {
        lemma: String,
    },
    /// Noun.
    ///
    /// Plural class is deliberately not modelled — see the module-level note on
    /// `GermanMorphology` in the definition's report. Gender is, and it is
    /// required even in the plural, where the article stops showing it.
    Noun {
        lemma: String,
        /// Inherent and lexical. A compound takes the gender of its last
        /// element (die Tür → die Haustür).
        gender: TernaryGender,
        number: BinaryNumber,
        case: GermanCase,
    },
    /// Cardinal numeral.
    ///
    /// Bare, because modern German cardinals above one are indeclinable. The
    /// declining members of the numeral system are elsewhere: ein before a noun
    /// is a determiner, and the ordinals inflect exactly like adjectives and are
    /// analysed as such.
    Numeral {
        lemma: String,
    },
    /// Particle.
    Particle {
        lemma: String,
        particle_type: GermanParticleType,
    },
    /// Pronoun — personal, reflexive, possessive, demonstrative, relative,
    /// interrogative or indefinite.
    Pronoun {
        lemma: String,
        /// Personal, possessive and reflexive pronouns only.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Third person singular only.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: GermanCase,
        /// Second-person forms only — the du/ihr against the Sie register.
        #[serde(skip_serializing_if = "Option::is_none")]
        politeness: Option<GermanPoliteness>,
    },
    /// Proper noun.
    ///
    /// Case is fully syntactic and the bare genitive -s is still productive
    /// (Annas Buch); gender and number surface only where the name takes an
    /// article (der Rhein, die Schweiz) or is plural-only (die Alpen).
    ProperNoun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: GermanCase,
    },
    /// Subordinating conjunction — dass, weil, wenn, ob, obwohl.
    SubordinatingConjunction {
        lemma: String,
    },
    /// Symbol.
    Symbol {
        lemma: String,
    },
    /// Verb.
    ///
    /// `verb_class` and `separability` describe the lemma and hold on every
    /// token of it; everything else is decided by `verb_form`, and each
    /// `Option` below marks a cell German genuinely does not have.
    ///
    /// There is no voice field, and that is a decision rather than an omission:
    /// German has no synthetic passive whatsoever. Both the werden-passive and
    /// the sein-passive are an inflected auxiliary plus a Partizip II that is
    /// the very same form used in the perfect, so a voice value would have to be
    /// a syntactic judgement about a neighbouring token rather than a feature of
    /// the one being analysed.
    Verb {
        lemma: String,
        verb_class: GermanVerbClass,
        /// Prefixed verbs only; a verb with no prefix has no separability.
        #[serde(skip_serializing_if = "Option::is_none")]
        separability: Option<GermanSeparability>,
        verb_form: GermanVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<GermanMood>,
        /// Finite forms other than the imperative, which has no tense.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<GermanTense>,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    /// Other, for unanalyzable tokens.
    Other {
        lemma: String,
    },
}

impl GermanMorphology {
    /// Extracts the adjective declension class for the declension pivot.
    ///
    /// `declension` is `Option` (uninflected adjectives have none), so the
    /// `MorphologyInfo` derive skips it for pivot generation. Written by hand
    /// because strong/weak/mixed is the dimension German most deserves a facet
    /// for: it is the one nominal contrast the learner cannot look up in a
    /// dictionary and has to drill against real determiner phrases.
    fn __pivot_declension(&self) -> Option<String> {
        match self {
            Self::Adjective { declension, .. } => declension
                .as_ref()
                .map(|d| panini_core::aggregable::ClosedValues::variant_str(d).to_string()),
            _ => None,
        }
    }

    /// Extracts the separability of a prefixed verb for its pivot.
    ///
    /// `separability` is `Option` (an unprefixed verb has none), so the derive
    /// skips it.
    fn __pivot_separability(&self) -> Option<String> {
        match self {
            Self::Verb { separability, .. } => separability
                .as_ref()
                .map(|s| panini_core::aggregable::ClosedValues::variant_str(s).to_string()),
            _ => None,
        }
    }

    /// Extracts the mood for the mood pivot.
    ///
    /// `mood` is `Option` (non-finite forms have none), so the derive skips it.
    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood
                .as_ref()
                .map(|m| panini_core::aggregable::ClosedValues::variant_str(m).to_string()),
            _ => None,
        }
    }

    /// Extracts the tense for the tense pivot.
    ///
    /// `tense` is `Option` (imperatives and non-finite forms have none), so the
    /// derive skips it.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|t| panini_core::aggregable::ClosedValues::variant_str(t).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for the adjective declension class. Defined manually
    /// because `declension` is optional (see
    /// [`GermanMorphology::__pivot_declension`]).
    pub const PIVOT_DECLENSION: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "declension",
            "Declension",
            <GermanAdjectiveDeclension as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_declension,
        );

    /// Typed pivot handle for verb separability. Defined manually because
    /// `separability` is optional (see
    /// [`GermanMorphology::__pivot_separability`]).
    pub const PIVOT_SEPARABILITY: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "separability",
            "Separability",
            <GermanSeparability as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_separability,
        );

    /// Typed pivot handle for verb mood. Defined manually because `mood` is
    /// optional (see [`GermanMorphology::__pivot_mood`]).
    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <GermanMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    /// Typed pivot handle for verb tense. Defined manually because `tense` is
    /// optional (see [`GermanMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <GermanTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );
}

pub struct German;

impl LinguisticDefinition for German {
    type Morphology = GermanMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Deu;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        GermanMorphology::PIVOT_CASE,
        GermanMorphology::PIVOT_GENDER,
        GermanMorphology::PIVOT_NUMBER,
        GermanMorphology::PIVOT_DECLENSION,
        GermanMorphology::PIVOT_VERB_CLASS,
        GermanMorphology::PIVOT_SEPARABILITY,
        GermanMorphology::PIVOT_VERB_FORM,
        GermanMorphology::PIVOT_MOOD,
        GermanMorphology::PIVOT_TENSE,
    ];

    /// German is written in the Latin script alone. Fraktur has its own ISO
    /// 15924 code (Latf), but it is a typeface for the same alphabet and has
    /// been out of ordinary use since 1941 — it is a historical rendering of
    /// Latn, not a second script the language is written in today.
    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[
            TypologicalFeature::Conjugation(&[Upos::Verb]),
            // Numerals are absent on purpose: unlike Polish or Russian ones,
            // German cardinals above one do not decline, so a declension cloze
            // must never be handed one.
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
        "1. Lemmatization: nouns to the nominative singular, keeping the capital letter (Häusern → Haus); \
         pluralia tantum stay plural (Eltern, Leute, Ferien, Geschwister). Verbs to the infinitive WITH the \
         separable prefix reattached — see directive 10. Adjectives and ordinals to the uninflected positive \
         form (guten → gut, besser → gut, am größten → groß); ordinals are the one exception and lemmatize \
         to their dictionary form in -te / -ste (zweiten → zweite). Determiners and pronouns to the \
         masculine nominative singular citation form (dem → der, meiner → mein, ihn → er, uns → wir). \
         Prepositions to the uncontracted preposition (im → in, zur → zu). Lemmas use post-1996 reformed \
         orthography (dass, muss) and the ß spelling even when the input is Swiss and writes ss \
         (Strasse → Straße), so one lexeme keeps one record.\n\
         2. CAPITALIZATION IS NOT A PROPER-NOUN SIGNAL IN GERMAN. Every common noun is capitalized, so a \
         capital letter says nothing at all: Haus, Freiheit and Auto are ordinary Nouns. Tag a proper noun \
         only for an actual name of a person, place, organisation or work. Conversely, a capitalized \
         nominalization IS a Noun: das Gute, das Essen, etwas Schönes, beim Laufen, der Angestellte — \
         lemmatize these to the form that follows the definite article (Gute, Essen, Angestellte). The one \
         place a capital does carry information is the polite Sie / Ihnen / Ihr — see directive 9.\n\
         3. Nouns: always give gender, number and case. Gender is inherent and lexical, so report it in the \
         PLURAL too, where the article no longer shows it (die Bücher → neuter, die Männer → masculine). A \
         compound takes the gender of its LAST element (die Tür → die Haustür feminine; der Wagen → der \
         Kinderwagen masculine); never guess the gender from the first element or from the ending alone.\n\
         4. Case is SYNTACTIC and must never be read off the ending, because the German endings are massively \
         syncretic. der is nominative singular masculine, but also genitive singular feminine, dative \
         singular feminine and genitive plural; die is nominative and accusative, singular feminine and \
         plural alike; den is accusative singular masculine and dative plural; dem is dative singular \
         masculine and neuter. Work from the function: subject and predicate nominal → nominative; direct \
         object → accusative; indirect object and the object of a dative verb (helfen, danken, gefallen, \
         gehören, folgen) → dative; adnominal possessor → genitive. A noun ending in -n in the plural after \
         a preposition or as an indirect object (mit den Kindern, den Freunden) is a reliable dative plural.\n\
         5. Prepositions: report the case governed IN THIS INSTANCE, never the preposition's whole range. \
         The nine two-way prepositions — an, auf, hinter, in, neben, über, unter, vor, zwischen — take the \
         accusative for a change of place answering wohin (in die Stadt, auf den Tisch) and the dative for a \
         location answering wo (in der Stadt, auf dem Tisch). With a non-spatial governing verb the case is \
         lexical, not directional: denken an and sich erinnern an take the accusative, Angst vor and teilnehmen \
         an take the dative. Fixed-case prepositions still get their case reported: durch, für, gegen, ohne, \
         um, bis are accusative; aus, bei, mit, nach, seit, von, zu, gegenüber, außer are dative; während, \
         wegen, trotz, statt, innerhalb, außerhalb are genitive.\n\
         6. The genitive is receding in speech in favour of von plus dative, and of the dative after \
         prepositions. REPORT THE CASE THE TEXT ACTUALLY REALIZES, never the prescriptive one: das Auto von \
         meinem Vater is von governing the DATIVE, not a genitive; wegen dem Regen is wegen governing the \
         DATIVE even though the standard asks for the genitive. Do not silently normalize either way.\n\
         7. Adjectives — the declension class is decided by the PRECEDING DETERMINER, not by the ending and \
         not by the adjective itself:\n\
         - after a der-word (der/die/das, dieser, jener, jeder, welcher, solcher, mancher, alle, beide, \
         derselbe) → weak.\n\
         - after an ein-word (ein, kein, mein, dein, sein, ihr, unser, euer, Ihr) → mixed, across the WHOLE \
         paradigm and not only in the three endingless cells where the strong ending surfaces (ein guter \
         Wein AND eines guten Weines are both mixed).\n\
         - with no determiner at all, or after an endingless quantifier (viel, wenig, etwas, mehr, ein paar, \
         a cardinal above one) → strong: guter Wein, kaltes Wasser, viel frisches Obst.\n\
         An attributive adjective gets declension, gender, number and case. A predicative one (der Wein ist \
         gut, sie wird müde) and an adverbially used one (er läuft schnell) are UNINFLECTED: report degree \
         only and OMIT declension, gender, number and case. The am -sten superlative is likewise uninflected \
         (am schnellsten → degree superlative, nothing else; am is a separate Adposition with case dative).\n\
         8. An INFLECTED participle is an Adjective, not a Verb: der lachende Mann, das gebaute Haus, ein \
         interessiertes Kind all get degree positive plus declension, gender, number and case. Only the \
         uninflected participles are Verbs (hat gebaut, wird gebaut, kam singend herein).\n\
         9. sie / sie / Sie is the one genuinely ambiguous German pronoun and must be resolved from \
         agreement and context, never from the spelling alone:\n\
         - sie with a SINGULAR verb → third person, singular, feminine (sie geht).\n\
         - sie with a PLURAL verb → third person, plural; omit gender (sie gehen).\n\
         - Sie capitalized MID-SENTENCE with a plural verb → second person, number plural, politeness \
         formal (Können Sie mir helfen). Mid-sentence Ihnen and Ihr are the same register.\n\
         - At the START of a sentence the capital is uninformative, because every sentence starts with one. \
         Decide from whether the clause addresses the interlocutor, and from the possessives and reflexives \
         around it (Ihr / sich versus ihr).\n\
         ihr is three different words: the second person plural nominative pronoun (ihr geht → person second, \
         plural, politeness familiar), the third person singular feminine DATIVE pronoun (ich gebe ihr das \
         Buch), and the possessive Determiner (ihr Buch = her or their). Give politeness only on \
         second-person forms: du, dich, dir, ihr, euch → familiar; Sie, Ihnen → formal. Omit it everywhere \
         else.\n\
         10. Verbs — SEPARABLE PREFIXES. In a main clause the prefix is stranded at the end of the clause \
         (Ich stehe früh auf; Er macht die Tür zu), and in a subordinate clause, an infinitive or a \
         participle it is attached (dass ich früh aufstehe; aufgestanden; aufzustehen). ALWAYS LEMMATIZE THE \
         STRANDED CASE BACK TO THE WHOLE VERB: steht ... auf → aufstehen, machte ... zu → zumachen, \
         ruft ... an → anrufen. The stranded prefix is emitted as its own Particle token with \
         particle_type separated_verb_prefix and its own written form as the lemma (auf, not aufstehen), so \
         the verb and the prefix are never counted as one lexeme twice. The zu infixed inside a separable \
         infinitive (aufzustehen, anzurufen) is NOT a separate token — one Verb token, verb_form infinitive.\n\
         Report separability on every prefixed verb. Always separable: ab-, an-, auf-, aus-, bei-, ein-, \
         mit-, nach-, vor-, zu-, zurück-, weg-, los-, fest-, her-, hin- and their compounds (herunter-, \
         hinein-). Always inseparable: be-, emp-, ent-, er-, ge-, miss-, ver-, zer-. The variable prefixes \
         durch-, hinter-, über-, um-, unter-, voll-, wider-, wieder- go by sense — separable when the prefix \
         keeps its literal spatial meaning (Er setzt uns über = ferries us across), inseparable when the \
         verb is figurative (Er übersetzt den Text = translates it). Verb-plus-verb and noun-plus-verb \
         compounds behave as separable: teilnehmen, kennenlernen, stattfinden, spazieren gehen. OMIT \
         separability entirely for a verb with no prefix (gehen, machen, singen).\n\
         11. Verbs — verb_class is a property of the LEMMA and must be identical on every token of it, \
         including the present tense where the class is invisible (ich singe → strong, ich mache → weak). \
         Weak: dental suffix, no stem-vowel change (machen, machte, gemacht). Strong: ablaut in the \
         preterite and a participle in -en (singen, sang, gesungen; gehen, ging, gegangen; stehen, stand, \
         gestanden). Mixed is a CLOSED LIST — brennen, bringen, denken, kennen, nennen, rennen, senden, \
         wenden, wissen — and nothing else. Modal is the six modals: dürfen, können, mögen, müssen, sollen, \
         wollen. Irregular is sein, haben, werden and tun, and nothing else. A prefixed verb INHERITS the \
         class of its base: verstehen and aufstehen are strong like stehen, erkennen is mixed like kennen, \
         besuchen is weak like suchen.\n\
         12. Verbs — which fields apply follows from verb_form, and each omission is a cell German does not \
         have:\n\
         - finite indicative, Konjunktiv I or Konjunktiv II: mood, tense, person and number.\n\
         - imperative: mood imperative, person and number, NO TENSE. The du-form is second singular (Geh!), \
         the ihr-form second plural (Geht!), the Sie-form second plural with the obligatory Sie pronoun \
         carrying the formal register (Gehen Sie!), and the adhortative wir-form is first plural \
         (Gehen wir!).\n\
         - infinitive: verb_class and separability only; no mood, tense, person or number.\n\
         - present_participle and past_participle: verb_class and separability only. If the participle is \
         inflected it is an Adjective instead (directive 8).\n\
         Tense names the STEM, not the time referred to: Konjunktiv I is built on the present stem and takes \
         tense present (er habe, er sei, er komme), Konjunktiv II on the preterite stem and takes tense past \
         (er hätte, er wäre, er käme, er würde, er könnte).\n\
         DISTINGUISH KONJUNKTIV II FROM THE PRETERITE INDICATIVE BY THE UMLAUT, not by the ending: konnte, \
         hatte, wurde, war, musste are indicative past; könnte, hätte, würde, wäre, müsste are Konjunktiv II. \
         Where Konjunktiv I is homophonous with the indicative (ich habe, wir haben, sie haben) German \
         substitutes Konjunktiv II — tag the form actually written, so hätten in reported speech is \
         Konjunktiv II, not Konjunktiv I.\n\
         13. Every compound tense is TWO tokens, never one value: hat gesagt is haben (finite, present, \
         indicative) plus sagen (past_participle); wird kommen is werden (finite, present, indicative) plus \
         kommen (infinitive); würde kommen is werden (finite, past, Konjunktiv II) plus kommen (infinitive); \
         wird gebaut is werden plus bauen (past_participle). German has no synthetic future and no synthetic \
         passive. There is no voice field in this model, so never try to encode the passive anywhere — \
         the Partizip II of wird gebaut and of hat gebaut is the same form and is analysed identically.\n\
         14. Particles: nicht is negation; the free-standing zu of a zu-infinitive is infinitival; a \
         stranded verb prefix is separated_verb_prefix (directive 10); an unstressed flavouring word is \
         modal. Modal particles are ambiguous with their literal homographs and must be decided from the \
         reading: denn is a CoordinatingConjunction in Ich bleibe, denn es regnet but a modal particle in \
         Was machst du denn; ja is an Interjection when it answers a question but a modal particle in Das ist \
         ja toll; doch, mal, schon, eben and nur likewise alternate between Adverb and modal particle. Words \
         that merely look like particles are not: kein is a Determiner, sich is a Pronoun, expletive es is a \
         Pronoun, and the am of am schnellsten is an Adposition.\n\
         15. Tokenization: a preposition contracted with its article (im, ins, am, ans, aufs, beim, vom, zum, \
         zur, fürs, durchs, übers) stays ONE Adposition token, lemma the bare preposition and case the one \
         the fused article marks — never emit a separate determiner for it. Compounds are ONE token with ONE \
         lemma (Geschwindigkeitsbegrenzung, Handschuh, Kindergarten): never split them and never lemmatize to \
         the head alone. In a suspended compound (Ein- und Ausgang) the fragment is a Noun whose lemma is the \
         compound it stands for (Ein- → Eingang). A reflexive sich, mich or dir is a separate Pronoun token \
         and the verb keeps its bare lemma (sich freuen → the verb lemma is freuen). The genitive -s of a \
         proper noun belongs to that token and is case genitive, not a separate particle (Annas → lemma Anna, \
         genitive). Split a written enclitic into its constituents (gibt's → gibt plus 's with lemma es). \
         Never emit punctuation as a token."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn german_identity_script_and_typology_are_exact() {
        let language = German;

        assert_eq!(German::ISO_LANG, IsoLang::Deu);
        assert_eq!(German::ISO_LANG.to_639_3(), "deu");
        assert_eq!(language.supported_scripts(), &[Script::LATN]);
        assert_eq!(language.default_script(), Script::LATN);
    }

    /// The two Konjunktive would collide under serde's naive snake_case rule,
    /// which inserts a separator before every uppercase letter. The explicit
    /// renames are what keep them apart on the wire, and nothing else checks it.
    #[test]
    fn the_two_konjunktive_have_distinct_wire_values() {
        use panini_core::aggregable::ClosedValues;

        assert_eq!(GermanMood::SubjunctiveI.variant_str(), "subjunctive_i");
        assert_eq!(GermanMood::SubjunctiveII.variant_str(), "subjunctive_ii");
        assert_eq!(
            GermanMood::all_variants(),
            &["indicative", "imperative", "subjunctive_i", "subjunctive_ii"]
        );
    }

    /// The four hand-written pivots exist precisely because the derive skips
    /// optional fields, so they are the ones with no generated coverage.
    #[test]
    fn hand_written_pivots_extract_from_optional_fields() {
        let attributive = GermanMorphology::Adjective {
            lemma: "gut".to_string(),
            degree: GermanDegree::Positive,
            declension: Some(GermanAdjectiveDeclension::Mixed),
            gender: Some(TernaryGender::Masculine),
            number: Some(BinaryNumber::Singular),
            case: Some(GermanCase::Nominative),
        };
        let separable = GermanMorphology::Verb {
            lemma: "aufstehen".to_string(),
            verb_class: GermanVerbClass::Strong,
            separability: Some(GermanSeparability::Separable),
            verb_form: GermanVerbForm::Finite,
            mood: Some(GermanMood::SubjunctiveII),
            tense: Some(GermanTense::Past),
            person: Some(Person::Third),
            number: Some(BinaryNumber::Singular),
        };

        assert_eq!(
            GermanMorphology::PIVOT_DECLENSION.value(&attributive),
            Some("mixed".to_string())
        );
        assert_eq!(GermanMorphology::PIVOT_DECLENSION.value(&separable), None);
        assert_eq!(
            GermanMorphology::PIVOT_SEPARABILITY.value(&separable),
            Some("separable".to_string())
        );
        assert_eq!(
            GermanMorphology::PIVOT_MOOD.value(&separable),
            Some("subjunctive_ii".to_string())
        );
        assert_eq!(
            GermanMorphology::PIVOT_TENSE.value(&separable),
            Some("past".to_string())
        );
    }

    /// A predicative adjective carries none of the four inflectional fields, so
    /// the declension pivot must yield nothing rather than a default.
    #[test]
    fn an_uninflected_adjective_has_no_declension_class() {
        let predicative = GermanMorphology::Adjective {
            lemma: "gut".to_string(),
            degree: GermanDegree::Positive,
            declension: None,
            gender: None,
            number: None,
            case: None,
        };

        assert_eq!(GermanMorphology::PIVOT_DECLENSION.value(&predicative), None);
    }
}
