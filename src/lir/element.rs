use crate::hir::assign::Access;
use crate::value::Value;
use crate::var::Variable;

use super::{Label, Operator, Scope};

#[derive(Clone, Debug)]
pub enum LirElement {
    Call {
        argn: u8,
        ident: Variable,
    },
    Cast {
        tyid: u16,
    },
    Entry {
        ident: Variable,
    },
    // Jmp(u16), Jt(u16), Jf(u16)
    Jump {
        condition: Option<bool>,
        label: Label,
    },
    Label(Label),
    // Add, Sub, Mul, Div, Pow, Rem, And, Or, Not, Eq, Ne, Ge, Gt, Le, Lt
    Operation(Operator),
    // Pushc(u16)
    PushConstant {
        value: Value,
    },
    // Pushl(u16), Pushg(u16),
    PushDynamic {
        ident: Variable,
        scope: Scope,
    },
    // Movel(u16), Moveg(u16)
    StoreDynamic {
        ident: Variable,
        scope: Scope,
    },

    Box,
    Discard,
    Duplicate,
    Get,
    Getr,
    Interrupt(u16),
    Load,
    Ret,
    Set,
    Slice,
}

impl LirElement {
    pub fn call(argn: u8, ident: Variable) -> Self {
        Self::Call { argn, ident }
    }

    pub fn cast(tyid: u16) -> Self {
        Self::Cast { tyid }
    }

    pub fn entry(ident: Variable) -> Self {
        Self::Entry { ident }
    }

    pub fn jump(label: Label) -> Self {
        Self::Jump {
            condition: None,
            label,
        }
    }

    pub fn jump_conditional(cond: bool, label: Label) -> Self {
        Self::Jump {
            condition: Some(cond),
            label,
        }
    }

    pub fn push_constant(value: Value) -> Self {
        Self::PushConstant { value }
    }

    pub fn push_dynamic(scope: Scope, ident: Variable) -> Self {
        Self::PushDynamic { ident, scope }
    }

    pub fn operation<T>(op: T) -> Self
    where
        T: Into<Operator>,
    {
        Self::Operation(op.into())
    }

    pub fn store(scope: Scope, ident: Variable) -> Self {
        Self::StoreDynamic { ident, scope }
    }
}
