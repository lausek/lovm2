#[repr(u8)]
pub enum Instruction {
    // push local variable
    Pushl = 1,
    // push global variable
    Pushg,
    // push constant
    Pushc,
    // store into local variable
    Movel,
    // store into global variable
    Moveg,
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
            16 => Instruction::Jmp,
            17 => Instruction::Jeq,
            18 => Instruction::Jgt,
            19 => Instruction::Jlt,
            20 => Instruction::Call,
            21 => Instruction::Ret,
            22 => Instruction::Interrupt,
            _ => return None,
        };
        Some(inx)
    }
}
