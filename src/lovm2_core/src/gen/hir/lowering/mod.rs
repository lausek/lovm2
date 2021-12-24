//! Transform Hir into actual bytecode.

mod branch;
mod repeat;
mod runtime;

use super::*;

pub(crate) use self::branch::HirLoweringBranch;
pub use self::repeat::HirLoweringRepeat;
pub use self::runtime::LV2HirLoweringRuntime;

pub(crate) type LabelCounterRef = std::rc::Rc<std::cell::RefCell<LV2LabelCounter>>;

/// Structures supporting transformation into LIR.
pub trait LV2HirLowering {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut LV2HirLoweringRuntime<'lir>);
}

/// Structures supporting custom jump targets.
pub trait LV2HirJumpable {
    fn new(_: LabelCounterRef) -> Self;

    fn end(&self) -> LV2Label;

    fn start(&self) -> LV2Label;
}

pub struct LV2LabelCounter {
    branch: usize,
    condition: usize,
    repeat: usize,
    other: usize,
}

impl LV2LabelCounter {
    pub fn create_branch_id(&mut self) -> usize {
        let id = self.branch;
        self.branch += 1;
        id
    }

    pub fn create_condition_id(&mut self) -> usize {
        let id = self.condition;
        self.condition += 1;
        id
    }

    pub fn create_new_label(&mut self) -> LV2Label {
        let id = self.other;
        self.other += 1;
        LV2Label {
            id,
            is_start: true,
            ty: LV2LabelTy::Condition,
        }
    }

    pub fn create_repeat_id(&mut self) -> usize {
        let id = self.repeat;
        self.repeat += 1;
        id
    }
}

impl std::default::Default for LV2LabelCounter {
    fn default() -> Self {
        Self {
            branch: 0,
            condition: 0,
            repeat: 0,
            other: 0,
        }
    }
}
