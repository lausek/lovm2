use serde::{Deserialize, Serialize};

/// Definition of the bytecode
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Instruction {
    /// Push local variable.
    LPush(u16),
    /// Push global variable.
    GPush(u16),
    /// Push constant.
    CPush(u16),
    /// Store into local variable.
    LMove(u16),
    /// Store into global variable.
    GMove(u16),
    /// Drops the value on top of stack.
    Drop,
    /// Duplicates top of stack.
    Dup,

    /// Get first argument as key from second argument and push it.
    Get,
    /// Get first argument as key from second argument by reference and push it.
    RGet,
    /// Write second argument into first argument which must be a reference.
    Set,
    /// Append second argument to first argument
    Append,

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

    /// Logical shift left `first` by `second` places.
    Shl,
    /// Logical shift right `first` by `second` places.
    Shr,

    /// Logical and for `Bool`, Bitwise and for `Int`.
    And,
    /// Logical or for `Bool`, Bitwise or for `Int`.
    Or,
    /// Logical xor for `Bool`, Bitwise xor for `Int`.
    XOr,
    /// Logical not for `Bool`, Bitwise not for `Int`.
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

    /// Jump to instruction offset.
    Jmp(u16),
    /// Jump to instruction offset if top of stack is true.
    Jt(u16),
    /// Jump to instruction offset if top of stack is false.
    Jf(u16),

    /// Call function with `ident index`, `argn`.
    Call(u16, u8),
    /// Call a function in the same module.
    LCall(u16, u8),
    /// Return early from a code object.
    Ret,

    /// Trigger interrupt `n`.
    Interrupt(u16),

    /// Convert top of stack into type. See `Value::type_id`.
    Conv(u16),
    /// Take top of stack as name of module to load and import functions without module prefix as
    /// well.
    Import,
    /// Take top of stack as name of module to import. Function name will be padded using the
    /// import hook.
    NImport,
    /// Turn the value on stack into a referenceable value.
    /// Lists and dicts are boxed deeply.
    Box,

    /// Create a new list from the first argument on stack.
    /// Second is starting index or nil, third is end index (exclusive) or nil.
    Slice,

    /// Create a new iterator from the first argument on stack.
    IterCreate,
    /// Create a new ranged iterator using the first argument as `from` and second argument as `to`.
    /// on of the arguments is allowed to be nil.
    IterCreateRanged,
    /// Consumes the iterator on top of stack and leaves a bool on top if the iterator has another
    /// element.
    IterHasNext,
    /// Consumes the iterator on top of stack and returns the next value if any.
    IterNext,
    /// Consumes the iterator on top of stack and create a new one in reverse.
    IterReverse,
}
