//! runs a `Block` forever or until a condition is met

use crate::hir::block::Block;
use crate::hir::element::HirElement;
use crate::hir::expr::Expr;
use crate::hir::lowering::{HirLowering, HirLoweringRuntime};
use crate::lir::LirElement;

#[derive(Clone)]
pub enum RepeatKind {
    Until(Expr),
    Endless,
}

#[derive(Clone)]
pub struct Repeat {
    pub block: Block,
    condition: RepeatKind,
}

impl Repeat {
    pub fn until(condition: Expr) -> Self {
        Self {
            block: Block::new(),
            condition: RepeatKind::Until(condition),
        }
    }

    pub fn endless() -> Self {
        Self {
            block: Block::new(),
            condition: RepeatKind::Endless,
        }
    }

    pub fn push<T>(&mut self, hir: T) -> &mut Self
    where
        T: Into<HirElement>,
    {
        self.block.push(hir.into());
        self
    }
}

impl HirLowering for Repeat {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        runtime.push_loop();

        match self.condition {
            RepeatKind::Until(expr) => {
                expr.lower(runtime);

                // if the condition is met, jump to end of repeat
                // which is equal to a break. the instruction will
                // receive its final address once the block has been
                // lowered.
                runtime.emit(LirElement::jump_conditional(true, todo!()));
                let offset = runtime.offset();
                runtime.loop_mut().unwrap().add_break(offset);
            }
            RepeatKind::Endless => {}
        }

        self.block.lower(runtime);

        // add a jump to the start of the loop. this is equal to
        // a continue statement.
        Continue::new().lower(runtime);
    }
}

#[derive(Clone)]
pub struct Break {}

impl HirLowering for Break {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        runtime.emit(LirElement::jump(todo!()));
        let offset = runtime.offset();
        runtime.loop_mut().unwrap().add_break(offset);
    }
}

impl Break {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone)]
pub struct Continue {}

impl Continue {
    pub fn new() -> Self {
        Self {}
    }
}

impl HirLowering for Continue {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        runtime.emit(LirElement::jump(todo!()));
        let offset = runtime.offset();
        runtime.loop_mut().unwrap().add_continue(offset);
    }
}
