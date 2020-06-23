use crate::bytecode::Instruction;
use crate::code::CodeObject;
use crate::code::CodeObjectRef;
use crate::context::Context;
use crate::module::Module;
use crate::value::RuValue;
use crate::var::Variable;

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

    pub fn load_and_import_all(&mut self, module: Module) -> Result<(), String> {
        self.ctx.load_and_import_all(module)
    }

    fn lookup_code_object(&self, name: &Variable) -> Option<CodeObjectRef> {
        self.ctx.scope.get(name).cloned()
    }

    pub fn run_object(&mut self, object: &CodeObject) {
        self.ctx.push_frame(0);

        let mut ip = 0;
        loop {
            let code = object.code.get(ip);
            if code.is_none() {
                break;
            }

            let inx = code.unwrap();
            match inx {
                Instruction::Pushl(lidx) => {
                    let variable = &object.locals[*lidx as usize];
                    let local = self.ctx.frame_mut().unwrap().locals.get(variable).cloned();
                    self.ctx.push_value(local.unwrap());
                }
                Instruction::Pushg(gidx) => {
                    let variable = &object.globals[*gidx as usize];
                    let global = self.ctx.globals.get(variable).cloned();
                    self.ctx.push_value(global.unwrap());
                }
                Instruction::Pushc(cidx) => {
                    use crate::value;
                    let value = value::instantiate(&mut self.ctx, &object.consts[*cidx as usize]);
                    self.ctx.push_value(value);
                }
                Instruction::Movel(lidx) => {
                    let first = self.ctx.pop_value().unwrap();
                    let variable = &object.locals[*lidx as usize];
                    self.ctx.frame_mut().unwrap().locals.insert(variable.clone(), first);
                }
                Instruction::Moveg(gidx) => {
                    let variable = &object.globals[*gidx as usize];
                    let value = self.ctx.pop_value().unwrap();
                    self.ctx.globals.insert(variable.clone(), value);
                }
                Instruction::Dup => {
                    let last = self.ctx.stack_mut().last().cloned().unwrap();
                    self.ctx.push_value(last);
                }
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
                Instruction::Mod => {
                    let first = self.ctx.pop_value().unwrap();
                    let second = self.ctx.pop_value().unwrap();
                    self.ctx.push_value(first % second);
                }
                Instruction::And => {
                    let first = self.ctx.pop_value().unwrap();
                    let second = self.ctx.pop_value().unwrap();
                    self.ctx.push_value(first & second);
                }
                Instruction::Or => {
                    let first = self.ctx.pop_value().unwrap();
                    let second = self.ctx.pop_value().unwrap();
                    self.ctx.push_value(first | second);
                }
                Instruction::Not => {
                    let first = self.ctx.pop_value().unwrap();
                    self.ctx.push_value(!first);
                }
                Instruction::Cmp => {
                    use std::cmp::Ordering;
                    let first = self.ctx.pop_value().unwrap();
                    let second = self.ctx.pop_value().unwrap();
                    let ordering = first.partial_cmp(&second).unwrap();
                    match ordering {
                        Ordering::Less => self.ctx.push_value(RuValue::Int(-1)),
                        Ordering::Equal => self.ctx.push_value(RuValue::Int(0)),
                        Ordering::Greater => self.ctx.push_value(RuValue::Int(1)),
                    }
                }
                Instruction::Jmp(addr) => {
                    ip = *addr as usize;
                    continue;
                }
                Instruction::Jeq(addr) => {
                    let first = self.ctx.pop_value().unwrap();
                    if first == RuValue::Int(0) {
                        ip = *addr as usize;
                        continue;
                    }
                }
                Instruction::Jgt(addr) => {}
                Instruction::Jlt(addr) => {}
                Instruction::Call(argn, gidx) => {
                    let func = &object.globals[*gidx as usize];

                    if let Some(co_object) = self.lookup_code_object(func) {
                        self.ctx.push_frame(*argn);
                        self.run_object(&*co_object);
                        self.ctx.pop_frame();
                    } else {
                        unimplemented!();
                    }
                }
                Instruction::Ret => break,
                Instruction::Interrupt(n) => {}
            }

            ip += 1;
        }

        self.ctx.pop_frame();
    }

    pub fn run(&mut self) {
        for module in self.ctx.modules.iter() {
        }
    }
}
