use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryGender, TypologicalFeature,
    Upos,
};

/// The four productive cases of Standard Modern Greek.
///
/// The inherited dative survives only inside fixed learned expressions and
/// has no value here; such frozen forms are analyzed according to their
/// current lexical use.
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
pub enum GreekCase {
    Nominative,
    Genitive,
    Accusative,
    Vocative,
}

/// The aspect contrast expressed by the Modern Greek verb stem.
///
/// Perfect constructions are analytic (`έχω γράψει`), so "perfect" is not a
/// third token-level value: the auxiliary and invariant perfective form are
/// analyzed separately.
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
pub enum GreekAspect {
    Imperfective,
    Perfective,
}

/// Morphologically marked mood of a finite verb.
///
/// The traditional "subjunctive" is a construction headed by `να` or `ας`,
/// not a third inflectional mood. Its following verb retains indicative-form
/// morphology, while aspect supplies the contrast (`να γράφω` / `να γράψω`).
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
pub enum GreekMood {
    Indicative,
    Imperative,
}

/// Tense carried by the verb form itself.
///
/// Past imperfective and past perfective forms are distinguished by
/// [`GreekAspect`]. Future time is built with the separate particle `θα`; there
/// is no synthetic future value on the lexical verb.
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
pub enum GreekTense {
    Present,
    Past,
}

/// The morphological opposition between the two Modern Greek conjugations.
///
/// `Mediopassive` names the form, not the meaning: the `-μαι` conjugation
/// includes passive, middle, reflexive and deponent verbs.
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
pub enum GreekVoice {
    Active,
    Mediopassive,
}

/// The four verb forms used in contemporary Standard Modern Greek.
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
pub enum GreekVerbForm {
    Finite,
    /// The invariant perfective form used only after `έχω`, as in `έχω γράψει`.
    PerfectDependent,
    /// An agreeing verbal participle, principally the `-μένος/-μένη/-μένο` type.
    Participle,
    /// The indeclinable adverbial form in `-οντας/-ώντας`.
    Converb,
}

