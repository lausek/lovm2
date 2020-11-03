//! highlevel intermediate representation

pub mod assign;
pub mod block;
pub mod branch;
pub mod call;
pub mod cast;
pub mod element;
pub mod expr;
pub mod include;
pub mod initialize;
pub mod interrupt;
pub mod lowering;
pub mod repeat;
pub mod r#return;
pub mod slice;

pub mod prelude;

use lovm2_error::*;

use crate::code::CodeObject;
use crate::hir::block::Block;
use crate::hir::element::HIRElement;
use crate::hir::lowering::LoweringRuntime;
use crate::hir::r#return::Return;
use crate::value::Value;
use crate::var::Variable;

#[derive(Clone)]
pub struct HIR {
    pub args: Vec<Variable>,
    pub consts: Vec<Value>,
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

    pub fn build(mut self) -> Lovm2CompileResult<CodeObject> {
        // automatically add a `return nil` if not present already
        match self.code.last_mut() {
            Some(HIRElement::Return(_)) => {}
            _ => self.code.push(Return::nil()),
        }
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
