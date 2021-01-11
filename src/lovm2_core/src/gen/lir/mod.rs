//! Lowlevel Intermediate Representation

mod element;
mod lowering;

use super::*;

pub use self::element::LirElement;
pub use self::lowering::LirLoweringRuntime;

/// Scope of an operation
#[derive(Clone, Debug, PartialEq)]
pub enum Scope {
    Global,
    Local,
}

/// Combination of all operators
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Operator1(Operator1),
    Operator2(Operator2),
}

impl From<&Operator1> for Operator {
    fn from(op: &Operator1) -> Self {
        Self::Operator1(op.clone())
    }
}

impl From<&Operator2> for Operator {
    fn from(op: &Operator2) -> Self {
        Self::Operator2(op.clone())
    }
}

/// Location inside the LIR that can be used as jump target
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label(String);

impl From<String> for Label {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
