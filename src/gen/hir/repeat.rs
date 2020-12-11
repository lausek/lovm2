//! runs a `Block` forever or until a condition is met

use super::*;

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
}

impl HasBlock for Repeat {
    #[inline]
    fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}

impl HirLowering for Repeat {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        let repeat = runtime.push_loop();
        let repeat_start = repeat.start();
        let repeat_end = repeat.end();

        runtime.emit(LirElement::Label(repeat_start));

        match self.condition {
            RepeatKind::Until(expr) => {
                expr.lower(runtime);

                // if the condition is met, jump to end of repeat
                // which is equal to a break. the instruction will
                // receive its final address once the block has been
                // lowered.
                runtime.emit(LirElement::jump_conditional(true, repeat_end.clone()));
            }
            RepeatKind::Endless => {}
        }

        self.block.lower(runtime);

        // add a jump to the start of the loop. this is equal to
        // a continue statement.
        Continue::new().lower(runtime);

        runtime.emit(LirElement::Label(repeat_end));

        runtime.pop_loop().unwrap();
    }
}

#[derive(Clone)]
pub struct Break {}

impl HirLowering for Break {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        let repeat_end = runtime.loop_mut().unwrap().end();
        runtime.emit(LirElement::jump(repeat_end));
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
        let repeat_start = runtime.loop_mut().unwrap().start();
        runtime.emit(LirElement::jump(repeat_start));
    }
}
