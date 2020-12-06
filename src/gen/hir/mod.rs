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
pub mod repeat;
pub mod r#return;
pub mod slice;

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

        ru.add_hir(self)
    }

    pub fn push<T>(&mut self, element: T)
    where
        T: Into<HirElement>,
    {
        self.code.push(element.into());
    }
}