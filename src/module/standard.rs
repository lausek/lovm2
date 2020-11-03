//! native implementation of lovm2s builtin functions

use std::rc::Rc;

use crate::code::CallProtocol;
use crate::context::Context;
use crate::lovm2_builtin;
use crate::module::Module;
use crate::value::Value;

pub const BUILTIN_FUNCTIONS: &[&str] = &["input", "len", "print"];

#[lovm2_builtin]
fn input(ctx: &mut Context) -> Lovm2Result<()> {
    use std::io::stdin;

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    ctx.push_value(Value::Str(input));

    Ok(())
}

#[lovm2_builtin]
fn len(ctx: &mut Context) -> Lovm2Result<()> {
    let target = ctx.pop_value()?;

    let val = target.len()?;
    ctx.push_value(Value::Int(val as i64));

    Ok(())
}

#[lovm2_builtin]
fn print(ctx: &mut Context) -> Lovm2Result<()> {
    use std::io::Write;

    let argn = ctx.frame_mut().unwrap().argn;
    let mut args: Vec<String> = (0..argn)
        .map(|_| ctx.pop_value().unwrap())
        .map(|x| format!("{}", x))
        .collect();

    args.reverse();

    print!("{}", args.join(" "));
    std::io::stdout().flush().unwrap();
    ctx.push_value(Value::Nil);

    Ok(())
}

/// create a `Module` of builtin functions. this gets automatically loaded on `Vm` creation.
pub fn create_standard_module() -> Module {
    let mut module = Module::new();

    module
        .slots
        .insert("input".into(), InputBuiltin::instantiate());
    module.slots.insert("len".into(), LenBuiltin::instantiate());
    module
        .slots
        .insert("print".into(), PrintBuiltin::instantiate());

    module
}
