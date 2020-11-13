//! building modules from HIR

use std::collections::HashMap;
//use std::rc::Rc;

use lovm2_error::*;

//use crate::code::CodeObject;
use crate::hir::{lowering::LoweringRuntime, HIR};
use crate::module::{standard::BUILTIN_FUNCTIONS, CodeObjectFunction, Module, ENTRY_POINT};
use crate::var::Variable;

use std::rc::Rc;

pub struct ModuleBuilder {
    name: String,
    pub slots: HashMap<Variable, ModuleBuilderSlot>,
    pub uses: Vec<String>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
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

    pub fn build(mut self) -> Lovm2CompileResult<Module> {
        //let mut module = Module::new();
        //let mut entries = vec![];
        let mut ru = LoweringRuntime::new();
        ru.name = self.name;
        ru.uses = self.uses;

        // main entry point must be at start (offset 0)
        let main_key = Variable::from(ENTRY_POINT);
        if let Some(co_builder) = self.slots.remove(&main_key) {
            let iidx = ru.index_ident(&main_key);
            ru.entries.push((iidx, ru.code.len()));
            co_builder.complete(&mut ru)?;
        }

        for (key, co_builder) in self.slots.into_iter() {
            let iidx = ru.index_ident(&key);
            ru.entries.push((iidx, ru.code.len()));
            co_builder.complete(&mut ru)?;
        }

        let mut module: Module = ru.complete()?.into();

        for (iidx, offset) in module.code_object.entries.iter() {
            let key = &module.code_object.idents[*iidx];
            let func = CodeObjectFunction::from(module.code_object.clone(), *offset);
            module.slots.insert(key.clone(), Rc::new(func));
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

    pub fn complete(self, ru: &mut LoweringRuntime) -> Lovm2CompileResult<()> {
        match self.hir {
            Some(hir) => hir.build(ru),
            None => Err("no hir for slot".into()),
        }
    }
}
