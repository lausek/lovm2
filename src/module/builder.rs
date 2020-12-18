//! Building modules from Hir

use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

use crate::gen::{CompileOptions, Hir, HirLoweringRuntime, LirElement};
use crate::var::Variable;

use super::*;

#[derive(Clone)]
pub struct ModuleBuilder {
    meta: ModuleMeta,
    pub hirs: HashMap<Variable, Hir>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            meta: ModuleMeta::default(),
            hirs: HashMap::new(),
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
            hirs: HashMap::new(),
        }
    }

    pub fn add_dependency<T>(&mut self, name: T)
    where
        T: ToString,
    {
        let name = name.to_string();
        if !self.meta.uses.contains(&name) {
            self.meta.uses.push(name);
        }
    }

    pub fn add<T>(&mut self, name: T) -> &mut Hir
    where
        T: Into<Variable>,
    {
        self.add_with_args(name, vec![])
    }

    pub fn add_with_args<T>(&mut self, name: T, args: Vec<Variable>) -> &mut Hir
    where
        T: Into<Variable>,
    {
        let name: Variable = name.into();
        self.hirs.insert(name.clone(), Hir::with_args(args));
        self.hirs.get_mut(&name).unwrap()
    }

    pub fn build(&mut self) -> Lovm2CompileResult<Module> {
        self.build_with_options(CompileOptions::default())
    }

    pub fn build_with_options(&mut self, options: CompileOptions) -> Lovm2CompileResult<Module> {
        // this needs a mutable reference to `self` because hir may gets a return
        // instruction appended. maybe this could be done somewhere else?

        let mut ru = HirLoweringRuntime::new(self.meta.clone(), options);

        // main entry point must be at start (offset 0)
        let entry_key = Variable::from(ENTRY_POINT);

        if let Some((key, hir)) = self.hirs.iter_mut().find(|(k, _)| **k == entry_key) {
            ru.emit(LirElement::entry(key.clone()));
            hir.build(&mut ru)?;
        }

        for (key, hir) in self.hirs.iter_mut().filter(|(k, _)| **k != entry_key) {
            ru.emit(LirElement::entry(key.clone()));
            hir.build(&mut ru)?;
        }

        let mut module: Module = ru.complete()?.into();

        for (iidx, offset) in module.code_object.entries.iter() {
            let key = &module.code_object.idents[*iidx];
            let func = CodeObjectFunction::from(module.code_object.clone(), *offset);
            module.slots.insert(key.clone(), Rc::new(func));
        }

        Ok(module)
    }

    pub fn entry(&mut self) -> &mut Hir {
        let name = Variable::from(ENTRY_POINT);
        if !self.hirs.contains_key(&name) {
            self.hirs.insert(name.clone(), Hir::new());
        }
        self.hirs.get_mut(&name).unwrap()
    }
}
