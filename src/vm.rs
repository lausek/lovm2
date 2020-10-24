use std::ops::*;
use std::rc::Rc;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::code::{CallProtocol, CodeObject};
use crate::context::Context;
use crate::hir::expr::Expr;
use crate::module::{create_standard_module, GenericModule, ENTRY_POINT};
use crate::value::{box_ruvalue, instantiate, RuValue};
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
        ctx.load_and_import_all(Rc::new(create_standard_module()) as GenericModule)
            .unwrap();
        Self { ctx }
    }

    pub fn call(&mut self, name: &str, args: &[RuValue]) -> Lovm2Result<RuValue> {
        let name = Variable::from(name);
        let co = self.ctx.lookup_code_object(&name)?;

        let mut argn: u8 = 0;
        for arg in args.iter() {
            argn += 1;
            let arg = arg.clone();
            let arg = match arg {
                RuValue::Dict(_) | RuValue::List(_) => box_ruvalue(arg),
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

    pub fn evaluate_expr(&mut self, expr: &Expr) -> Lovm2Result<RuValue> {
        match expr {
            Expr::Access(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::DynamicValue(_) => todo!(),
            Expr::Operation1(_, _) => todo!(),
            Expr::Operation2(_, _, _) => todo!(),
            Expr::Slice(_) => todo!(),
            Expr::Value { val, .. } => Ok(instantiate(val)),
            Expr::Variable(var) => match self.ctx.globals.get(&var) {
                Some(val) => Ok(val.clone()),
                _ => Err(format!("variable `{}` not found", var).into()),
            },
        }
    }

    // TODO: add `ImportOptions` parameter to specify what names to import
    pub fn load_and_import_all<T>(&mut self, module: T) -> Lovm2Result<()>
    where
        T: Into<GenericModule>,
    {
        self.ctx.load_and_import_all(module.into())
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
        let co = self.ctx.lookup_code_object(&ENTRY_POINT.into())?;
        self.run_object(co.as_ref())
    }
}

macro_rules! ruvalue_operation {
    ($ctx:expr, $fn:ident) => {{
        let second = $ctx.pop_value()?;
        let first = $ctx.pop_value()?;
        $ctx.push_value(first.$fn(second));
    }};
}

macro_rules! ruvalue_compare {
    ($ctx:expr, $fn:ident) => {{
        let second = $ctx.pop_value()?;
        let first = $ctx.pop_value()?;
        $ctx.push_value(RuValue::Bool(first.$fn(&second)));
    }};
}

fn deref_total(val: &mut RuValue) {
    while let RuValue::Ref(Some(r)) = val {
        let r = r.borrow().clone();
        *val = r;
    }
}

fn create_slice(target: RuValue, start: RuValue, end: RuValue) -> Lovm2Result<RuValue> {
    let start_idx = match start {
        RuValue::Nil => 0,
        _ => {
            if let RuValue::Int(n) = start.into_integer()? {
                n
            } else {
                unreachable!()
            }
        }
    };
    let end_idx = match end {
        RuValue::Nil => target.len()? as i64,
        _ => {
            if let RuValue::Int(n) = end.into_integer()? {
                n
            } else {
                unreachable!()
            }
        }
    };
    let mut slice = vec![];

    for idx in start_idx..end_idx {
        slice.push(target.get(RuValue::from(idx))?);
    }

    Ok(RuValue::List(slice))
}

/// implementation of lovm2 bytecode behavior
///
/// *Note:* this function does not push a stack frame and could therefore mess up local variables
/// if not handled correctly. see `Vm.run_object`
pub fn run_bytecode(co: &CodeObject, ctx: &mut Context) -> Lovm2Result<()> {
    let mut ip = 0;
    while let Some(inx) = co.code.get(ip) {
        match inx {
            Instruction::Pushl(lidx) => {
                let variable = &co.locals[*lidx as usize];
                match ctx.frame_mut()?.value_of(variable) {
                    Some(local) => ctx.push_value(local),
                    _ => return Err(format!("local `{}` not found", variable).into()),
                }
            }
            Instruction::Pushg(gidx) => {
                let variable = &co.globals[*gidx as usize];
                match ctx.value_of(variable) {
                    Some(global) => ctx.push_value(global),
                    _ => return Err(format!("global `{}` not found", variable).into()),
                }
            }
            Instruction::Pushc(cidx) => {
                use crate::value;
                let value = value::instantiate(&co.consts[*cidx as usize]);
                ctx.push_value(value);
            }
            Instruction::Movel(lidx) => {
                let variable = &co.locals[*lidx as usize];
                let value = ctx.pop_value()?;
                ctx.frame_mut()?.locals.insert(variable.clone(), value);
            }
            Instruction::Moveg(gidx) => {
                let variable = &co.globals[*gidx as usize];
                let value = ctx.pop_value()?;
                ctx.globals.insert(variable.clone(), value);
            }
            Instruction::Discard => {
                ctx.pop_value()?;
            }
            Instruction::Dup => match ctx.stack_mut().last().cloned() {
                Some(last) => ctx.push_value(last),
                _ => return Err("no value on stack to duplicate".into()),
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

                // TODO: make sure that this is a not found error
                if obj.get(key.clone()).is_err() {
                    obj.set(key.clone(), box_ruvalue(RuValue::Nil))?;
                }

                let val = obj.get(key)?;
                ctx.push_value(val);
            }
            Instruction::Set => {
                let mut val = ctx.pop_value()?;
                let target = ctx.pop_value()?;

                deref_total(&mut val);

                match target {
                    RuValue::Ref(Some(r)) => *r.borrow_mut() = val,
                    _ => return Err(format!("cannot use {:?} as set target", target).into()),
                }
            }
            Instruction::Add => ruvalue_operation!(ctx, add),
            Instruction::Sub => ruvalue_operation!(ctx, sub),
            Instruction::Mul => ruvalue_operation!(ctx, mul),
            Instruction::Div => ruvalue_operation!(ctx, div),
            Instruction::Pow => ruvalue_operation!(ctx, pow),
            Instruction::Rem => ruvalue_operation!(ctx, rem),
            Instruction::And => ruvalue_operation!(ctx, bitand),
            Instruction::Or => ruvalue_operation!(ctx, bitor),
            Instruction::Not => {
                let first = ctx.pop_value()?;
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
                let first = ctx.pop_value()?;
                // TODO: allow to_bool conversion
                if first == RuValue::Bool(true) {
                    ip = *addr as usize;
                    continue;
                }
            }
            Instruction::Jf(addr) => {
                let first = ctx.pop_value()?;
                // TODO: allow to_bool conversion
                if first == RuValue::Bool(false) {
                    ip = *addr as usize;
                    continue;
                }
            }
            Instruction::Call(argn, gidx) => {
                let func = &co.globals[*gidx as usize];
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
                let relative_to = if let Some(mname) = co.module() {
                    ctx.modules
                        .get(&mname)
                        .and_then(|module| module.location())
                        .and_then(|module_location| {
                            std::path::Path::new(module_location)
                                .parent()
                                .map(|path| path.to_str().unwrap().to_string())
                        })
                } else {
                    None
                };
                ctx.load_and_import_by_name(name.as_ref(), relative_to)?;
            }
            Instruction::Box => {
                let value = ctx.pop_value()?;
                ctx.push_value(box_ruvalue(value));
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
