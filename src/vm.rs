use crate::bytecode::Instruction;
use crate::code::{CallProtocol, CodeObject};
use crate::context::Context;
use crate::module::{create_standard_module, Module};
use crate::value::RuValue;

pub const ENTRY_POINT: &str = "main";

pub struct Vm {
    ctx: Context,
}

impl Vm {
    pub fn new() -> Self {
        let mut ctx = Context::new();
        ctx.load_and_import_all(create_standard_module()).unwrap();
        Self { ctx }
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    // TODO: add `ImportOptions` parameter to specify what names to import
    pub fn load_and_import_all(&mut self, module: Module) -> Result<(), String> {
        self.ctx.load_and_import_all(module)
    }

    pub fn run_object(&mut self, co: &dyn CallProtocol) -> Result<(), String> {
        self.ctx.push_frame(0);
        co.run(&mut self.ctx)?;
        self.ctx.pop_frame();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        match self.ctx.lookup_code_object(&ENTRY_POINT.into()) {
            Some(co) => self.run_object(co.as_ref()),
            None => Err(format!("no entry function called `{}`", ENTRY_POINT)),
        }
    }
}

pub fn run_bytecode(co: &CodeObject, ctx: &mut Context) -> Result<(), String> {
    let mut ip = 0;
    while let Some(inx) = co.code.get(ip) {
        match inx {
            Instruction::Pushl(lidx) => {
                let variable = &co.locals[*lidx as usize];
                let local = ctx.frame_mut().unwrap().locals.get(variable).cloned();
                ctx.push_value(local.unwrap());
            }
            Instruction::Pushg(gidx) => {
                let variable = &co.globals[*gidx as usize];
                let global = ctx.globals.get(variable).cloned();
                ctx.push_value(global.unwrap());
            }
            Instruction::Pushc(cidx) => {
                use crate::value;
                let value = value::instantiate(ctx, &co.consts[*cidx as usize]);
                ctx.push_value(value);
            }
            Instruction::Movel(lidx) => {
                let first = ctx.pop_value().unwrap();
                let variable = &co.locals[*lidx as usize];
                ctx.frame_mut()
                    .unwrap()
                    .locals
                    .insert(variable.clone(), first);
            }
            Instruction::Moveg(gidx) => {
                let variable = &co.globals[*gidx as usize];
                let value = ctx.pop_value().unwrap();
                ctx.globals.insert(variable.clone(), value);
            }
            Instruction::Dup => {
                let last = ctx.stack_mut().last().cloned().unwrap();
                ctx.push_value(last);
            }
            Instruction::Swap => {}
            Instruction::Add => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(first + second);
            }
            Instruction::Sub => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(first - second);
            }
            Instruction::Mul => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(first * second);
            }
            Instruction::Div => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(first / second);
            }
            Instruction::Rem => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(first % second);
            }
            Instruction::And => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(first & second);
            }
            Instruction::Or => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(first | second);
            }
            Instruction::Not => {
                let first = ctx.pop_value().unwrap();
                ctx.push_value(!first);
            }
            Instruction::Cmp => {
                use std::cmp::Ordering;
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                let ordering = first.partial_cmp(&second).unwrap();
                match ordering {
                    Ordering::Less => ctx.push_value(RuValue::Int(-1)),
                    Ordering::Equal => ctx.push_value(RuValue::Int(0)),
                    Ordering::Greater => ctx.push_value(RuValue::Int(1)),
                }
            }
            Instruction::Jmp(addr) => {
                ip = *addr as usize;
                continue;
            }
            Instruction::Jeq(addr) => {
                let first = ctx.pop_value().unwrap();
                if first == RuValue::Int(0) {
                    ip = *addr as usize;
                    continue;
                }
            }
            Instruction::Jgt(_addr) => {}
            Instruction::Jlt(_addr) => {}
            Instruction::Call(argn, gidx) => {
                let func = &co.globals[*gidx as usize];

                if let Some(other_co) = ctx.lookup_code_object(func) {
                    ctx.push_frame(*argn);
                    other_co.run(ctx)?;
                    ctx.pop_frame();
                } else {
                    unimplemented!();
                }
            }
            Instruction::Ret => break,
            Instruction::Interrupt(n) => {
                if let Some(func) = &ctx.interrupts[*n as usize] {
                    func.clone()(ctx);
                }
            }
        }

        ip += 1;
    }

    Ok(())
}
