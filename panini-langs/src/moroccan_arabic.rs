use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TernaryNumber,
    TypologicalFeature, Upos,
};

/// Definiteness of a Moroccan Arabic nominal.
///
/// Moroccan marks definiteness with the prefixed article `ل-/ال-`, which
/// assimilates to a following sun letter in speech but is still written. There
/// is no indefinite article: `واحد ال-` is a specific-indefinite determiner whose
/// noun keeps the definite article, so it is not an exponent of this dimension.
/// A construct-state noun takes its definiteness from the following possessor.
#[panini_macro::closed_enum]
pub enum MoroccanArabicDefiniteness {
    Indefinite,
    Definite,
}

/// Whether a noun is free or is the head of a synthetic possessive construction.
///
/// Moroccan has largely replaced the synthetic idafa with the analytic genitive
/// `ديال`/`د`, so the construct state is a small residue: kinship terms, body
/// parts, fixed compounds, and any host carrying a possessive suffix. The head
/// of a `ديال` phrase is a free noun, not a construct one.
#[panini_macro::closed_enum]
pub enum MoroccanArabicNominalState {
    Free,
    Construct,
}

/// The synchronically relevant source of an adjectival form.
///
/// Moroccan active participles carry present, resultative or prospective force
/// as predicates but still agree like adjectives. The elative `كبر`/`حسن` is one
/// form whose comparative or superlative reading comes from syntax, so those are
/// not split here.
#[panini_macro::closed_enum]
pub enum MoroccanArabicAdjectiveForm {
    Lexical,
    ActiveParticiple,
    PassiveParticiple,
    Elative,
}

/// The verbal measures of Moroccan Arabic, in the Moroccanist labelling.
///
/// Moroccan derives its medio-passives with a prefixed `ت-/تّ-` rather than with
/// the Standard Arabic Forms V, VI and VII, so the descriptive tradition
/// (Harrell 1962; Caubet 1993) labels them Ia, IIa and IIIa — the t-stems of
/// Forms I, II and III. Forms IV, V, VI and VII are not living Moroccan
/// patterns and are deliberately absent from this inventory. Forms VIII, IX and
/// X survive on closed lexical sets: `حتارم`, the colour/defect verbs `حمار`,
/// and `ستاغرب`. Quadriliterals have their own base and t-stem.
#[panini_macro::closed_enum]
pub enum MoroccanArabicVerbPattern {
    // Roman numerals need explicit names: `rename_all = "snake_case"` starts a new
    // word at every capital, so `FormII` would serialize as `form_i_i`.
    #[serde(rename = "form_i")]
    FormI,
    // The tt-/t- medio-passive of Form I; the eastern n- variant is the same value.
    #[serde(rename = "form_ia")]
    FormIa,
    #[serde(rename = "form_ii")]
    FormII,
    #[serde(rename = "form_iia")]
    FormIIa,
    #[serde(rename = "form_iii")]
    FormIII,
    #[serde(rename = "form_iiia")]
    FormIIIa,
    #[serde(rename = "form_viii")]
    FormVIII,
    #[serde(rename = "form_ix")]
    FormIX,
    #[serde(rename = "form_x")]
    FormX,
    Quadriliteral,
    QuadriliteralA,
}

/// The five finite paradigms a learner must distinguish in Moroccan Arabic.
///
/// Moroccan has no inflectional case and no indicative/subjunctive/jussive
/// ending system. Its imperfective stem contrasts a bare form — the one
/// required after a governor such as `بغيت`, `خاص` or `يمكن` — with the
/// habitual/progressive `كا-` series and the future `غا-` series. There is no
/// `بـ` imperfective and no `حـ` future: those are Egyptian, not Moroccan.
#[panini_macro::closed_enum]
pub enum MoroccanArabicVerbForm {
    Perfective,
    BareImperfective,
    // `كا-`; the southern `تا-` and the reduced `دا-` are the same value.
    KaImperfective,
    // `غا-/غاد-` bound to the verb, or the verb governed by a free-standing
    // `غادي`, which is then also a `future` particle: `غادي نسافر`.
    GhaImperfective,
    Imperative,
}

