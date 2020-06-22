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
    Cmp,

    Jmp(u16),
    Jeq(u16),
    Jgt(u16),
    Jlt(u16),

    Call(u8, u16),
    Ret,

    Interrupt(u16),
}
