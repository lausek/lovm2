//! Sum type for every LIR element

use std::borrow::Cow;

use crate::value::{Value, ValueType};
use crate::var::Variable;

use super::{Label, Operator};

/// Sum type for every LIR element.
#[derive(Clone, Debug)]
pub enum LirElement<'hir> {
    Call {
        argn: u8,
        ident: &'hir Variable,
    },
    Conv {
        ty: ValueType,
    },
    Entry {
        ident: &'hir Variable,
    },
    // Jmp(u16), Jt(u16), Jf(u16)
    Jump {
        condition: Option<bool>,
        label: Label,
    },
    Label(Label),
    // Add, Sub, Mul, Div, Pow, Rem, And, Or, Not, Eq, Ne, Ge, Gt, Le, Lt
    Operation(Operator),
    // CPush(u16)
    PushConstant {
        value: Cow<'hir, Value>,
    },
    ScopeGlobal {
        ident: &'hir Variable,
    },
    ScopeLocal {
        ident: &'hir Variable,
    },
    // LPush(u16), GPush(u16),
    PushDynamic {
        ident: &'hir Variable,
    },
    // LMove(u16), GMove(u16)
    StoreDynamic {
        ident: &'hir Variable,
    },
    Import {
        namespaced: bool,
    },

    Box,
    Drop,
    Duplicate,
    Get,
    RGet,
    Interrupt { n: u16 },
    Ret,
    Set,
    Slice,

    IterCreate,
    IterCreateRanged,
    IterHasNext,
    IterNext,
    IterReverse,
}

impl<'hir> LirElement<'hir> {
    pub fn call(ident: &'hir Variable, argn: u8) -> Self {
        Self::Call { argn, ident }
    }

    pub fn entry(ident: &'hir Variable) -> Self {
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

    pub fn push_constant(value: &'hir Value) -> Self {
        Self::PushConstant {
            value: Cow::Borrowed(value),
        }
    }

    pub fn push_constant_owned(value: Value) -> Self {
        Self::PushConstant {
            value: Cow::Owned(value),
        }
    }

    pub fn push_dynamic(ident: &'hir Variable) -> Self {
        Self::PushDynamic { ident }
    }

    pub fn operation<T>(op: T) -> Self
    where
        T: Into<Operator>,
    {
        Self::Operation(op.into())
    }

    pub fn store(ident: &'hir Variable) -> Self {
        Self::StoreDynamic { ident }
    }
}

impl std::fmt::Display for LirElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Call { argn, ident } => write!(f, "\tCall({}, {})", ident, argn),
            Self::Conv { ty } => write!(f, "\tConv({:?})", ty),
            Self::Entry { ident } => write!(f, "{}:", ident),
            Self::Jump { condition, label } => match condition {
                Some(true) => write!(f, "\tJumpIfTrue(.{})", label),
                Some(false) => write!(f, "\tJumpIfFalse(.{})", label),
                _ => write!(f, "\tJump(.{})", label),
            },
            Self::Label(label) => write!(f, ".{}:", label),
            Self::Operation(operator) => write!(f, "\t{:?}", operator),
            Self::PushConstant { value } => write!(f, "\tCPush({})", value),
            Self::PushDynamic { ident } => write!(f, "\tPush({})", ident),
            Self::StoreDynamic { ident } => write!(f, "\tStore({})", ident),
            Self::Interrupt {n} => write!(f, "\tInterrupt({})", n),
            _ => write!(f, "\t{:?}", self),
        }
    }
}
