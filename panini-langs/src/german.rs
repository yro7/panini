use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryGender, TypologicalFeature,
    Upos,
};

/// The four cases of modern German.
///
/// The vocative has no forms of its own (direct address takes the
/// nominative); frozen adverbs such as deswegen, meinetwegen are lexicalised,
/// not an instrumental.
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
/// The ending on an attributive adjective is decided by how much of its
/// gender, number and case the *preceding determiner* has already spelled
/// out.
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
/// so they are analysed as the two verb tokens they are written as.
///
/// The value names the stem the finite form is built on: Konjunktiv I, built
/// on the present stem, is `Present`; Konjunktiv II, built on the preterite
/// stem, is `Past`, whatever time either one refers to.
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
/// Konjunktiv I is built on the present stem and is the mood of reported
/// speech (der Minister sagte, er habe nichts gewusst); Konjunktiv II is built
/// on the preterite stem, usually with umlaut, and is the mood of the
/// counterfactual, the wish and the polite request (wenn ich Zeit hätte; ich
/// könnte).
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
    // Both are renamed explicitly: serde's snake_case rule inserts a separator
    // before every uppercase letter, so `SubjunctiveII` would otherwise
    // serialise as `subjunctive_i_i`.
    #[serde(rename = "subjunctive_i")]
    SubjunctiveI, // Konjunktiv I — present stem, reported speech
    #[serde(rename = "subjunctive_ii")]
    SubjunctiveII, // Konjunktiv II — preterite stem, counterfactual and polite
}

/// Which slot of the verbal system a verb token occupies.
///
/// Required on every verb; which of the remaining fields apply follows from
/// it. Only the *uninflected* participles are verbs here: an attributive
/// participle (der lachende Mann, das gebaute Haus) takes adjective endings
/// and is analysed as an adjective.
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
/// A property of the lemma, not of the token: it is invisible in the present
/// tense (ich singe, ich mache) and decides the preterite and the participle.
/// Reported on every token of the lemma.
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
    // Schwach — dental suffix, no stem-vowel change: machen, machte, gemacht.
    Weak,
    // Stark — ablaut in the preterite, participle in -en: singen, sang, gesungen.
    Strong,
    // Gemischt — dental suffix *and* a stem-vowel change. A closed list:
    // brennen, bringen, denken, kennen, nennen, rennen, senden, wenden, wissen,
    // and their prefixed derivatives.
    Mixed,
    // The six modal verbs, whose present tense inflects like a strong preterite
    // (ich kann, er kann) and whose preterite is weak.
    Modal,
    // sein, haben, werden and tun, whose paradigms none of the classes above
    // describes.
    Irregular,
}

