use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature,
};

/// The level of speech a Thai lexeme belongs to.
///
/// Thai lexicalises social relation rather than inflecting for it: whole
/// lexemes are paired across registers (`หมา` / `สุนัข`, `หัว` / `ศีรษะ`,
/// `กิน` / `รับประทาน` / `เสวย` / `ฉัน`), so choosing the register-appropriate
/// word is a per-token decision a learner has to make and the place Thai's
/// register diglossia actually lives. `Neutral` is the honest unmarked value,
/// not a missing extraction.
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
pub enum ThaiRegister {
    // ราชาศัพท์ — ทรง, เสวย, บรรทม, พระราชทาน, เพคะ
    Royal,
    // คำพระ — อาตมา, นิมนต์, มรณภาพ, ฉัน (of a monk eating)
    Monastic,
    // ภาษาทางการ/ภาษาเขียน — รับประทาน, ศีรษะ, สุนัข, สตรี, ข้าพเจ้า
    Formal,
    // ภาษากลาง — the unmarked everyday word
    Neutral,
    // ภาษาพูด — หมา, เยอะ, ยังไง, มั้ย, เค้า
    Colloquial,
    // ภาษาหยาบ — กู, มึง, แดก, วะ, โว้ย
    Vulgar,
}

/// What kind of counting unit a Thai ลักษณนาม is.
///
/// Deliberately functional rather than semantic. Which classifier a given noun
/// takes is captured by the open `classifier` pivot on `Noun`; this dimension
/// answers the orthogonal question of what sort of unit the classifier counts,
/// where the boundaries are crisp enough to be extracted reliably.
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
pub enum ThaiClassifierType {
    // คน, ตัว, เล่ม, ใบ, คัน, ลูก, แผ่น, อัน, ต้น, หลัง, ฉบับ
    Sortal,
    // กิโล, เมตร, บาท, ชั่วโมง, แก้ว, ขวด, ช้อน
    Measure,
    // คู่, ชุด, ฝูง, กลุ่ม, โหล, พวก
    Collective,
    // ครั้ง, ที, หน, รอบ
    Event,
    // the noun serving as its own classifier — ประเทศ, จังหวัด, ห้อง, คำ, วัน
    Repeater,
    // องค์, รูป, พระองค์ — monks, royalty, deities, sacred images
    Deferential,
    // a genuine classifier outside the classes above
    Other,
}

/// The learner-relevant grammatical function of a Thai adverb.
///
/// Thai marks tense, aspect, negation, degree and epistemic stance with free
/// words standing before or after the verb. Keeping those markers on their own
/// token, dimensioned here, stops an analytic construction from being
/// misreported as verb inflection Thai does not have.
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
pub enum ThaiAdverbType {
    // an ordinary lexical adverb, including manner and time expressions
    Lexical,
    // จะ, กำลัง, แล้ว, เคย, ยัง, อยู่, เพิ่ง, completive ได้
    TemporalAspect,
    // ไม่, ไม่ได้, มิ, prohibitive อย่า
    Negation,
    // มาก, ที่สุด, ค่อนข้าง, เกินไป, นิดหน่อย, จัง
    Degree,
    // คง, คงจะ, อาจ, น่าจะ, ย่อม
    Modal,
}

/// The discourse function of a Thai particle.
///
/// Sentence-final particles carry politeness, speaker gender and illocutionary
/// force, and several forms serve different functions in different positions,
/// so the inventory is functional rather than positional.
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
pub enum ThaiParticleType {
    // ครับ, ค่ะ, คะ, ขา, จ้ะ, ฮะ, วะ, ขอรับ, เพคะ
    Politeness,
    // ไหม, มั้ย, หรือ, เหรอ, รึ, เปล่า
    Interrogative,
    // นะ, สิ, ซิ, เถอะ, หรอก, แหละ, ล่ะ
    Modal,
    // เอง, focusing ก็
    Focus,
    // น่ะ, นี่, เนี่ย marking a preceding topic
    Topic,
    // connective and turn-management uses such as คือ and ก็
    Discourse,
    // a genuine particle outside the functions above
    Other,
}

