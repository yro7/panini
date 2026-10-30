use std::collections::{BTreeMap, BTreeSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::aggregable::FieldKind;
use crate::traits::{LinguisticDefinition, MorphologyCatalog, MorphologyGroupSchema};

use super::LfgValue;

/// A partial projection of one morphological category, not a token analysis.
/// Values use the existing catalog's canonical atomic strings, including
/// "true"/"false" for boolean dimensions, just as aggregation does.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LfgMorphology {
    pub category: String,
    pub features: Vec<LfgFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LfgFeature {
    pub name: String,
    pub value: String,
}

pub(super) fn catalog<L: LinguisticDefinition>() -> Vec<MorphologyGroupSchema> {
    L::Morphology::feature_descriptors()
}

pub(super) fn attribute_name(field: &str) -> String {
    match field {
        "number" => "NUM".into(),
        "person" => "PERS".into(),
        "gender" => "GEND".into(),
        _ => field.to_uppercase().replace('_', "-"),
    }
}

pub(super) fn temporal(name: &str) -> bool {
    matches!(
        name,
        "tense" | "aspect" | "mood" | "tense_aspect" | "evidentiality"
    )
}

impl LfgMorphology {
    pub(super) fn resolve<L: LinguisticDefinition>(
        &self,
    ) -> Result<BTreeMap<String, LfgValue>, String> {
        let groups = catalog::<L>();
        let group = groups
            .iter()
            .find(|g| g.key == self.category)
            .ok_or_else(|| format!("Unknown morphology category {:?}", self.category))?;
        let mut seen = BTreeSet::new();
        let mut result = BTreeMap::new();
        result.insert(
            "MORPHOLOGY-GROUP".into(),
            LfgValue::Atom(self.category.clone()),
        );
        for feature in &self.features {
            if !seen.insert(&feature.name) {
                return Err(format!("Duplicate morphology feature {}", feature.name));
            }
            let field = group
                .dimensions
                .iter()
                .find(|d| d.name == feature.name && d.name != "lemma")
                .ok_or_else(|| {
                    format!(
                        "{} has no morphology feature {}",
                        self.category, feature.name
                    )
                })?;
            if feature.value.trim().is_empty()
                || matches!(&field.kind,
                FieldKind::Closed(values) if !values.contains(&feature.value.as_str()))
            {
                return Err(format!(
                    "Invalid {}.{} value {:?}",
                    self.category, feature.name, feature.value
                ));
            }
            result.insert(
                attribute_name(&feature.name),
                LfgValue::Atom(feature.value.clone()),
            );
        }
        Ok(result)
    }
}

/// Specialize the derived wire schema from the language's existing catalog.
/// Category and feature value remain in the same branch: inventories are never
/// flattened into a union across noun, pronoun, verb, etc.
fn schema<L: LinguisticDefinition>() -> serde_json::Value {
    let branches: Vec<_> = catalog::<L>()
        .into_iter()
        .filter(|g| g.dimensions.iter().any(|d| d.name != "lemma"))
        .map(|group| {
            let fields: Vec<_> = group
                .dimensions
                .iter()
                .filter(|d| d.name != "lemma")
                .map(|field| {
                    let value = match &field.kind {
                        FieldKind::Open => serde_json::json!({"type":"string"}),
                        FieldKind::Closed(values) => {
                            serde_json::json!({"type":"string","enum":values})
                        }
                    };
                    serde_json::json!({
                        "type":"object","additionalProperties":false,"required":["name","value"],
                        "properties":{"name":{"type":"string","enum":[field.name]},"value":value}
                    })
                })
                .collect();
            let features = serde_json::json!({"type":"array","items":{"anyOf":fields}});
            serde_json::json!({
                "type":"object","additionalProperties":false,"required":["category","features"],
                "properties":{"category":{"type":"string","enum":[group.key]},"features":features}
            })
        })
        .collect();
    if branches.is_empty() {
        // Handwritten test/external languages may expose no catalog at all.
        // In that case the optional morphology bundle can only be null.
        serde_json::json!({"type":"null"})
    } else {
        serde_json::json!({"anyOf":branches})
    }
}

/// Schema-only parameter: the transport still deserializes `LfgMorphology`,
/// but its generated schema uses the requested language's existing inventories.
pub(crate) struct LfgMorphologySchema<L>(std::marker::PhantomData<L>);

impl<L: LinguisticDefinition> JsonSchema for LfgMorphologySchema<L> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("LfgMorphology_{}", L::ISO_LANG.to_639_3()).into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema::<L>()
            .try_into()
            .expect("morphology projection is an object schema")
    }
}
