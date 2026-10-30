use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::aggregable::ClosedValues;
use crate::traits::{BinaryVoice, LinguisticDefinition};

use super::{LfgAnalysis, LfgStructure, LfgValue, features};

/// Governed functions supported by this profile. ADJUNCT, TOPIC and FOCUS are
/// deliberately absent: they cannot occur in a predicate's selected frame.
#[panini_macro::closed_enum(crate = "crate")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LfgFunction {
    Subj,
    Obj,
    Obj2,
    Comp,
    Xcomp,
    Obl,
    OblAg,
    OblGoal,
    OblSource,
    OblLoc,
    OblInst,
    Predlink,
}

impl LfgFunction {
    pub fn as_str(&self) -> &str {
        self.variant_str()
    }

    fn from_name(name: &str) -> Result<Self, String> {
        serde_json::from_value(serde_json::Value::String(name.into()))
            .map_err(|_| format!("Unsupported governed function {name}"))
    }

    fn can_control(self) -> bool {
        !matches!(self, Self::Comp | Self::Xcomp | Self::Predlink)
    }
}

/// Every structural attribute carries its own value type. There is no
/// arbitrary name/value escape hatch in the LLM output.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(transform = attribute_schema)]
pub enum LfgAttribute {
    Function {
        function: LfgFunction,
        target: String,
    },
    Adjunct {
        members: Vec<String>,
    },
    Topic {
        target: String,
    },
    Focus {
        target: String,
    },
    TenseAspect {
        target: String,
    },
    Definite {
        value: bool,
    },
    Voice {
        value: BinaryVoice,
    },
}

fn attribute_schema(schema: &mut schemars::Schema) {
    use schemars::transform::Transform;
    // Structured-output APIs support enum/anyOf more widely than const/oneOf.
    // These branches are disjoint by their kind tag, so anyOf is equivalent.
    schemars::transform::ReplaceConstValue::default().transform(schema);
    if let Some(object) = schema.as_object_mut()
        && let Some(variants) = object.remove("oneOf")
    {
        object.insert("anyOf".into(), variants);
    }
}

fn reference<'a>(node: &'a LfgStructure, name: &str) -> Result<&'a str, String> {
    match node.attributes.get(name) {
        Some(LfgValue::Reference(r)) => Ok(&r.target),
        _ => Err(format!("{}: {name} must reference a structure", node.id)),
    }
}

