use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature, Upos,
};

#[panini_macro::closed_enum]
pub enum DanishGender {
    Common, // Fælleskøn (n-ord)
    Neuter, // Intetkøn (t-ord)
}

#[panini_macro::closed_enum]
pub enum DanishDefiniteness {
    Indefinite, // Ubestemt
    Definite,   // Bestemt
}

#[panini_macro::closed_enum]
pub enum DanishCase {
    Nominative, // Nominativ
    Genitive,   // Genitiv (-s)
    Objective,  // Akkusativ/Dativ (mostly for pronouns)
}

#[panini_macro::closed_enum]
pub enum DanishTense {
    Present, // Nutid (præsens)
    Past,    // Datid (præteritum)
}

#[panini_macro::closed_enum]
pub enum DanishMood {
    Indicative, // Fremsættende måde
    Imperative, // Bydemåde
    Infinitive, // Navnemåde
    Participle, // Tillægsform
}

#[panini_macro::closed_enum]
pub enum DanishVoice {
    Active,  // Aktiv
    Passive, // Passiv (s-passiv or auxiliary)
}

#[panini_macro::closed_enum]
pub enum DanishDegree {
    Positive,    // Positiv
    Comparative, // Komparativ
    Superlative, // Superlativ
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
pub enum DanishMorphology {
    Adjective {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<DanishGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        definiteness: Option<DanishDefiniteness>,
        degree: DanishDegree,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<DanishGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        gender: DanishGender,
        number: BinaryNumber,
        definiteness: DanishDefiniteness,
        case: DanishCase,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<DanishGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        case: DanishCase,
    },
    ProperNoun {
        lemma: String,
        case: DanishCase,
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
        tense: Option<DanishTense>,
        mood: DanishMood,
        voice: DanishVoice,
    },
    Other {
        lemma: String,
    },
}

pub struct Danish;

impl LinguisticDefinition for Danish {
    type Morphology = DanishMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Dan;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        DanishMorphology::PIVOT_GENDER,
        DanishMorphology::PIVOT_NUMBER,
        DanishMorphology::PIVOT_CASE,
        DanishMorphology::PIVOT_DEFINITENESS,
        DanishMorphology::PIVOT_MOOD,
        DanishMorphology::PIVOT_VOICE,
        DanishMorphology::PIVOT_DEGREE,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        &[TypologicalFeature::Conjugation(&[Upos::Verb])]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Lemmatization: Nouns should be in singular indefinite form. Verbs should be in the infinitive. Adjectives should be in the common singular positive form, and a suppletive form lemmatises to that same citation form ('små' and 'mindre' to 'lille', 'bedre' and 'bedst' to 'god'). Determiners lemmatise to their common-gender form: 'et' to 'en', 'mit' to 'min', 'dit' to 'din'. 'mange' is its own lemma ('mange', never 'mangen'); 'flere' and 'flest' lemmatise to it as its comparative and superlative.\n\
         2. Nouns: Specify gender (common/neuter), number (singular/plural), definiteness (indefinite/definite), and case (usually nominative unless ending in -s for genitive). Gender is lexical and cannot be read off the ending: settle it by the indefinite article the noun takes ('en bil' common, 'et bær', 'et træ', 'et år', 'et marked' neuter), and carry that same gender into the plural, where no article is there to show it. Number follows the count, not the ending: many neuter nouns have an uninflected plural ('år', 'æg', 'øre', 'bær'), so a numeral or quantity above one makes the noun plural even though the written form is unchanged.\n\
         3. Verbs: Identify tense (present/past) for finite forms. Distinguish between active and passive (s-passive) forms. Identify mood (indicative, imperative, infinitive, participle).\n\
         4. Adjectives: Specify degree (positive, comparative, superlative). Note agreement in gender, number, and definiteness where applicable (e.g., 'stort' vs 'stor'). Ordinals are adjectives and agree like them: 'anden' is the common form against neuter 'andet', so give it gender, number and definiteness; leave those out only for a genuinely invariable ordinal such as 'tredje'.\n\
         5. Pronouns: Identify person, number, and case (subjective 'jeg' vs objective 'mig'). Gender belongs to the third-person singular only ('han', 'hun', 'den', 'det'); 'jeg', 'du', 'vi', 'I' and 'de' carry none. A possessive before a noun ('min bil', 'mit hus', 'dine børn') is a determiner with the noun's gender and number, lemmatised as in 1, never a pronoun; the pronoun reading is for a possessive standing alone ('bilen er min').\n\
         6. Definiteness: Distinguish between the suffixed definite article (e.g., 'manden') and the standalone definite determiner used with adjectives (e.g., 'den gamle mand').\n\
         7. Fixed temporal phrases ('i morgen', 'i går', 'i aften', 'om morgenen', 'om sommeren') keep their internal parts: tag the preposition as an adposition and the following word as the noun it is, with its gender, number, definiteness and case. Do not tag a noun as an adverb because the phrase as a whole is adverbial.\n\
         8. Courtesy formulas keep one part of speech across every sentence, because the lexicon is keyed on lemma and part of speech. 'tak' is a common-gender noun wherever it appears — bare 'Tak!', 'mange tak', 'tak for mad', 'selv tak' — and never an interjection. 'undskyld' is an interjection when it stands alone to apologise or to open an approach ('Undskyld, hvor er toilettet?'), and the imperative of 'undskylde' only when it governs an object ('Undskyld mig').\n\
         9. 'en' and 'et' standing before a noun are the indefinite article: tag them as a determiner carrying gender, common for 'en' and neuter for 'et', never as a numeral or as other. This holds when the noun phrase answers 'hvor mange' ('kun et æble og en pære' is still determiner neuter and determiner common). The numeral reading is only for the stressed, accent-marked 'én'/'ét' that counts one against another number. Getting this wrong erases the en/et contrast, which is the single most taught point of Danish grammar.\n\
         10. A word qualifying a noun, whether attributive ('kold mælk', 'det kolde vand') or predicative ('vandet er koldt'), is an adjective with its degree, never an adverb. Tag an adverb only for a word modifying a verb, an adjective or the whole clause ('meget', 'ofte', 'ikke')."
    }


    fn alignment_directives(&self) -> Option<&'static str> {
        Some(
            "1. The suffixed definite article is a segment: [\"bog\", \"en\"], [\"hus\", \"et\"], [\"bøger\", \"ne\"]; the genitive -s likewise: [\"Peter\", \"s\"].\n\
             2. A compound is one written word; split it at the constituent boundary when the other sentence has separate words, the linking -s-/-e- staying with the first constituent: [\"fødsels\", \"dag\"], [\"arbejds\", \"tid\"], [\"køkken\", \"bord\"].\n\
             3. A particle verb (står op, tager af sted) is several written words in one link; ikke and the reflexive sig are separate words.",
        )
    }
}
