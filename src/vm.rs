use crate::bytecode::Instruction;
use crate::code::CodeObject;
use crate::context::Context;

const CONST_WIDTH: usize = 2;
const LOCAL_WIDTH: usize = 2;
const GLOBAL_WIDTH: usize = 2;

fn take_bytes(code: &Vec<u8>, ip: &mut usize, n: usize) -> usize {
    let mut idx = 0usize;
    for _ in 0..n {
        *ip += 1;
        let byte = code.get(*ip).unwrap();
        idx <<= 8;
        idx |= *byte as usize;
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
        let mut ip = 0;
        loop {
            let code = object.code.get(ip);
            if code.is_none() {
                break;
            }
            let code = code.unwrap();
            if let Some(inx) = Instruction::from(*code) {
                match inx {
                    Instruction::Pushl => {
                        let lidx = take_bytes(&object.code, &mut ip, LOCAL_WIDTH);
                        let variable = &object.locals[lidx];
                        let local = self.ctx.frame_mut().unwrap().locals.get(variable).cloned();
                        self.ctx.push_value(local.unwrap());
                    }
                    Instruction::Pushg => {}
                    Instruction::Pushc => {
                        use crate::value;
                        let cidx = take_bytes(&object.code, &mut ip, CONST_WIDTH);
                        let value = value::instantiate(&mut self.ctx, &object.consts[cidx]);
                        self.ctx.push_value(value);
                    }
                    Instruction::Movel => {},
                    Instruction::Moveg => {
                        let gidx = take_bytes(&object.code, &mut ip, GLOBAL_WIDTH);
                        let variable = &object.globals[gidx];
                        let value = self.ctx.pop_value().unwrap();
                        self.ctx.globals.insert(variable.clone(), value);
                    },
                    Instruction::Dup => {}
                    Instruction::Swap => {}
                    Instruction::Add => {
                        let first = self.ctx.pop_value().unwrap();
                        let second = self.ctx.pop_value().unwrap();
                        self.ctx.push_value(first + second);
                    }
                    Instruction::Sub => {
                        let first = self.ctx.pop_value().unwrap();
                        let second = self.ctx.pop_value().unwrap();
                        self.ctx.push_value(first - second);
                    }
                    Instruction::Mul => {
                        let first = self.ctx.pop_value().unwrap();
                        let second = self.ctx.pop_value().unwrap();
                        self.ctx.push_value(first * second);
                    }
                    Instruction::Div => {
                        let first = self.ctx.pop_value().unwrap();
                        let second = self.ctx.pop_value().unwrap();
                        self.ctx.push_value(first / second);
                    }
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

            ip += 1;
        }
    }

    pub fn run(&mut self) {
        for module in self.ctx.modules.iter() {
        }
    }
}
