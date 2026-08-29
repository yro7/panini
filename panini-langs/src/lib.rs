/// The languages this crate ships — the single place the list lives.
///
/// X-macro: pass the name of a callback macro and it is re-invoked with one
/// `(module, Struct)` pair per language. Module declarations, re-exports, the
/// extraction registry and the digest test all expand from this list, so adding
/// a language is one line here and nothing else in this crate.
macro_rules! with_languages {
    ($callback:ident) => {
        $callback! {
            (danish, Danish),
            (eastern_armenian, EasternArmenian),
            (english, English),
            (french, French),
            (german, German),
            (hindi, Hindi),
            (indonesian, Indonesian),
            (italian, Italian),
            (korean, Korean),
            (mandarin_chinese, MandarinChinese),
            (polish, Polish),
            (portuguese, Portuguese),
            (russian, Russian),
            (spanish, Spanish),
            (turkish, Turkish),
        }
    };
}

macro_rules! declare_languages {
    ($(($module:ident, $struct:ident)),* $(,)?) => {
        $(
            pub mod $module;
            pub use $module::*;
        )*
    };
}

with_languages!(declare_languages);

#[cfg(feature = "registry")]
pub mod registry;

#[cfg(test)]
mod lang_digest_tests {
    use panini_core::lang_digest::LanguageDigest;
    use panini_core::traits::{IsoLang, LinguisticDefinition};

    use super::*;

    macro_rules! digest_vec {
        ($(($module:ident, $struct:ident)),* $(,)?) => {
            vec![$((stringify!($struct), crate::$module::$struct.lang_digest())),*]
        };
    }

    /// Every language's digest, paired with its name. Expanded from
    /// `with_languages!`, so a new language is covered the moment it is listed.
    fn all_digests() -> Vec<(&'static str, LanguageDigest)> {
        with_languages!(digest_vec)
    }

    /// The distinctness test below is vacuous on an empty vec, which is exactly
    /// what a mis-expanded `with_languages!` would produce. No count to maintain
    /// here: dropping a language from the list also drops its `pub mod`, so an
    /// incomplete list fails to compile rather than silently shrinking coverage.
    #[test]
    fn the_language_list_actually_reaches_the_digest_tests() {
        assert!(
            all_digests().len() > 1,
            "with_languages! expanded to nothing — every digest test below is vacuous"
        );
    }

    /// The digest is only worth storing if it actually reflects each language's
    /// value space. If the derived catalogs came back empty, every language
    /// would hash identically and the mechanism would be silently inert — which
    /// is precisely the failure a fingerprint is supposed to prevent.
    #[test]
    fn every_language_has_a_distinct_value_space() {
        let digests = all_digests();

        for (name, digest) in &digests {
            let collisions: Vec<&str> = digests
                .iter()
                .filter(|(other_name, other)| other_name != name && other.hash() == digest.hash())
                .map(|(other_name, _)| *other_name)
                .collect();

            assert!(
                collisions.is_empty(),
                "{name} hashes to the same value space as {collisions:?} — \
                 the catalogs are probably empty, making the digest inert"
            );
        }
    }

    #[test]
    fn digests_are_deterministic_within_a_build() {
        for (name, digest) in all_digests() {
            let recomputed = all_digests()
                .into_iter()
                .find(|(other, _)| *other == name)
                .expect("language is present twice")
                .1;

            assert_eq!(digest, recomputed, "{name} digest is not reproducible");
        }
    }

    /// Turkish is the agglutinative case the whole mechanism exists for: its
    /// morpheme functions, not just its morphology, must reach the hash.
    #[test]
    fn turkish_digest_covers_morpheme_functions() {
        use panini_core::traits::MorphemeFunctionCatalog;

        let with_functions = turkish::Turkish.lang_digest();
        let morphology_only = LanguageDigest::compute::<
            <turkish::Turkish as LinguisticDefinition>::Morphology,
            (),
        >(IsoLang::Tur);

        assert!(
            !<turkish::Turkish as LinguisticDefinition>::MorphemeFunction::function_descriptors()
                .is_empty(),
            "Turkish must expose morpheme functions for this test to mean anything"
        );
        assert_ne!(
            with_functions, morphology_only,
            "dropping the morpheme functions must change the digest"
        );
    }
}

