use crate::hir::assign::Access;
use crate::value::Value;
use crate::var::Variable;

use super::{Label, Operator, Scope};

#[derive(Clone, Debug)]
pub enum LirElement {
    Access(Access),
    Call {
        argn: u8,
        ident: Variable,
    },
    Cast {
        tyid: u16,
    },
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
    // Add, Sub, Mul, Div, Pow, Rem, And, Or, Not, Eq, Ne, Ge, Gt, Le, Lt
    Operation(Operator),
    // Jmp(u16), Jt(u16), Jf(u16)
    Jump {
        condition: Option<bool>,
        target: Label,
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
    pub fn jump_absolute(target: Label) -> Self {
        Self::Jump {
            condition: None,
            target,
        }
    }
}