/// Diathesis as expressed by a Moroccan Arabic verb in context.
///
/// Passive and middle/reflexive readings are built with the t-stem measures,
/// never with an internal-vowel passive.
#[panini_macro::closed_enum]
pub enum MoroccanArabicVoice {
    Active,
    Passive,
    MiddleReflexive,
}

#[panini_macro::closed_enum]
pub enum MoroccanArabicPolarity {
    Affirmative,
    Negative,
}

#[panini_macro::closed_enum]
pub enum MoroccanArabicPronounType {
    Personal,
    Demonstrative,
    Relative,
    Interrogative,
    Reflexive,
    Indefinite,
}

/// Whether a Moroccan Arabic adverb is a question word.
///
/// The interrogative adverbs `فين`, `فوقاش`, `إمتى`, `علاش`, `كيفاش`, `منين`
/// are a closed set that opens a content question, and a learner meets them
/// as a paradigm beside `شكون`/`شنو` and `شحال`; every other adverb is lexical.
#[panini_macro::closed_enum]
pub enum MoroccanArabicAdverbType {
    // `دابا`, `بزاف`, `ديما`, `هنا`, `البارح`, `غير`.
    Lexical,
    // `فين`, `فوقاش`, `إمتى`, `علاش`, `كيفاش`, `منين`.
    Interrogative,
}

/// Syntactic function of an attached personal-pronoun clitic.
///
/// Moroccan stacks a direct and an indirect object on one host
/// (`عطاهالي`), so both functions can occur in a single word. The same
/// object-shaped suffix supplies the *subject* of the assertive `را-` and of
/// the pseudo-verb `خاص`, so that reading has its own value.
#[panini_macro::closed_enum]
pub enum MoroccanArabicAttachmentFunction {
    Possessive,
    DirectObject,
    IndirectObject,
    Prepositional,
    // The suffix that is the subject of its host: `راني هنا`, `راه مشى`, `خاصني نمشي`.
    Subject,
}

/// Kind of determiner heading a Moroccan Arabic noun phrase.
///
/// Moroccan has two indefinite determiners and they contrast: `واحد ال-`
/// introduces a specific referent the speaker has in mind, `شي` an
/// unspecified one, and only the first keeps the article on its noun.
#[panini_macro::closed_enum]
pub enum MoroccanArabicDeterminerType {
    DefiniteArticle,
    Demonstrative,
    // `واحد` heading a still-articled noun: `واحد الراجل`.
    SpecificIndefinite,
    // `شي` heading a bare noun: `شي مشكلة`, `شي حاجة`, `شي واحد`.
    Indefinite,
    Quantifier,
    Interrogative,
}

/// Function of a Moroccan Arabic particle or morphologically split clitic.
#[panini_macro::closed_enum]
pub enum MoroccanArabicParticleFunction {
    Negation,
    Interrogative,
    Vocative,
    Future,
    // The assertive/deictic `را-` and presentative `ها-`: `راني هنا`, `ها هو`.
    Presentative,
}

