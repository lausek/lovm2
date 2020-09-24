//! definition of the bytecode

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Instruction {
    /// push local variable
    Pushl(u16),
    /// push global variable
    Pushg(u16),
    /// push constant
    Pushc(u16),
    /// store into local variable
    Movel(u16),
    /// store into global variable
    Moveg(u16),
    /// drops the value on top of stack
    Discard,
    /// duplicates top of stack
    Dup,
    /// TODO: implement
    Swap,

    /// get(obj, key): get key from object and push it
    Get,
    /// set(obj, key, val): set key on object
    Set,

    /// = first + second
    Add,
    /// = first - second
    Sub,
    /// = first * second
    Mul,
    /// = first / second
    Div,
    /// = first ** second
    Pow,
    /// = first % second
    Rem,

    /// logical and for `Bool`, bitwise and for `Int`
    And,
    /// logical or for `Bool`, bitwise or for `Int`
    Or,
    /// logical not for `Bool`, bitwise not for `Int`
    Not,

    /// = first == second
    Eq,
    /// = first != second
    Ne,
    /// = first >= second
    Ge,
    /// = first > second
    Gt,
    /// = first <= second
    Le,
    /// = first < second
    Lt,

    /// jump to instruction offset
    Jmp(u16),
    /// jump to instruction offset if top of stack is true
    Jt(u16),
    /// jump to instruction offset if top of stack is false
    Jf(u16),

    /// call function with `argn`, `global index`
    Call(u8, u16),
    /// return early from a code object
    Ret,

    /// trigger interrupt `n`
    Interrupt(u16),

    /// convert top of stack into type. see `RuValue::type_id`
    Cast(u16),
    /// take top of stack as name of module to load
    Load,
}
