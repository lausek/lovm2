use std::collections::HashMap;
use std::rc::Rc;

use crate::code::CodeObject;
use crate::code::CodeObjectBuilder;
use crate::code::CodeObjectRef;
use crate::hir::HIR;
use crate::var::Variable;

#[derive(Debug)]
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
    slots: HashMap<Variable, ModuleBuilderSlot>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn add<T>(&mut self, name: T) -> &mut ModuleBuilderSlot 
        where T: Into<Variable>
    {
        let name: Variable = name.into();
        self.slots.insert(name.clone(), ModuleBuilderSlot::new());
        self.slots.get_mut(&name).unwrap()
    }

    pub fn build(self) -> Result<Module, String> {
        let mut module = Module::new();

        for (key, co_builder) in self.slots.into_iter() {
            match co_builder.complete() {
                Ok(co) => {
                    module.slots.insert(key, Rc::new(co));
                }
                Err(msg) => return Err(msg),
            }
        }

        Ok(module)
    }
}

pub struct ModuleBuilderSlot {
    hir: Option<HIR>,
}

impl ModuleBuilderSlot {
    pub fn new() -> Self {
        Self {
            hir: None,
        }
    }

    pub fn hir(&mut self, hir: HIR) {
        self.hir = Some(hir);
    }

    pub fn complete(self) -> Result<CodeObject, String> {
        match self.hir {
            Some(hir) => hir.build(),
            None => Err("no hir for slot".to_string()),
        }
    }
}
