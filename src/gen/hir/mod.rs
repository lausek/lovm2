//! Highlevel Intermediate Representation

mod assign;
mod block;
mod branch;
mod call;
mod cast;
mod element;
mod expr;
mod include;
mod initialize;
mod interrupt;
mod repeat;
mod r#return;
mod slice;

use lovm2_error::*;

use crate::value::Value;
use crate::var::Variable;

use super::*;

pub use self::assign::{Access, Assign};
pub use self::block::Block;
pub use self::branch::Branch;
pub use self::call::Call;
pub use self::cast::Cast;
pub use self::element::HirElement;
pub use self::expr::{Expr, Operator1, Operator2};
pub use self::include::Include;
pub use self::initialize::Initialize;
pub use self::interrupt::Interrupt;
pub use self::r#return::Return;
pub use self::repeat::{Break, Continue, Repeat};
pub use self::slice::Slice;

#[derive(Clone)]
pub struct Hir {
    pub args: Vec<Variable>,
    pub code: Block,
}

impl Hir {
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

    pub fn build(&mut self, ru: &mut HirLoweringRuntime) -> Lovm2CompileResult<()> {
        // automatically add a `return nil` if not present already
        match self.code.last_mut() {
            Some(HirElement::Return(_)) => {}
            _ => self.code.step(Return::nil()),
        }

        // TODO: avoid clone here. needs change of `HirLoweringRuntime`
        ru.add_hir(self.clone())
    }
}

impl HasBlock for Hir {
    #[inline]
    fn block_mut(&mut self) -> &mut Block {
        &mut self.code
    }
}

pub trait HasBlock {
    fn block_mut(&mut self) -> &mut Block;

    #[inline]
    fn step<T>(&mut self, element: T) -> &mut Self
    where
        T: Into<HirElement>,
    {
        self.block_mut().step(element);
        self
    }

    #[inline]
    fn branch(&mut self) -> &mut Branch {
        self.block_mut().branch()
    }

    #[inline]
    fn repeat(&mut self) -> &mut Repeat {
        self.block_mut().repeat()
    }

    #[inline]
    fn repeat_until(&mut self, condition: Expr) -> &mut Repeat {
        self.block_mut().repeat_until(condition)
    }
}