/// Morphological features of contemporary Moroccan Arabic (`ary`).
///
/// Moroccan has no productive nominal case endings and no finite mood endings.
/// Its finite system is organized around a perfective and three imperfective
/// series (bare, `كا-`, `غا-`), and its agreement is markedly flatter than
/// eastern Arabic: no dual and no feminine plural anywhere in the verb, and no
/// masculine/feminine contrast at all in the second-person perfective.
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
pub enum MoroccanArabicMorphology {
    Adjective {
        lemma: String,
        /// Present only for a synchronically transparent Arabic root.
        #[serde(skip_serializing_if = "Option::is_none")]
        root: Option<String>,
        adjective_form: MoroccanArabicAdjectiveForm,
        /// Omitted when the adjective's form has no gender contrast, notably on
        /// the plural agreement form and on the elative.
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_gender: Option<BinaryGender>,
        agreement_number: BinaryNumber,
        definiteness: MoroccanArabicDefiniteness,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        adverb_type: MoroccanArabicAdverbType,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
        determiner_type: MoroccanArabicDeterminerType,
        /// Present only on a determiner whose form encodes number: the distal
        /// `داك`/`ديك`/`دوك` series does, invariable `هاد` and the article do not.
        #[serde(skip_serializing_if = "Option::is_none")]
        referent_number: Option<BinaryNumber>,
        /// Present only on a determiner whose form encodes gender: singular
        /// `داك` versus `ديك`. Invariable `هاد`, plural `دوك` and the article
        /// carry no contrast.
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
        definiteness: MoroccanArabicDefiniteness,
        state: MoroccanArabicNominalState,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_function: MoroccanArabicParticleFunction,
    },
    Pronoun {
        lemma: String,
        pronoun_type: MoroccanArabicPronounType,
        clitic: bool,
        /// Present only for an attached personal pronoun.
        #[serde(skip_serializing_if = "Option::is_none")]
        attachment_function: Option<MoroccanArabicAttachmentFunction>,
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
        pattern: MoroccanArabicVerbPattern,
        verb_form: MoroccanArabicVerbForm,
        voice: MoroccanArabicVoice,
        polarity: MoroccanArabicPolarity,
        person: Person,
        agreement_number: BinaryNumber,
        /// Moroccan's gender contrast is paradigm-specific: the perfective has
        /// it only in the third singular, the imperfective in the second and
        /// third singular, the imperative in the singular. Every plural, every
        /// first person and the whole second-person perfective omit it.
        #[serde(skip_serializing_if = "Option::is_none")]
        agreement_gender: Option<BinaryGender>,
    },
    Other {
        lemma: String,
    },
}

