use lovm2::prelude::*;
use lovm2::value::box_value;
use lovm2_extend::prelude::*;

#[lovm2_function]
fn argn(vm: &Vm) -> Lovm2Result<i64> {
    todo!()
}

#[lovm2_function]
fn call(vm: &mut Vm, function_name: String, args: Value) -> Lovm2Result<Value> {
    todo!()
}

lovm2_module_init!(functional);
