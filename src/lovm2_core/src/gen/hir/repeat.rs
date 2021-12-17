//! Runs a `Block` forever or until a condition is met

use super::*;

/// Runs a [Block] forever or until a condition is met
#[derive(Clone)]
pub enum Repeat {
    Endless {
        block: Block,
    },
    Iterating {
        collection: Expr,
        item: Variable,
        block: Block,
    },
    Until {
        condition: Expr,
        block: Block,
    },
}

impl Repeat {
    pub fn endless() -> Self {
        Self::Endless {
            block: Block::new(),
        }
    }

    pub fn until(condition: Expr) -> Self {
        Self::Until {
            block: Block::new(),
            condition,
        }
    }

    pub fn iterating<U, T>(collection: U, item: T) -> Self
    where
        U: Into<Expr>,
        T: Into<Variable>,
    {
        Self::Iterating {
            block: Block::new(),
            collection: collection.into(),
            item: item.into(),
        }
    }
}

impl HasBlock for Repeat {
    #[inline]
    fn block_mut(&mut self) -> &mut Block {
        match self {
            Self::Until { block, .. } => block,
            Self::Endless { block, .. } => block,
            Self::Iterating { block, .. } => block,
        }
    }
}

impl HirLowering for Repeat {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>)
    {
        runtime.push_loop();

        match self {
            Self::Endless { block } => endless_lower(runtime, block),
            Self::Until { condition, block } => until_lower(runtime, condition, block),
            Self::Iterating {
                collection,
                item,
                block,
            } => iterating_lower(runtime, collection, item, block),
        }

        runtime.pop_loop().unwrap();
    }
}

fn endless_lower<'lir, 'hir: 'lir>(runtime: &mut HirLoweringRuntime<'lir>, block: &'hir Block)
{
    prelude(runtime);
    postlude(runtime, block);
}

fn iterating_lower<'lir, 'hir: 'lir>(
    runtime: &mut HirLoweringRuntime<'lir>,
    collection: &'hir Expr,
    item: &'hir Variable,
    block: &'hir Block,
)
{
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
    runtime: &mut HirLoweringRuntime<'lir>,
    condition: &'hir Expr,
    block: &'hir Block,
)
{
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

fn prelude(runtime: &mut HirLoweringRuntime) {
    let repeat_start = runtime.loop_mut().unwrap().start();

    runtime.emit(LirElement::Label(repeat_start));
}

fn postlude<'lir, 'hir: 'lir>(runtime: &mut HirLoweringRuntime<'lir>, block: &'hir Block)
{
    let repeat = runtime.loop_mut().unwrap();
    let repeat_start = repeat.start();
    let repeat_end = repeat.end();

    block.lower(runtime);
    runtime.emit(LirElement::jump(repeat_start));

    runtime.emit(LirElement::Label(repeat_end));
}