impl MoroccanArabicMorphology {
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
                agreement_gender, ..
            }
            | Self::Verb {
                agreement_gender, ..
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
    /// the Amazigh, French and Spanish layers of the lexicon and the derive
    /// therefore cannot generate a handle for it.
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
            <MoroccanArabicAttachmentFunction as panini_core::aggregable::ClosedValues>::all_variants,
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

pub struct MoroccanArabic;

impl LinguisticDefinition for MoroccanArabic {
    type Morphology = MoroccanArabicMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Ary;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        MoroccanArabicMorphology::PIVOT_ROOT,
        MoroccanArabicMorphology::PIVOT_PATTERN,
        MoroccanArabicMorphology::PIVOT_VERB_FORM,
        MoroccanArabicMorphology::PIVOT_VOICE,
        MoroccanArabicMorphology::PIVOT_POLARITY,
        MoroccanArabicMorphology::PIVOT_PERSON,
        MoroccanArabicMorphology::PIVOT_GENDER,
        MoroccanArabicMorphology::PIVOT_AGREEMENT_GENDER,
        MoroccanArabicMorphology::PIVOT_NUMBER,
        MoroccanArabicMorphology::PIVOT_AGREEMENT_NUMBER,
        MoroccanArabicMorphology::PIVOT_DEFINITENESS,
        MoroccanArabicMorphology::PIVOT_STATE,
        MoroccanArabicMorphology::PIVOT_ADJECTIVE_FORM,
        MoroccanArabicMorphology::PIVOT_PRONOUN_TYPE,
        MoroccanArabicMorphology::PIVOT_CLITIC,
        MoroccanArabicMorphology::PIVOT_ATTACHMENT_FUNCTION,
        MoroccanArabicMorphology::PIVOT_REFERENT_NUMBER,
        MoroccanArabicMorphology::PIVOT_REFERENT_GENDER,
        MoroccanArabicMorphology::PIVOT_PARTICLE_FUNCTION,
        MoroccanArabicMorphology::PIVOT_ADVERB_TYPE,
    ];

    fn supported_scripts(&self) -> &[Script] {
        // Arabic script is the ordinary written medium. Latin-script Arabizi,
        // with its 3/7/9 digit letters, is also in heavy contemporary use in
        // Moroccan digital writing.
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
        "1. Scope and lemmatization: analyze contemporary Moroccan Arabic / Darija (`ary`), with the Casablanca–Rabat urban koine as the default, and NEVER silently normalize it into Modern Standard Arabic or into an eastern dialect. Lemmatize verbs to the third-person masculine singular perfective (كايكتب/غايكتب/كتب -> كتب; كايتكلم -> تكلم; the imperative خوذ/خود -> خدا; a defective verb ends its lemma in ا, never ى: مشا، شرا، بقا، قرا، بغا، كلا), nouns to the indefinite singular, adjectives to the masculine singular lexical or participial form, and pronouns/determiners to their independent citation form. A participle's lemma is its OWN masculine singular, never the verb it derives from: كاينين -> كاين (not كان), گالس -> گالس (not گلس), عارفة -> عارف. There is no infinitive citation form.\n\
         2. Canonical script: keep `word` exactly in the source script. Write every lemma and root in Arabic script so Arabic-script and Latin/Arabizi input aggregate to one lexicon, mapping Arabizi digits back to letters (3 -> ع, 7 -> ح, 9/q -> ق, 5/kh -> خ, gh -> غ, sh/ch -> ش). Write a root as hyphen-separated radicals (`ك-ت-ب`, `ت-ر-ج-م`, including four radicals when real). Preserve Moroccan spellings and NEVER replace a Moroccan lemma with an MSA or Egyptian cognate: `بغا` is not `أراد`, `شاف` is not `رأى`, `دابا` is not `الآن`, `بزاف` is not `كثيرا`. Each preposition has ONE lemma whatever its written shape: ف and في -> `ف`, مع and معا -> `معا`, على and علي -> `على`, ب -> `ب`, ل -> `ل`.\n\
         3. Roots: include `root` only when a synchronically transparent Arabic root exists. `كتاب` has `ك-ت-ب`; `مكالمة` has `ك-ل-م`. OMIT root for the Amazigh, French and Spanish layers of the Moroccan lexicon — `خيزو`, `موش`, `طوموبيل`, `تيليفزيون`, `فيشطة`, `كوزينة`, `سيمانة`, `بلاصة` — and never manufacture a root from every consonant in a loan. A loan verb fully assimilated to a Moroccan pattern does take the radicals of its assimilated stem, and every ordinary patterned verb must have a root, including a real quadriliteral such as `ترجم` (`ت-ر-ج-م`). Write the radicals with their conventional Arabic letters even when the word is spelled with گ: `گالس` has `ج-ل-س`, never `گ-ل-س`.\n\
         4. Nouns: always report lexical gender, number, definiteness and state. Moroccan has NO productive nominal dual: two of anything is `جوج` plus a plural (`جوج ديال الكتب`), and `dual` is reserved for the small frozen residue of measure, time and paired-body nouns — `يومين`, `عامين`, `شهرين`, `سيمانتين`, `مرتين`, `عينين`, `يدين`. NEVER coin a dual on an ordinary noun, and never emit dual agreement anywhere else. There are no nominal case endings and no nunation. `باب الدار` is construct and definite; a possessive suffix also puts its host in construct state and makes it definite (`كتابو`). The analytic genitive is the Moroccan default and does NOT create a construct: in `الكتاب ديال الولد` both `كتاب` and `ولد` are `state: free` and definite. Free nouns are `state: free`.\n\
         5. Adjectives and participles: tag active and passive participles as Adjective even when predicative (`أنا عارف`, `هي جايا`, `كاين`, `مقاد`), choosing `active_participle` or `passive_participle`; use `elative` for the single أفعل-type or CCeC-type comparative form (`كبر`, `حسن`, `قصر`) whether context makes it comparative or superlative, and OMIT agreement_gender on it because the elative does not inflect. Report agreement_number. Report agreement_gender on a contrasting singular (`كبير` masculine versus `كبيرة` feminine), but OMIT it on a plural form such as `كبار` or `مزيانين`. An inanimate plural in Moroccan normally takes plural adjective agreement, not the eastern feminine-singular agreement. Attributive adjectives copy definiteness; predicate adjectives are indefinite.\n\
         6. Verbs: use exactly `perfective` (`كتب`), `bare_imperfective` (`يكتب` after بغيت/خاص/يمكن/باش/قبل ما or another governor), `ka_imperfective` (`كايكتب` for the ordinary present, habitual and progressive), `gha_imperfective` (`غايكتب`, and equally `يكتب` governed by a free-standing `غادي`: in `غادي نسافر` and `غادي يكون` the verb is `gha_imperfective` and `غادي` is a `future` particle, never `bare_imperfective`), or `imperative` (`كتب`). Moroccan has NO `بـ` imperfective and NO `حـ` future — those are Egyptian; never emit them and never read a Moroccan `بـ` as aspectual, since it is only the preposition `with/by`. The southern `تا-` and reduced `دا-` preverbs are `ka_imperfective`. Do not invent MSA tense or indicative/subjunctive/jussive ending values: Moroccan lost those endings. Always give pattern, person, agreement_number, voice and polarity; verb agreement is singular/plural only, never dual and never a distinct feminine plural.\n\
         7. Verb gender: Moroccan's gender contrast is paradigm-specific and this is where eastern-Arabic habits go wrong. In the PERFECTIVE, set agreement_gender ONLY in the third singular — `هو كتب` masculine, `هي كتبات` feminine — and OMIT it for the second singular, because `كتبتي` is used for both `نتا` and `نتي`. In the IMPERFECTIVE (bare, `كا-` and `غا-`), set it in the second singular (`كاتكتب` masculine versus `كاتكتبي` feminine) and in the third singular (`كايكتب` versus `كاتكتب`). In the IMPERATIVE, set it in the singular (`كتب` versus `كتبي`). OMIT agreement_gender for every first person, for every plural, and for the second-person perfective. Note that the imperfective prefix `ن-` is FIRST PERSON SINGULAR (`كانكتب` = I write) and `ن-...-و` first person plural (`كانكتبو`): never read `ن-` as a plural-only marker.\n\
         8. Voice and pattern: determine voice from the construction — `كتب` active, `تكتب`/`تّكتب` passive, `تحرك` middle_reflexive — and do not label every t-stem passive merely from its shape. Use the Moroccan measures: `form_i` (`كتب`), `form_ia` for the `تّـ/تـ` medio-passive of Form I (`تّكتب`, `تّخلع`, and the eastern `نـ` variant), `form_ii` (`بدل`, `علم`), `form_iia` for its t-stem (`تبدل`, `تعلم`), `form_iii` (`سافر`, `قابل`), `form_iiia` for its t-stem (`تقابل`, `تسارع`), `form_viii` (`حتارم`, `ختار`), `form_ix` for colour and defect verbs (`حمار`, `صفار`, `زراق`), `form_x` (`ستاغرب`, `ستاعمل`), `quadriliteral` (`ترجم`, `سيفط`) and `quadriliteral_a` for its t-stem (`تّرجم`). Moroccan has NO living Form IV, V, VI or VII: never emit one, and never assign a pattern because an MSA cognate has it — a Standard Arabic Form V verb is Moroccan `form_iia`, a Form VI verb is `form_iiia`, and a Form VII verb is `form_ia`.\n\
         9. Determiners: classify the attached article `ال-`, attributive demonstratives, the specific-indefinite `واحد`, quantifiers and interrogatives. The proximal `هاد` is INVARIABLE — `هاد الولد`, `هاد البنت`, `هاد الدراري` — so it MUST omit both referent_number and referent_gender; only the distal series inflects, `داك` masculine singular, `ديك` feminine singular, `دوك` plural, so `دوك` takes referent_number but omits referent_gender. The article `ال-` encodes neither and MUST omit both. `واحد` heading a still-articled noun (`واحد الراجل`) is `specific_indefinite`, not a numeral; `شي` heading a bare noun (`شي مشكلة`, `شي حاجة`, `شي واحد`) is `indefinite`, its non-specific counterpart, and never `specific_indefinite` or `quantifier`; `كل`, `گاع` and `بزاف د` are `quantifier`.\n\
         10. Pronouns: `clitic` is true only for an attached pronoun. For `كتابو`, attachment_function is `possessive`; for `شفتو`, it is `direct_object`; for the `ل` + pronoun series it is ALWAYS `indirect_object` — `ليا`, `ليك`, `ليه`, `ليها`, `لينا`, `ليكم`, `ليهم`, whether written on the verb (`كتبتليه`) or apart (`سمح ليا`); for any other preposition-hosted suffix such as `معاه`, `عليه`, `فيك`, `عندي` it is `prepositional`. An attached pronoun's lemma is the independent pronoun of the same person and number: ني/ي/يا -> `أنا`, ك -> `نتا`, و/ه -> `هو`, ها -> `هي`, نا -> `حنا`, كم -> `نتوما`, هم -> `هوما` (never `هما`); the clitic itself is never a lemma. When a host stacks both objects (`عطاهالي`), give each clitic its own analysis with its own function. OMIT attachment_function on independent `هو`. Set referent_person for personal forms (`أنا` first versus `نتا` second) and OMIT it for `اللي`; set referent_number for `أنا` singular versus `حنا` plural and OMIT it for `شكون`; set referent_gender for the independent forms that contrast it — `نتا` versus `نتي`, `هو` versus `هي` — and OMIT it for `أنا`, `حنا`, `نتوما`, `هوما`, `اللي` and any form without a gender contrast. Moroccan has no dual pronoun. The reflexive is `راس` plus a suffix (`راسي`, `راسو`): tag `راس` as `pronoun` with pronoun_type `reflexive` and split its suffix.\n\
         11. Negation: verbal negation is the circumfix `ما ... ش` (`ما كتبش`, `ما كايكتبش`, `ما غاديش يكتب`), and unlike Egyptian it triggers no stress shift or vowel change on the host. Non-verbal predicates, nouns, adjectives and participles are negated with `ماشي` (`ماشي هو`, `ماشي مزيان`), which is a `particle` with particle_function `negation`, never a verb. `ما` also appears WITHOUT `ش` before an absolute negator — `ما شفت حتى واحد`, `ما عندي والو` — and that clause is still negative. Mark the governed verb negative even when the negative pieces are split. Do not rewrite any of this as MSA `لم/لن/ليس`.\n\
         12. Tokenization: ALWAYS split these productive clitics into their own analyses, in every sentence and never only sometimes — conjunctions `و-/ف-`, prepositions `ب-/ل-/ف-` (`فالدار` is `ف` plus `ال` plus `دار`), the article `ال-`, the genitive `د-` when written bound (`دالولد` is `د` plus `ال` plus `ولد`), negative `ما-/-ش`, the presentative `را-/ها-`, and attached pronouns — while retaining their syntactic effect on the host. Do NOT split the aspect prefix `كا-/تا-` or the future prefix `غا-/غاد-`: those are recorded on the verb through `verb_form` as `ka_imperfective` and `gha_imperfective`, and must never also surface as a separate particle. Do NOT split person/number/gender inflection (`ن-/ت-/ي-`, `-ت`, `-تي`, `-ي`, `-و`). When you split a written token, each analysis's `word` is exactly the piece it analyses; the pieces must not overlap and must never restate the whole token, so `عليها` is `علي` plus `ها` and never `عليها` alongside `ها`. This applies to EVERY host, including ones that look like single words: `عليكم` is `علي` plus `كم`, `معاه` is `معا` plus `ه`, `كتابو` is `كتاب` plus `و`, `راني` is `را` plus `ني`. A host must always reach the lexicon under its bare surface, so never record `عليكم` or `راني` as a whole surface for the lemma `على` or `را`. The article is split inside greeting formulas and day names too: `السلام عليكم` is `ال` plus `سلام`, `نهار الحد` is `نهار` plus `ال` plus `حد`. Three exceptions are lexicalized and stay whole: the time adverbs `البارح`, `اليوم` and `الليلة` (yesterday, today, tonight) are one `adverb` each with the written form as lemma, never `noun` and never split — `فالصباح` by contrast is `ف` plus `ال` plus `صباح`; and the conjunctions `ولا` (or) and `ولكن` (but) are single `coordinating_conjunction` tokens, never `و` plus `لا`/`لكن`. `كيداير`/`كيدايرة`/`كيدايرين` (how are you) is `كي` (`adverb`) plus the participle `داير` (`adjective`, `active_participle`, lemma `داير`), never an `interjection`. Omit punctuation.\n\
         13. Particles: every `particle` you emit MUST carry a `particle_function`. `ما`, `ش` and `ماشي` are `negation`; the yes/no question marker `واش` is `interrogative`; the vocative `آ` and `يا` are `vocative`; a free-standing `غادي`/`غادية`/`غاديين` immediately before a verb is `future`; the assertive `را-` series (`راني`, `راك`, `راه`) and the presentative `ها` are `presentative`. `كاين`/`كاينة`/`كاينين` is NOT a particle: it is the active participle of `كان`, so tag it as `adjective` with adjective_form `active_participle`. The genitive `ديال`/`د` is an `adposition`, never a particle and never a noun.\n\
         14. Conjunctions, names and formulas: decide coordination versus subordination for every conjunction and emit the specific value. `coordinating_conjunction` covers و، ف، أو، ولا، بصح، ولكن; `subordinating_conjunction` covers ملي، فاش، باش، حيت، علاحقاش، إلا، واخا، بلا ما، قبل ما، من بعد ما، حتى. NEVER emit a bare `conjunction` or `conj`: an unspecific tag is filed as coordinating and silently mis-analyses Moroccan subordination. `اللي` is not a conjunction; it is a relative pronoun. A personal or place name is `proper_noun`, which takes only a lemma — `فاطمة`، `يوسف`، `المغرب`، `الدار البيضاء`، `مراكش` — and never a manufactured root. Terms of address are NOT proper nouns: `سيدي` is the noun `سيد` in construct state plus the possessive `ي`, `خويا` is `خو` plus `يا`, `ختي` is `خت` plus `ي`, and `لالة` is a free feminine noun. `بعد`, `قبل`, `قدام`, `حدا` and `عند` governing a noun or a suffix are `adposition` (`من بعد يومين`, `عندي`), not adverbs. Greeting and reaction formulas — `أهلا`، `شكرا`، `صافي`، `يالله`، `واخا`، `الله يخليك`، `إن شاء الله` — are `interjection`, not nouns.\n\
         15. VALUE RULES: noun `number` is exactly singular/dual/plural, with `dual` only on the frozen residue in rule 4; `agreement_number` and referent_number are only singular/plural. Gender values are only masculine/feminine and must never appear in a number field. Every `noun` MUST carry gender, number, definiteness and state. Never emit `case`, `mood`, MSA nunation, dual verb agreement, feminine-plural verb agreement, a `bi_imperfective`, a `ha_imperfective`, or a Form IV, V, VI or VII pattern."
    }


    fn alignment_directives(&self) -> Option<&'static str> {
        Some(
            "1. Proclitics written attached to the next word are segments of that word: conjunctions و-/ف-, prepositions ب-/ل-/ف-, the article ال-, bound genitive د-, negative ما-, aspectual كا-/تا-, future غا-/غادي-, presentative را-/ها- — [\"ف\", \"ال\", \"دار\"], [\"د\", \"ال\", \"ولد\"], [\"كا\", \"ن\", \"كتب\"], [\"غا\", \"ي\", \"مشي\"].\n\
             2. Enclitics are segments of their host: object and possessive pronouns (ني/ي، ك، و/ه، ها، نا، كم، هم), negative -ش, and the ل + pronoun dative series written on the verb — [\"كتاب\", \"و\"], [\"شفت\", \"ك\"], [\"كتبت\", \"ليه\"]. A feminine ة written ت before a suffix stays in the host segment.\n\
             3. Negation ما … ش is one discontinuous unit: [\"ما\"], [\"كتبت\", \"ش\"] in one link; ما alone before والو/حتى is that same unit; ماشي is a separate word.\n\
             4. Subject inflection on the verb (يـ/تـ/ن/ك- prefixes, -ت, -ي, -و) is a segment only when the other sentence expresses that subject as its own unit.\n\
             5. Latin-script Arabizi (3 = ع, 7 = ح, 9 = ق) splits at the same boundaries: [\"f\", \"dar\"], [\"ktab\", \"i\"].",
        )
    }
}

