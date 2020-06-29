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
use crate::block::Block;
use crate::code::CodeObject;
use crate::element::HIRElement;
use crate::expr::Expr;
use crate::lowering::LoweringRuntime;
use crate::repeat::Repeat;
use crate::value::CoValue;
use crate::var::Variable;

pub struct HIR {
    pub consts: Vec<CoValue>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,

    pub code: Block,
}

impl HIR {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],

            code: Block::new(),
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
        self.code.push_inplace(element.into());
    }

    pub fn branch(&mut self) -> &mut Branch {
        self.code.push_inplace(Branch::new());
        match self.code.last_mut().unwrap() {
            HIRElement::Branch(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn repeat(&mut self, condition: Option<Expr>) -> &mut Repeat {
        if let Some(condition) = condition {
            self.code.push_inplace(Repeat::until(condition));
        } else {
            self.code.push_inplace(Repeat::endless());
        }
        match self.code.last_mut().unwrap() {
            HIRElement::Repeat(ref mut r) => r,
            _ => unreachable!(),
        }
    }
}
