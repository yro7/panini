//! A language-independent, finite acyclic f-structure graph.
//!
//! This is a deliberately bounded LFG profile, not an XLE grammar or a proof
//! that an analysis is linguistically correct. Attribute names follow ParGram
//! conventions; language-specific features need no Rust language definition.

use std::collections::{BTreeMap, BTreeSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LfgAnalysis {
    pub root: String,
    pub structures: Vec<LfgStructure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LfgStructure {
    pub id: String,
    /// None for feature-only structures, such as TNS-ASP.
    pub pred: Option<LfgPredicate>,
    pub attributes: BTreeMap<String, LfgValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LfgPredicate {
    pub lemma: String,
    /// Selected grammatical functions in this analysis (not semantic roles).
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LfgValue {
    Atom(String),
    Reference(LfgReference),
    Set(LfgSet),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LfgReference {
    #[serde(rename = "ref")]
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LfgSet {
    pub set: Vec<String>,
}

fn governed(name: &str) -> bool {
    // Retain the open role spellings for legacy v1 readers. New extractions
    // additionally pass the closed, typed profile in validate_for_language().
    <LfgFunction as crate::aggregable::ClosedValues>::all_variants().contains(&name)
        || name.starts_with("OBL_")
        || name.starts_with("OBJ_")
}

fn attribute_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_uppercase())
        && name
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-' || c == '_')
        && name != "PRED"
}

impl LfgValue {
    fn references(&self) -> &[String] {
        match self {
            Self::Atom(_) => &[],
            Self::Reference(r) => std::slice::from_ref(&r.target),
            Self::Set(s) => &s.set,
        }
    }
}

impl LfgAnalysis {
    /// Check graph integrity and local predicate/frame consistency.
    /// Frames themselves are model hypotheses, not independently checked valency.
    pub fn validate(&self) -> Result<(), String> {
        let mut nodes = BTreeMap::new();
        for node in &self.structures {
            if node.id.trim().is_empty() || nodes.insert(node.id.as_str(), node).is_some() {
                return Err(format!("Empty or duplicate structure id: {:?}", node.id));
            }
        }
        if !nodes.contains_key(self.root.as_str()) {
            return Err(format!("Root {:?} does not name a structure", self.root));
        }
        if nodes[self.root.as_str()].pred.is_none() {
            return Err("Root must have a predicate, not only grammatical features".to_string());
        }
        for node in &self.structures {
            let mut arguments = BTreeSet::new();
            if let Some(pred) = &node.pred {
                if pred.lemma.trim().is_empty() {
                    return Err(format!("{}: empty predicate lemma", node.id));
                }
                for arg in &pred.arguments {
                    if !governed(arg) || !arguments.insert(arg.as_str()) {
                        return Err(format!(
                            "{}: invalid or duplicate predicate argument {arg}",
                            node.id
                        ));
                    }
                    if !matches!(node.attributes.get(arg), Some(LfgValue::Reference(_))) {
                        return Err(format!(
                            "{}: selected argument {arg} must reference a structure",
                            node.id
                        ));
                    }
                }
            }
            for (name, value) in &node.attributes {
                if !attribute_name(name) {
                    return Err(format!("{}: invalid attribute name {name}", node.id));
                }
                if governed(name) && !arguments.contains(name.as_str()) {
                    return Err(format!(
                        "{}: governed function {name} is absent from pred.arguments",
                        node.id
                    ));
                }
                if name == "ADJUNCT" && !matches!(value, LfgValue::Set(_)) {
                    return Err(format!(
                        "{}: ADJUNCT must be a set of structure references",
                        node.id
                    ));
                }
                if matches!(name.as_str(), "TOPIC" | "FOCUS")
                    && !matches!(value, LfgValue::Reference(_))
                {
                    return Err(format!("{}: {name} must reference a structure", node.id));
                }
                if let LfgValue::Atom(atom) = value
                    && atom.trim().is_empty()
                {
                    return Err(format!("{}: empty atomic value for {name}", node.id));
                }
                let mut members = BTreeSet::new();
                for target in value.references() {
                    if !members.insert(target) {
                        return Err(format!(
                            "{}: duplicate set member {target} in {name}",
                            node.id
                        ));
                    }
                    if !nodes.contains_key(target.as_str()) {
                        return Err(format!(
                            "{}: dangling reference {name} -> {target}",
                            node.id
                        ));
                    }
                }
            }
        }
        // Iterative DFS accepts reentrancy but rejects cycles without recursion.
        let mut active = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut stack = vec![(self.root.as_str(), false)];
        while let Some((id, leaving)) = stack.pop() {
            if leaving {
                active.remove(id);
                visited.insert(id);
            } else {
                if active.contains(id) {
                    return Err(format!(
                        "Cycle through structure {id}; this profile requires an acyclic graph"
                    ));
                }
                if visited.contains(id) {
                    continue;
                }
                active.insert(id);
                stack.push((id, true));
                for value in nodes[id].attributes.values() {
                    for target in value.references() {
                        stack.push((target, false));
                    }
                }
            }
        }
        if visited.len() != nodes.len() {
            return Err("All structures must be reachable from root".to_string());
        }
        Ok(())
    }
}

/// Strict-output transport: named entries instead of open JSON property maps.
/// Post-processing resolves it into the public attribute-map representation.
pub(crate) mod wire {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct LfgWireAnalysis {
        pub root: String,
        pub structures: Vec<LfgWireStructure>,
    }

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    #[schemars(extend("required" = ["id", "pred", "attributes"]))]
    pub struct LfgWireStructure {
        pub id: String,
        pub pred: Option<LfgPredicate>,
        pub attributes: Vec<LfgWireAttribute>,
    }

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct LfgWireAttribute {
        pub name: String,
        pub value: LfgWireValue,
    }

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
    pub enum LfgWireValue {
        Atom { value: String },
        Reference { target: String },
        Set { members: Vec<String> },
    }

    impl LfgWireAnalysis {
        pub fn resolve(self) -> Result<LfgAnalysis, String> {
            let mut structures = Vec::new();
            for node in self.structures {
                let mut attributes = BTreeMap::new();
                for attr in node.attributes {
                    let value = match attr.value {
                        LfgWireValue::Atom { value } => LfgValue::Atom(value),
                        LfgWireValue::Reference { target } => {
                            LfgValue::Reference(LfgReference { target })
                        }
                        LfgWireValue::Set { members } => LfgValue::Set(LfgSet { set: members }),
                    };
                    if attributes.insert(attr.name.clone(), value).is_some() {
                        return Err(format!("{}: duplicate attribute {}", node.id, attr.name));
                    }
                }
                structures.push(LfgStructure {
                    id: node.id,
                    pred: node.pred,
                    attributes,
                });
            }
            let analysis = LfgAnalysis {
                root: self.root,
                structures,
            };
            analysis.validate()?;
            Ok(analysis)
        }
    }
}

mod features;
mod grammar;
pub use features::{LfgFeature, LfgMorphology};
pub use grammar::{LfgAttribute, LfgFunction};
