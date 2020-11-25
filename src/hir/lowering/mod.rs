//! transform HIR into actual bytecode

pub mod branch;
pub mod repeat;
pub mod runtime;

use crate::bytecode::Instruction;

pub use branch::{HirLoweringBranch, HirLoweringCondition};
pub use repeat::HirLoweringLoop;
pub use runtime::HirLoweringRuntime;

pub trait HirLowering {
    fn lower(self, runtime: &mut HirLoweringRuntime);
}
