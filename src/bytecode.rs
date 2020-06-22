#[repr(u8)]
pub enum Instruction {
    // push local variable
    Pushl,
    // push global variable
    Pushg,
    // push constant
    Pushc,
    Dup,
    Swap,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    And,
    Or,
    Not,

    Jmp,
    Jeq,
    Jgt,
    Jlt,

    Call,
    Ret,

    Interrupt,
}
