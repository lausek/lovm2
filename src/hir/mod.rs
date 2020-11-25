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

use crate::hir::block::Block;
use crate::hir::element::HirElement;
use crate::hir::lowering::HirLoweringRuntime;
use crate::hir::r#return::Return;
use crate::var::Variable;

#[derive(Clone)]
pub struct HIR {
    pub args: Vec<Variable>,
    pub code: Block,
}

impl HIR {
    pub fn new() -> Self {
        Self {
            args: vec![],
            code: Block::new(),
        }
    }

    pub fn with_args(args: Vec<Variable>) -> Self {
        let mut hir = Self::new();
        hir.args = args;
        hir
    }

    pub fn build(mut self, ru: &mut HirLoweringRuntime) -> Lovm2CompileResult<()> {
        // automatically add a `return nil` if not present already
        match self.code.last_mut() {
            Some(HirElement::Return(_)) => {}
            _ => self.code.push(Return::nil()),
        }

        // TODO: optimise codeobject here; eg. `Not, Jf` is equal to `Jt`
        ru.add_hir(self)
    }

    pub fn push<T>(&mut self, element: T)
    where
        T: Into<HirElement>,
    {
        self.code.push(element.into());
    }
}
