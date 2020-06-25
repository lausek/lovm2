pub mod assign;
pub mod block;
pub mod branch;
pub mod call;
pub mod element;
pub mod expr;
pub mod interrupt;
pub mod lowering;
pub mod repeat;

pub mod prelude;

use crate::branch::Branch;
use crate::code::CodeObject;
use crate::element::HIRElement;
use crate::expr::Expr;
use crate::lowering::LoweringRuntime;
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
        // TODO: optimise codeobject here; eg. `Not, Jf` is equal to `Jt`
        LoweringRuntime::complete(self)
    }

    pub fn push<T>(&mut self, element: T)
    where
        T: Into<HIRElement>,
    {
        self.code.push(element.into());
    }

    pub fn branch(&mut self) -> &mut Branch {
        self.code.push(Branch::new().into());
        match self.code.last_mut().unwrap() {
            HIRElement::Branch(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat(&mut self, _condition: Option<Expr>) {}
}
