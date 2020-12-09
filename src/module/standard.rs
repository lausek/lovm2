//! native implementation of lovm2s builtin functions

use std::rc::Rc;

use crate::code::{CallProtocol, CodeObject};
use crate::lovm2_builtin;
use crate::module::Module;
use crate::value::Value;
use crate::vm::Vm;

pub const BUILTIN_FUNCTIONS: &[&str] = &["input", "len", "print"];

#[lovm2_builtin]
fn input(vm: &mut Vm) -> Lovm2Result<()> {
    use std::io::stdin;

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    vm.ctx.push_value(Value::Str(input));

    Ok(())
}

#[lovm2_builtin]
fn len(vm: &mut Vm) -> Lovm2Result<()> {
    let target = vm.ctx.pop_value()?;

    let val = target.len()?;
    vm.ctx.push_value(Value::Int(val as i64));

    Ok(())
}

#[lovm2_builtin]
fn print(vm: &mut Vm) -> Lovm2Result<()> {
    use std::io::Write;

    let argn = vm.ctx.frame_mut().unwrap().argn;
    let mut args: Vec<String> = (0..argn)
        .map(|_| vm.ctx.pop_value().unwrap())
        .map(|x| format!("{}", x))
        .collect();

    args.reverse();

    print!("{}", args.join(" "));
    std::io::stdout().flush().unwrap();
    vm.ctx.push_value(Value::Nil);

    Ok(())
}

/// create a `Module` of builtin functions. this gets automatically loaded on `Vm` creation.
pub fn create_standard_module() -> Module {
    let mut module: Module = CodeObject {
        name: "std".to_string(),
        ..CodeObject::default()
    }
    .into();

    module.slots.insert("input", InputBuiltin::instantiate());
    module.slots.insert("len", LenBuiltin::instantiate());
    module.slots.insert("print", PrintBuiltin::instantiate());

    module
}
