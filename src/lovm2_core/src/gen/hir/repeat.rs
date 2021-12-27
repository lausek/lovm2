//! Runs a [LV2Block] forever or until a condition is met.

use super::*;

/// Conditional/Unconditional repeat.
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

/// Runs a [LV2Block] forever or until a condition is met.
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
        lv2_lower_repeat(runtime, &self.ty, &self.block);
    }
}

/// Lowering for [LV2Repeat].
pub fn lv2_lower_repeat<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    ty: &'hir LV2RepeatType,
    block: &'hir LV2Block,
) {
    runtime.push_loop();

    match &ty {
        LV2RepeatType::Endless => lower_repeat_endless(runtime, block),
        LV2RepeatType::Until { condition } => lower_repeat_until(runtime, condition, block),
        LV2RepeatType::Iterating { collection, item } => {
            lower_repeat_iterating(runtime, collection, item, block)
        }
    }

    runtime.pop_loop().unwrap();
}

fn lower_repeat_endless<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    block: &'hir LV2Block,
) {
    lower_repeat_prelude(runtime);
    lower_repeat_postlude(runtime, block);
}

fn lower_repeat_iterating<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    collection: &'hir LV2Expr,
    item: &'hir LV2Variable,
    block: &'hir LV2Block,
) {
    collection.lower(runtime);
    runtime.emit(LirElement::IterCreate);

    lower_repeat_prelude(runtime);

    let repeat_end = runtime.loop_mut().unwrap().end();

    runtime.emit(LirElement::Duplicate);
    runtime.emit(LirElement::IterHasNext);

    // break loop if iterator does not have another item
    runtime.emit(LirElement::jump_conditional(false, repeat_end));

    runtime.emit(LirElement::Duplicate);
    runtime.emit(LirElement::IterNext);
    runtime.emit(LirElement::store(item));

    lower_repeat_postlude(runtime, block);

    runtime.emit(LirElement::Drop);
}

fn lower_repeat_until<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    condition: &'hir LV2Expr,
    block: &'hir LV2Block,
) {
    lower_repeat_prelude(runtime);

    let repeat_end = runtime.loop_mut().unwrap().end();

    condition.lower(runtime);
    // if the condition is met, jump to end of repeat
    // which is equal to a break. the instruction will
    // receive its final address once the block has been
    // lowered.
    runtime.emit(LirElement::jump_conditional(true, repeat_end));

    lower_repeat_postlude(runtime, block);
}

fn lower_repeat_prelude(runtime: &mut LV2HirLoweringRuntime) {
    let repeat_start = runtime.loop_mut().unwrap().start();

    runtime.emit(LirElement::Label(repeat_start));
}

fn lower_repeat_postlude<'lir, 'hir: 'lir>(
    runtime: &mut LV2HirLoweringRuntime<'lir>,
    block: &'hir LV2Block,
) {
    let repeat = runtime.loop_mut().unwrap();
    let repeat_start = repeat.start();
    let repeat_end = repeat.end();

    block.lower(runtime);
    runtime.emit(LirElement::jump(repeat_start));

    runtime.emit(LirElement::Label(repeat_end));
}
