use serde::{Deserialize, Serialize};

/// Definition of the bytecode
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Instruction {
    /// push local variable
    LPush(u16),
    /// push global variable
    GPush(u16),
    /// push constant
    CPush(u16),
    /// store into local variable
    LMove(u16),
    /// store into global variable
    GMove(u16),
    /// drops the value on top of stack
    Drop,
    /// duplicates top of stack
    Dup,

    /// get(obj, key): get key from object and push it
    Get,
    /// rget(obj, key): get key from object by reference and push it
    RGet,
    /// set(ref, val): write value into a value reference
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

    /// logical shift left `first` by `second` places
    Shl,
    /// logical shift right `first` by `second` places
    Shr,

    /// logical and for `Bool`, bitwise and for `Int`
    And,
    /// logical or for `Bool`, bitwise or for `Int`
    Or,
    /// logical xor for `Bool`, bitwise xor for `Int`
    XOr,
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

    /// call function with `ident index`, `argn`
    Call(u16, u8),
    /// call a function in the same module
    LCall(u16, u8),
    /// return early from a code object
    Ret,

    /// trigger interrupt `n`
    Interrupt(u16),

    /// convert top of stack into type. see `Value::type_id`
    Conv(u16),
    /// take top of stack as name of module to load and import functions without module prefix as
    /// well
    Import,
    /// take top of stack as name of module to import. function name will be padded using the
    /// import hook
    NImport,
    /// turn the value on stack into a referenceable value.
    /// lists and dicts are boxed deeply
    Box,

    /// create a new list from the first argument on stack.
    /// second is starting index or nil, third is end index (exclusive) or nil
    Slice,

    IterCreate,
    IterCreateRanged,
    IterHasNext,
    IterNext,
    IterReverse,
}