/// How an expression functioning as a Thai pronoun establishes reference.
///
/// `KinshipAddress` and `TitleAddress` stay distinct from dedicated personal
/// pronouns because Thai person reference is relational: `พี่` or a given name
/// denotes the speaker, the addressee or a third person according to who is
/// talking to whom.
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
pub enum ThaiPronounType {
    Personal,
    KinshipAddress,
    TitleAddress,
    Demonstrative,
    Interrogative,
    Relative,
    Reflexive,
    Reciprocal,
    IndefiniteGeneric,
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
pub enum ThaiMorphology {
    Adjective {
        lemma: String,
        register: ThaiRegister,
    },
    Adposition {
        lemma: String,
    },
    Adverb {
        lemma: String,
        adverb_type: ThaiAdverbType,
    },
    Classifier {
        lemma: String,
        classifier_type: ThaiClassifierType,
    },
    CoordinatingConjunction {
        lemma: String,
    },
    Determiner {
        lemma: String,
    },
    Interjection {
        lemma: String,
    },
    Noun {
        lemma: String,
        register: ThaiRegister,
        /// The Thai form of the classifier this noun conventionally takes when
        /// counted. Present only for a well-established pairing; absent for
        /// abstract, mass and nominalized nouns, and whenever the pairing is
        /// uncertain.
        #[serde(skip_serializing_if = "Option::is_none")]
        classifier: Option<String>,
    },
    Numeral {
        lemma: String,
    },
    Particle {
        lemma: String,
        particle_type: ThaiParticleType,
        register: ThaiRegister,
    },
    Pronoun {
        lemma: String,
        pronoun_type: ThaiPronounType,
        register: ThaiRegister,
        /// Present only when this occurrence refers to a speech participant or
        /// a third person; relational address terms take their person from
        /// context, not from the lemma.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Present only where the pronominal expression itself establishes
        /// number.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    ProperNoun {
        lemma: String,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        register: ThaiRegister,
    },
    Other {
        lemma: String,
    },
}

impl ThaiMorphology {
    /// Extracts the conventional classifier of a noun for the classifier pivot.
    ///
    /// `classifier` is `Option` on the noun — a การ-/ความ- nominalization or an
    /// abstract noun genuinely has none — so the `MorphologyInfo` derive skips
    /// it for pivot generation. This hand-written handle keeps `PIVOT_CLASSIFIER`
    /// available for lexicon faceting, because pairing a noun with its
    /// classifier is the single hardest thing about Thai grammar and the drill
    /// a learner most needs to slice their lexicon by.
    fn __pivot_classifier(&self) -> Option<String> {
        match self {
            Self::Noun { classifier, .. } => classifier.clone(),
            _ => None,
        }
    }

    /// Typed pivot handle for the noun-classifier pairing. Open, because the
    /// classifier inventory is large, semi-productive and best published as
    /// whatever was actually extracted. Defined manually because `classifier`
    /// is optional (see [`ThaiMorphology::__pivot_classifier`]).
    pub const PIVOT_CLASSIFIER: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::open("classifier", "Classifier", Self::__pivot_classifier);
}

pub struct Thai;

impl LinguisticDefinition for Thai {
    type Morphology = ThaiMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Tha;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        ThaiMorphology::PIVOT_REGISTER,
        ThaiMorphology::PIVOT_CLASSIFIER,
        ThaiMorphology::PIVOT_CLASSIFIER_TYPE,
        ThaiMorphology::PIVOT_PRONOUN_TYPE,
        ThaiMorphology::PIVOT_PARTICLE_TYPE,
        ThaiMorphology::PIVOT_ADVERB_TYPE,
    ];

