//! Runs a [LV2Block] forever or until a condition is met

use super::*;

#[derive(Clone)]
pub enum LV2RepeatType {
    Endless,
    Iterating {
        collection: LV2Expr,
        item: LV2Variable,
    },
    Until {
        condition: LV2Expr,
    },
}

/// Runs a [LV2Block] forever or until a condition is met
#[derive(Clone)]
pub struct LV2Repeat {
    block: LV2Block,
    ty: LV2RepeatType,
}

impl LV2Repeat {
    pub fn endless() -> Self {
        Self {
            block: LV2Block::new(),
            ty: LV2RepeatType::Endless,
        }
    }

    pub fn until(condition: LV2Expr) -> Self {
        Self {
            block: LV2Block::new(),
            ty: LV2RepeatType::Until { condition },
        }
    }

    pub fn iterating<U, T>(collection: U, item: T) -> Self
    where
        U: Into<LV2Expr>,
        T: Into<LV2Variable>,
    {
        Self {
            block: LV2Block::new(),
            ty: LV2RepeatType::Iterating {
                collection: collection.into(),
                item: item.into(),
            },
        }
    }
}

impl LV2AddStatements for LV2Repeat {
    #[inline]
    fn block_mut(&mut self) -> &mut LV2Block {
        &mut self.block
    }
}

impl LV2HirLowering for LV2Repeat {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut LV2HirLoweringRuntime<'lir>) {
        runtime.push_loop();

        match &self.ty {
            LV2RepeatType::Endless => endless_lower(runtime, &self.block),
            LV2RepeatType::Until { condition } => until_lower(runtime, condition, &self.block),
            LV2RepeatType::Iterating { collection, item } => {
                iterating_lower(runtime, collection, item, &self.block)
            }
        }

        runtime.pop_loop().unwrap();
    }
}

fn endless_lower<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    block: &'hir LV2Block,
) {
    prelude(runtime);
    postlude(runtime, block);
}

fn iterating_lower<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    collection: &'hir LV2Expr,
    item: &'hir LV2Variable,
    block: &'hir LV2Block,
) {
    collection.lower(runtime);
    runtime.emit(LirElement::IterCreate);

    prelude(runtime);

    let repeat_end = runtime.loop_mut().unwrap().end();

    runtime.emit(LirElement::Duplicate);
    runtime.emit(LirElement::IterHasNext);

    // break loop if iterator does not have another item
    runtime.emit(LirElement::jump_conditional(false, repeat_end));

    runtime.emit(LirElement::Duplicate);
    runtime.emit(LirElement::IterNext);
    runtime.emit(LirElement::store(&item));

    postlude(runtime, block);

    runtime.emit(LirElement::Drop);
}

fn until_lower<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    condition: &'hir LV2Expr,
    block: &'hir LV2Block,
) {
    prelude(runtime);

    let repeat_end = runtime.loop_mut().unwrap().end();

    condition.lower(runtime);
    // if the condition is met, jump to end of repeat
    // which is equal to a break. the instruction will
    // receive its final address once the block has been
    // lowered.
    runtime.emit(LirElement::jump_conditional(true, repeat_end));

    postlude(runtime, block);
}

fn prelude(runtime: &mut LV2HirLoweringRuntime) {
    let repeat_start = runtime.loop_mut().unwrap().start();

    runtime.emit(LirElement::Label(repeat_start));
}

fn postlude<'lir, 'hir: 'lir>(runtime: &mut LV2HirLoweringRuntime<'lir>, block: &'hir LV2Block) {
    let repeat = runtime.loop_mut().unwrap();
    let repeat_start = repeat.start();
    let repeat_end = repeat.end();

    block.lower(runtime);
    runtime.emit(LirElement::jump(repeat_start));

    runtime.emit(LirElement::Label(repeat_end));
}
