use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, BinaryVoice, IsoLang, LinguisticDefinition, Person, Script,
    TypologicalFeature, Upos,
};

#[panini_macro::closed_enum]
pub enum FrenchTense {
    // present is also the tense of the infinitive, the present participle and the gerund
    Present,
    Imperfect,
    Future,
    // passé simple
    SimplePast,
    // past participle only: compound tenses are split into auxiliary + participle
    Past,
}

#[panini_macro::closed_enum]
pub enum FrenchMood {
    Indicative,
    Subjunctive,
    Conditional,
    Imperative,
    Infinitive,
    Participle,
    // gérondif: en + -ant
    Gerund,
}

#[panini_macro::closed_enum]
pub enum FrenchPronounCase {
    Subject,
    DirectObject,
    IndirectObject,
    Reflexive,
    Tonic,
}

#[panini_macro::closed_enum]
pub enum FrenchPronounType {
    Personal,
    // en, y
    Adverbial,
    // impersonal il: il pleut, il faut, il y a, quelle heure est-il
    Expletive,
    Possessive,
    Demonstrative,
    Relative,
    Interrogative,
    Indefinite,
}

#[panini_macro::closed_enum]
pub enum FrenchDeterminerType {
    DefiniteArticle,
    IndefiniteArticle,
    PartitiveArticle,
    Possessive,
    Demonstrative,
    Interrogative,
    Exclamative,
    Indefinite,
}

/// Negative `plus` (no more) and comparative `plus` (more) are the same word.
#[panini_macro::closed_enum]
pub enum FrenchNegation {
    // ne, pas, plus, jamais, rien, guère, point in a negation
    Negative,
    // ne … que: only
    Restrictive,
    // ne explétif: avant qu'il ne parte, je crains qu'il ne vienne
    Expletive,
}

/// What the reflexive clitic of a pronominal verb does; `se` alone cannot tell.
#[panini_macro::closed_enum]
pub enum FrenchPronominalUse {
    // il se lave
    Reflexive,
    // ils se parlent
    Reciprocal,
    // ça se vend bien
    Passive,
    // il se souvient, elle s'évanouit: no non-pronominal counterpart with that meaning
    Inherent,
}