    /// Standard Thai is written in the Thai script alone. The script is an
    /// abugida written left to right; Thai Braille and the RTGS romanization
    /// are transcription conventions, not alternative orthographies, and the
    /// regional Tai scripts (Tai Tham, Tai Noi) belong to the separate
    /// Northern and Isan languages, which are out of scope for this course.
    fn supported_scripts(&self) -> &[Script] {
        &[Script::THAI]
    }

    fn default_script(&self) -> Script {
        Script::THAI
    }

    /// Thai is isolating: no word inflects for tense, aspect, number, gender,
    /// person or case, so it has neither a conjugation nor a declension the
    /// cloze card models could draw on, and no agglutinative morphology to
    /// segment.
    fn typological_features(&self) -> &[TypologicalFeature] {
        &[]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope, script and orthographic integrity: analyze contemporary Standard (Central) Thai written in the Thai script. Every surface form and every lemma must be in Thai script — never romanize, transliterate, or add an RTGS or phonetic respelling. Reproduce each token exactly as written, including the tone marks ไม้เอก, ไม้โท, ไม้ตรี and ไม้จัตวา, every vowel sign written above, below, before or after its consonant, ไม้หันอากาศ, ไม้ไต่คู้ and การันต์. These marks are part of the orthographic word and are never affixes: removing or reordering one produces a different word or a non-word, since เขา, เข่า and เข้า are three distinct lexemes, as are ปา, ป่า and ป้า. Do not convert Thai digits to Arabic digits or the reverse.\n\
         2. Tokenization — this is the single most important instruction here. Thai is written WITHOUT SPACES BETWEEN WORDS. A space in Thai marks a clause, phrase, list or sentence boundary, or brackets a numeral, a name or a foreign word; a space is NOT a word boundary, and a run of characters between two spaces is NOT one token. Segment the unspaced string into orthographic words using lexical knowledge of Thai, the way a dictionary would. Emit one TokenAnalysis per word, in reading order, and make each surface the exact contiguous substring of the input: add no character, delete none, and never insert a space inside a surface form. Concatenating the surfaces in order must reproduce the input apart from the whitespace between them.\n\
         3. Where to cut and where not to: keep an established lexical compound as ONE token (โรงเรียน, รถไฟ, หนังสือพิมพ์, ห้องน้ำ, โทรศัพท์, ผู้จัดการ, เครื่องบิน, ต่างประเทศ) and never split it at an internal syllable boundary merely because each syllable is a word on its own — โรงเรียน is never โรง plus เรียน. The nominalizing prefixes การ and ความ, and the agentive or instrument prefixes นัก, ผู้, ช่าง, เครื่อง and ชาว, bind to their base and form one token (การเดินทาง, ความสุข, นักเรียน, ชาวนา). Conversely, never fuse a productive phrase into one token: a noun, its numeral and its classifier are three tokens (หนังสือ, สาม, เล่ม); a verb and its aspect marker are two (กำลัง, กิน); a verb and its directional complement are two (เดิน, ไป); and a string of sentence-final particles is one token per particle (นะ then ครับ; ไหม then คะ). A personal name written with a space between given name and surname is two ProperNoun tokens. Where a string genuinely admits two segmentations, choose the one that yields established dictionary words rather than the one that yields more, shorter fragments.\n\
         4. Lemmatization is near-trivial and must stay that way: Thai has no inflectional morphology at all — nothing marks tense, aspect, number, gender, person, case or agreement on a word. The surface form IS the lemma. Copy the token's surface into lemma unchanged. Never de-conjugate, never strip a prefix or a supposed affix, never delete a tone mark or a vowel sign to reach a root, and never substitute a citation form that differs from what appears in the text. The only orthographic wrinkle is the repetition mark ๆ: keep it attached to the word it repeats and let the reduplicated form stand as its own lemma (เด็กๆ, เร็วๆ, บ่อยๆ).\n\
         5. Verbs and analytic predicates: a Verb carries no tense, aspect, mood, voice, person or number — those categories do not exist in Thai and must never be invented or inferred. Tense and aspect are free words analyzed as separate tokens; see directive 6. Tag เป็น, คือ, existential and locative อยู่ and มี as Verbs. Under Panini's auxiliary-to-verb policy, tag the preverbal modal and passive words ต้อง, ควร, สามารถ, อยาก, ถูก and โดน as Verbs in their own right and never transfer their meaning onto the lexical verb. In a serial verb construction every verb is its own token: the directional and resultative complements ไป, มา, ขึ้น, ลง, เข้า, ออก and เสร็จ are separate Verbs, or Adverbs where they mark aspect, never suffixes on the preceding verb.\n\
         6. Adverbs: every Adverb gets adverb_type. Use temporal_aspect for the free tense and aspect markers, which are ordinary words standing before or after the verb and never inflection: จะ, กำลัง, แล้ว, เคย, ยัง, postverbal อยู่, เพิ่ง, and ได้ in its completive use. Their precise temporal or aspectual reading comes from context; report the marker, do not infer a tense. Use negation for ไม่, ไม่ได้, มิ and prohibitive อย่า; degree for มาก, ที่สุด, ค่อนข้าง, เกินไป, นิดหน่อย and จัง; modal for the epistemic stance words คง, คงจะ, อาจ, น่าจะ and ย่อม; otherwise lexical.\n\
         7. Property words: Thai property words predicate directly with no copula, so บ้านใหญ่ already means the house is big. Tag conventional property lexemes such as ใหญ่, ดี, สวย, ร้อน and แพง as Adjectives rather than as inflected verbs, and never supply a missing เป็น. An attributive adjective follows its noun and stays a separate token from it.\n\
         8. Classifiers (ลักษณนาม): tag every counting or measure word as Classifier and give it a classifier_type. Use sortal for the ordinary noun-class classifiers (คน, ตัว, เล่ม, ใบ, คัน, ลูก, แผ่น, อัน, ต้น, หลัง, ฉบับ, ชิ้น, เส้น); measure for units of measurement and quantity (กิโล, เมตร, บาท, ชั่วโมง, แก้ว, ขวด, ช้อน); collective for group units (คู่, ชุด, ฝูง, กลุ่ม, โหล, พวก); event for occurrence units (ครั้ง, ที, หน, รอบ); repeater when the noun serves as its own classifier (ประเทศ, จังหวัด, ห้อง, คำ, ข้อ, วัน); deferential for องค์, รูป and พระองค์ used of monks, royalty, deities and sacred images; and other only for a genuine classifier outside these classes. A classifier also appears with a demonstrative and no numeral (หนังสือเล่มนี้) and with an adjective (เสื้อตัวใหญ่). Most classifier forms are also ordinary nouns — คน a person, ใบ a leaf, ตัว a body, ลูก a child — so decide by syntax, not by the word: immediately after a numeral, or between a noun and นี้, นั้น or ไหน, it is a Classifier; heading its own noun phrase it is a Noun. A classifier is a grammatical counting word, not a vocabulary item standing in for the noun it counts.\n\
         9. The classifier field on a noun: on a Noun, set classifier to the Thai form of the classifier that noun conventionally takes when counted — หนังสือ takes เล่ม, รถ takes คัน, หมา takes ตัว, ไข่ takes ใบ, บ้าน takes หลัง, คน takes คน. Give the classifier alone, in Thai script, with no numeral attached. OMIT the field entirely when the noun is abstract or a mass noun, when it is a การ or ความ nominalization, when it is a proper name or a recent borrowing with no established pairing, and whenever you are not confident of the conventional pairing. Never fall back to อัน as a default and never guess: an omitted field is correct, while a wrong pairing teaches the learner an error.\n\
         10. Register: every Noun, Verb, Adjective, Pronoun and Particle gets register, the level of speech the lexeme itself belongs to. Use neutral for the ordinary unmarked everyday word — the correct value for the large majority of tokens — and note that a word is not formal merely because its origin is Pali or Sanskrit: โรงเรียน, ประเทศ and อาหาร are neutral. Use formal for the elevated written, official and polite-alternative vocabulary (รับประทาน, ศีรษะ, สุนัข, สตรี, บุรุษ, มารดา, ข้าพเจ้า, กระผม, รถยนต์). Use colloquial for the spoken and reduced forms (หมา, เยอะ, ไง, ยังไง, มั้ย, เนี่ย, เค้า, จ้ะ). Use vulgar for the crude register (กู, มึง, แดก, วะ, โว้ย). Use royal only for genuine ราชาศัพท์ (ทรง, เสวย, บรรทม, พระราชทาน, พระเนตร, ตรัส, เพคะ), and monastic only for genuine monastic vocabulary (อาตมา, นิมนต์, มรณภาพ, โยม, and ฉัน in its monk-eating sense). Register is a property of the lexeme, not of the sentence: do not mark every word of a formal sentence formal.\n\
         11. Pronouns and person reference: every Pronoun gets pronoun_type and register. Thai person reference is a large, socially indexed and semi-open system, not a two-way formal-versus-informal contrast, so never collapse it onto one politeness scale. Use personal for the dedicated pronouns (ผม, ดิฉัน, ฉัน, หนู, เรา, กระผม, ข้าพเจ้า, คุณ, เธอ, ท่าน, แก, เขา, มัน, กู, มึง); kinship_address when a kinship term functions as person reference (พี่, น้อง, พ่อ, แม่, ป้า, ลุง, น้า, อา, ปู่, ย่า, ตา, ยาย); and title_address when an occupational, status or honorific term, or a personal name, does the same (อาจารย์, ครู, หมอ, คุณครู, พระ, and a given name used to mean I or you). Assign person from the referent in this occurrence and never permanently from the lemma: พี่ and หนู can be first, second or third person depending on who is speaking to whom. Add number only when the expression itself establishes it, as with a พวก compound or group เรา. Omit person and number for demonstrative (นี่, นั่น, โน่น), interrogative (ใคร, อะไร, ไหน), relative (ที่, ซึ่ง, อัน), reflexive (ตัวเอง, ตนเอง), reciprocal (กัน) and indefinite_generic (ใครๆ, impersonal เขา) pronouns. Thai drops pronouns pervasively: analyze only the pronouns actually present and never insert an implied one.\n\
         12. Particles: every Particle gets particle_type and register. Use politeness for the speaker-gender-indexed sentence-final particles — ครับ and ครับผม for male speakers, ค่ะ, คะ and ขา for female speakers, plus จ้ะ, จ๊ะ, ฮะ, วะ, ขอรับ and เพคะ — and keep the written tone mark exactly as it appears, because ค่ะ and คะ are different tokens with different functions. Use interrogative for ไหม, มั้ย, หรือ, เหรอ, รึ and question-frame เปล่า; modal for the stance and illocutionary particles นะ, สิ, ซิ, เถอะ, หรอก, แหละ and ล่ะ; focus for เอง and focusing ก็; topic for น่ะ, นี่ and เนี่ย marking a preceding topic; discourse for connective and turn-management uses such as คือ and ก็; and other only for a genuine particle outside these functions. A sentence-final particle is a grammatical word, not a content word: never tag it Noun, Verb or Interjection, never merge it into the preceding word, and never drop it because it has no English translation.\n\
         13. Determiners, numerals and residue: tag adnominal นี้, นั้น, โน้น, ทุก, บาง, แต่ละ, หลาย, ต่าง and อื่น as Determiners, while the standalone forms นี่, นั่น and โน่น are Pronouns. In the ordinal frame ที่ followed by a numeral, tag ที่ as a Determiner and the numeral as a Numeral. Tag numerals as Numeral and preserve Thai or Arabic digits exactly as written. Thai nouns are number-neutral: never report a noun as plural, and let พวก, a numeral or ๆ carry plurality on its own token. Never emit punctuation as a token; tag ฯลฯ and other non-alphabetic signs as Symbol, and use Other only for material that genuinely fits no part of speech."
    }
}

