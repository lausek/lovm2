pub mod element;
pub mod lowering;
pub mod prelude;

use crate::hir::expr::{Operator1, Operator2};

pub use self::element::LirElement;
pub use self::lowering::LirLoweringRuntime;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Label {
    Custom(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Operator1(Operator1),
    Operator2(Operator2),
}

impl From<Operator1> for Operator {
    fn from(op: Operator1) -> Self {
        Self::Operator1(op)
    }
}

impl From<Operator2> for Operator {
    fn from(op: Operator2) -> Self {
        Self::Operator2(op)
    }
}
