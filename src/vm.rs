use std::ops::*;

use crate::bytecode::Instruction;
use crate::code::{CallProtocol, CodeObject};
use crate::context::Context;
use crate::module::{create_standard_module, ModuleProtocol, ENTRY_POINT};
use crate::value::{box_ruvalue, RuValue};
use crate::var::Variable;

/// virtual machine for running bytecode
///
/// call convention is pascal style. if you have a function like `f(a, b, c)` it will be translated
/// to
///
/// ```ignore
/// push a
/// push b
/// push c
/// call f, 3
/// ```
///
/// and the function has to do the popping in reverse
///
/// ```ignore
/// pop c
/// pop b
/// pop a
/// ```
///
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

    pub fn call(&mut self, name: &str, args: &[RuValue]) -> Result<RuValue, String> {
        let name = Variable::from(name);
        match self.ctx.lookup_code_object(&name) {
            Some(co) => {
                let mut argn: u8 = 0;
                for arg in args.iter() {
                    argn += 1;
                    self.ctx.push_value(arg.clone());
                }

                self.ctx.push_frame(argn);
                co.run(&mut self.ctx)?;
                self.ctx.pop_frame();

                let val = self.context_mut().pop_value().unwrap();
                Ok(val)
            }
            _ => Err(format!("no code object named {}", name)),
        }
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

    /// a wrapper for `run_bytecode` that handles pushing and popping stack frames
    pub fn run_object(&mut self, co: &dyn CallProtocol) -> Result<(), String> {
        self.ctx.push_frame(0);
        co.run(&mut self.ctx)?;
        self.ctx.pop_frame();

        Ok(())
    }

    /// start the execution at `ENTRY_POINT`
    pub fn run(&mut self) -> Result<(), String> {
        match self.ctx.lookup_code_object(&ENTRY_POINT.into()) {
            Some(co) => self.run_object(co.as_ref()),
            None => Err(format!("no entry function called `{}`", ENTRY_POINT)),
        }
    }
}

macro_rules! ruvalue_operation {
    ($ctx:expr, $fn:ident) => {{
        let second = $ctx.pop_value().unwrap();
        let first = $ctx.pop_value().unwrap();
        $ctx.push_value(first.$fn(second));
    }};
}

macro_rules! ruvalue_compare {
    ($ctx:expr, $fn:ident) => {{
        let second = $ctx.pop_value().unwrap();
        let first = $ctx.pop_value().unwrap();
        $ctx.push_value(RuValue::Bool(first.$fn(&second)));
    }};
}

/// implementation of lovm2 bytecode behavior
///
/// *Note:* this function does not push a stack frame and could therefore mess up local variables
/// if not handled correctly. see `Vm.run_object`
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
                let value = value::instantiate(&co.consts[*cidx as usize]);
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
            Instruction::Get => {
                let key = ctx.pop_value().unwrap();
                let obj = ctx.pop_value().unwrap();
                let val = obj.get(key)?;
                ctx.push_value(val);
            }
            Instruction::Set => {
                let val = ctx.pop_value().unwrap();
                let key = ctx.pop_value().unwrap();
                let mut obj = ctx.pop_value().unwrap();
                obj.set(key, val)?;
            }
            Instruction::Add => ruvalue_operation!(ctx, add),
            Instruction::Sub => ruvalue_operation!(ctx, sub),
            Instruction::Mul => ruvalue_operation!(ctx, mul),
            Instruction::Div => ruvalue_operation!(ctx, div),
            Instruction::Rem => ruvalue_operation!(ctx, rem),
            Instruction::And => ruvalue_operation!(ctx, bitand),
            Instruction::Or => ruvalue_operation!(ctx, bitor),
            Instruction::Not => {
                let first = ctx.pop_value().unwrap();
                ctx.push_value(!first);
            }
            Instruction::Eq => ruvalue_compare!(ctx, eq),
            Instruction::Ne => ruvalue_compare!(ctx, ne),
            Instruction::Ge => ruvalue_compare!(ctx, ge),
            Instruction::Gt => ruvalue_compare!(ctx, gt),
            Instruction::Le => ruvalue_compare!(ctx, le),
            Instruction::Lt => ruvalue_compare!(ctx, lt),
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
                    return Err(format!("function `{}` not found", func));
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
