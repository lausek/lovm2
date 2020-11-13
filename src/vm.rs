//! runs modules and maintains program state

use std::ops::*;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::code::{CallProtocol, CodeObject, NewCodeObject};
use crate::context::Context;
use crate::hir::expr::Expr;
use crate::module::{/*create_standard_module, */ LoadableModule, ENTRY_POINT};
use crate::value::{box_value, Value};
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
        // TODO: add a new `Generic` variant to LoadableModule
        //ctx.load_and_import_all(create_standard_module()).unwrap();
        Self { ctx }
    }

    pub fn call(&mut self, name: &str, args: &[Value]) -> Lovm2Result<Value> {
        let name = Variable::from(name);
        let co = self.ctx.lookup_code_object(&name)?;

        let mut argn: u8 = 0;
        for arg in args.iter() {
            argn += 1;
            let arg = arg.clone();
            let arg = match arg {
                Value::Dict(_) | Value::List(_) => box_value(arg),
                _ => arg,
            };
            self.ctx.push_value(arg);
        }

        self.ctx.push_frame(argn);
        co.run(&mut self.ctx)?;
        self.ctx.pop_frame();

        let val = self.context_mut().pop_value()?;
        Ok(val)
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    pub fn evaluate_expr(&mut self, expr: &Expr) -> Lovm2Result<Value> {
        match expr {
            Expr::Access(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::DynamicValue(_) => todo!(),
            Expr::Operation1(_, _) => todo!(),
            Expr::Operation2(_, _, _) => todo!(),
            Expr::Slice(_) => todo!(),
            Expr::Value { val, .. } => Ok(val.clone()),
            Expr::Variable(var) => match self.ctx.globals.get(&var) {
                Some(val) => Ok(val.clone()),
                _ => Err((Lovm2ErrorTy::LookupFailed, var).into()),
            },
        }
    }

    // TODO: add `ImportOptions` parameter to specify what names to import
    pub fn load_and_import_all<T>(&mut self, module: T) -> Lovm2Result<()>
    where
        T: Into<LoadableModule>,
    {
        self.ctx.load_and_import_all(module)
    }

    /// a wrapper for `run_bytecode` that handles pushing and popping stack frames
    pub fn run_object(&mut self, co: &dyn CallProtocol) -> Lovm2Result<()> {
        self.ctx.push_frame(0);
        co.run(&mut self.ctx)?;
        self.ctx.pop_frame();

        Ok(())
    }

    /// start the execution at `ENTRY_POINT`
    pub fn run(&mut self) -> Lovm2Result<()> {
        match self.ctx.entry.take() {
            Some(callable) => self.run_object(callable.as_ref()),
            _ => todo!(),
        }
        //let co = self.ctx.lookup_code_object(&ENTRY_POINT.into())?;
    }
}

macro_rules! value_operation {
    ($ctx:expr, $fn:ident) => {{
        let second = $ctx.pop_value()?;
        let first = $ctx.pop_value()?;
        $ctx.push_value(first.$fn(second));
    }};
}

macro_rules! value_compare {
    ($ctx:expr, $fn:ident) => {{
        let second = $ctx.pop_value()?;
        let first = $ctx.pop_value()?;
        $ctx.push_value(Value::Bool(first.$fn(&second)));
    }};
}

fn deref_total(val: &mut Value) {
    while let Value::Ref(Some(r)) = val {
        let r = r.borrow().clone();
        *val = r;
    }
}

fn create_slice(target: Value, start: Value, end: Value) -> Lovm2Result<Value> {
    let start_idx = match start {
        Value::Nil => 0,
        _ => {
            if let Value::Int(n) = start.into_integer()? {
                n
            } else {
                unreachable!()
            }
        }
    };
    let end_idx = match end {
        Value::Nil => target.len()? as i64,
        _ => {
            if let Value::Int(n) = end.into_integer()? {
                n
            } else {
                unreachable!()
            }
        }
    };
    let mut slice = vec![];

    for idx in start_idx..end_idx {
        slice.push(target.get(Value::from(idx))?);
    }

    Ok(box_value(Value::List(slice)))
}

/// implementation of lovm2 bytecode behavior
///
/// *Note:* this function does not push a stack frame and could therefore mess up local variables
/// if not handled correctly. see `Vm.run_object`
pub fn run_bytecode(co: &NewCodeObject, ctx: &mut Context, offset: usize) -> Lovm2Result<()> {
    let mut ip = offset;
    while let Some(inx) = co.code.get(ip) {
        println!("{:?} {:?}", inx, ctx.vstack);
        match inx {
            Instruction::Pushl(lidx) => {
                let variable = &co.idents[*lidx as usize];
                match ctx.frame_mut()?.value_of(variable) {
                    Some(local) => ctx.push_value(local),
                    _ => return Err((Lovm2ErrorTy::LookupFailed, variable).into()),
                }
            }
            Instruction::Pushg(gidx) => {
                let variable = &co.idents[*gidx as usize];
                match ctx.value_of(variable) {
                    Some(global) => ctx.push_value(global),
                    _ => return Err((Lovm2ErrorTy::LookupFailed, variable).into()),
                }
            }
            Instruction::Pushc(cidx) => {
                let value = &co.consts[*cidx as usize];
                ctx.push_value(value.clone());
            }
            Instruction::Movel(lidx) => {
                let variable = &co.idents[*lidx as usize];
                let value = ctx.pop_value()?;
                ctx.frame_mut()?.locals.insert(variable.clone(), value);
            }
            Instruction::Moveg(gidx) => {
                let variable = &co.idents[*gidx as usize];
                let value = ctx.pop_value()?;
                ctx.globals.insert(variable.clone(), value);
            }
            Instruction::Discard => {
                ctx.pop_value()?;
            }
            Instruction::Dup => match ctx.stack_mut().last().cloned() {
                Some(last) => ctx.push_value(last),
                _ => return Err(Lovm2ErrorTy::ValueStackEmpty.into()),
            },
            Instruction::Swap => {}
            Instruction::Get => {
                let key = ctx.pop_value()?;
                let obj = ctx.pop_value()?;
                let val = obj.get(key)?;
                ctx.push_value(val.deref().unwrap());
            }
            Instruction::Getr => {
                let key = ctx.pop_value()?;
                let mut obj = ctx.pop_value()?;

                if let Err(e) = obj.get(key.clone()) {
                    if Lovm2ErrorTy::KeyNotFound != e.ty {
                        return Err(e);
                    }
                    obj.set(key.clone(), box_value(Value::Nil))?;
                }

                let val = obj.get(key)?;
                ctx.push_value(val);
            }
            Instruction::Set => {
                let mut val = ctx.pop_value()?;
                let target = ctx.pop_value()?;

                deref_total(&mut val);

                match target {
                    Value::Ref(Some(r)) => *r.borrow_mut() = val,
                    _ => return Err(format!("cannot use {:?} as set target", target).into()),
                }
            }
            Instruction::Add => value_operation!(ctx, add),
            Instruction::Sub => value_operation!(ctx, sub),
            Instruction::Mul => value_operation!(ctx, mul),
            Instruction::Div => value_operation!(ctx, div),
            Instruction::Pow => value_operation!(ctx, pow),
            Instruction::Rem => value_operation!(ctx, rem),
            Instruction::And => value_operation!(ctx, bitand),
            Instruction::Or => value_operation!(ctx, bitor),
            Instruction::Not => {
                let first = ctx.pop_value()?;
                ctx.push_value(!first);
            }
            Instruction::Eq => value_compare!(ctx, eq),
            Instruction::Ne => value_compare!(ctx, ne),
            Instruction::Ge => value_compare!(ctx, ge),
            Instruction::Gt => value_compare!(ctx, gt),
            Instruction::Le => value_compare!(ctx, le),
            Instruction::Lt => value_compare!(ctx, lt),
            Instruction::Jmp(addr) => {
                ip = *addr as usize;
                continue;
            }
            Instruction::Jt(addr) => {
                let first = ctx.pop_value()?;
                if first.into_bool()? == Value::Bool(true) {
                    ip = *addr as usize;
                    continue;
                }
            }
            Instruction::Jf(addr) => {
                let first = ctx.pop_value()?;
                if first.into_bool()? == Value::Bool(false) {
                    ip = *addr as usize;
                    continue;
                }
            }
            Instruction::Call(argn, gidx) => {
                let func = &co.idents[*gidx as usize];
                let other_co = ctx.lookup_code_object(func)?;
                ctx.push_frame(*argn);
                other_co.run(ctx)?;
                ctx.pop_frame();
            }
            Instruction::Ret => break,
            Instruction::Interrupt(n) => {
                if let Some(func) = &ctx.interrupts[*n as usize] {
                    func.clone()(ctx)?;
                }
            }
            Instruction::Cast(tid) => {
                let val = ctx.pop_value()?;
                let val = val.cast(*tid)?;
                ctx.push_value(val);
            }
            Instruction::Load => {
                let name = ctx.pop_value()?;
                // TODO: use to_string() here
                let name = format!("{}", name);
                // path to the modules source code
                let relative_to = if let Some(mname) = co.module() {
                    ctx.modules
                        .get(&mname)
                        .and_then(|module| module.location())
                        .map(String::to_string)
                } else {
                    None
                };
                ctx.load_and_import_by_name(name.as_ref(), relative_to)?;
            }
            Instruction::Box => {
                let value = ctx.pop_value()?;
                ctx.push_value(box_value(value));
            }
            Instruction::Slice => {
                let end = ctx.pop_value()?;
                let start = ctx.pop_value()?;
                let target = ctx.pop_value()?;
                let slice = create_slice(target, start, end)?;
                ctx.push_value(slice);
            }
        }

        ip += 1;
    }

    Ok(())
}
