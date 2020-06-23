use std::rc::Rc;

use crate::code::CallProtocol;
use crate::context::Context;
use crate::module::Module;
use crate::value::RuValue;

macro_rules! derive_call_protocol {
    ($name:ident, $func:tt) => {
        #[derive(Debug)]
        struct $name;

        impl $name {
            pub fn instantiate() -> Rc<Self> {
                Rc::new(Self {})
            }
        }

        impl CallProtocol for $name {
            fn run(&self, ctx: &mut Context) -> Result<(), String> {
                $func(ctx)
            }
        }
    };
}

derive_call_protocol!(Print, (|ctx: &mut Context| {
    let argn = ctx.frame_mut().unwrap().argn;
    let args: Vec<String> = (0..argn)
        .map(|_| ctx.pop_value().unwrap())
        .map(|x| format!("{}", x))
        .collect();
    print!("{}", args.join(" "));
    Ok(())
}));

pub fn create_standard_module() -> Module {
    let mut module = Module::new();

    module.slots.insert("print".into(), Print::instantiate());

    module
}
