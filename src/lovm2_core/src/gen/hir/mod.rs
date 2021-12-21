//! Highlevel Intermediate Representation

mod lowering;

mod block;
mod branch;
mod call;
mod expr;
mod repeat;
mod stmt;

use crate::value::LV2Value;
use crate::var::LV2Variable;

use super::*;

pub use self::block::LV2Block;
pub use self::branch::LV2Branch;
pub use self::call::LV2Call;
pub use self::expr::{LV2Expr, LV2Operator1, LV2Operator2};
pub use self::lowering::{LV2HirJumpable, LV2HirLowering, LV2HirLoweringRuntime};
pub use self::repeat::LV2Repeat;
pub use self::stmt::LV2Statement;

/// Highlevel representation of a function
#[derive(Clone)]
pub struct LV2Function {
    args: Vec<LV2Variable>,
    block: LV2Block,
}

impl LV2Function {
    /// Create a new function
    pub fn new() -> Self {
        Self {
            args: vec![],
            block: LV2Block::new(),
        }
    }

    /// Create a new function with arguments
    pub fn with_args(args: Vec<LV2Variable>) -> Self {
        let mut hir = Self::new();
        hir.args = args;
        hir
    }

    /// Add a HIR to the lowering runtime
    pub fn build<'hir, 'lir>(
        &'hir self,
        ru: &mut LV2HirLoweringRuntime<'lir>,
    ) -> LV2CompileResult<()>
    where
        'hir: 'lir,
    {
        ru.add_hir(self)
    }
}

impl LV2AddStatements for LV2Function {
    #[inline]
    fn block_mut(&mut self) -> &mut LV2Block {
        &mut self.block
    }
}

/// Supplying functionality for all structures containing a [LV2Block]
pub trait LV2AddStatements {
    fn block_mut(&mut self) -> &mut LV2Block;

    #[inline]
    fn assign<T: Into<LV2Variable>, U: Into<LV2Expr>>(&mut self, var: T, expr: U) -> &mut Self {
        self.block_mut().assign(var, expr);
        self
    }

    #[inline]
    fn branch(&mut self) -> &mut LV2Branch {
        self.block_mut().branch()
    }

    #[inline]
    fn break_repeat(&mut self) -> &mut Self {
        self.block_mut().break_repeat();
        self
    }

    #[inline]
    fn continue_repeat(&mut self) -> &mut Self {
        self.block_mut().continue_repeat();
        self
    }

    #[inline]
    fn decrement(&mut self, ident: &LV2Variable) -> &mut Self {
        self.block_mut().decrement(ident);
        self
    }

    #[inline]
    fn extend(&mut self, block: LV2Block) -> &mut Self {
        self.block_mut().extend(block);
        self
    }

    #[inline]
    fn global(&mut self, var: &LV2Variable) -> &mut Self {
        self.block_mut().global(var);
        self
    }

    #[inline]
    fn import<T: Into<LV2Expr>>(&mut self, name: T) -> &mut Self {
        self.block_mut().import(name);
        self
    }

    #[inline]
    fn import_from<T: Into<LV2Expr>>(&mut self, name: T) -> &mut Self {
        self.block_mut().import_from(name);
        self
    }

    #[inline]
    fn increment(&mut self, ident: &LV2Variable) -> &mut Self {
        self.block_mut().increment(ident);
        self
    }

    #[inline]
    fn local(&mut self, var: &LV2Variable) -> &mut Self {
        self.block_mut().local(var);
        self
    }

    #[inline]
    fn repeat(&mut self) -> &mut LV2Repeat {
        self.block_mut().repeat()
    }

    #[inline]
    fn repeat_until(&mut self, condition: LV2Expr) -> &mut LV2Repeat {
        self.block_mut().repeat_until(condition)
    }

    #[inline]
    fn repeat_iterating<T: Into<LV2Expr>, U: Into<LV2Variable>>(
        &mut self,
        collection: T,
        item: U,
    ) -> &mut LV2Repeat {
        self.block_mut().repeat_iterating(collection, item)
    }

    #[inline]
    fn return_nil(&mut self) -> &mut Self {
        self.block_mut().return_nil();
        self
    }

    #[inline]
    fn return_value<T: Into<LV2Expr>>(&mut self, value: T) -> &mut Self {
        self.block_mut().return_value(value);
        self
    }

    #[inline]
    fn set<T: Into<LV2Expr>, U: Into<LV2Expr>>(&mut self, target: T, source: U) -> &mut Self {
        self.block_mut().set(target, source);
        self
    }

    #[inline]
    fn step<T: Into<LV2Statement>>(&mut self, element: T) -> &mut Self {
        self.block_mut().step(element);
        self
    }

    #[inline]
    fn trigger(&mut self, n: u16) -> &mut Self {
        self.block_mut().trigger(n);
        self
    }
}
