//! Transform Hir into actual bytecode

mod branch;
mod repeat;
mod runtime;

use super::*;

pub(crate) use self::branch::HirLoweringBranch;
pub use self::repeat::HirLoweringRepeat;
pub use self::runtime::LV2HirLoweringRuntime;

pub(crate) type LabelCounterRef = std::rc::Rc<std::cell::RefCell<LabelCounter>>;

/// Structures supporting transformation into LIR
pub trait LV2HirLowering {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut LV2HirLoweringRuntime<'lir>);
}

/// Structures supporting custom jump targets
pub trait LV2HirJumpable {
    fn new(_: LabelCounterRef) -> Self;

    fn end(&self) -> Label;

    fn start(&self) -> Label;
}

pub struct LabelCounter {
    branch: usize,
    condition: usize,
    repeat: usize,
    other: usize,
}

impl LabelCounter {
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

    pub fn create_new_label(&mut self) -> Label {
        let id = self.other;
        self.other += 1;
        Label::from(format!("_{}", id))
    }

    pub fn create_repeat_id(&mut self) -> usize {
        let id = self.repeat;
        self.repeat += 1;
        id
    }
}

impl std::default::Default for LabelCounter {
    fn default() -> Self {
        Self {
            branch: 0,
            condition: 0,
            repeat: 0,
            other: 0,
        }
    }
}
