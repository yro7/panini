use serde::{Deserialize, Serialize};

use panini_core::traits::{
    BinaryGender, BinaryNumber, IsoLang, LinguisticDefinition, Person, Script, TypologicalFeature,
    Upos,
};

/// The five simple synthetic paradigms of the Spanish verb.
///
/// Every compound tense is periphrastic — `haber` plus an invariable
/// participio — and is analysed as two tokens, so no `perfect` or `pluperfect`
/// value is needed. The `condicional` is a tense of the indicative, alongside
/// the futuro it is built from, not a mood.
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
pub enum SpanishTense {
    Present,     // presente: indicative `hablo`, subjunctive `hable`
    Preterite,   // pretérito perfecto simple, the indefinido: `hablé`, `fue`
    Imperfect,   // pretérito imperfecto: `hablaba`, and the subjunctive `hablara` / `hablase`
    Future,      // futuro simple `hablaré`, and the residual future subjunctive `hablare`
    Conditional, // condicional simple `hablaría`, a tense of the indicative
}

/// The form a Spanish verb token is in.
///
/// The first three are the finite moods; the last three are the *formas no
/// personales*, which carry no person and no tense. Every verb token is in
/// exactly one of the six.
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
pub enum SpanishMood {
    Indicative,
    Subjunctive,
    Imperative,
    Infinitive,
    Gerund,
    Participle,
}

/// Degree of comparison on the adjective. Only the forms that carry it in
/// their own shape: the suppletive comparatives `mejor`, `peor`, `mayor`,
/// `menor`, and the synthetic absolute superlative in `-ísimo` / `-érrimo`.
/// `más alto` is an Adverb plus a positive adjective.
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
pub enum SpanishDegree {
    Positive,    // `rico`, `alto`
    Comparative, // `mejor`, `peor`, `mayor`, `menor`
    Superlative, // `riquísimo`, `altísimo`, `celebérrimo`
}

