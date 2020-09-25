use std::collections::HashMap;
use std::rc::Rc;

use crate::code::CodeObject;
use crate::error::*;
use crate::hir::HIR;
use crate::module::{standard::BUILTIN_FUNCTIONS, Module, ENTRY_POINT};
use crate::var::Variable;

pub struct ModuleBuilder {
    pub slots: HashMap<Variable, ModuleBuilderSlot>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn add<T>(&mut self, name: T) -> &mut ModuleBuilderSlot
    where
        T: Into<Variable>,
    {
        let name: Variable = name.into();

        if BUILTIN_FUNCTIONS.contains(&name.as_ref()) {
            panic!("shadowing builtin function `{}` is not allowed", name);
        }

        self.slots.insert(name.clone(), ModuleBuilderSlot::new());
        self.slots.get_mut(&name).unwrap()
    }

    pub fn build(self) -> Lovm2CompileResult<Module> {
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

    pub fn entry(&mut self) -> &mut ModuleBuilderSlot {
        let name = Variable::from(ENTRY_POINT);
        if !self.slots.contains_key(&name) {
            self.slots.insert(name.clone(), ModuleBuilderSlot::new());
        }
        self.slots.get_mut(&name).unwrap()
    }
}

pub struct ModuleBuilderSlot {
    hir: Option<HIR>,
}

impl ModuleBuilderSlot {
    pub fn new() -> Self {
        Self { hir: None }
    }

    pub fn hir(&mut self, hir: HIR) {
        self.hir = Some(hir);
    }

    pub fn complete(self) -> Lovm2CompileResult<CodeObject> {
        match self.hir {
            Some(hir) => hir.build(),
            None => Err("no hir for slot".to_string()),
        }
    }
}