#[cfg(test)]
mod tests {
    use panini_core::aggregable::ClosedValues;
    use panini_core::pivot::PivotValueKind;

    use super::*;

    #[test]
    fn thai_identity_script_and_typology_are_exact() {
        let language = Thai;

        assert_eq!(Thai::ISO_LANG, IsoLang::Tha);
        assert_eq!(Thai::ISO_LANG.to_639_3(), "tha");
        assert_eq!(language.supported_scripts(), &[Script::THAI]);
        assert_eq!(language.default_script(), Script::THAI);
        assert_eq!(language.default_script().resolve().num, "352");
        assert!(
            language.typological_features().is_empty(),
            "Thai is isolating: declaring conjugation or declension would offer cloze \
             exercises the grammar cannot support"
        );
    }

    #[test]
    fn register_and_classifier_lead_the_curated_pivots() {
        let keys = Thai::MORPHOLOGY_PIVOTS
            .iter()
            .map(|pivot| pivot.key)
            .collect::<Vec<_>>();

        assert_eq!(
            keys,
            [
                "register",
                "classifier",
                "classifier_type",
                "pronoun_type",
                "particle_type",
                "adverb_type",
            ]
        );
    }

    #[test]
    fn the_noun_classifier_pairing_is_an_open_pivot() {
        let book = ThaiMorphology::Noun {
            lemma: "หนังสือ".to_string(),
            register: ThaiRegister::Neutral,
            classifier: Some("เล่ม".to_string()),
        };
        let happiness = ThaiMorphology::Noun {
            lemma: "ความสุข".to_string(),
            register: ThaiRegister::Neutral,
            classifier: None,
        };

        assert_eq!(
            ThaiMorphology::PIVOT_CLASSIFIER.value_kind,
            PivotValueKind::Open
        );
        assert_eq!(
            ThaiMorphology::PIVOT_CLASSIFIER.value(&book),
            Some("เล่ม".to_string())
        );
        assert_eq!(ThaiMorphology::PIVOT_CLASSIFIER.value(&happiness), None);
    }

