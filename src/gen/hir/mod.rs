//! Highlevel Intermediate Representation

mod lowering;

mod assign;
mod block;
mod branch;
mod call;
mod conv;
mod element;
mod expr;
mod include;
mod initialize;
mod interrupt;
mod iter;
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
pub use self::conv::Conv;
pub use self::element::HirElement;
pub use self::expr::{Expr, Operator1, Operator2};
pub use self::include::Include;
pub use self::initialize::Initialize;
pub use self::interrupt::Interrupt;
pub use self::iter::Iter;
pub use self::lowering::{HirLowering, HirLoweringRuntime, Jumpable};
pub use self::r#return::Return;
pub use self::repeat::{Break, Continue, Repeat};
pub use self::slice::Slice;

/// Highlevel representation of a function
#[derive(Clone)]
pub struct Hir {
    args: Vec<Variable>,
    block: Block,
}

impl Hir {
    /// Create a new function
    pub fn new() -> Self {
        Self {
            args: vec![],
            block: Block::new(),
        }
    }

    /// Create a new function with arguments
    pub fn with_args(args: Vec<Variable>) -> Self {
        let mut hir = Self::new();
        hir.args = args;
        hir
    }

    /// Add a HIR to the lowering runtime
    pub fn build<'hir, 'lir>(
        &'hir self,
        ru: &mut HirLoweringRuntime<'lir>,
    ) -> Lovm2CompileResult<()>
    where
        'hir: 'lir,
    {
        // TODO: avoid clone here. needs change of `HirLoweringRuntime`
        ru.add_hir(self)
    }
}

impl HasBlock for Hir {
    #[inline]
    fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}

/// Supplying functionality for all structures containing a [Block]
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

    #[inline]
    fn repeat_iterating<U, T>(&mut self, collection: U, item: T) -> &mut Repeat
    where
        U: Into<Expr>,
        T: Into<Variable>,
    {
        self.block_mut().repeat_iterating(collection, item)
    }
}