/// Degrees distinguished in the contemporary adjective/adverb system.
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
pub enum GreekDegree {
    Positive,
    Comparative,
    /// Relative superlative, normally article plus comparative form.
    Superlative,
    /// Absolute superlative in `-ότατος/-ότατα` and related formations.
    AbsoluteSuperlative,
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
pub enum GreekDefiniteness {
    Definite,
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
pub enum GreekDeterminerType {
    Article,
    Possessive,
    Demonstrative,
    Interrogative,
    Relative,
    Indefinite,
    Quantifier,
    Emphatic,
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
pub enum GreekPronounType {
    Personal,
    Possessive,
    Demonstrative,
    Reflexive,
    Reciprocal,
    Interrogative,
    Relative,
    Indefinite,
    Emphatic,
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
pub enum ModernGreekMorphology {
    /// Adjectives agree with their head in gender, number and case.
    Adjective {
        lemma: String,
        gender: TernaryGender,
        number: BinaryNumber,
        case: GreekCase,
        /// Absent for adjectives to which comparison genuinely does not apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<GreekDegree>,
    },
    /// Preposition, with the case governed in this occurrence.
    Adposition {
        lemma: String,
        case: GreekCase,
    },
    Adverb {
        lemma: String,
        /// Absent for non-gradable adverbs such as temporal `σήμερα`.
        #[serde(skip_serializing_if = "Option::is_none")]
        degree: Option<GreekDegree>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: GreekDeterminerType,
        /// Articles only; other determiner types do not encode definiteness.
        #[serde(skip_serializing_if = "Option::is_none")]
        definiteness: Option<GreekDefiniteness>,
        /// Invariant determiners such as `κάθε` omit all three agreement fields.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<GreekCase>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        gender: TernaryGender,
        number: BinaryNumber,
        case: GreekCase,
    },
    /// Numerals which inflect expose their marked agreement features; invariant
    /// cardinals do not acquire invented values merely from the noun they count.
    Numeral {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<GreekCase>,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        pronoun_type: GreekPronounType,
        /// Personal pronouns only.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Omit when the pronoun does not distinguish gender.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Omit for invariant pronouns which stand outside the number contrast.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: GreekCase,
        /// True only for an unstressed weak personal-pronoun form.
        clitic: bool,
    },
    ProperNoun {
        lemma: String,
        gender: TernaryGender,
        number: BinaryNumber,
        case: GreekCase,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    /// Finite forms, the invariant perfect form, participles and converbs.
    Verb {
        lemma: String,
        aspect: GreekAspect,
        /// Morphological conjugation, independent of semantic voice.
        voice: GreekVoice,
        verb_form: GreekVerbForm,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        mood: Option<GreekMood>,
        /// Finite indicative present and past forms only. Imperatives,
        /// perfective non-past forms and non-finite forms omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<GreekTense>,
        /// Finite forms only.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms and agreeing participles only.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Agreeing participles only.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<TernaryGender>,
        /// Agreeing participles only.
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<GreekCase>,
    },
    Other {
        lemma: String,
    },
}

impl ModernGreekMorphology {
    fn __pivot_degree(&self) -> Option<String> {
        match self {
            Self::Adjective { degree, .. } | Self::Adverb { degree, .. } => degree
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_definiteness(&self) -> Option<String> {
        match self {
            Self::Determiner { definiteness, .. } => definiteness
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_mood(&self) -> Option<String> {
        match self {
            Self::Verb { mood, .. } => mood
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    pub const PIVOT_DEGREE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "degree",
            "Degree",
            <GreekDegree as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_degree,
        );

    pub const PIVOT_DEFINITENESS: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "definiteness",
            "Definiteness",
            <GreekDefiniteness as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_definiteness,
        );

    pub const PIVOT_MOOD: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "mood",
            "Mood",
            <GreekMood as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_mood,
        );

    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <GreekTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );
}

pub struct ModernGreek;

impl LinguisticDefinition for ModernGreek {
    type Morphology = ModernGreekMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Ell;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        ModernGreekMorphology::PIVOT_CASE,
        ModernGreekMorphology::PIVOT_GENDER,
        ModernGreekMorphology::PIVOT_NUMBER,
        ModernGreekMorphology::PIVOT_DEGREE,
        ModernGreekMorphology::PIVOT_DEFINITENESS,
        ModernGreekMorphology::PIVOT_CLITIC,
        ModernGreekMorphology::PIVOT_ASPECT,
        ModernGreekMorphology::PIVOT_VOICE,
        ModernGreekMorphology::PIVOT_VERB_FORM,
        ModernGreekMorphology::PIVOT_MOOD,
        ModernGreekMorphology::PIVOT_TENSE,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::GREK]
    }