#[cfg(test)]
mod tests {
    use panini_core::pivot::PivotValueKind;

    use super::*;

    #[test]
    fn moroccan_identity_and_scripts_are_exact() {
        let language = MoroccanArabic;

        assert_eq!(MoroccanArabic::ISO_LANG, IsoLang::Ary);
        assert_eq!(language.supported_scripts(), &[Script::ARAB, Script::LATN]);
        assert_eq!(language.default_script(), Script::ARAB);
    }

    #[test]
    fn optional_root_remains_an_open_pivot() {
        let verb = MoroccanArabicMorphology::Verb {
            lemma: "كتب".to_string(),
            root: Some("ك-ت-ب".to_string()),
            pattern: MoroccanArabicVerbPattern::FormI,
            verb_form: MoroccanArabicVerbForm::KaImperfective,
            voice: MoroccanArabicVoice::Active,
            polarity: MoroccanArabicPolarity::Affirmative,
            person: Person::Third,
            agreement_number: BinaryNumber::Singular,
            agreement_gender: Some(BinaryGender::Masculine),
        };

        assert_eq!(
            MoroccanArabicMorphology::PIVOT_ROOT.value_kind,
            PivotValueKind::Open
        );
        assert_eq!(
            MoroccanArabicMorphology::PIVOT_ROOT.value(&verb),
            Some("ك-ت-ب".to_string())
        );
    }

