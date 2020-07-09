use crate::hir::assign::Assign;
use crate::hir::branch::Branch;
use crate::hir::call::Call;
use crate::hir::include::Include;
use crate::hir::interrupt::Interrupt;
use crate::hir::lowering::{Lowering, LoweringRuntime};
use crate::hir::r#return::Return;
use crate::hir::repeat::{Break, Continue, Repeat};

#[derive(Clone)]
pub enum HIRElement {
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

impl Lowering for HIRElement {
    fn lower(self, runtime: &mut LoweringRuntime) {
        match self {
            HIRElement::Assign(assign) => assign.lower(runtime),
            HIRElement::Branch(branch) => branch.lower(runtime),
            HIRElement::Break(cmd) => cmd.lower(runtime),
            HIRElement::Call(call) => call.lower(runtime),
            HIRElement::Continue(cmd) => cmd.lower(runtime),
            HIRElement::Include(include) => include.lower(runtime),
            HIRElement::Interrupt(interrupt) => interrupt.lower(runtime),
            HIRElement::Repeat(repeat) => repeat.lower(runtime),
            HIRElement::Return(ret) => ret.lower(runtime),
        }
    }
}

impl From<Assign> for HIRElement {
    fn from(assign: Assign) -> Self {
        HIRElement::Assign(assign)
    }
}

impl From<Branch> for HIRElement {
    fn from(branch: Branch) -> Self {
        HIRElement::Branch(branch)
    }
}

impl From<Break> for HIRElement {
    fn from(cmd: Break) -> Self {
        HIRElement::Break(cmd)
    }
}

impl From<Call> for HIRElement {
    fn from(call: Call) -> Self {
        HIRElement::Call(call)
    }
}

impl From<Continue> for HIRElement {
    fn from(cmd: Continue) -> Self {
        HIRElement::Continue(cmd)
    }
}

impl From<Include> for HIRElement {
    fn from(include: Include) -> Self {
        HIRElement::Include(include)
    }
}

impl From<Interrupt> for HIRElement {
    fn from(interrupt: Interrupt) -> Self {
        HIRElement::Interrupt(interrupt)
    }
}

impl From<Repeat> for HIRElement {
    fn from(repeat: Repeat) -> Self {
        HIRElement::Repeat(repeat)
    }
}

impl From<&mut Repeat> for HIRElement {
    fn from(repeat: &mut Repeat) -> Self {
        HIRElement::Repeat(repeat.clone())
    }
}

impl From<Return> for HIRElement {
    fn from(ret: Return) -> Self {
        HIRElement::Return(ret)
    }
}
