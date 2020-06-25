use crate::hir::block::Block;
use crate::hir::element::HIRElement;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};

pub enum RepeatKind {
    Until(Expr),
    Endless,
}

pub struct Repeat {
    block: Block,
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

    pub fn push<T>(mut self, hir: T) -> Self
    where
        T: Into<HIRElement>,
    {
        self.block.push(hir.into());
        self
    }
}

impl Lowering for Repeat {
    fn lower(self, runtime: &mut LoweringRuntime) {
        runtime.push_loop();

        self.condition.lower(runtime);

        self.block.lower(runtime);

        let lowering_loop = runtime.pop_loop().unwrap();
    }
}

pub struct Break {}

impl Lowering for Break {
    fn lower(self, _runtime: &mut LoweringRuntime) {}
}

pub struct Continue {}

impl Lowering for Continue {
    fn lower(self, _runtime: &mut LoweringRuntime) {}
}
