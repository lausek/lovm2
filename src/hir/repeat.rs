use crate::bytecode::Instruction;
use crate::hir::block::Block;
use crate::hir::element::HIRElement;
use crate::hir::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};

fn patch_addr(runtime: &mut LoweringRuntime, positions: &Vec<usize>, addr: usize) {
    let addr = addr as u16;
    for pos in positions.iter() {
        if let Some(inx) = runtime.code.get_mut(*pos) {
            let unaddr = match inx {
                Instruction::Jmp(ref mut unaddr) => unaddr,
                Instruction::Jt(ref mut unaddr) => unaddr,
                Instruction::Jf(ref mut unaddr) => unaddr,
                _ => unimplemented!(),
            };
            if *unaddr != std::u16::MAX {
                panic!("address is already initialized");
            }
            *unaddr = addr;
        } else {
            unreachable!();
        }
    }
}

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
        runtime.emit(Instruction::Jmp(std::u16::MAX));
        let offset = runtime.offset();
        runtime.loop_mut().unwrap().add_continue(offset);

        let lowering_loop = runtime.pop_loop().unwrap();

        patch_addr(runtime, &lowering_loop.breaks, lowering_loop.end.unwrap());
        patch_addr(runtime, &lowering_loop.continues, lowering_loop.start);
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
