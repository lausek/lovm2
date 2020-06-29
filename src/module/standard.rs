use std::rc::Rc;

use crate::code::CallProtocol;
use crate::context::Context;
use crate::lovm2_builtin;
use crate::module::Module;
use crate::value::RuValue;

#[lovm2_builtin]
fn print(ctx: &mut Context) -> Result<(), String> {
    let argn = ctx.frame_mut().unwrap().argn;
    let args: Vec<String> = (0..argn)
        .map(|_| ctx.pop_value().unwrap())
        .map(|x| format!("{}", x))
        .collect();
    print!("{}", args.join(" "));
    Ok(())
}

#[lovm2_builtin]
fn input(ctx: &mut Context) -> Result<(), String> {
    use std::io::stdin;

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    ctx.push_value(RuValue::Str(input));

    Ok(())
}

pub fn create_standard_module() -> Module {
    let mut module = Module::new();

    module
        .slots
        .insert("input".into(), InputBuiltin::instantiate());
    module
        .slots
        .insert("print".into(), PrintBuiltin::instantiate());

    module
}
