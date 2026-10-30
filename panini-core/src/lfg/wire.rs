use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::traits::LinguisticDefinition;

use super::{
    LfgAnalysis, LfgAttribute, LfgFunction, LfgMorphology, LfgPredicate, LfgStructure, LfgValue,
};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LfgWireAnalysis<M = LfgMorphology> {
    pub root: String,
    pub structures: Vec<LfgWireStructure<M>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(extend("required" = ["id", "pred", "attributes", "morphology", "controller"]))]
pub struct LfgWireStructure<M = LfgMorphology> {
    pub id: String,
    pub pred: Option<LfgWirePredicate>,
    pub attributes: Vec<LfgAttribute>,
    pub morphology: Option<M>,
    /// The governed function whose structure is shared with this node's XCOMP.SUBJ.
    pub controller: Option<LfgFunction>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LfgWirePredicate {
    pub lemma: String,
    pub arguments: Vec<LfgFunction>,
}

impl LfgWireAnalysis {
    pub fn resolve<L: LinguisticDefinition>(self, lang: &L) -> Result<LfgAnalysis, String> {
        let mut structures = Vec::new();
        for node in self.structures {
            let mut attributes = node
                .morphology
                .map(|m| m.resolve::<L>())
                .transpose()?
                .unwrap_or_default();
            for attribute in node.attributes {
                let (name, value) = attribute.resolve();
                if attributes.insert(name.clone(), value).is_some() {
                    return Err(format!("{}: duplicate attribute {name}", node.id));
                }
            }
            if let Some(controller) = node.controller {
                attributes.insert(
                    "XCOMP-CONTROLLER".into(),
                    LfgValue::Atom(controller.as_str().into()),
                );
            }
            let pred = node.pred.map(|p| LfgPredicate {
                lemma: p.lemma,
                arguments: p.arguments.into_iter().map(|a| a.as_str().into()).collect(),
            });
            structures.push(LfgStructure {
                id: node.id,
                pred,
                attributes,
            });
        }
        let analysis = LfgAnalysis {
            root: self.root,
            structures,
        };
        analysis.validate_for_language(lang)?;
        Ok(analysis)
    }
}