    /// Moroccan's preverbs are `كا-` and `غا-`; a `bi_`/`ha_` here means the wrong grammar was copied.
    #[test]
    fn verb_form_inventory_is_moroccan_not_egyptian() {
        assert_eq!(
            MoroccanArabicMorphology::PIVOT_VERB_FORM.values(),
            &[
                "perfective",
                "bare_imperfective",
                "ka_imperfective",
                "gha_imperfective",
                "imperative",
            ]
        );
    }

    /// Moroccan t-stems are Ia/IIa/IIIa, and Forms IV to VII are not living patterns.
    #[test]
    fn verb_pattern_inventory_uses_the_moroccan_measures() {
        assert_eq!(
            MoroccanArabicMorphology::PIVOT_PATTERN.values(),
            &[
                "form_i",
                "form_ia",
                "form_ii",
                "form_iia",
                "form_iii",
                "form_iiia",
                "form_viii",
                "form_ix",
                "form_x",
                "quadriliteral",
                "quadriliteral_a",
            ]
        );
    }

    #[test]
    fn conjugation_and_nominal_inflection_are_enabled() {
        assert_eq!(
            MoroccanArabic.typological_features(),
            &[
                TypologicalFeature::Conjugation(&[Upos::Verb]),
                TypologicalFeature::Declension(&[Upos::Noun, Upos::Adjective]),
            ]
        );
    }

