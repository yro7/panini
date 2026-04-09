use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::domain::MultiwordExpression;
use crate::traits::LinguisticDefinition;

/// Extracts multi-word expressions (idioms, collocations, phrasal expressions).
#[derive(Debug, Clone, Default)]
pub struct MultiwordExpressions;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for MultiwordExpressions {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for MultiwordExpressions {
    fn name(&self) -> &'static str {
        "Multiword Expressions"
    }

    fn schema_key(&self) -> &'static str {
        "multiword_expressions"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        let r#gen = schemars::SchemaGenerator::default();
        let schema = r#gen.into_root_schema_for::<Vec<MultiwordExpression>>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, _lang: &L, _ctx: &ComponentContext) -> String {
        "Extract multi-word expressions (idioms, collocations, phrasal expressions) found in the sentence. \
         Only include expressions whose meaning cannot be guessed purely from word-by-word translation. \
         Use the base/generic form of the expression, not the inflected form as it appears."
            .to_string()
    }
}
