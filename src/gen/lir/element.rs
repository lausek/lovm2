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

impl std::fmt::Display for LirElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Call { argn, ident } => write!(f, "\tCall({}, {})", argn, ident),
            Self::Cast { tyid } => write!(f, "\tCast({})", tyid),
            Self::Entry { ident } => write!(f, "{}:", ident),
            Self::Jump { condition, label } => write!(f, "\tJump({:?}, {})", condition, label),
            Self::Label(label) => write!(f, "{}:", label),
            Self::Operation(operator) => write!(f, "\t{:?}", operator),
            Self::PushConstant { value } => write!(f, "\tPushc({})", value),
            Self::PushDynamic { ident, scope } => write!(f, "\tPush({:?}, {})", scope, ident),
            Self::StoreDynamic { ident, scope } => write!(f, "\tStore({:?}, {})", scope, ident),
            Self::Interrupt(n) => write!(f, "\tInterrupt({})", n),
            _ => write!(f, "\t{:?}", self),
        }
    }
}