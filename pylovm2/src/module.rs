use std::collections::HashMap;
use std::rc::Rc;

use pyo3::exceptions::*;
use pyo3::prelude::*;

use lovm2::hir;
use lovm2::module;
use lovm2::value;

use crate::code::CodeObject;

#[pyclass]
pub struct Module {
    pub inner: Option<module::Module>,
}

impl Module {
    pub fn from(inner: module::Module) -> Self {
        Self {
            inner: Some(inner),
        }
    }
}

#[pyclass]
pub struct ModuleBuilder {
    slots: HashMap<String, Py<ModuleBuilderSlot>>,
}

#[pymethods]
impl ModuleBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn add(&mut self, py: Python, name: String) -> Py<ModuleBuilderSlot>
    {
        let inst = Py::new(py, ModuleBuilderSlot::new()).unwrap();
        self.slots.insert(name.clone(), inst);
        self.slots.get(&name).unwrap().clone_ref(py)
    }

    pub fn build(&mut self, py: Python) -> PyResult<Module> {
        let mut module = module::Module::new();

        for (key, co_builder) in self.slots.drain() {
            let mut co_builder: PyRefMut<ModuleBuilderSlot> = co_builder.as_ref(py).borrow_mut();
            match co_builder.complete() {
                Ok(co) => {
                    module.slots.insert(key, Rc::new(co));
                }
                Err(msg) => return Err(msg),
            }
        }

        Ok(Module::from(module))
    }
}

#[pyclass]
pub struct ModuleBuilderSlot {
    inner: Option<hir::HIR>,
}

#[pymethods]
impl ModuleBuilderSlot {
    #[new]
    pub fn new() -> Self {
        Self { inner: Some(hir::HIR::new()) }
    }

    pub fn assign(&mut self, n: String, expr: i64) {
        use lovm2::prelude::*;
        self.inner.as_mut().unwrap().push(Assign::local(n, value::CoValue::Int(expr).into()));
    }

    pub fn call(&mut self, name: String, arg1: String) {
        use lovm2::prelude::*;
        self.inner.as_mut().unwrap().push(Call::new(name).arg(value::CoValue::Str(arg1)));
    }

    pub fn complete(&mut self) -> PyResult<CodeObject> {
        if let Some(hir) = self.inner.take() {
            return match hir.build() {
                Ok(co) => Ok(CodeObject::from(co)),
                Err(msg) => TypeError::into(msg),
            };
        }
        TypeError::into("hir was already built")
    }
}