/// The tu/vous register choice, addressing one person.
#[panini_macro::closed_enum]
pub enum FrenchAddress {
    Familiar,
    Formal,
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
pub enum FrenchMorphology {
    Adjective {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        negation: Option<FrenchNegation>,
        // adverbial tout agrees: toute petite, toutes petites
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: FrenchDeterminerType,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        number: BinaryNumber,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<FrenchAddress>,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    Numeral {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Particle {
        lemma: String,
    },
    Pronoun {
        lemma: String,
        pronoun_type: FrenchPronounType,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<FrenchPronounCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        possessor_number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<FrenchAddress>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pronominal_use: Option<FrenchPronominalUse>,
    },
    ProperNoun {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        tense: FrenchTense,
        mood: FrenchMood,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<FrenchAddress>,
        voice: BinaryVoice,
    },
    Other {
        lemma: String,
    },
}

pub struct French;

impl LinguisticDefinition for French {
    type Morphology = FrenchMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Fra;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        FrenchMorphology::PIVOT_TENSE,
        FrenchMorphology::PIVOT_MOOD,
        FrenchMorphology::PIVOT_GENDER,
        FrenchMorphology::PIVOT_NUMBER,
        FrenchMorphology::PIVOT_VOICE,
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
        "1. Lemmatization: infinitive for verbs, masculine singular for nouns, adjectives and determiners. A possessive keeps its own possessor: its lemma is the masculine singular for that same possessor — ma/mes -> 'mon', sa/ses -> 'son', vos -> 'votre', leurs -> 'leur', la mienne -> 'mien', les vôtres -> 'vôtre', les nôtres -> 'nôtre'. Only third person personal pronouns are normalised, to the masculine singular of their series: subject elle/ils/elles -> 'il', la/les -> 'le', leur -> 'lui', tonic elle/eux/elles -> 'lui'. Every other personal pronoun keeps its own form as lemma: je, me, moi, tu, te, toi, nous, vous, se, soi.\n\
         2. Gender and Number: always specify both for Nouns and Adjectives. For Determiners, specify Gender only when the form marks it ('le'/'la', 'mon'/'ma', 'ce'/'cette'), and omit it for forms that do not ('les', 'des', 'mes', 'leurs', 'ces', 'l''). A cardinal numeral takes gender only when it is or ends in 'un'/'une' ('vingt et une femmes' -> 'une' feminine).\n\
         3. Compound tenses: the auxiliary and the past participle are two separate verb tokens. 'j'ai mangé' is 'ai' (present, indicative) plus 'mangé' (past, participle), and the same applies to plus-que-parfait, futur antérieur, subjonctif passé and conditionnel passé.\n\
         4. Tense values: présent -> present, imparfait (indicative and subjunctive) -> imperfect, futur simple -> future, passé simple -> simple_past. The conditional is a mood: 'aurait' is present tense, conditional mood. Reserve past for the past participle; a finite verb is never tagged past. Infinitive, imperative, present participle and gerund are present.\n\
         5. Non-finite forms: 'en chantant' is 'en' (adposition) plus 'chantant' (gerund); a bare '-ant' form is a participle. Specify Person and Number only on finite verbs.\n\
         6. Past participles: add Gender and Number only when the participle agrees — with the subject under 'être' ('elle est partie'), with a preceding direct object under 'avoir' ('les pommes que j'ai cueillies'), and in pronominal verbs whose reflexive is the direct object ('elles se sont lavées'). Omit both on the invariable form ('elle a mangé', 'elles se sont lavé les mains').\n\
         7. Articles: 'le/la/les' are definite_article; 'un/une/des' are indefinite_article, with lemma 'un'; the partitive 'du', 'de la', 'de l'' before a mass noun is partitive_article, with lemma 'du'. The reduced 'de'/'d'' that replaces an indefinite or partitive article after a negation or before a plural adjective ('pas de pain', 'de bons gâteaux') is a determiner with lemma 'de' and the type of the article it replaces, never an adposition.\n\
         8. Contractions: 'au', 'aux' and a 'du'/'des' that means 'de' + 'le'/'les' (la maison du voisin, les fruits des arbres) are split into the adposition and the definite article: 'à' + 'le', 'de' + 'les'. The relatives and interrogatives 'auquel', 'auxquels', 'auxquelles', 'duquel', 'desquels', 'desquelles' split the same way into 'à' or 'de' + 'lequel'. A partitive 'du' or an indefinite 'des' is one article and is never split.\n\
         9. Possessives: on possessive determiners and pronouns, possessor_person and possessor_number describe the possessor, gender and number the thing possessed — 'son' and 'sa' are third person singular possessor, 'leur' and 'leurs' third person plural possessor, 'le mien' first person singular. Do not set person on them.\n\
         10. Pronouns: specify the case of personal pronouns (subject, direct_object, indirect_object, reflexive, tonic). 'en' and 'y' are adverbial. The impersonal 'il' of 'il pleut', 'il faut', 'il y a', 'quelle heure est-il' is expletive, not personal. Set Person only on personal pronouns. Neuter pronouns carry no gender: 'ce', 'ça', 'cela', 'ceci', and the 'le' that stands for a clause or an adjective ('je le crois', 'elle est malade, elle le restera'). On a reflexive clitic (me, te, se, nous, vous in a pronominal verb), set pronominal_use: reflexive when the subject acts on itself ('il se lave'), reciprocal when the subjects act on each other ('ils se parlent'), passive when the subject undergoes the action and no agent is expressed ('ça se vend bien', 'la ville se visite'), inherent when the verb has no non-pronominal counterpart with that meaning ('se souvenir', 's'évanouir', 'se moquer').\n\
         11. Address: set address on every second person form that addresses ONE person — pronouns, possessives and verbs, including a bare imperative: familiar for the tu forms ('tu', 'te', 'toi', 'ton', 'tien', 'viens'), formal for the vous forms ('vous', 'votre', 'vôtre', 'venez' said to one person). Number and possessor_number stay those of the form, so a formal 'vous' or 'votre' is still plural. Omit address when 'vous' addresses several people.\n\
         12. Negation: set negation on the adverbs of a negation — negative for 'ne' and 'pas', 'plus', 'jamais', 'rien' used adverbially, 'guère', 'point'; restrictive for both halves of 'ne … que'; expletive for the 'ne' that negates nothing ('avant qu'il ne parte'). Omit it for comparative 'plus' ('plus grand', 'j'en veux plus').\n\
         13. 'que': a 'que' introducing a clause or a comparison is a subordinating_conjunction, never a coordinating one; the relative 'que' is a relative pronoun.\n\
         14. 'quel': interrogative in a question, exclamative in an exclamation ('Quel beau jour !').\n\
         15. Elision: restore an elided word as its own token: 'j'aime' -> 'je' + 'aime', 'l'école' -> 'le' + 'école', 'n'en' -> 'ne' + 'en'.\n\
         16. Adverbial 'tout' ('very, entirely') agrees before a feminine adjective beginning with a consonant or aspirated h: give 'toute petite' gender feminine, number singular and 'toutes petites' feminine plural. Omit gender and number on every other adverb, including the invariable 'tout' ('tout étonnées', 'tout petits').\n\
         17. Greetings and formulas said as a whole utterance — 'bonjour', 'bonsoir', 'salut', 'merci', 'coucou' — are interjections, never nouns. Multi-word formulas are analysed word by word, so 'au revoir' is 'à' + 'le' + the noun 'revoir' and 's'il vous plaît' keeps 'vous' as formal and 's'il te plaît' keeps 'te' as familiar."
    }


    fn alignment_directives(&self) -> Option<&'static str> {
        Some(
            "1. Elision and hyphenation open a word: the apostrophe stays with the clitic and the hyphen with the piece it introduces — [\"j'\"], [\"aime\"]; [\"l'\"], [\"école\"]; [\"qu'\"], [\"il\"]; [\"est\"], [\"-ce\"]; [\"donne\"], [\"-le\"], [\"-moi\"]; [\"va\"], [\"-t\"], [\"-il\"].\n\
             2. Contracted articles au, aux, du, des stay ONE segment linked to every unit they cover; never rewrite them.\n\
             3. Negation is discontinuous — ne … pas / plus / jamais / rien / personne / que — one link holding both words, the first often elided as [\"n'\"].",
        )
    }
}