/// The case-like contrast carried by the pronoun system — the only corner of
/// Spanish nominal morphology that has one.
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
pub enum SpanishPronounCase {
    Subject,
    DirectObject,
    IndirectObject,
    Reflexive,
    Prepositional,
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
pub enum SpanishPronounType {
    Personal,
    Possessive,
    Demonstrative,
    Relative,
    Interrogative,
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
pub enum SpanishDeterminerType {
    Article,
    Possessive,
    Demonstrative,
    Interrogative,
    Indefinite,
    Relative,
}

/// Spanish morphology.
///
/// No `Particle` variant: every candidate the tag would attract is analysed
/// otherwise by Spanish grammar — `no` is an adverb, the personal `a` a
/// preposition, `se` a pronoun, completive `que` a subordinating conjunction.
/// The extraction directives route them explicitly.
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
pub enum SpanishMorphology {
    /// Gender and number are agreement features here, reported even when the
    /// form is syncretic (`feliz` in `la mujer feliz` is feminine singular).
    Adjective {
        lemma: String,
        gender: BinaryGender,
        number: BinaryNumber,
        /// `riquísimo` keeps the lemma `rico` and records the superlative here.
        degree: SpanishDegree,
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
        determiner_type: SpanishDeterminerType,
        /// Absent for the neuter article `lo`, which agrees with nothing.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        number: BinaryNumber,
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
        /// Only the numerals that inflect (`un` / `una`, `doscientos` /
        /// `doscientas`, the ordinals); the rest are invariable.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    Pronoun {
        lemma: String,
        pronoun_type: SpanishPronounType,
        /// The átono / tónico split: `me` against `mí`, `se` against `sí`.
        /// Required because every pronoun token is one or the other, and not
        /// recoverable from `case` — `se` and `sí` are both reflexive.
        clitic: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// Absent for the pronouns with no case contrast at all
        /// (`esto`, `quien`, `alguien`, `el mío`).
        #[serde(skip_serializing_if = "Option::is_none")]
        case: Option<SpanishPronounCase>,
    },
    /// Optional features: a proper noun only has gender and number where
    /// Spanish actually agrees with it (`la vieja Europa`, `los Andes`).
    ProperNoun {
        lemma: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
    },
    SubordinatingConjunction {
        lemma: String,
    },
    Symbol {
        lemma: String,
    },
    Verb {
        lemma: String,
        mood: SpanishMood,
        /// Finite indicative and subjunctive forms only. The imperative has no
        /// tense contrast, and neither do the formas no personales.
        #[serde(skip_serializing_if = "Option::is_none")]
        tense: Option<SpanishTense>,
        /// Finite forms only, the imperative included.
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        /// Finite forms, plus a participle that agrees.
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<BinaryNumber>,
        /// A participle that actually agrees — passive, `estar` + participio,
        /// adjectival use. Never the invariable participle of a `haber`
        /// compound.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<BinaryGender>,
        /// Second-person singular finite forms only: `true` for the voseo
        /// form (`hablás`, `tenés`, `vení`), `false` for the tuteo form
        /// (`hablas`, `tienes`, `ven`). Absent on every other form.
        #[serde(skip_serializing_if = "Option::is_none")]
        voseo: Option<bool>,
    },
    /// Other, for unanalyzable tokens.
    Other {
        lemma: String,
    },
}

impl SpanishMorphology {
    /// `tense` is optional on the verb (absent for the imperative and the
    /// formas no personales), so the `MorphologyInfo` derive skips it. Spanish
    /// tense is too central to the lexicon to lose, so the handle is written by
    /// hand.
    fn __pivot_tense(&self) -> Option<String> {
        match self {
            Self::Verb { tense, .. } => tense
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    /// Likewise for pronoun case: optional, but it is the axis leísmo, laísmo
    /// and the OD/OI contrast live on.
    fn __pivot_case(&self) -> Option<String> {
        match self {
            Self::Pronoun { case, .. } => case
                .as_ref()
                .map(|value| panini_core::aggregable::ClosedValues::variant_str(value).to_string()),
            _ => None,
        }
    }

    /// Typed pivot handle for verb tense. Defined manually because `tense` is
    /// optional (see [`SpanishMorphology::__pivot_tense`]).
    pub const PIVOT_TENSE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "tense",
            "Tense",
            <SpanishTense as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_tense,
        );

    /// Typed pivot handle for pronoun case. Defined manually because `case` is
    /// optional (see [`SpanishMorphology::__pivot_case`]).
    pub const PIVOT_CASE: panini_core::pivot::PivotField<Self> =
        panini_core::pivot::PivotField::closed(
            "case",
            "Case",
            <SpanishPronounCase as panini_core::aggregable::ClosedValues>::all_variants,
            Self::__pivot_case,
        );
}

pub struct Spanish;

impl LinguisticDefinition for Spanish {
    type Morphology = SpanishMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Spa;
    const MORPHOLOGY_PIVOTS: &'static [panini_core::pivot::PivotField<Self::Morphology>] = &[
        SpanishMorphology::PIVOT_TENSE,
        SpanishMorphology::PIVOT_MOOD,
        SpanishMorphology::PIVOT_GENDER,
        SpanishMorphology::PIVOT_NUMBER,
        SpanishMorphology::PIVOT_CASE,
        SpanishMorphology::PIVOT_CLITIC,
    ];

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn typological_features(&self) -> &[TypologicalFeature] {
        // Conjugation only. Spanish nouns, adjectives, determiners and numerals
        // carry no case whatsoever: they agree in gender and number, and gender
        // is a lexical class rather than an inflection — `mesa` has no masculine
        // counterpart. The one category with a case-like paradigm is the
        // pronoun, and the declension cloze hands the learner "the root of the
        // object to be declined" as its hint, which `lo` / `le` / `se` cannot
        // supply: they are monosyllables with no stem to give away. Declaring
        // Declension would buy exercises the grammar cannot fill.
        &[TypologicalFeature::Conjugation(&[Upos::Verb])]
    }

