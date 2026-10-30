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
