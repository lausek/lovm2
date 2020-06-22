use crate::bytecode::Instruction;
use crate::code::CodeObject;
use crate::context::Context;

const CONST_WIDTH: usize = 2;
const LOCAL_WIDTH: usize = 2;
const GLOBAL_WIDTH: usize = 2;

fn take_bytes(it: &mut dyn Iterator<Item = &u8>, n: usize) -> usize {
    let mut idx = 0usize;
    for byte in it.take(n) {
        idx <<= 8;
        idx += *byte as usize;
    }
    idx
}

pub struct Vm {
    ctx: Context,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
        }
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    pub fn run_object(&mut self, object: &CodeObject) {
        let mut it = object.code.iter();
        while let Some(code) = it.next() {
            println!("{}", code);
            if let Some(inx) = Instruction::from(*code) {
                match inx {
                    Instruction::Pushl => {
                        let lidx = take_bytes(&mut it, LOCAL_WIDTH);
                        let variable = &object.locals[lidx];
                        let local = self.ctx.frame_mut().unwrap().locals.get(variable).cloned();
                        self.ctx.push_value(local.unwrap());
                    }
                    Instruction::Pushg => {}
                    Instruction::Pushc => {
                        use crate::value;
                        let cidx = take_bytes(&mut it, CONST_WIDTH);
                        let value = value::instantiate(&mut self.ctx, &object.consts[cidx]);
                        self.ctx.push_value(value);
                    }
                    Instruction::Movel => {},
                    Instruction::Moveg => {
                        let gidx = take_bytes(&mut it, GLOBAL_WIDTH);
                        let variable = &object.globals[gidx];
                        let value = self.ctx.pop_value().unwrap();
                        self.ctx.globals.insert(variable.clone(), value);
                    },
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
