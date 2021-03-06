//! Building modules from Hir

use std::collections::HashMap;
use std::rc::Rc;

use crate::code::CodeObjectFunction;
use crate::error::*;
use crate::gen::opt::Optimizer;
use crate::gen::{CompileOptions, Hir, HirLoweringRuntime, LirElement, ModuleMeta};
use crate::module::{Module, ENTRY_POINT};
use crate::var::Variable;

/// Representation of modules before lowering.
#[derive(Clone)]
pub struct ModuleBuilder {
    /// Meta information about name, location and static uses.
    meta: ModuleMeta,
    /// Functions contained in the module.
    hirs: HashMap<Variable, Hir>,
}

impl ModuleBuilder {
    /// Create a builder.
    pub fn new() -> Self {
        Self {
            meta: ModuleMeta::default(),
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
        T: Into<ModuleMeta>,
    {
        Self {
            meta: meta.into(),
            hirs: HashMap::new(),
        }
    }

    /// Add a module dependency for static inclusion.
    pub fn add_dependency<T>(&mut self, name: T)
    where
        T: ToString,
    {
        let name = name.to_string();

        if !self.meta.uses.contains(&name) {
            self.meta.uses.push(name);
        }
    }

    /// Create a new function hir without arguments.
    pub fn add<T>(&mut self, name: T) -> &mut Hir
    where
        T: Into<Variable>,
    {
        self.add_with_args(name, vec![])
    }

    /// Create a new function hir with arguments.
    pub fn add_with_args<T>(&mut self, name: T, args: Vec<Variable>) -> &mut Hir
    where
        T: Into<Variable>,
    {
        let name: Variable = name.into();

        self.hirs.insert(name.clone(), Hir::with_args(args));
        self.hirs.get_mut(&name).unwrap()
    }

    /// Generate a module from the current data. This uses the default [CompileOptions] e.g.
    /// optimization is enabled.
    pub fn build(&self) -> Lovm2CompileResult<Module> {
        self.build_with_options(CompileOptions::default())
    }

    /// Generate a module from the current data but use custom compile options.
    pub fn build_with_options(&self, options: CompileOptions) -> Lovm2CompileResult<Module> {
        use crate::gen::{NoOptimizer, StandardOptimizer};

        if options.optimize {
            self.build_with_options_and_optimizer(options, StandardOptimizer::new())
        } else {
            self.build_with_options_and_optimizer(options, NoOptimizer)
        }
    }

    fn build_with_options_and_optimizer(
        &self,
        options: CompileOptions,
        mut optimizer: impl Optimizer,
    ) -> Lovm2CompileResult<Module> {
        let mut ru = HirLoweringRuntime::new(self.meta.clone(), options, &mut optimizer);

        // Main entry point must be at start (offset 0)
        let entry_key = Variable::from(ENTRY_POINT);

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

        let mut module: Module = ru.complete()?.into();

        // Make all function offsets of the `CodeObject` available as module slots.
        for (iidx, offset) in module.code_object.entries.iter() {
            let key = &module.code_object.idents[*iidx];
            let func = CodeObjectFunction::from(module.code_object.clone(), *offset);

            module.slots.insert(key.clone(), Rc::new(func));
        }

        Ok(module)
    }

    /// Create a new function handle with the [ENTRY_POINT] name.
    pub fn entry(&mut self) -> &mut Hir {
        let name = Variable::from(ENTRY_POINT);

        if !self.hirs.contains_key(&name) {
            self.hirs.insert(name.clone(), Hir::new());
        }

        self.hirs.get_mut(&name).unwrap()
    }
}
