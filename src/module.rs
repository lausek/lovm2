use std::collections::HashMap;
use std::rc::Rc;

use crate::code::CodeObjectBuilder;
use crate::code::CodeObjectRef;
use crate::var::Variable;

pub struct Module {
    pub slots: HashMap<Variable, CodeObjectRef>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
}

pub struct ModuleBuilder {
    slots: HashMap<Variable, CodeObjectBuilder>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn build(self) -> Result<Module, String> {
        let mut module = Module::new();

        for (key, co_builder) in self.slots.into_iter() {
            match co_builder.build() {
                Ok(co) => {
                    module.slots.insert(key, Rc::new(co));
                }
                Err(msg) => return Err(msg),
            }
        }

        Ok(module)
    }
}
