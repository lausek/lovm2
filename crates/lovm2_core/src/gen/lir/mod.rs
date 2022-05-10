//! Lowlevel Intermediate Representation.

mod element;
mod lowering;

use super::*;

pub use self::element::LirElement;
pub use self::lowering::LirLoweringRuntime;

/// Operation scope.
#[derive(Clone, Debug, PartialEq)]
pub enum Scope {
    Global,
    Local,
}

/// Combination of all operators.
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Operator1(LV2Operator1),
    Operator2(LV2Operator2),
}

impl From<&LV2Operator1> for Operator {
    fn from(op: &LV2Operator1) -> Self {
        Self::Operator1(op.clone())
    }
}

impl From<&LV2Operator2> for Operator {
    fn from(op: &LV2Operator2) -> Self {
        Self::Operator2(op.clone())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LV2LabelTy {
    Branch,
    Condition,
    Repeat,
}

/// Location inside the LIR that can be used as jump target.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LV2Label {
    pub id: usize,
    pub ty: LV2LabelTy,
    pub is_start: bool,
}

impl std::fmt::Display for LV2Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let position = if self.is_start { "start" } else { "end" };
        write!(f, "{:?}_{}_{}", self.ty, self.id, position)
    }
}
