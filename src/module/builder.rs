//! building modules from HIR

use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

use crate::gen::{CompileOptions, HirLoweringRuntime, LirElement, HIR};

use crate::var::Variable;

use super::*;

#[derive(Clone)]
pub struct ModuleBuilder {
    meta: ModuleMeta,
    pub slots: HashMap<Variable, ModuleBuilderSlot>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            meta: ModuleMeta::default(),
            slots: HashMap::new(),
        }
    }

    pub fn named<T>(name: T) -> Self
    where
        T: ToString,
    {
        Self::with_meta(name.to_string())
    }

    pub fn with_meta<T>(meta: T) -> Self
    where
        T: Into<ModuleMeta>,
    {
        Self {
            meta: meta.into(),
            slots: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, name: String) {
        if !self.meta.uses.contains(&name) {
            self.meta.uses.push(name);
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
        self.build_with_options(CompileOptions::default())
    }

    pub fn build_with_options(mut self, options: CompileOptions) -> Lovm2CompileResult<Module> {
        let mut ru = HirLoweringRuntime::new(self.meta, options);

        // main entry point must be at start (offset 0)
        let main_key = Variable::from(ENTRY_POINT);
        if let Some(co_builder) = self.slots.remove(&main_key) {
            ru.emit(LirElement::entry(main_key));
            co_builder.complete(&mut ru)?;
        }

        for (key, co_builder) in self.slots.into_iter() {
            ru.emit(LirElement::entry(key));
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

#[derive(Clone)]
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

    pub fn complete(self, ru: &mut HirLoweringRuntime) -> Lovm2CompileResult<()> {
        match self.hir {
            Some(hir) => hir.build(ru),
            None => Err("no hir for slot".into()),
        }
    }
}
