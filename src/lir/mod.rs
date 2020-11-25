pub mod element;
pub mod lowering;
pub mod prelude;

use crate::hir::expr::{Operator1, Operator2};

pub use self::element::LirElement;

#[derive(Clone, Debug, PartialEq)]
pub struct Label(usize);

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