    /// `هاد` is invariable for gender and number.
    #[test]
    fn the_invariable_proximal_demonstrative_carries_no_agreement() {
        let proximal = MoroccanArabicMorphology::Determiner {
            lemma: "هاد".to_string(),
            determiner_type: MoroccanArabicDeterminerType::Demonstrative,
            referent_number: None,
            referent_gender: None,
        };
        let distal_feminine = MoroccanArabicMorphology::Determiner {
            lemma: "ديك".to_string(),
            determiner_type: MoroccanArabicDeterminerType::Demonstrative,
            referent_number: Some(BinaryNumber::Singular),
            referent_gender: Some(BinaryGender::Feminine),
        };

        assert_eq!(
            MoroccanArabicMorphology::PIVOT_REFERENT_NUMBER.value(&proximal),
            None
        );
        assert_eq!(
            MoroccanArabicMorphology::PIVOT_REFERENT_GENDER.value(&proximal),
            None
        );
        assert_eq!(
            MoroccanArabicMorphology::PIVOT_REFERENT_GENDER.value(&distal_feminine),
            Some("feminine".to_string())
        );
    }

    /// The second-person perfective is gender-syncretic: `كتبتي` serves both
    /// `نتا` and `نتي`, so the field is genuinely absent rather than unknown.
    #[test]
    fn the_second_person_perfective_has_no_gender() {
        let second_perfective = MoroccanArabicMorphology::Verb {
            lemma: "كتب".to_string(),
            root: Some("ك-ت-ب".to_string()),
            pattern: MoroccanArabicVerbPattern::FormI,
            verb_form: MoroccanArabicVerbForm::Perfective,
            voice: MoroccanArabicVoice::Active,
            polarity: MoroccanArabicPolarity::Affirmative,
            person: Person::Second,
            agreement_number: BinaryNumber::Singular,
            agreement_gender: None,
        };

        assert_eq!(
            MoroccanArabicMorphology::PIVOT_AGREEMENT_GENDER.value(&second_perfective),
            None
        );
    }
}