    fn extraction_directives(&self) -> &'static str {
        "1. Scope and lemmatization: analyze whatever variety of Spanish the input is written in — peninsular, Latin American or Rioplatense — and never normalize one into another. Lemmatize every verb form, participles and gerunds included, to the infinitive ('hablado', 'diciendo' -> 'decir'); nouns to their own singular, keeping their gender ('hermanas' -> 'hermana', 'ingeniera' -> 'ingeniera', 'abuela' -> 'abuela', never 'hermano'); adjectives, determiners and numerals to the masculine singular ('las buenas' -> 'bueno'); and personal pronouns to their nominative paradigm base (me/mí/conmigo -> 'yo'; te/ti/contigo -> 'tú'; lo/la/le/los/las/les/ella -> 'él'; nos -> 'nosotros'; os -> 'vosotros'; se/sí/consigo -> 'se'). Keep 'vos', 'usted' and 'ustedes' as their own lemmas: they are distinct pronouns, not spellings of 'tú'.\n\
         2. Gender and Number are agreement features on Nouns, Adjectives and Determiners, so report them even when the form does not mark them: 'feliz' in 'la mujer feliz' is feminine singular, 'su' in 'su casa' is feminine singular. An Adjective's Degree is positive unless its own form carries more: 'mejor', 'peor', 'mayor', 'menor' are comparative, and the -ísimo / -érrimo forms are superlative with the plain adjective as lemma ('riquísimo' -> 'rico', 'buenísimo' -> 'bueno'); 'más alto' is Adverb 'más' plus a positive 'alto'. Gender is lexical and cannot be read off the ending — 'el problema', 'el tema', 'el idioma', 'el día', 'el mapa', 'el planeta' are masculine; 'la mano', 'la foto', 'la moto', 'la radio' are feminine; nouns in -ista and -ante take their gender from the context ('el artista' / 'la artista').\n\
         3. The 'el agua' rule: a feminine noun beginning with a stressed a- or ha- takes 'el' and 'un' for phonological reasons alone. 'el agua fría', 'el hacha afilada', 'un águila' are FEMININE — tag the noun, the determiner and every agreeing adjective feminine.\n\
         4. The neuter is a residue with no noun class behind it: omit Gender entirely for the article 'lo' ('lo bueno'), for the neuter demonstratives 'esto', 'eso' and 'aquello', for 'ello', and for the clitic 'lo' when it stands for a clause or a predicate ('lo sé', 'sí que lo es'). Never force one of these into masculine or feminine. Relative and interrogative pronouns ('que', 'quien', 'cual', 'qué', 'cuál') never carry case, not even after a preposition ('con quien hablé', 'la vecina de la que hablaban'). In the relative complexes 'el que', 'la que', 'lo que', 'el cual', the article is a Determiner of type article and 'que' / 'cual' the relative Pronoun — 'lo que dices' is Determiner 'lo' + Pronoun 'que', never a relative 'lo'.\n\
         5. Verbs: Mood is always required and has six values. indicative, subjunctive and imperative are the finite moods; infinitive, gerund and participle are the formas no personales. Give Tense to finite indicative and subjunctive forms only — omit it for the imperative and for all three non-finite forms. Give Person and Number to finite forms only, the imperative included. Give a participle Gender and Number only where it actually agrees: passive ('fue escrita'), 'estar' + participio ('están cerradas') and adjectival use ('las puertas cerradas'). The participle of a 'haber' compound is invariable — omit both ('hemos escrito la carta'). A verb's participle stays a Verb of mood participle wherever it stands: 'estoy cansado' is 'cansar', participle, masculine singular, and 'el año pasado' / 'la semana pasada' is 'pasar', participle, exactly like 'está abierta' is 'abrir' — it is never an Adjective with a verb for its lemma.\n\
         6. Tense values cover the simple synthetic paradigms only: presente -> present; pretérito perfecto simple / indefinido ('hablé', 'fue') -> preterite; pretérito imperfecto ('hablaba') -> imperfect; futuro simple ('hablaré') -> future; condicional simple ('hablaría') -> conditional. The condicional is a TENSE OF THE INDICATIVE, not a mood: tag 'hablaría' as tense conditional, mood indicative. There is no conditional mood value and no pluperfect tense value. Mood imperative covers every command whatever form it borrows: tú 'ven', vos 'vení', vosotros 'hablad', and the usted, ustedes, nosotros and negative commands built on subjunctive forms — 'disculpe' and 'hágalo' are imperative third singular, 'hablemos' imperative first plural, 'no salgan' imperative third plural, 'no me lo des' imperative second singular. Subjunctive is the mood of the subordinate and optative clauses ('para que nos ayude', 'ojalá venga').\n\
         7. The imperfect subjunctive has two paradigms and one identity: 'hablara' and 'hablase', 'fuera' and 'fuese' are all mood subjunctive, tense imperfect. Never read an -ra form as an indicative pluperfect. The future subjunctive ('hablare', 'fuere'), confined to legal formulas and proverbs, is mood subjunctive, tense future.\n\
         8. Compound tenses and verbal periphrases are always more than one verb token, each analyzed on its own: 'he hablado' -> 'haber' (present, indicative, first, singular) + 'hablado' (participle), and likewise for 'había hablado', 'habré hablado', 'habría hablado', 'hube hablado' and every compound subjunctive. 'voy a comer' -> 'ir' + 'a' + 'comer'; 'estoy comiendo' -> 'estar' + 'comiendo'; 'acabo de llegar' -> 'acabar' + 'de' + 'llegar'. Only a simple synthetic form is a single verb token.\n\
         9. Address across the Spanish-speaking world — tag a verb by the agreement it actually carries, never by who is being addressed. 'hablas' (tú) and the voseo forms 'hablás', 'tenés', 'venís' are second person singular, as are the voseo imperatives 'hablá', 'tené', 'vení'; every second person singular finite form also carries the voseo flag — true for 'hablás', 'tenés', 'sos', 'vení', false for 'hablas', 'tienes', 'eres', 'ven' — and no other form carries it. 'habláis' and the imperative 'hablad' (vosotros) are second person plural. But 'usted habla' and 'ustedes hablan' carry THIRD person agreement on the verb — tag the verb third singular and third plural — while the pronouns 'usted' and 'ustedes' are second-person address forms and take person second with their own number. 'os' is second person plural; the object clitics used with 'usted' and 'ustedes' ('lo', 'la', 'los', 'las', 'le', 'les') are third person.\n\
         10. Pronouns: give the type, the clitic flag, and the case wherever the form has one. Set clitic true for every unstressed form — me, te, se, lo, la, le, nos, os, los, las, les — proclitic, enclitic or doubled alike, and false for the stressed forms (yo, tú, vos, usted, ustedes, él, ella, nosotros, vosotros, mí, ti, sí, conmigo, contigo, consigo) and for every demonstrative, relative, interrogative, possessive and indefinite pronoun. Case is subject for 'yo', 'tú', 'vos', 'él'; direct_object for 'lo', 'la', 'los', 'las'; indirect_object for 'le', 'les'; reflexive for 'se' and 'sí' in reflexive or reciprocal use; prepositional for 'mí', 'ti', 'sí' and any stressed pronoun governed by a preposition, including a doubled object ('a él lo vi'). 'me', 'te', 'nos' and 'os' take whichever of direct_object, indirect_object or reflexive their function in the sentence gives them. Every personal pronoun carries Person and every one but 'se' carries Number ('me voy' -> 'me' first singular); 'se' is third person in every use and carries neither Number nor Gender. Omit Person for non-personal pronouns, and omit Gender for forms that do not distinguish it ('me', 'te', 'se', 'nos', 'os', 'le', 'les', 'yo', 'tú').\n\
         11. Do not silently repair leísmo, laísmo or loísmo: tag the case by the syntactic function the pronoun has in the sentence as written, so 'le vi' with a direct object is a direct_object 'le' and 'la dije la verdad' is an indirect_object 'la'. In 'se lo di', 'se' is the dative allomorph of 'le' before another third-person clitic — tag it indirect_object, never reflexive. Impersonal, pasiva refleja and accidental 'se' ('se vive bien', 'se venden pisos', 'se dice que', 'se me olvidaron las llaves') take case reflexive like reflexive and reciprocal 'se'; the 'me' of the accidental construction is indirect_object.\n\
         12. Tokenization: split an enclitic cluster into the verb and each clitic, each keeping its own lemma, and drop the accent that enclisis adds — 'dámelo' -> 'dar' (imperative, second, singular) + 'me' + 'lo'; 'diciéndoselo' -> 'decir' (gerund) + 'se' + 'lo'; 'vámonos' -> 'ir' (imperative, first, plural) + 'nos'; 'sentaos' -> 'sentar' (imperative, second, plural) + 'os' (reflexive), exactly two tokens, since the -d vosotros drops before 'os' is not a token; 'hacerlo' -> 'hacer' + 'lo'. Spanish has exactly two contractions and both are split: 'del' -> 'de' + 'el', 'al' -> 'a' + 'el'. Both entries keep the contraction itself as their word — 'del barrio' is word 'del' Adposition 'de', word 'del' Determiner 'el', then word 'barrio' Noun — and every piece of an enclitic cluster keeps the whole cluster as its word; the article is never filed under the noun that follows it.\n\
         13. Value guardrails for the confusions that actually occur: the personal 'a' ('veo a María') is an Adposition; 'no' is an Adverb; there is no Particle category in this model at all, so route anything you would have tagged a particle to Adverb, Pronoun, Adposition or Subordinating Conjunction by its function. 'que' is a Subordinating Conjunction when it introduces a completive clause ('dice que viene') and a relative Pronoun when it has an antecedent ('el libro que leí'). 'se' is always a Pronoun, whether reflexive, reciprocal, impersonal, pasiva refleja or dative allomorph. 'hasta', 'incluso', 'excepto' and 'salvo' before a subject form ('hasta yo lo vi', 'incluso yo') are Adverbs and the pronoun is case subject. Separate the accented homographs from the unaccented ones by their written form and syntax: él/el, tú/tu, mí/mi, sí/si, sé/se, dé/de, té/te, más/mas, qué/que, cómo/como, dónde/donde. Never put a gender value in the number field or a number value in the gender field.\n\
         14. 'cómo', 'dónde', 'adónde' and 'cuándo' are interrogative Adverbs ('¿cómo estás?' -> Adverb 'cómo'); 'donde', 'cuando' and 'como' with an antecedent are relative Adverbs ('la universidad donde estudia', 'en diciembre, cuando termina', 'así es como lo hicimos'); opening a clause with no antecedent they are Subordinating Conjunctions ('cuando vamos al parque', 'como si lo supiera'). 'qué', 'quién', 'cuál' and 'cuánto' are interrogative Pronouns standing alone and interrogative Determiners before a noun, exclamations included ('¿qué hora es?', '¿cuántos años tienes?', '¡qué desastre!' -> Determiner 'qué').\n\
         15. The quantifiers that stand before a noun and agree with it — mucho, poco, tanto, demasiado, bastante, todo, otro, varios, cada, alguno, ninguno, cualquier — are Determiners of type indefinite ('mucho viento', 'mucha paciencia', 'varios amigos', 'toda América'); standing alone they are indefinite Pronouns ('todos están nerviosos'), and 'mucho' next to a verb or an adjective is an Adverb ('trabajas mucho'). 'un', 'una', 'unos', 'unas' before a noun are the indefinite article — Determiner, article, lemma 'un' ('un amigo') — and a Numeral with lemma 'uno' only when they count against other numbers ('veintiún años', 'treinta y una personas'). 'varios' keeps the lemma 'varios', and a possessive determiner keeps its own unaccented lemma ('tu casa' -> 'tu', never 'tú'). The speaker labels of a dialogue ('A:', 'B:') are Symbols."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spanish_identity_script_and_typology_are_exact() {
        let language = Spanish;

        assert_eq!(Spanish::ISO_LANG, IsoLang::Spa);
        assert_eq!(language.supported_scripts(), &[Script::LATN]);
        assert_eq!(language.default_script(), Script::LATN);
        assert_eq!(
            language.typological_features(),
            &[TypologicalFeature::Conjugation(&[Upos::Verb])]
        );
    }

