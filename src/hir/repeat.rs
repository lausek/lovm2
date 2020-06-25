use crate::bytecode::Instruction;
use crate::hir::block::Block;
use crate::hir::element::HIRElement;
use crate::hir::expr::Expr;
use crate::hir::lowering::{patch_addrs, Lowering, LoweringRuntime};

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
        self.block.push_inplace(hir.into());
        self
    }
}

impl Lowering for Repeat {
    fn lower(self, runtime: &mut LoweringRuntime) {
        runtime.push_loop();

        match self.condition {
            RepeatKind::Until(expr) => {
                expr.lower(runtime);

                // if the condition is met, jump to end of repeat
                // which is equal to a break. the instruction will
                // receive its final address once the block has been
                // lowered.
                runtime.emit(Instruction::Jt(std::u16::MAX));
                let offset = runtime.offset();
                runtime.loop_mut().unwrap().add_break(offset);
            }
            RepeatKind::Endless => {}
        }

        self.block.lower(runtime);

        // add a jump to the start of the loop. this is equal to
        // a continue statement.
        Continue::new().lower(runtime);

        let lowering_loop = runtime.pop_loop().unwrap();
        patch_addrs(runtime, &lowering_loop.breaks, lowering_loop.end.unwrap());
        patch_addrs(runtime, &lowering_loop.continues, lowering_loop.start);
    }
}

pub struct Break {}

impl Lowering for Break {
    fn lower(self, runtime: &mut LoweringRuntime) {
        runtime.emit(Instruction::Jmp(std::u16::MAX));
        let offset = runtime.offset();
        runtime.loop_mut().unwrap().add_break(offset);
    }
}

impl Break {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Continue {}

impl Continue {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lowering for Continue {
    fn lower(self, runtime: &mut LoweringRuntime) {
        runtime.emit(Instruction::Jmp(std::u16::MAX));
        let offset = runtime.offset();
        runtime.loop_mut().unwrap().add_continue(offset);
    }
}