    fn default_script(&self) -> Script {
        Script::GREK
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
        "0. NON-NEGOTIABLE OUTPUT NORMALIZATION: before assigning morphology, split EVERY complete στη/στο/στον/στην/στις/στους/στα token, including repeated and sentence-initial occurrences, into lowercase word 'σε' plus the restored article. Never emit a fused στ- form as the feature word. A standalone Greek letter glyph such as σ or ς, whether target or context, is always Symbol with canonical lemma 'σ', never Other. A noun governed by the restored article receives the syntactic case, so 'στο τέλος' emits 'σε' (Adposition, accusative), 'το' (definite article, neuter singular accusative), and 'τέλος' (neuter singular accusative).\n\
         0a. Feature partition, completeness and dialogue markup: each lexical occurrence belongs to exactly one list. Emit every non-punctuation Greek word in the sentence exactly once: a requested occurrence in target_features and every other occurrence in context_features. Within each list preserve the occurrences' left-to-right order in the sentence after the other list's items are removed. Never repeat a target occurrence in context_features or silently omit an ordinary context word such as 'γρήγορα' or interrogative 'πού'; include the same spelling in context only when it is a separate occurrence elsewhere in the sentence. Match the requested surface spelling exactly when capitalization distinguishes occurrences: in 'Χάρηκα! Κι εγώ χάρηκα', requested capitalized Χάρηκα is target-only, while context_features contains Κι, εγώ, then the later lowercase χάρηκα in that order. Target 'πιω' in 'θέλω να πιω τσάι' must not reappear in context. Dialogue labels 'A:' and 'B:' are structural markup, not Greek words: omit A and B entirely rather than emitting Other.\n\
         0b. Compound targets are excluded from context in full: when 'την' and 'οποία' are both targets in 'με την οποία', emit both only in target_features and emit neither occurrence again in context_features.\n\
         0c. A fused σε-plus-article target remains target-only after decomposition. If requested 'στον' in 'Στο τηλέφωνο μιλούν συχνά ο ένας στον άλλον' is emitted as target constituents 'σε' plus 'τον', neither constituent may be duplicated in context and the original fused 'στον' may not appear there as Other. For that exact sentence, context_features ends after συχνά; its only contraction constituents are the initial context 'Στο' -> 'σε' plus 'το'.\n\
         1. Scope, orthography and lemmas: analyze contemporary Standard Modern Greek in monotonic Greek script. Keep the written tonos and dialytika, normalize final sigma correctly, and never add polytonic breathings. Day and month names are lowercase common Nouns unless sentence-initial capitalization applies ('παρασκευή', 'αύγουστος', 'ιούλιος'), never ProperNoun merely because English capitalizes them. Lemmatize nouns and proper nouns to nominative singular (keep a conventional plural lemma for pluralia tantum such as 'διακοπές'), adjectives to masculine nominative singular positive, and verbs to the imperfective dictionary first-person singular present ('έγραψα' -> 'γράφω', 'ήρθα' -> 'έρχομαι'). Aggregate a productive mediopassive form under the active dictionary lemma when both belong to one lexeme ('διαβάζεται' -> 'διαβάζω', 'γράφτηκε' -> 'γράφω', middle/reflexive 'αγχώνεσαι' -> 'αγχώνω'); keep the -μαι lemma for deponent or middle-only lexemes such as 'έρχομαι'. Aggregate suppletive perfective partners under that same imperfective lemma: 'δω' -> 'βλέπω', bounded perfective 'πάω' -> 'πηγαίνω', and 'πω' -> 'λέω'; use context because imperfective 'πάω' is its own lemma, especially in the idiom 'μου πάει' meaning 'it suits me', which MUST NOT become 'πηγαίνω'. A genuine verbal participle keeps the parent verb lemma; a lexicalized participle used as an adjective gets its adjectival lemma.\n\
         1a. Stabilize common variant and suppletive-looking lemmas: greeting 'Χάρηκα/χάρηκα' is the perfective past mediopassive form of deponent 'χαίρομαι', never active 'χαίρω'. Imperative 'περάστε' meaning 'come in/go through' aggregates under the common dictionary lemma 'περνάω', not variant 'περνώ'. Use canonical 'χαμογελώ' for inflected χαμογελά, not the synonymous variant 'χαμογελάω', and canonical 'βοηθώ' for βοηθούσαν, not variant 'βοηθάω'. Comparative adverb αργότερα has positive lemma 'αργά', not surface-form lemma 'αργότερα'.\n\
         1b. Lexical category follows the current use: in 'κάποιον καλό τεχνικό', τεχνικό is the substantivized masculine Noun 'technician', not an Adjective and never gradable in that occurrence.\n\
         1c. Established indeclinable loanwords keep their Greek syntactic category even in Latin script. In 'γράφω ένα email', email is a neuter singular accusative Noun with lemma 'email', never Other.\n\
         2. Nominals: nouns and proper nouns always get lexical gender plus number and SYNTACTIC case, even when surface forms are syncretic or indeclinable. Adjectives always get agreement gender, number and case, including indeclinable colour adjectives ('τον μπλε φάκελο' -> masculine singular accusative Adjective, never Adverb). In a copular property contrast such as 'οι σημειώσεις είναι μπερδεμένες αλλά καθαρές', lexicalized 'μπερδεμένες' is an Adjective with lemma 'μπερδεμένος', not a verbal participle. Modern Greek has nominative, genitive, accusative and vocative only; do not invent dative for fixed learned expressions such as 'εν τω μεταξύ'. Treat a substantivized language name as a Noun when it fills a nominal argument slot: in 'μαθαίνω ελληνικά', 'ελληνικά' has the conventional plural lemma 'ελληνικά', neuter plural accusative. Distinguish genuinely adverbial use from that nominal object use rather than falling back to Other.\n\
         3. Case is decided from syntax, not endings. 'η μητέρα' is nominative as a subject and 'τη μητέρα' accusative as an object; 'της μητέρας' is genitive; direct address 'μητέρα!' is vocative. Genitive weak pronouns mark ordinary Standard Greek indirect objects ('του έδωσα'), while preposition complements are normally accusative. An elliptical service request such as 'το μενού;' means '[φέρετε] το μενού' and therefore analyzes the article and noun as accusative, even though their neuter forms are syncretic. For an adposition report the case it governs in this occurrence. In the complex expression 'πριν από τον ύπνο', 'πριν' is Adverb and 'από' is Adposition governing accusative; in 'αντί για τον ιούλιο', both 'αντί' and 'για' participate in a complex adposition governing the displayed accusative, so do not assign standalone genitive government to 'αντί'.\n\
         4. Degree is genuinely optional only when the lexeme is not gradable. Encode positive on unmarked gradable adjectives and adverbs ('σωστός', 'πολύ', 'γρήγορα'), comparative on 'καλύτερος', and contrast absolute_superlative 'ωραιότατος'; encode superlative for contextual 'ο καλύτερος'. Omit degree from a non-gradable relational adjective such as 'ιατρικός', possessive-relational 'δικός', an ordinal adjective such as 'πρώτος', a relational adjective such as 'άλλος', and a non-gradable adverb such as 'σήμερα' or negative-polarity 'καθόλου'. Interrogative πού, πώς and πότε are Adverb with degree omitted, never Interjection. In periphrastic 'πιο καλός', keep 'πιο' as its own comparative adverb and analyze 'καλός' as positive; likewise 'πιο άνετα' has comparative Adverb πιο plus positive Adjective άνετα, never comparative on both words. The separate πιο supplies the comparison.\n\
         4a. Invariant approximation adverb 'περίπου' is non-gradable in uses such as 'σε περίπου είκοσι λεπτά': omit degree; never add positive merely because the schema permits degree on adverbs.\n\
         4b. Response adverb 'βεβαίως' is non-gradable and omits degree. Response word 'ναι' is a Particle, never Other. Formulaic 'ευχαριστώ' remains the finite active Verb ευχαριστώ, indicative present first singular, even when it stands alone as 'thank you'; never reclassify it as Interjection.\n\
         5. Determiners: classify their type from attributive noun-phrase syntax. Articles encode definiteness: 'ο' is definite and 'ένας' is indefinite; every non-article determiner MUST omit definiteness. Before an overt noun or its required article, κάποιος is indefinite, ποιος interrogative, όποιος relative, δικός in 'ο δικός μου φίλος' possessive, ίδιος in the doubled frame 'ο ίδιος ο γιατρός' emphatic, αυτός/εκείνος in 'αυτό το βιβλίο' demonstrative, and inflecting όλος quantifier ('όλη μέρα'), not a gradable Adjective. This remains attributive across a preposition: in 'μεταξύ αυτών των δύο λύσεων', 'αυτών' is a feminine plural genitive demonstrative Determiner and 'των' a feminine plural genitive article, not genderless forms or a demonstrative Pronoun. Inflecting determiners encode the overt noun's agreement: 'ο' is masculine singular nominative, 'τις' is feminine plural accusative, and 'το βράδυ' has a neuter singular accusative article. Resolve forms shared with weak pronouns from syntax: before an overt nominal, τον/τη(ν)/το/του/της/τους/τις/τα is a definite-article Determiner with lemma 'ο', not a clitic Pronoun ('του φίλου', 'τα βιβλία'). The invariant quantifier 'κάθε' is the omission case: omit gender, number and case rather than copying them from its noun.\n\
         5a. An intervening modifier does not remove the nominal head: 'κάποιον καλό τεχνικό' has indefinite Determiner κάποιον, not Pronoun. NON-NEGOTIABLE doubled-article analysis: in the exact construction 'ο ίδιος ο διευθυντής', including when the requested targets are 'ο' and 'ίδιος', target ίδιος is an emphatic Determiner with masculine singular nominative agreement; it is never an emphatic Pronoun. A possessive δικός phrase before its overt head is likewise determinative: 'ο δικός μου γείτονας' has possessive Determiner δικός, not Adjective, and 'τη δική του άποψη' has possessive Determiner δική because άποψη follows. Contrast predicative 'η τσάντα είναι δική σου', where δική is Adjective. In an elliptical service order 'Έναν καφέ, παρακαλώ', Έναν is the masculine accusative indefinite article Determiner, not a cardinal Numeral.\n\
         6. Numerals: add gender, number and case only where the numeral form itself participates in the contrast. 'μία' is feminine versus neuter 'ένα'; 'ένας' is singular versus plural 'χίλιοι'; 'ενός' is genitive versus accusative 'έναν'. Omit all three fields on invariant 'πέντε' — do not copy plural, gender or case from the counted noun.\n\
         7. Pronouns: classify the type from pronominal syntax, assign syntactic case, and set clitic true only when an unstressed weak form substitutes for an argument or possessor; the same spelling before an overt nominal may instead be the article or determiner covered above. A post-nominal genitive weak form expressing the possessor is pronoun_type possessive ('ο φίλος σου', 'το σπίτι του'); this remains a clitic after a predicative strong possessive ('η τσάντα είναι δική σου', 'είναι δική μου'). Third-person possessive του/της/τους encodes third person and the possessor gender where its form distinguishes it; never omit person from 'τον εαυτό του'. Use possessive too in lexicalized 'γεια σου/σας'. A weak direct or indirect object remains pronoun_type personal ('του έδωσα', 'τα βλέπω'). In the leave-taking idiom 'τα λέμε', 'τα' substitutes for the things to be said: it is a third-person neuter plural accusative clitic personal Pronoun, never a Determiner. Without an overt following noun, αυτός/εκείνος/τέτοιος is demonstrative, κάποιος/κανείς/κάτι/τίποτα indefinite, and ο ίδιος/η ίδια/το ίδιο emphatic: in 'θα το κάνει ο ίδιος', 'ο' is a definite-article Determiner and 'ίδιος' an emphatic Pronoun. Only the doubled attributive frame 'ο ίδιος ο γιατρός' makes 'ίδιος' an emphatic Determiner. Standalone όλοι/όλες/όλα meaning 'everyone/all of them' is also an indefinite Pronoun with lemma 'όλος'; contrast attributive 'όλοι οι μαθητές', where 'όλοι' is a quantifier Determiner. A left-dislocated strong pronoun resumed by a weak object clitic shares that clitic's syntactic case: 'εκείνο θα το κρατήσω' has accusative 'εκείνο', not nominative. In the conventional reflexive construction classify εαυτός in 'κοιτάζει τον εαυτό του' as reflexive Pronoun and retain the separate article and possessive clitic. In reciprocal 'ο ένας τον άλλον' / 'ο ένας στον άλλον', only the content words 'ένας' and 'άλλος' are reciprocal Pronouns with their syntactic case; the preceding 'ο' and 'τον' remain definite-article Determiners, and fused 'στον' still splits into Adposition 'σε' plus article 'τον'. Do not reduce the reciprocal content words to unrelated Numeral and Adjective tokens or turn their articles into pronouns. Relative 'που' is a relative Pronoun when it occupies an argument slot for an antecedent ('το Η, που διαβάζεται /i/' -> nominative subject); use SubordinatingConjunction only when 'που' merely introduces a subordinate clause without such a nominal role. Relative-pronoun 'που' is invariant: assign its syntactic case but always omit person, gender and number. Person: 'εγώ' is first versus 'εσύ' second; omit person from interrogative 'ποιος'. Gender: 'αυτός' is masculine versus 'αυτή' feminine; omit gender from 'εγώ'. Number: 'εγώ' is singular versus 'εμείς' plural; omit number from invariant relative 'ό,τι'. Lemmatize weak forms to the corresponding strong nominative citation paradigm: μου/με -> εγώ, σου/σε -> εσύ, μας -> εμείς, σας -> εσείς, and pronominal third-person τον/τη(ν)/το/του/της/τους/τις/τα -> αυτός. Never lemmatize a third-person clitic to εγώ.\n\
         7a. The determiner/pronoun boundary depends on an actual overt nominal head, not on whether a clause follows. Standalone 'Όποιος θέλει' is a relative Pronoun and standalone 'Κάποιος χτύπησε' is an indefinite Pronoun; neither is a Determiner. By contrast, 'όποιο βιβλίο' and 'κάποιος άνθρωπος' are Determiners because an overt noun follows. In the compound relative 'ο οποίος', emit the article as a definite-article Determiner and the inflected οποίος as a relative Pronoun.\n\
         7b. In a resumed free relative such as 'θα βοηθήσω όποιον το χρειάζεται', 'το' is a third-person neuter singular accusative personal clitic with lemma 'αυτός'; it cannot be first person and can never have lemma 'εγώ'. The deponent verb 'χρειάζεται' has lemma 'χρειάζομαι', not the unrelated active-form lemma 'χρειάζω'.\n\
         7c. Weak personal pronouns governed by relational adverbs retain the genitive clitic construction: 'μαζί μας' has first-person plural genitive μας with clitic true, and 'κοντά μου' has first-person singular genitive μου with clitic true. Do not reinterpret these syncretic forms as accusative strong pronouns.\n\
         7d. In 'κανέναν άλλο' meaning 'anyone else', κανέναν is the indefinite Pronoun and άλλο is its agreeing non-gradable Adjective modifier (masculine singular accusative, lemma 'άλλος', degree omitted). Do not analyze both words as indefinite pronouns.\n\
         7e. Plural possessive clitic τους encodes third person and plural number but omits gender because the form does not distinguish it. In 'τα παιδιά γράφουν τις ασκήσεις τους', never invent masculine gender for τους.\n\
         7f. Every finite form of the copula paradigm with lemma 'είμαι' uses mediopassive morphological voice in this schema, including είμαι, είσαι, είναι, είμαστε and είστε. Never switch these forms to active voice because their surface ending is syncretic or irregular.\n\
         8. Every verb gets aspect, morphological voice and verb_form. Aspect is imperfective versus perfective: 'γράφω/έγραφα' contrasts with 'γράψω/έγραψα'. Voice is the morphology of the ACTUAL FORM, not meaning or merely the citation ending: -ω/-ώ forms are active and -μαι forms mediopassive. Therefore present 'κάθομαι' is mediopassive but its perfective imperative 'κάθισε' is active-form morphology; do not copy mediopassive voice from the lemma. Deponents and middle/reflexive uses whose displayed form is in -μαι, such as 'έρχομαι' and 'πλένομαι', remain mediopassive even though they are not semantically passive.\n\
         9. Finite forms encode mood, person and number and agree with their syntactic subject. 'γράφω' is indicative first singular versus 'γράφεις' indicative second singular and 'γράφουμε' first plural; plural 'αυτά είναι' requires third plural 'είναι', never third singular merely because the copular spelling is syncretic. 'Γράψε' contrasts as imperative second singular. Finite imperfective non-past 'γράφω' encodes present and aorist 'έγραψα' encodes past. Perfective non-past 'γράψω' retains indicative-form mood and person/number but MUST omit tense, whether cited in an explicit stem pair or licensed after 'να', 'ας', 'θα', 'αν' or another particle/conjunction; never call it present or future at token level. Imperative 'γράψε' also MUST omit tense. The licensing particle creates the construction, not a synthetic subjunctive or future morphology on the verb.\n\
         9a. Resolve aspectual syncretism from constructional meaning. In bounded 'θα το κάνω' meaning 'I will do it' as one completed act, κάνω is perfective finite indicative first singular and omits tense; do not default it to imperfective present merely because the surface form matches present κάνω.\n\
         9b. Prohibitives with μην use indicative-form morphology rather than a synthetic imperative form. In imperfective 'Μην περιμένεις', περιμένεις is finite indicative present second singular; in perfective 'μην ζητήσεις', ζητήσεις is finite indicative second singular with tense omitted. Reserve mood imperative for actual imperative forms such as περίμενε and γράψε.\n\
         10. Non-finite omission rules are strict. The invariant perfect-dependent form in 'έχω γράψει' contrasts with finite 'γράφει': tag 'γράψει' perfect_dependent, perfective, active and omit mood, tense, person, number, gender and case. It is not a productive infinitive and must never be generated or analyzed as a standalone complement. The converb 'γράφοντας' is imperfective active and omits those same six fields. An agreeing participle encodes number, gender and case: nominative masculine singular 'γραμμένος' contrasts with genitive feminine singular 'γραμμένης'; both omit mood, tense and person.\n\
         11. Split analytic constructions into lexical tokens. 'έχω γράψει' is finite 'έχω' plus perfect-dependent 'γράψει'; 'θα γράψω' is particle 'θα' plus finite perfective 'γράψω'; 'να γράφω' is particle 'να' plus finite imperfective 'γράφω'. The forms after 'θα' and 'να' remain finite and inflect for person and number — do not confuse them with the invariant perfect-dependent form. Never give every token the whole construction's tense or mood. Treat 'θα', 'να', 'ας', 'δεν' and 'μη(ν)' as particles, not verbs. Conditional 'αν' is a SubordinatingConjunction, including after 'κι' in a free-relative construction such as 'όποιο λεωφορείο κι αν πάρεις'; never tag it Particle. Before a finite clause, πριν is also a SubordinatingConjunction ('πριν φύγετε'); reserve Adverb for independent πριν and for the first element of 'πριν από' described above.\n\
         12. Formulaic interaction: analyze γεια, καλημέρα and καλησπέρα as Interjection when they perform a greeting or leave-taking function. Standalone apology formula 'Συγγνώμη' is likewise an Interjection, not a Noun merely because συγγνώμη is nominal in other syntax. Politeness-formula παρακαλώ is also Interjection when it means 'please', 'you're welcome' or 'go ahead', whether parenthetical or standalone ('Έναν καφέ, παρακαλώ', 'Παρακαλώ, περάστε'); reserve Verb for a syntactically verbal use such as 'σας παρακαλώ να περιμένετε'. Do not invent verbal or nominal features for the formulaic uses.\n\
         12a. Alphabetic metalinguistic use: an isolated Greek alphabet glyph named, contrasted or identified as a letter ('Το Β είναι το κεφαλαίο του β', 'το γράμμα Η') is a Symbol, never a ProperNoun or Noun. Use the ordinary lowercase glyph as its lemma so case and positional variants aggregate together ('Β' and 'β' -> 'β'; 'Σ', 'σ' and final 'ς' -> 'σ'). A phonetic transcription token written inside slashes such as /i/, /r/ or /x/ is also a Symbol, never Other. Preserve both delimiting slashes in its emitted word and lemma exactly: emit word '/x/' with lemma '/x/', not bare 'x'. This rule is only for standalone glyphs and transcription symbols; analyze a complete cited word such as 'ύλη' by its ordinary lexical category.\n\
         12b. Cited-word morphology: a complete word mentioned autonymically after metalanguage such as 'στη λέξη καλός' keeps the lexical category and morphology of the displayed form; it does not agree with 'λέξη' or inherit the preposition's accusative case. Thus cited 'καλός' is a positive masculine singular nominative Adjective, and cited dictionary-form 'μέσο' is a neuter singular nominative Noun. Use the citation form when an ending is syncretic and the cited word has no independent syntactic role.\n\
         13. Tokenization: split ONLY complete orthographic tokens that are fused σε + article forms into both underlying tokens ('στον' -> 'σε' + 'τον', 'στην' -> 'σε' + 'την', 'στα' -> 'σε' + 'τα'). Apply this case-insensitively at sentence start too: 'Στο' -> lowercase emitted words 'σε' + 'το', 'Στον' -> 'σε' + 'τον', and 'Στις' -> 'σε' + 'τις'; never emit 'Στο', 'Στον' or 'Στις' as the adposition word. Never split a lexical word merely because it begins with στ-/στη-/στο-: verbs such as 'στηρίζουν' and nouns such as 'στοιχείο' remain one token, with no invented 'σε' or article. The contraction rule also applies when an adjective or adverb follows the complete fused token before the noun: emit 'στο πάνω ράφι' as the four words 'σε', 'το', 'πάνω', 'ράφι'. Split EVERY contraction occurrence independently even when one sentence contains several: 'Στη λέξη καλός γράφουμε ς στο τέλος' must contain both 'σε' + 'τη' and the later 'σε' + 'το', never a residual 'στο'. In the emitted features, set the first word exactly to 'σε', never to the unsplit surface form such as 'στο' or 'στην', and the second word exactly to the restored article. Restore ordinary elisions ('απ\''' -> 'από'). Keep weak pronouns as separate tokens. Strip punctuation from analyzed tokens and never emit punctuation as morphology."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modern_greek_identity_and_script_are_exact() {
        let language = ModernGreek;
        assert_eq!(ModernGreek::ISO_LANG, IsoLang::Ell);
        assert_eq!(language.supported_scripts(), &[Script::GREK]);
        assert_eq!(language.default_script(), Script::GREK);
    }

    #[test]
    fn optional_learner_dimensions_remain_closed_pivots() {
        let adjective = ModernGreekMorphology::Adjective {
            lemma: "καλός".to_string(),
            gender: TernaryGender::Masculine,
            number: BinaryNumber::Singular,
            case: GreekCase::Nominative,
            degree: Some(GreekDegree::Comparative),
        };
        let article = ModernGreekMorphology::Determiner {
            lemma: "ο".to_string(),
            determiner_type: GreekDeterminerType::Article,
            definiteness: Some(GreekDefiniteness::Definite),
            gender: Some(TernaryGender::Masculine),
            number: Some(BinaryNumber::Singular),
            case: Some(GreekCase::Nominative),
        };

        assert_eq!(
            ModernGreekMorphology::PIVOT_DEGREE.value(&adjective),
            Some("comparative".to_string())
        );
        assert_eq!(
            ModernGreekMorphology::PIVOT_DEFINITENESS.value(&article),
            Some("definite".to_string())
        );
    }

    #[test]
    fn imperative_perfective_non_past_and_non_finite_forms_do_not_acquire_tense() {
        let imperative = ModernGreekMorphology::Verb {
            lemma: "γράφω".to_string(),
            aspect: GreekAspect::Perfective,
            voice: GreekVoice::Active,
            verb_form: GreekVerbForm::Finite,
            mood: Some(GreekMood::Imperative),
            tense: None,
            person: Some(Person::Second),
            number: Some(BinaryNumber::Singular),
            gender: None,
            case: None,
        };
        let perfect_dependent = ModernGreekMorphology::Verb {
            lemma: "γράφω".to_string(),
            aspect: GreekAspect::Perfective,
            voice: GreekVoice::Active,
            verb_form: GreekVerbForm::PerfectDependent,
            mood: None,
            tense: None,
            person: None,
            number: None,
            gender: None,
            case: None,
        };
        let perfective_non_past = ModernGreekMorphology::Verb {
            lemma: "γράφω".to_string(),
            aspect: GreekAspect::Perfective,
            voice: GreekVoice::Active,
            verb_form: GreekVerbForm::Finite,
            mood: Some(GreekMood::Indicative),
            tense: None,
            person: Some(Person::First),
            number: Some(BinaryNumber::Singular),
            gender: None,
            case: None,
        };

        assert_eq!(
            ModernGreekMorphology::PIVOT_MOOD.value(&imperative),
            Some("imperative".to_string())
        );
        assert_eq!(ModernGreekMorphology::PIVOT_TENSE.value(&imperative), None);
        assert_eq!(
            ModernGreekMorphology::PIVOT_MOOD.value(&perfect_dependent),
            None
        );
        assert_eq!(
            ModernGreekMorphology::PIVOT_TENSE.value(&perfect_dependent),
            None
        );
        assert_eq!(
            ModernGreekMorphology::PIVOT_MOOD.value(&perfective_non_past),
            Some("indicative".to_string())
        );
        assert_eq!(
            ModernGreekMorphology::PIVOT_TENSE.value(&perfective_non_past),
            None
        );
    }

    #[test]
    fn cloze_features_cover_the_real_inflecting_pos() {
        let features = ModernGreek.typological_features();

        assert_eq!(
            features,
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
        );
    }
}
