pub mod assign;
pub mod block;
pub mod branch;
pub mod element;
pub mod expr;
pub mod repeat;

use crate::branch::Branch;
use crate::element::HIRElement;
use crate::expr::Expr;
use crate::value::CoValue;
use crate::var::Variable;

pub struct HIR {
    pub consts: Vec<CoValue>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,

    code: Vec<HIRElement>,
}

impl HIR {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],

            code: vec![],
        }
    }

    pub fn branch(&mut self) -> &mut Branch {
        let mut branch = Branch::new();
        self.code.push(branch.into());
        match self.code.last_mut().unwrap() {
            HIRElement::Branch(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat(&mut self, condition: Option<Expr>) {
    }
}
