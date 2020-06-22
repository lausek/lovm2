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

/*
impl Instruction {
    pub fn from(code: u8) -> Option<Self> {
        let inx = match code {
            1 => Instruction::Pushl,
            2 => Instruction::Pushg,
            3 => Instruction::Pushc,
            4 => Instruction::Movel,
            5 => Instruction::Moveg,
            6 => Instruction::Dup,
            7 => Instruction::Swap,
            8 => Instruction::Add,
            9 => Instruction::Sub,
            10 => Instruction::Mul,
            11 => Instruction::Div,
            12 => Instruction::Mod,
            13 => Instruction::And,
            14 => Instruction::Or,
            15 => Instruction::Not,
            16 => Instruction::Cmp,
            17 => Instruction::Jmp,
            18 => Instruction::Jeq,
            19 => Instruction::Jgt,
            20 => Instruction::Jlt,
            21 => Instruction::Call,
            22 => Instruction::Ret,
            23 => Instruction::Interrupt,
            _ => return None,
        };
        Some(inx)
    }
}
*/
