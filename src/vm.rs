use crate::bytecode::Instruction;
use crate::code::{CallProtocol, CodeObject};
use crate::context::Context;
use crate::module::{create_standard_module, ModuleProtocol};
use crate::value::{box_ruvalue, RuValue};

pub const ENTRY_POINT: &str = "main";

pub struct Vm {
    ctx: Context,
}

impl Vm {
    pub fn new() -> Self {
        let mut ctx = Context::new();
        ctx.load_and_import_all(Box::new(create_standard_module()) as Box<dyn ModuleProtocol>)
            .unwrap();
        Self { ctx }
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    // TODO: add `ImportOptions` parameter to specify what names to import
    pub fn load_and_import_all<T>(&mut self, module: T) -> Result<(), String>
    where
        T: Into<Box<dyn ModuleProtocol>>,
    {
        self.ctx.load_and_import_all(module.into())
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
                let copy = local.unwrap().borrow().clone();
                ctx.push_value(copy);
            }
            Instruction::Pushg(gidx) => {
                let variable = &co.globals[*gidx as usize];
                let global = ctx.globals.get(variable).unwrap();
                let global = global.borrow().clone();
                ctx.push_value(global);
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
                    .insert(variable.clone(), box_ruvalue(first));
            }
            Instruction::Moveg(gidx) => {
                let variable = &co.globals[*gidx as usize];
                let value = ctx.pop_value().unwrap();
                ctx.globals.insert(variable.clone(), box_ruvalue(value));
            }
            Instruction::Discard => {
                ctx.pop_value().unwrap();
            }
            Instruction::Dup => {
                let last = ctx.stack_mut().last().cloned().unwrap();
                ctx.push_value(last);
            }
            Instruction::Swap => {}
            // TODO: create macro for these simple operations; rely on std::ops trait, not operator
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
            Instruction::Eq => {
                let first = ctx.pop_value().unwrap();
                let second = ctx.pop_value().unwrap();
                ctx.push_value(RuValue::Bool(first == second));
            }
            Instruction::Ne => unimplemented!(),
            Instruction::Ge => unimplemented!(),
            Instruction::Gt => unimplemented!(),
            Instruction::Le => unimplemented!(),
            Instruction::Lt => unimplemented!(),
            Instruction::Jmp(addr) => {
                ip = *addr as usize;
                continue;
            }
            Instruction::Jt(addr) => {
                let first = ctx.pop_value().unwrap();
                // TODO: allow to_bool conversion
                if first == RuValue::Bool(true) {
                    ip = *addr as usize;
                    continue;
                }
            }
            Instruction::Jf(addr) => {
                let first = ctx.pop_value().unwrap();
                // TODO: allow to_bool conversion
                if first == RuValue::Bool(false) {
                    ip = *addr as usize;
                    continue;
                }
            }
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
            Instruction::Cast(tid) => {
                let val = ctx.pop_value().unwrap();
                let val = val.cast(*tid)?;
                ctx.push_value(val);
            }
            Instruction::Load => {
                let name = ctx.pop_value().unwrap();
                // TODO: use to_string() here
                let name = format!("{}", name);
                ctx.load_and_import_by_name(name.as_ref())?;
            }
        }

        ip += 1;
    }

    Ok(())
}
