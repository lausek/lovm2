use pyo3::exceptions::*;
use pyo3::prelude::*;

use lovm2::hir;
use lovm2::module;

#[pyclass]
pub struct CodeObject {}

#[pyclass]
pub struct Module {
    inner: module::Module,
}

impl Module {
    pub fn from(inner: module::Module) -> Self {
        Self {
            inner,
        }
    }
}

#[pyclass]
pub struct ModuleBuilder {
    inner: Option<module::builder::ModuleBuilder>,
}

#[pymethods]
impl ModuleBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Some(module::builder::ModuleBuilder::new()),
        }
    }

    pub fn add(&mut self, name: String) -> &mut ModuleBuilderSlot
    {
        if let Some(inner) = self.inner.as_mut() {
            // let name: Variable = name.into();
            inner.slots.insert(name.clone(), ModuleBuilderSlot::new());
            inner.slots.get_mut(&name).unwrap()
        }
    }

    pub fn build(&mut self) -> PyResult<Module> {
        if let Some(module_builder) = self.inner.take() {
            return match module_builder.build() {
                Ok(module) => Ok(Module::from(module)),
                Err(msg) => TypeError::into(msg),
            };
        }
        TypeError::into("module was already built")
    }
}

#[pyclass]
pub struct ModuleBuilderSlot {
    inner: Option<hir::HIR>,
    // hir: Option<HIR>,
}

#[pymethods]
impl ModuleBuilderSlot {
    #[new]
    pub fn new() -> Self {
        Self { inner: Some(hir::HIR::new()) }
    }

    /*
    pub fn hir(&mut self, hir: HIR) {
        self.hir = Some(hir);
    }
    */

    pub fn complete(&mut self) -> PyResult<CodeObject> {
        match self.inner.take() {
            Some(inner) => inner.complete(),
            None => TypeError::into("no hir for slot"),
        }
    }
}

#[pymodule]
pub fn hir(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ModuleBuilder>()?;

    Ok(())
}
