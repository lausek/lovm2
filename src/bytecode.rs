use serde::{Deserialize, Serialize};

// TODO: this serializes the variant as string. not good
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Instruction {
    // push local variable
    Pushl(u16),
    // push global variable
    Pushg(u16),
    // push constant
    Pushc(u16),
    // store into local variable
    Movel(u16),
    // store into global variable
    Moveg(u16),
    Discard,
    Dup,
    Swap,

    Add,
    Sub,
    Mul,
    Div,
    Rem,

    And,
    Or,
    Not,

    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,

    Jmp(u16),
    Jt(u16),
    Jf(u16),

    // call function with `argn`, `global index`
    Call(u8, u16),
    Ret,

    Interrupt(u16),

    Cast(u16),
    Load,
}