/// Whether a prefixed verb strands its prefix or keeps it attached.
///
/// A verb with no prefix (gehen, machen, singen) has no separability at all.
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
/// The formal Sie borrows third-person-plural morphology to address a second
/// person: report it as second person with plural agreement, `formal`.
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
/// Required. The stranded half of a separable verb (the `auf` of steht ... auf)
/// is a particle, not the preposition auf. The Modalpartikeln are the
/// unstressed flavouring words (doch, mal, ja, halt, eben), each of which also
/// exists as an adverb, a conjunction or an interjection with a different
/// meaning.
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
    Modal, // Modalpartikel / Abtönungspartikel — doch, mal, ja, halt, eben, wohl, and unstressed clause-internal bitte
    Negation,   // nicht
    Infinitival, // the zu of a zu-infinitive, written apart (ohne zu fragen)
    SeparatedVerbPrefix, // the stranded prefix of a separable verb (steht ... auf)
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
    /// A PREDICATIVE adjective, the complement of sein, werden or bleiben
    /// (der Wein ist gut, sie wird müde, der Biergarten ist voll), belongs here
    /// and NOT under `Adverb`, even though it carries no ending.
    ///
    /// Everything but `degree` is optional together: an attributive adjective
    /// inflects and carries all four fields, while a predicative one is bare
    /// and carries none of them.
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
    /// Required: the nine two-way prepositions govern the accusative for a
    /// change of place and the dative for a location, so only the occurrence
    /// settles which (in die Stadt vs in der Stadt).
    Adposition {
        lemma: String,
        case: GermanCase,
    },
    /// Adverb — a word modifying a verb, an adjective or the whole clause
    /// (er läuft schnell, sie kommt heute).
    ///
    /// Never the complement of sein, werden or bleiben: that is a predicative
    /// Adjective.
    Adverb {
        lemma: String,
        /// For an adverb that compares in its own form — regularly (früh /
        /// früher, schnell / schneller) or suppletively (oft / öfter, gern /
        /// lieber, bald / eher, gut / besser) — and then always, so the
        /// positive is reported as `positive` rather than omitted. Absent on
        /// an adverb that does not compare (heute, hier, nachts, vorbei).
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<GermanDegree>,
    },
    /// Coordinating conjunction — und, oder, aber, denn, sondern — and the
    /// als and wie of comparison (größer als ich, so groß wie er), which link
    /// two constituents of equal rank rather than open a subordinate clause.
    CoordinatingConjunction {
        lemma: String,
    },
    /// Determiner — the articles (der, die, das, ein), the demonstratives
    /// (dieser, jener), the possessives before a noun (mein, sein, Ihr, in
    /// every inflected form), and the quantifiers.
    Determiner {
        lemma: String,
        /// The plural article and the plural possessive do not distinguish it.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        number: BinaryNumber,
        case: GermanCase,
        /// Possessive determiners of the second person only: `Ihr Wörterbuch`
        /// (your, formal) is `formal`, `dein` and `euer` are `familiar`. The
        /// third-person `ihr Wörterbuch` (her / their) and every other
        /// determiner omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        politeness: Option<GermanPoliteness>,
    },
    /// Interjection.
    Interjection {
        lemma: String,
    },
    /// Noun.
    ///
    /// Gender is reported even in the plural, where the article stops showing
    /// it; only a plurale tantum has none to report.
    Noun {
        lemma: String,
        /// Inherent and lexical, and reported in the plural too (die Bücher →
        /// neuter). A compound takes the gender of its last element (die Tür
        /// → die Haustür). Absent only on a plurale tantum — Leute, Eltern,
        /// Ferien, Geschwister, Kosten — which has no singular to carry one.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        number: BinaryNumber,
        case: GermanCase,
    },
    /// Cardinal numeral.
    ///
    /// Cardinals above one are indeclinable. ein before a noun is a
    /// determiner; ordinals inflect like adjectives and are analysed as
    /// adjectives.
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
    /// token of it; everything else is decided by `verb_form`. There is no
    /// voice field: the werden- and sein-passives are an auxiliary plus a
    /// Partizip II, analysed as the two tokens they are written as.
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
    /// Gender across every variant that carries it. Hand-written because
    /// `Noun.gender` became `Option` for the pluralia tantum, and the derive
    /// skips optional fields — while gender is the first facet a German
    /// learner reaches for.
    fn __pivot_gender(&self) -> Option<String> {
        let gender = match self {
            Self::Noun { gender, .. }
            | Self::Adjective { gender, .. }
            | Self::Determiner { gender, .. }
            | Self::Pronoun { gender, .. }
            | Self::ProperNoun { gender, .. } => gender.as_ref(),
            _ => None,
        };
        gender.map(|g| panini_core::aggregable::ClosedValues::variant_str(g).to_string())
    }

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
    pub const PIVOT_GENDER: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "gender",
            "Gender",
            <TernaryGender as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_gender,
        );
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
        "1. A DETERMINER lemmatizes to the masculine nominative singular citation form: die, das, den, \
         dem, der (any cell but masc.nom.sg) → der; eine, einen, einem → ein; meine, meiner → mein; \
         dieses, diese → dieser. A PERSONAL pronoun lemmatizes to the nominative of its OWN person and number, never to \
         another one: ihn and ihm → er, ihr (dative) → sie, uns → wir, mich and mir → ich, dich and \
         dir → du, euch → ihr, sich → sich. A form that is already nominative is its own lemma \
         (ich → ich, wir → wir, du → du, es → es) — never lemmatize it to a different pronoun.\n\
         2. The noun is the ONLY part of speech that keeps gender in the plural. A plural determiner, \
         adjective, pronoun or proper noun does not distinguish it and must OMIT gender entirely \
         (meine Hausaufgaben → the determiner is number plural, case accusative, no gender; die Bücher → \
         the determiner has no gender while the noun is neuter).\n\
         3. A PREDICATIVE adjective after sein, werden or bleiben is an Adjective, not an Adverb \
         (der Biergarten ist voll, sie wird müde, das ist teuer). It is uninflected, so report degree \
         and OMIT declension, gender, number and case. Tag Adverb only for a word modifying a verb, an \
         adjective or the whole clause (er läuft schnell).\n\
         4. `other` is for genuinely unanalyzable tokens only. A word with an ordinary part of speech \
         NEVER gets it: the adverbs nachts, morgens, vorbei, hier, dort and gern are Adverbs.\n\
         5. A preposition contracted with its article (im, ins, am, ans, aufs, beim, vom, zum, zur, \
         fürs, durchs, übers) stays ONE Adposition token, lemma the bare preposition, case the one the \
         fused article marks. That article is still a DEFINITE article for everything that follows, so \
         an adjective after it takes the WEAK declension (ans andere Ufer → andere is weak, not mixed; \
         im neuen Haus → neuen is weak).\n\
         6. verb_class is a property of the LEMMA, identical on every token of it, including the \
         present tense where the class is invisible. `mixed` is a CLOSED LIST — brennen, bringen, \
         denken, kennen, nennen, rennen, senden, wenden, wissen and their prefixed derivatives — and \
         `irregular` is a CLOSED LIST too: sein, haben, werden and tun, and nothing else. So weiß is \
         wissen and mixed, never irregular. `modal` is the six modals (dürfen, können, mögen, müssen, \
         sollen, wollen). A prefixed verb INHERITS its base's class: aufstehen is strong like stehen, \
         erkennen is mixed like kennen, besuchen is weak like suchen, übersetzen is weak like setzen \
         (setzte, gesetzt) — a stem-vowel change in the present (setzt) is not ablaut.\n\
         7. politeness belongs to SECOND-PERSON forms only, on Pronouns AND on possessive \
         Determiners — du, dich, dir, ihr, euch, dein, euer are familiar; Sie, Ihnen and the \
         possessive Ihr are formal. This is the ONLY thing separating the formal Ihr Wörterbuch \
         (your) from the third-person ihr Wörterbuch (her / their), which are otherwise identical, so \
         decide it from whether the clause addresses the interlocutor. OMIT politeness everywhere \
         else: a first-person or third-person form (ich, wir, er, sie, es, sein, ihr = her) never \
         carries it.\n\
         8. The articles der, die, das, ein, eine and their inflected forms (den, dem, des, einen, \
         einem, einer, eines), the demonstratives (dieser, jener) and the possessives before a noun \
         (mein, dein, sein, ihr, unser, euer, Ihr, in every inflected form: meinem, seinem, ihrer) \
         are Determiners. A possessive is a Pronoun only when it stands alone in place of the noun \
         (Das ist meins).\n\
         9. ProperNoun is for an actual name of a person, place, organisation or work (Anna, Berlin, \
         die Schweiz). Capitalization carries no information about it in German, where every common \
         noun is capitalized: Haus, Freiheit and Auto are ordinary Nouns, as is a capitalized \
         nominalization (das Gute, das Essen, beim Laufen).\n\
         10. The suppletive adverbs lemmatize to their positive form: lieber and am liebsten → gern; \
         besser and am besten → gut; öfter → oft; eher → bald; mehr → viel.\n\
         11. Unstressed bitte inside a clause (Nehmen Sie bitte Platz; Gib mir bitte das Buch) is a \
         Particle, particle_type modal — the same family as mal and doch.\n\
         12. After a comparative, or after so / genauso / ebenso, als and wie are \
         CoordinatingConjunctions (größer als ich, so groß wie er), and the phrase after them takes \
         the CASE of the constituent it is compared with: die alten Bäume sind größer als die jungen \
         → die jungen is nominative like die alten Bäume; ich kenne ihn besser als dich → dich is \
         accusative like ihn. The wie that opens a question (Wie geht es dir? Wie heißen Sie?) is an \
         Adverb.\n\
         13. A separable prefix stranded at the end of a main clause (Ich stehe früh auf; Paul kommt \
         nicht mit; Er macht die Tür zu) is a Particle, particle_type separated_verb_prefix, with its \
         own written form as the lemma (auf, mit, zu). The finite verb it belongs to is lemmatized \
         to the WHOLE verb: stehe … auf → aufstehen, kommt … mit → mitkommen, macht … zu → \
         zumachen, with separability separable. The stranded prefix is never an Adposition and never \
         an Adverb."
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

    /// A plurale tantum has no gender to report, and the gender pivot — hand
    /// written because the field is optional — must yield nothing for it while
    /// still reading every other noun.
    #[test]
    fn a_plurale_tantum_has_no_gender_and_other_nouns_do() {
        let leute = GermanMorphology::Noun {
            lemma: "Leute".to_string(),
            gender: None,
            number: BinaryNumber::Plural,
            case: GermanCase::Nominative,
        };
        let buecher = GermanMorphology::Noun {
            lemma: "Buch".to_string(),
            gender: Some(TernaryGender::Neuter),
            number: BinaryNumber::Plural,
            case: GermanCase::Nominative,
        };

        assert_eq!(GermanMorphology::PIVOT_GENDER.value(&leute), None);
        assert_eq!(
            GermanMorphology::PIVOT_GENDER.value(&buecher),
            Some("neuter".to_string())
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
