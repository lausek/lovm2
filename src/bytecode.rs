#[repr(u8)]
pub enum Instruction {
    // push local variable
    Pushl = 1,
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

impl Instruction {
    pub fn from(code: u8) -> Option<Self> {
        let inx = match code {
            1 => Instruction::Pushl,
            2 => Instruction::Pushg,
            3 => Instruction::Pushc,
            4 => Instruction::Dup,
            5 => Instruction::Swap,
            6 => Instruction::Add,
            7 => Instruction::Sub,
            8 => Instruction::Mul,
            9 => Instruction::Div,
            10 => Instruction::Mod,
            11 => Instruction::And,
            12 => Instruction::Or,
            13 => Instruction::Not,
            14 => Instruction::Jmp,
            15 => Instruction::Jeq,
            16 => Instruction::Jgt,
            17 => Instruction::Jlt,
            18 => Instruction::Call,
            19 => Instruction::Ret,
            20 => Instruction::Interrupt,
            _ => return None,
        };
        Some(inx)
    }
}
