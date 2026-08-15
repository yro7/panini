/// The languages this crate ships — the single place the list lives.
///
/// X-macro: pass the name of a callback macro and it is re-invoked with one
/// `(module, Struct)` pair per language. Module declarations, re-exports, the
/// extraction registry and the digest test all expand from this list, so adding
/// a language is one line here and nothing else in this crate.
macro_rules! with_languages {
    ($callback:ident) => {
        $callback! {
            (arabic, Arabic),
            (danish, Danish),
            (french, French),
            (italian, Italian),
            (mandarin_chinese, MandarinChinese),
            (polish, Polish),
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
    use panini_core::traits::LinguisticDefinition;

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
        >(turkish::Turkish.iso_code());

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

#[cfg(test)]
mod pivot_tests {
    use panini_core::pivot::PivotValueKind;
    use panini_core::traits::{BinaryGender, TernaryNumber};

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
    fn morphology_open_field_handle_extracts_root() {
        let morphology = arabic::ArabicMorphology::Noun {
            lemma: "كتاب".to_string(),
            root: "ك-ت-ب".to_string(),
            pattern: None,
            gender: BinaryGender::Masculine,
            number: TernaryNumber::Singular,
            case: arabic::ArabicCase::Nominative,
            state: arabic::ArabicState::Absolute,
            definiteness: arabic::ArabicDefiniteness::Indefinite,
        };

        assert_eq!(
            arabic::ArabicMorphology::PIVOT_ROOT.value_kind,
            PivotValueKind::Open
        );
        assert_eq!(
            arabic::ArabicMorphology::PIVOT_ROOT.value(&morphology),
            Some("ك-ت-ب".to_string())
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
