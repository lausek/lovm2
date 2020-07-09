pub mod assign;
pub mod block;
pub mod branch;
pub mod call;
pub mod cast;
pub mod element;
pub mod expr;
pub mod include;
pub mod interrupt;
pub mod lowering;
pub mod repeat;

pub mod prelude;

use crate::block::Block;
use crate::code::CodeObject;
use crate::element::HIRElement;
use crate::lowering::LoweringRuntime;
use crate::value::CoValue;
use crate::var::Variable;

pub struct HIR {
    pub args: Vec<Variable>,
    pub consts: Vec<CoValue>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,

    pub code: Block,
}

impl HIR {
    pub fn new() -> Self {
        Self {
            args: vec![],
            consts: vec![],
            locals: vec![],
            globals: vec![],

            code: Block::new(),
        }
    }

    pub fn with_args(args: Vec<Variable>) -> Self {
        let mut hir = Self::new();
        hir.args = args;
        hir
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
}
