//! transform HIR into actual bytecode

pub mod branch;
pub mod repeat;
pub mod runtime;

use crate::bytecode::Instruction;
use crate::lir::Label;

pub use self::branch::{HirLoweringBranch, HirLoweringCondition};
pub use self::repeat::HirLoweringRepeat;
pub use self::runtime::HirLoweringRuntime;

pub type LabelCounterRef = std::rc::Rc<std::cell::RefCell<LabelCounter>>;

pub trait HirLowering {
    fn lower(self, runtime: &mut HirLoweringRuntime);
}

pub trait Jumpable {
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
        Label::Custom(format!("_{}", id))
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