impl LfgAnalysis {
    /// Validate the current extraction profile, including category-specific
    /// morphological inventories and declared local functional control.
    /// `validate()` remains available for reading the less constrained v1 shape.
    /// This checks declared control, not a verb's independently known lexical frame.
    pub fn validate_for_language<L: LinguisticDefinition>(&self, _lang: &L) -> Result<(), String> {
        self.validate()?;
        let nodes: BTreeMap<_, _> = self.structures.iter().map(|n| (n.id.as_str(), n)).collect();
        let catalog = features::catalog::<L>();
        for node in &self.structures {
            if let Some(pred) = &node.pred {
                for argument in &pred.arguments {
                    LfgFunction::from_name(argument)?;
                }
            }
            let category = match node.attributes.get("MORPHOLOGY-GROUP") {
                None => None,
                Some(LfgValue::Atom(name)) => {
                    Some(catalog.iter().find(|g| g.key == *name).ok_or_else(|| {
                        format!("{}: unknown morphology category {name}", node.id)
                    })?)
                }
                _ => return Err(format!("{}: MORPHOLOGY-GROUP must be an atom", node.id)),
            };
            let mut morphological = Vec::new();
            for (name, value) in &node.attributes {
                let dimension = category.and_then(|g| {
                    g.dimensions
                        .iter()
                        .find(|d| d.name != "lemma" && features::attribute_name(&d.name) == *name)
                });
                if dimension.is_none()
                    && (LfgFunction::from_name(name).is_ok()
                        || matches!(
                            name.as_str(),
                            "ADJUNCT"
                                | "TOPIC"
                                | "FOCUS"
                                | "TNS-ASP"
                                | "DEF"
                                | "VOICE"
                                | "XCOMP-CONTROLLER"
                        ))
                {
                    if node.pred.is_none() {
                        return Err(format!(
                            "{}: feature-only structures cannot carry {name}",
                            node.id
                        ));
                    }
                    continue;
                }
                if name == "MORPHOLOGY-GROUP" {
                    continue;
                }
                let dimension = dimension.ok_or_else(|| {
                    format!(
                        "{}: feature {name} is not in this morphology category",
                        node.id
                    )
                })?;
                if node.pred.is_some() && features::temporal(&dimension.name) {
                    return Err(format!(
                        "{}: put {name} in a TNS-ASP feature structure",
                        node.id
                    ));
                }
                let LfgValue::Atom(value) = value else {
                    return Err(format!(
                        "{}: morphology feature {name} must be atomic",
                        node.id
                    ));
                };
                morphological.push(super::LfgFeature {
                    name: dimension.name.clone(),
                    value: value.clone(),
                });
            }
            if let Some(group) = category {
                super::LfgMorphology {
                    category: group.key.clone(),
                    features: morphological,
                }
                .resolve::<L>()?;
            }
            if let Some(value) = node.attributes.get("DEF")
                && !matches!(value, LfgValue::Atom(v) if v == "+" || v == "-")
            {
                return Err(format!("{}: DEF must be + or -", node.id));
            }
            if let Some(value) = node.attributes.get("VOICE")
                && !category.is_some_and(|g| g.dimensions.iter().any(|d| d.name == "voice"))
                && !matches!(value, LfgValue::Atom(v) if BinaryVoice::all_variants().contains(&v.as_str()))
            {
                return Err(format!("{}: VOICE must use BinaryVoice values", node.id));
            }
            if node.attributes.contains_key("TNS-ASP") {
                let bundle = nodes[reference(node, "TNS-ASP")?];
                if bundle.pred.is_some()
                    || !bundle
                        .attributes
                        .keys()
                        .any(|name| name != "MORPHOLOGY-GROUP")
                {
                    return Err(format!(
                        "{}: TNS-ASP must reference a nonempty feature-only structure",
                        node.id
                    ));
                }
                let Some(LfgValue::Atom(group)) = bundle.attributes.get("MORPHOLOGY-GROUP") else {
                    return Err(format!("{}: TNS-ASP needs a morphology category", node.id));
                };
                let group = catalog
                    .iter()
                    .find(|g| g.key == *group)
                    .ok_or_else(|| format!("Unknown morphology category {group}"))?;
                for name in bundle
                    .attributes
                    .keys()
                    .filter(|n| *n != "MORPHOLOGY-GROUP")
                {
                    if !group.dimensions.iter().any(|d| {
                        features::temporal(&d.name) && features::attribute_name(&d.name) == *name
                    }) {
                        return Err(format!(
                            "{}: non-temporal feature {name} in TNS-ASP",
                            node.id
                        ));
                    }
                }
            }
            match (
                node.attributes.get("XCOMP"),
                node.attributes.get("XCOMP-CONTROLLER"),
            ) {
                (Some(_), Some(LfgValue::Atom(controller))) => {
                    let function = LfgFunction::from_name(controller)?;
                    if !function.can_control() {
                        return Err(format!(
                            "{}: {controller} cannot control XCOMP.SUBJ in this profile",
                            node.id
                        ));
                    }
                    let xcomp = nodes[reference(node, "XCOMP")?];
                    let controlled_subject = reference(node, controller)?;
                    let subject = reference(xcomp, "SUBJ").map_err(|_| format!(
                        "{}: XCOMP target {} must include SUBJ in pred.arguments and a \
                         function attribute SUBJ targeting {controlled_subject}, shared with \
                         the declared {controller} controller. Do not omit the infinitive's implicit subject.",
                        node.id, xcomp.id
                    ))?;
                    if controlled_subject != subject {
                        return Err(format!(
                            "{}: XCOMP target {} SUBJ must share the declared {controller} controller: \
                             use target {controlled_subject}, not {subject}",
                            node.id, xcomp.id
                        ));
                    }
                }
                (Some(_), _) => {
                    return Err(format!(
                        "{}: XCOMP requires an explicit XCOMP-CONTROLLER",
                        node.id
                    ));
                }
                (None, Some(_)) => {
                    return Err(format!("{}: controller declared without XCOMP", node.id));
                }
                (None, None) => {}
            }
        }
        Ok(())
    }
}