    /// `vosotros` is not a separate person: the shared `Person` × `BinaryNumber`
    /// space already spells it, and peninsular, American and Rioplatense address
    /// all have to land in it for a single pan-Hispanic course to work.
    #[test]
    fn peninsular_american_and_rioplatense_address_are_all_representable() {
        let hablais = SpanishMorphology::Verb {
            lemma: "hablar".to_string(),
            mood: SpanishMood::Indicative,
            tense: Some(SpanishTense::Present),
            person: Some(Person::Second),
            number: Some(BinaryNumber::Plural),
            gender: None,
            voseo: None,
        };
        let hablas_voseo = SpanishMorphology::Verb {
            lemma: "hablar".to_string(),
            mood: SpanishMood::Indicative,
            tense: Some(SpanishTense::Present),
            person: Some(Person::Second),
            number: Some(BinaryNumber::Singular),
            gender: None,
            voseo: Some(true),
        };
        let hablas_tuteo = SpanishMorphology::Verb {
            lemma: "hablar".to_string(),
            mood: SpanishMood::Indicative,
            tense: Some(SpanishTense::Present),
            person: Some(Person::Second),
            number: Some(BinaryNumber::Singular),
            gender: None,
            voseo: Some(false),
        };
        // `ustedes hablan`: the verb carries third-person agreement, the pronoun
        // is a second-person address form.
        let hablan_ustedes = SpanishMorphology::Verb {
            lemma: "hablar".to_string(),
            mood: SpanishMood::Indicative,
            tense: Some(SpanishTense::Present),
            person: Some(Person::Third),
            number: Some(BinaryNumber::Plural),
            gender: None,
            voseo: None,
        };

        assert_ne!(hablais, hablas_voseo);
        assert_ne!(hablais, hablan_ustedes);
        // `vos hablás` and `tú hablas` share every agreement feature; the
        // voseo flag is what keeps them two records.
        assert_ne!(hablas_voseo, hablas_tuteo);
        for form in [&hablais, &hablas_voseo, &hablas_tuteo, &hablan_ustedes] {
            assert_eq!(
                SpanishMorphology::PIVOT_TENSE.value(form),
                Some("present".to_string())
            );
        }
    }

    /// Both hand-written handles have to survive the `Option` the derive skips.
    #[test]
    fn optional_tense_and_case_remain_closed_pivots() {
        let hablando = SpanishMorphology::Verb {
            lemma: "hablar".to_string(),
            mood: SpanishMood::Gerund,
            tense: None,
            person: None,
            number: None,
            gender: None,
            voseo: None,
        };
        let le = SpanishMorphology::Pronoun {
            lemma: "él".to_string(),
            pronoun_type: SpanishPronounType::Personal,
            clitic: true,
            person: Some(Person::Third),
            gender: None,
            number: Some(BinaryNumber::Singular),
            case: Some(SpanishPronounCase::IndirectObject),
        };

        assert_eq!(SpanishMorphology::PIVOT_TENSE.value(&hablando), None);
        assert_eq!(
            SpanishMorphology::PIVOT_MOOD.value(&hablando),
            Some("gerund".to_string())
        );
        assert_eq!(
            SpanishMorphology::PIVOT_CASE.value(&le),
            Some("indirect_object".to_string())
        );
        assert_eq!(
            SpanishMorphology::PIVOT_CLITIC.value(&le),
            Some("true".to_string())
        );
    }
}
