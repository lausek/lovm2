use crate::bytecode::Instruction;
use crate::code::CodeObject;
use crate::context::Context;

const CONST_WIDTH: usize = 2;
const LOCAL_WIDTH: usize = 2;
const GLOBAL_WIDTH: usize = 2;

fn take_local(it: &mut dyn Iterator<Item = &u8>) -> usize {
    let mut idx = 0usize;
    for byte in it.take(LOCAL_WIDTH) {
        idx <<= 8;
        idx += *byte as usize;
    }
    idx
}

struct Vm {
    ctx: Context,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
        }
    }

    pub fn run_object(&mut self, object: &CodeObject) {
        let mut it = object.code.iter();
        while let Some(code) = it.next() {
            if let Some(inx) = Instruction::from(*code) {
                match inx {
                    Instruction::Pushl => {
                        let lidx = take_local(&mut it);
                        let variable = &object.locals[lidx];
                        let local = self.ctx.frame_mut().unwrap().locals.get(variable).cloned();
                        self.ctx.push_value(local.unwrap());
                    }
                    Instruction::Pushg => {}
                    Instruction::Pushc => {}
                    Instruction::Dup => {}
                    Instruction::Swap => {}
                    Instruction::Add => {}
                    Instruction::Sub => {}
                    Instruction::Mul => {}
                    Instruction::Div => {}
                    Instruction::Mod => {}
                    Instruction::And => {}
                    Instruction::Or => {}
                    Instruction::Not => {}
                    Instruction::Jmp => {}
                    Instruction::Jeq => {}
                    Instruction::Jgt => {}
                    Instruction::Jlt => {}
                    Instruction::Call => {}
                    Instruction::Ret => {}
                    Instruction::Interrupt => {}
                }
            } else {
                unimplemented!();
            }
        }
    }

    pub fn run(&mut self) {
        for module in self.ctx.modules.iter() {
        }
    }
}
