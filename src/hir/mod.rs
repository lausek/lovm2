pub mod assign;
pub mod block;
pub mod branch;
pub mod call;
pub mod element;
pub mod expr;
pub mod lowering;
pub mod repeat;

pub mod prelude;

use crate::branch::Branch;
use crate::code::{CodeObject, CodeObjectBuilder};
use crate::element::HIRElement;
use crate::expr::Expr;
use crate::lowering::{Lowering, LoweringRuntime};
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

    pub fn build(self) -> Result<CodeObject, String> {
        LoweringRuntime::complete(self)
    }

    pub fn push<T>(&mut self, element: T) 
        where T: Into<HIRElement>
    {
        self.code.push(element.into());
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
