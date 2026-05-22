pub mod arabic;
pub mod polish;
pub mod turkish;

pub use arabic::*;
pub use polish::*;
pub use turkish::*;

pub mod french;
pub use french::*;

pub mod italian;
pub use italian::*;

pub mod danish;
pub use danish::*;

#[cfg(feature = "registry")]
pub mod registry;

pub mod mandarin_chinese;
pub use mandarin_chinese::*;

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
    fn grammatical_function_handle_extracts_matching_category_only() {
        let polarity = turkish::TurkishGrammaticalFunction::Polarity {
            value: turkish::TurkishPolarity::Negative,
        };
        let tense = turkish::TurkishGrammaticalFunction::Tense {
            value: turkish::TurkishTense::Past,
        };

        assert_eq!(
            turkish::TurkishGrammaticalFunction::PIVOT_POLARITY.value(&polarity),
            Some("negative".to_string())
        );
        assert_eq!(
            turkish::TurkishGrammaticalFunction::PIVOT_POLARITY.value(&tense),
            None
        );
    }
}