    #[test]
    fn register_levels_have_stable_wire_values() {
        assert_eq!(
            ThaiRegister::all_variants(),
            &[
                "royal",
                "monastic",
                "formal",
                "neutral",
                "colloquial",
                "vulgar",
            ]
        );
    }

    #[test]
    fn relational_address_person_is_contextual_and_register_marked() {
        let elder_sibling_as_addressee = ThaiMorphology::Pronoun {
            lemma: "พี่".to_string(),
            pronoun_type: ThaiPronounType::KinshipAddress,
            register: ThaiRegister::Neutral,
            person: Some(Person::Second),
            number: Some(BinaryNumber::Singular),
        };

        let serialized =
            serde_json::to_value(elder_sibling_as_addressee).expect("pronoun is serializable");
        assert_eq!(serialized["pos"], "pronoun");
        assert_eq!(serialized["pronoun_type"], "kinship_address");
        assert_eq!(serialized["register"], "neutral");
        assert_eq!(serialized["person"], "second");
    }

    #[test]
    fn a_verb_carries_no_inflectional_dimension() {
        let eat = ThaiMorphology::Verb {
            lemma: "กิน".to_string(),
            register: ThaiRegister::Neutral,
        };

        let serialized = serde_json::to_value(eat).expect("verb is serializable");
        let fields = serialized
            .as_object()
            .expect("verb serializes to an object")
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            fields,
            ["pos", "lemma", "register"],
            "Thai verbs never inflect: tense and aspect live on their own adverb tokens"
        );
    }
}
