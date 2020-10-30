use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

use crate::code::CodeObject;
use crate::hir::HIR;
use crate::module::{standard::BUILTIN_FUNCTIONS, Module, ENTRY_POINT};
use crate::var::Variable;

pub struct ModuleBuilder {
    name: String,
    pub slots: HashMap<Variable, ModuleBuilderSlot>,
    pub uses: Vec<String>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            name: "<unnamed>".to_string(),
            slots: HashMap::new(),
            uses: vec![],
        }
    }

    pub fn named<T>(name: T) -> Self
    where
        T: ToString,
    {
        let mut builder = Self::new();
        builder.name = name.to_string();
        builder
    }

    pub fn add_dependency(&mut self, name: String) {
        if !self.uses.contains(&name) {
            self.uses.push(name);
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

        module.name = self.name;
        module.uses = self.uses;

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
            None => Err("no hir for slot".into()),
        }
    }
}
