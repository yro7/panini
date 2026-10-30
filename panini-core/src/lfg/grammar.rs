use crate::aggregable::ClosedValues;

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