/// The `pos` tag is written twice by two different mechanisms that must agree:
/// serde names the variants for the schema the LLM answers against, and
/// `MorphologyAnalysis::pre_process` rewrites the model's tag before
/// deserialization. Nothing connects them, so a language whose serde naming
/// disagrees with the normalizer ships parts of speech that can never be
/// extracted — Turkish did, for `rename_all = "lowercase"`, until 2026-08-15.
#[cfg(test)]
mod pos_tag_wire_tests {
    use panini_core::text_processing::normalize_pos_tags;
    use panini_core::traits::LinguisticDefinition;

    /// Every `pos` value the schema advertises, harvested from the generated
    /// JSON Schema so it reflects serde's naming and nothing else.
    fn pos_tags_of<L: LinguisticDefinition>() -> Vec<String> {
        let schema = schemars::SchemaGenerator::default().into_root_schema_for::<L::Morphology>();
        let mut tags = Vec::new();
        collect_pos_tags(&serde_json::to_value(&schema).unwrap(), &mut tags);
        tags
    }

    fn collect_pos_tags(node: &serde_json::Value, out: &mut Vec<String>) {
        match node {
            serde_json::Value::Object(map) => {
                if let Some(pos) = map.get("pos") {
                    if let Some(value) = pos.get("const").and_then(serde_json::Value::as_str) {
                        out.push(value.to_string());
                    }
                    if let Some(values) = pos.get("enum").and_then(serde_json::Value::as_array) {
                        out.extend(
                            values
                                .iter()
                                .filter_map(serde_json::Value::as_str)
                                .map(str::to_string),
                        );
                    }
                }
                for value in map.values() {
                    collect_pos_tags(value, out);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    collect_pos_tags(item, out);
                }
            }
            _ => {}
        }
    }

    macro_rules! pos_tag_vec {
        ($(($module:ident, $struct:ident)),* $(,)?) => {
            vec![$((stringify!($struct), pos_tags_of::<crate::$module::$struct>())),*]
        };
    }

    #[test]
    fn every_schema_pos_tag_survives_the_prompt_normalizer() {
        for (name, tags) in with_languages!(pos_tag_vec) {
            assert!(
                !tags.is_empty(),
                "{name}: no pos tags found in the schema — the walker is stale, not the language"
            );

            for tag in tags {
                let probe = format!(r#"{{"pos": "{tag}"}}"#);
                assert_eq!(
                    normalize_pos_tags(&probe),
                    probe,
                    "{name}: normalize_pos_tags rewrites the schema's own tag `{tag}`, so that \
                     part of speech can never deserialize. Use `rename_all = \"snake_case\"` on \
                     the morphology enum."
                );
            }
        }
    }
}

#[cfg(test)]
mod pivot_tests {
    use panini_core::pivot::PivotValueKind;

    use super::*;

    #[test]
    fn morphology_closed_field_handle_exposes_values_and_extracts() {
        let morphology = polish::PolishMorphology::Noun {
            lemma: "dom".to_string(),
            gender: polish::PolishGender::MasculineInanimate,
            number: polish::PolishNumber::Singular,
            case: polish::PolishCase::Nominative,
        };

        assert_eq!(
            polish::PolishMorphology::PIVOT_CASE.value_kind,
            PivotValueKind::Closed
        );
        assert!(
            polish::PolishMorphology::PIVOT_CASE
                .values()
                .contains(&"nominative")
        );
        assert_eq!(
            polish::PolishMorphology::PIVOT_CASE.value(&morphology),
            Some("nominative".to_string())
        );
    }

    #[test]
    fn morpheme_function_handle_extracts_matching_category_only() {
        let polarity = turkish::TurkishMorphemeFunction::Polarity {
            value: turkish::TurkishPolarity::Negative,
        };
        let tense = turkish::TurkishMorphemeFunction::Tense {
            value: turkish::TurkishTense::PastDefinite,
        };

        assert_eq!(
            turkish::TurkishMorphemeFunction::PIVOT_POLARITY.value(&polarity),
            Some("negative".to_string())
        );
        assert_eq!(
            turkish::TurkishMorphemeFunction::PIVOT_POLARITY.value(&tense),
            None
        );
    }
}

#[cfg(test)]
mod macrolanguage_tests {
    use panini_core::traits::LinguisticDefinition;

