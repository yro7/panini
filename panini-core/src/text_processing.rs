/// Normalizes `"pos"` field values in a JSON string before deserialization:
/// 1. Lowercases all `"pos": "..."` values
/// 2. Maps common UD abbreviations AND long-form aliases to canonical enum names
///
/// # Panics
/// Panics if the internal regex fails to compile (should never happen).
#[must_use]
pub fn normalize_pos_tags(json: &str) -> String {
    // UD abbreviation / long-form alias → canonical lowercase enum variant
    let ud_map: &[(&str, &str)] = &[
        // UD tag abbreviations
        ("adj", "adjective"),
        ("adp", "adposition"),
        ("adv", "adverb"),
        // UD's AUX has no Panglotive counterpart, by choice rather than by claim:
        // an auxiliary is filed under Verb so one lexeme keeps one mastery record
        // (auxiliary `être` and lexical `être` are the same word to a learner).
        // Where an "auxiliary" is a suffix rather than a word — Turkish `-dir`,
        // Japanese `-ta` — it is a MorphemeFunction and never reaches this table.
        ("aux", "verb"),
        ("cconj", "coordinating_conjunction"),
        ("det", "determiner"),
        ("intj", "interjection"),
        ("n", "noun"),
        ("num", "numeral"),
        ("part", "particle"),
        ("prep", "adposition"),
        ("pron", "pronoun"),
        ("propn", "proper_noun"),
        ("sconj", "subordinating_conjunction"),
        ("v", "verb"),
        ("conj", "coordinating_conjunction"),
        ("interj", "interjection"),
        ("sym", "symbol"),
        ("x", "other"),
        // Long-form aliases that some LLMs emit
        ("preposition", "adposition"),
        ("conjunction", "coordinating_conjunction"),
        ("coordinating conjunction", "coordinating_conjunction"),
        ("subordinating conjunction", "subordinating_conjunction"),
        ("proper noun", "proper_noun"),
        // Concatenated forms: what a PascalCase variant name lowercases to, and
        // what Turkish's schema advertised while it used `rename_all = "lowercase"`.
        ("propernoun", "proper_noun"),
        ("coordinatingconjunction", "coordinating_conjunction"),
        ("subordinatingconjunction", "subordinating_conjunction"),
    ];

    // Regex: match `"pos"` (with optional whitespace) `:` string value
    let re = regex::Regex::new(r#""pos"\s*:\s*"([^"]+)""#).unwrap();

    re.replace_all(json, |caps: &regex::Captures| {
        let raw_val = caps[1].to_lowercase();
        let normalized = ud_map.iter().find(|(abbr, _)| *abbr == raw_val).map_or(
            // Not in the map — use the lowercased value as-is
            raw_val.as_str(),
            |(_, canonical)| *canonical,
        );
        format!(r#""pos": "{normalized}""#)
    })
    .to_string()
}

use std::borrow::Cow;
use unicode_normalization::{UnicodeNormalization, is_nfc};

/// Returns the string in Unicode Normalization Form C (NFC).
/// Returns a borrowed slice if the string is already NFC to avoid allocations.
#[must_use]
pub fn normalize_nfc_str(text: &str) -> Cow<'_, str> {
    if is_nfc(text) {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(text.nfc().collect())
    }
}

/// Recursively canonicalizes every string in a JSON value (both object keys and string values) to Unicode NFC.
pub fn normalize_json_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(text) => {
            if !is_nfc(text) {
                *text = text.nfc().collect();
            }
        }
        serde_json::Value::Array(values) => {
            for v in values {
                normalize_json_value(v);
            }
        }
        serde_json::Value::Object(map) => {
            for v in map.values_mut() {
                normalize_json_value(v);
            }
            if map.keys().any(|k| !is_nfc(k)) {
                let old_map = std::mem::take(map);
                for (k, v) in old_map {
                    let normalized_key = if is_nfc(&k) { k } else { k.nfc().collect() };
                    map.insert(normalized_key, v);
                }
            }
        }
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_pos_lowercases() {
        let input = r#"{"pos": "Noun", "lemma": "dom"}"#;
        assert_eq!(
            normalize_pos_tags(input),
            r#"{"pos": "noun", "lemma": "dom"}"#
        );
    }

    #[test]
    fn normalize_pos_maps_ud_abbreviations() {
        let input = r#"{"pos": "ADJ", "lemma": "duży"}"#;
        assert_eq!(
            normalize_pos_tags(input),
            r#"{"pos": "adjective", "lemma": "duży"}"#
        );

        let input2 = r#"{"pos": "prep", "lemma": "na"}"#;
        assert_eq!(
            normalize_pos_tags(input2),
            r#"{"pos": "adposition", "lemma": "na"}"#
        );

        let input3 = r#"{"pos": "ADP", "lemma": "na"}"#;
        assert_eq!(
            normalize_pos_tags(input3),
            r#"{"pos": "adposition", "lemma": "na"}"#
        );
    }

    /// `Auxiliary` was removed from `Upos`; the mapping outlived it and pointed
    /// at a name nothing could deserialize, so every UD `AUX` tag failed its card.
    #[test]
    fn normalize_pos_maps_the_ud_auxiliary_tag_onto_verb() {
        let input = r#"{"pos": "AUX", "lemma": "være"}"#;
        assert_eq!(
            normalize_pos_tags(input),
            r#"{"pos": "verb", "lemma": "være"}"#
        );
    }

    #[test]
    fn normalize_pos_maps_long_form_aliases() {
        let input = r#"{"pos": "preposition", "lemma": "w"}"#;
        assert_eq!(
            normalize_pos_tags(input),
            r#"{"pos": "adposition", "lemma": "w"}"#
        );

        let input2 = r#"{"pos": "Preposition", "lemma": "na"}"#;
        assert_eq!(
            normalize_pos_tags(input2),
            r#"{"pos": "adposition", "lemma": "na"}"#
        );
    }

    #[test]
    fn normalize_pos_handles_multiple_occurrences() {
        let input = r#"[{"pos": "PREP"}, {"pos": "Verb"}]"#;
        assert_eq!(
            normalize_pos_tags(input),
            r#"[{"pos": "adposition"}, {"pos": "verb"}]"#
        );
    }

    #[test]
    fn normalize_pos_leaves_valid_values_unchanged() {
        let input = r#"{"pos": "noun", "lemma": "dom"}"#;
        assert_eq!(normalize_pos_tags(input), input);
    }

    #[test]
    fn test_normalize_nfc_str() {
        // Already NFC Polish: returns borrowed Cow
        let nfc_str = "Dzień dobry, książka";
        let res = normalize_nfc_str(nfc_str);
        assert!(matches!(res, std::borrow::Cow::Borrowed(_)));
        assert_eq!(res, nfc_str);

        // Decomposed NFD: returns owned Cow normalized to NFC
        let nfd_str = "a\u{0328}"; // 'a' with combining ogonek -> 'ą'
        let res_decomposed = normalize_nfc_str(nfd_str);
        assert!(matches!(res_decomposed, std::borrow::Cow::Owned(_)));
        assert_eq!(res_decomposed, "ą");
    }

    #[test]
    fn test_normalize_json_value() {
        let mut val = serde_json::json!({
            "normal_key": "a\u{0328}", // decomposed 'ą'
            "c\u{0301}": {              // decomposed 'ć' in key
                "nested_arr": ["e\u{0301}", 42, true, null], // decomposed 'é'
                "nested_obj": {
                    "hint": "n\u{0301}" // decomposed 'ń'
                }
            }
        });

        normalize_json_value(&mut val);

        assert_eq!(val["normal_key"], "ą");
        assert_eq!(val["ć"]["nested_arr"][0], "é");
        assert_eq!(val["ć"]["nested_arr"][1], 42);
        assert_eq!(val["ć"]["nested_obj"]["hint"], "ń");
    }
}
