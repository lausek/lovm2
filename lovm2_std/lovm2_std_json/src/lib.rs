use lovm2_extend::prelude::*;

#[lovm2_function]
fn decode(json: String) -> Lovm2Result<Value> { todo!() }

#[lovm2_function]
fn encode(val: Value) -> Lovm2Result<String> { todo!() }

lovm2_module_init!();
