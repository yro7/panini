//! Runtime lookup of a language's alignment segmentation inventory.
//!
//! Translation alignment involves two languages, but an analysis component is
//! generic over the source one only; the translation side is a bare
//! [`IsoLang`]. Callers resolve that side here and hand the text to the
//! request. Driven by `with_languages!` in `lib.rs` — this module never names
//! a language.

use panini_core::traits::{IsoLang, LinguisticDefinition};

macro_rules! generate_alignment_directives {
    ($(($module:ident, $lang:ident)),* $(,)?) => {
        /// The [`LinguisticDefinition::alignment_directives`] of the language
        /// with this ISO code. `None` for a language this crate does not
        /// define, or one that declares no inventory.
        #[must_use]
        pub fn alignment_directives_for(lang: IsoLang) -> Option<&'static str> {
            match lang {
                $(
                    s if s == <$crate::$lang as LinguisticDefinition>::ISO_LANG => {
                        $crate::$lang.alignment_directives()
                    }
                )*
                _ => None,
            }
        }
    };
}

with_languages!(generate_alignment_directives);

#[cfg(test)]
mod tests {
    use super::*;

    struct Entry {
        struct_name: &'static str,
        iso: IsoLang,
        name: String,
        directives: Option<&'static str>,
    }

    macro_rules! lang_entries {
        ($(($module:ident, $struct:ident)),* $(,)?) => {
            vec![$(Entry {
                struct_name: stringify!($struct),
                iso: $crate::$module::$struct.iso_code(),
                name: $crate::$module::$struct.name().to_string(),
                directives: $crate::$module::$struct.alignment_directives(),
            }),*]
        };
    }

    #[test]
    fn an_undefined_language_resolves_to_none() {
        assert_eq!(alignment_directives_for(IsoLang::Urd), None);
    }

    #[test]
    fn every_defined_language_resolves_to_its_own_directives() {
        let languages = with_languages!(lang_entries);
        assert!(!languages.is_empty());
        for entry in &languages {
            assert_eq!(
                alignment_directives_for(entry.iso),
                entry.directives,
                "{}: the lookup does not return this language's own directives",
                entry.struct_name
            );
        }
    }

    /// The inventory serves whether the language is the source or the
    /// translation, so it must not be written for one particular pair.
    #[test]
    fn no_directive_names_another_defined_language() {
        let languages = with_languages!(lang_entries);
        for entry in &languages {
            let Some(directives) = entry.directives else {
                continue;
            };
            let struct_name = entry.struct_name;
            for other in &languages {
                let other_name = &other.name;
                if other.struct_name == struct_name {
                    continue;
                }
                assert!(
                    !directives.contains(other_name.as_str()),
                    "{struct_name}: alignment directives mention {other_name} — they must \
                     describe this language alone, never a language pair"
                );
            }
        }
    }
}
