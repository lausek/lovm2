//! Tools for building [LV2Module].

use std::collections::HashMap;
use std::rc::Rc;

use crate::code::LV2CodeObjectFunction;
use crate::error::*;
use crate::gen::opt::Optimizer;
use crate::gen::{
    LV2CompileOptions, LV2Function, LV2HirLoweringRuntime, LV2ModuleMeta, LirElement,
};
use crate::module::{LV2Module, LV2_ENTRY_POINT};
use crate::var::LV2Variable;

/// Representation of modules before lowering.
#[derive(Clone)]
pub struct LV2ModuleBuilder {
    /// Meta information about name, location and static uses.
    meta: LV2ModuleMeta,
    /// Functions contained in the module.
    hirs: HashMap<LV2Variable, LV2Function>,
}

impl LV2ModuleBuilder {
    /// Create a builder.
    pub fn new() -> Self {
        Self {
            meta: LV2ModuleMeta::default(),
            hirs: HashMap::new(),
        }
    }

    /// Create a builder with a name.
    pub fn named<T>(name: T) -> Self
    where
        T: ToString,
    {
        Self::with_meta(name.to_string())
    }

    /// Create a builder with module meta information.
    pub fn with_meta<T>(meta: T) -> Self
    where
        T: Into<LV2ModuleMeta>,
    {
        Self {
            meta: meta.into(),
            hirs: HashMap::new(),
        }
    }

    /// Create a new function hir without arguments.
    pub fn add<T>(&mut self, name: T) -> &mut LV2Function
    where
        T: Into<LV2Variable>,
    {
        self.add_with_args(name, vec![])
    }

    /// Create a new function hir with arguments.
    pub fn add_with_args<T>(&mut self, name: T, args: Vec<LV2Variable>) -> &mut LV2Function
    where
        T: Into<LV2Variable>,
    {
        let name: LV2Variable = name.into();

        self.hirs.insert(name.clone(), LV2Function::with_args(args));
        self.hirs.get_mut(&name).unwrap()
    }

    /// Generate a module from the current data. This uses the default [LV2CompileOptions] e.g.
    /// optimization is enabled.
    pub fn build(&self) -> LV2CompileResult<LV2Module> {
        self.build_with_options(LV2CompileOptions::default())
    }

    /// Generate a module from the current data but use custom compile options.
    pub fn build_with_options(&self, options: LV2CompileOptions) -> LV2CompileResult<LV2Module> {
        use crate::gen::{NoOptimizer, StandardOptimizer};

        if options.optimize {
            self.build_with_options_and_optimizer(options, StandardOptimizer::new())
        } else {
            self.build_with_options_and_optimizer(options, NoOptimizer)
        }
    }

    fn build_with_options_and_optimizer(
        &self,
        options: LV2CompileOptions,
        mut optimizer: impl Optimizer,
    ) -> LV2CompileResult<LV2Module> {
        let mut ru = LV2HirLoweringRuntime::new(self.meta.clone(), options, &mut optimizer);

        // Main entry point must be at start (offset 0)
        let entry_key = LV2Variable::from(LV2_ENTRY_POINT);

        if let Some((key, hir)) = self.hirs.iter().find(|(k, _)| **k == entry_key) {
            ru.emit(LirElement::entry(key));
            hir.build(&mut ru)?;
        } else {
            // If the module does not have an entry point, a return should be lowered by default
            // to avoid fallthrough into another function.
            ru.emit(LirElement::Ret);
        }

        // Lower non-entry points after first one
        for (key, hir) in self.hirs.iter().filter(|(k, _)| **k != entry_key) {
            ru.emit(LirElement::entry(key));
            hir.build(&mut ru)?;
        }

        let mut module: LV2Module = ru.complete()?.into();

        // Make all function offsets of the `LV2CodeObject` available as module slots.
        for (iidx, offset) in module.code_object.entries.iter() {
            let key = &module.code_object.idents[*iidx];
            let func = LV2CodeObjectFunction::from(module.code_object.clone(), *offset);

            module.slots.insert(key.clone(), Rc::new(func));
        }

        Ok(module)
    }

    /// Create a new function handle with the [LV2_ENTRY_POINT] name.
    pub fn entry(&mut self) -> &mut LV2Function {
        let name = LV2Variable::from(LV2_ENTRY_POINT);

        if !self.hirs.contains_key(&name) {
            self.hirs.insert(name.clone(), LV2Function::new());
        }

        self.hirs.get_mut(&name).unwrap()
    }
}
