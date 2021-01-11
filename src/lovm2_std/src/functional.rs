use lovm2_extend::prelude::*;

#[lovm2_function]
pub fn argn(vm: &mut Vm) -> Lovm2Result<i64> {
    let frame = vm
        .context_mut()
        .lstack_mut()
        .iter_mut()
        .rev()
        .nth(1)
        .unwrap();
    Ok(frame.argn as i64)
}

#[lovm2_function]
pub fn call(vm: &mut Vm, function_name: String, mut args: Value) -> Lovm2Result<Value> {
    args.unref_inplace()?;
    match args {
        Value::List(args) => vm.call(function_name.as_ref(), args.as_slice()),
        _ => err_not_supported(),
    }
}

#[lovm2_function]
pub fn pop_vstack(vm: &mut Vm) -> Lovm2Result<Value> {
    vm.context_mut().pop_value()
}

#[lovm2_function]
pub fn push_vstack(vm: &mut Vm, val: Value) {
    vm.context_mut().push_value(val)
}
