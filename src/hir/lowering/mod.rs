pub mod branch;
pub mod repeat;
pub mod runtime;

use crate::bytecode::Instruction;
use crate::code::{CodeObject, CodeObjectBuilder};
use crate::hir::{HIRElement, HIR};
use crate::value::CoValue;
use crate::var::Variable;

pub use branch::{LoweringBranch, LoweringCondition};
pub use repeat::LoweringLoop;
pub use runtime::LoweringRuntime;

pub trait Lowering {
    fn lower(self, runtime: &mut LoweringRuntime);
}

