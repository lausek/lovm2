use std::rc::Rc;

use crate::code::CallProtocol;
use crate::context::Context;
use crate::module::Module;
use crate::value::RuValue;

pub fn create_standard_module() -> Module {
    let mut module = Module::new();

    module.slots.insert("print".into(), Print::instantiate());

    module
}

#[derive(Debug)]
struct Print;

impl Print {
    pub fn instantiate() -> Rc<Self> {
        Rc::new(Self {})
    }
}

impl CallProtocol for Print {
    fn run(&self, ctx: &mut Context) -> Result<(), String> {
        let argn = ctx.frame_mut().unwrap().argn;
        let args: Vec<String> = (0..argn)
                                    .map(|_| ctx.pop_value().unwrap())
                                    .map(|x| format!("{}", x))
                                    .collect();
        print!("{}", args.join(" "));
        Ok(())
    }
}