    /// Official ISO 639-3 macrolanguages (SIL International / ISO 639-3 Registration Authority).
    /// Macrolanguages are administrative umbrellas that conflate multiple distinct individual
    /// languages with divergent morphological type systems. Panini strictly forbids them.
    const ISO_639_3_MACROLANGUAGES: &[(&str, &str)] = &[
        ("aka", "Akan"),
        (
            "ara",
            "Arabic (use 'arb' for Standard Arabic, 'arz', 'apc', etc.)",
        ),
        ("aym", "Aymara"),
        ("aze", "Azerbaijani (use 'azj' or 'azb')"),
        ("bal", "Balochi"),
        ("bik", "Bikol"),
        ("bua", "Buriat"),
        ("chm", "Mari"),
        ("cre", "Cree"),
        ("del", "Delaware"),
        ("den", "Slave"),
        ("din", "Dinka"),
        ("doi", "Dogri"),
        ("est", "Estonian (use 'ekk' for Standard Estonian)"),
        (
            "fas",
            "Persian (use 'pes' for Western Persian or 'prs' for Dari)",
        ),
        ("ful", "Fulah"),
        ("gba", "Gbaya"),
        ("gon", "Gondi"),
        ("grb", "Grebo"),
        ("hai", "Haida"),
        ("hbs", "Serbo-Croatian (use 'bos', 'hrv', 'srp', or 'cnr')"),
        ("hmn", "Hmong"),
        ("iku", "Inuktitut (use 'ike' or 'ikt')"),
        ("ipk", "Inupiaq"),
        ("jrb", "Judeo-Arabic"),
        ("kln", "Kalenjin"),
        ("kok", "Konkani"),
        ("kom", "Komi"),
        ("kon", "Kongo"),
        ("kpe", "Kpelle"),
        ("krn", "Kanuri"),
        ("kur", "Kurdish (use 'kmr' for Kurmanji, 'ckb' for Sorani)"),
        ("lah", "Lahnda"),
        ("man", "Mandingo"),
        ("mlg", "Malagasy"),
        ("mon", "Mongolian (use 'khk' for Halh Mongolian)"),
        (
            "msa",
            "Malay (use 'ind' for Indonesian or 'zlm' for Standard Malay)",
        ),
        ("mwr", "Marwari"),
        (
            "nor",
            "Norwegian (use 'nob' for Bokmål or 'nno' for Nynorsk)",
        ),
        ("oji", "Ojibwa"),
        ("orm", "Oromo"),
        ("pus", "Pashto"),
        ("que", "Quechua"),
        ("raj", "Rajasthani"),
        ("rom", "Romany (use 'rmy', 'rmc', etc.)"),
        ("sqi", "Albanian (use 'als' for Tosk or 'aln' for Gheg)"),
        ("srd", "Sardinian"),
        ("swa", "Swahili (use 'swh' for Coastal Swahili)"),
        ("syr", "Syriac"),
        ("tmh", "Tamashek"),
        ("uzb", "Uzbek"),
        ("yid", "Yiddish (use 'ydd' for Eastern Yiddish)"),
        ("zap", "Zapotec"),
        ("zha", "Zhuang"),
        (
            "zho",
            "Chinese (use 'cmn' for Mandarin, 'yue' for Cantonese, etc.)",
        ),
        ("zza", "Zaza"),
    ];

    macro_rules! lang_iso_pair {
        ($(($module:ident, $struct:ident)),* $(,)?) => {
            vec![$((stringify!($struct), crate::$module::$struct.iso_code())),*]
        };
    }

    #[test]
    fn no_registered_language_uses_a_macrolanguage() {
        let languages = with_languages!(lang_iso_pair);

        for (struct_name, iso_lang) in languages {
            if let Some((_, hint)) = ISO_639_3_MACROLANGUAGES
                .iter()
                .find(|(macro_code, _)| *macro_code == iso_lang.to_639_3())
            {
                panic!(
                    "Language `{struct_name}` is registered with ISO 639-3 code `{iso_code}`, \
                     which is a MACROLANGUAGE. Panini strictly enforces individual ISO 639-3 \
                     languages only. Hint: {hint}",
                    iso_code = iso_lang.to_639_3()
                );
            }
        }
    }
}
