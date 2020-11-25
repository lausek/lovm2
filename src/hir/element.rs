//! sum type for every HIR element

use crate::hir::assign::Assign;
use crate::hir::branch::Branch;
use crate::hir::call::Call;
use crate::hir::include::Include;
use crate::hir::interrupt::Interrupt;
use crate::hir::lowering::{HirLowering, HirLoweringRuntime};
use crate::hir::r#return::Return;
use crate::hir::repeat::{Break, Continue, Repeat};

#[derive(Clone)]
pub enum HirElement {
    Assign(Assign),
    Branch(Branch),
    Break(Break),
    Call(Call),
    Continue(Continue),
    Include(Include),
    Interrupt(Interrupt),
    Repeat(Repeat),
    Return(Return),
}

impl HirLowering for HirElement {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        match self {
            HirElement::Assign(assign) => assign.lower(runtime),
            HirElement::Branch(branch) => branch.lower(runtime),
            HirElement::Break(cmd) => cmd.lower(runtime),
            HirElement::Call(call) => call.lower(runtime),
            HirElement::Continue(cmd) => cmd.lower(runtime),
            HirElement::Include(include) => include.lower(runtime),
            HirElement::Interrupt(interrupt) => interrupt.lower(runtime),
            HirElement::Repeat(repeat) => repeat.lower(runtime),
            HirElement::Return(ret) => ret.lower(runtime),
        }
    }
}

impl From<Assign> for HirElement {
    fn from(assign: Assign) -> Self {
        HirElement::Assign(assign)
    }
}

impl From<Branch> for HirElement {
    fn from(branch: Branch) -> Self {
        HirElement::Branch(branch)
    }
}

impl From<Break> for HirElement {
    fn from(cmd: Break) -> Self {
        HirElement::Break(cmd)
    }
}

impl From<Call> for HirElement {
    fn from(call: Call) -> Self {
        HirElement::Call(call)
    }
}

impl From<Continue> for HirElement {
    fn from(cmd: Continue) -> Self {
        HirElement::Continue(cmd)
    }
}

impl From<Include> for HirElement {
    fn from(include: Include) -> Self {
        HirElement::Include(include)
    }
}

impl From<Interrupt> for HirElement {
    fn from(interrupt: Interrupt) -> Self {
        HirElement::Interrupt(interrupt)
    }
}

impl From<Repeat> for HirElement {
    fn from(repeat: Repeat) -> Self {
        HirElement::Repeat(repeat)
    }
}

impl From<&mut Repeat> for HirElement {
    fn from(repeat: &mut Repeat) -> Self {
        HirElement::Repeat(repeat.clone())
    }
}

impl From<Return> for HirElement {
    fn from(ret: Return) -> Self {
        HirElement::Return(ret)
    }
}
