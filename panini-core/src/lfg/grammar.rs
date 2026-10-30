use crate::aggregable::ClosedValues;
use crate::traits::BinaryVoice;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
