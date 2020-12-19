use std::collections::HashMap;
use std::rc::Rc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use lovm2::gen::{hir, HasBlock};
use lovm2::module::{ModuleMeta, DEFAULT_MODULE_NAME};

use crate::code::CodeObject;
use crate::expr::{any_to_access, any_to_expr, any_to_ident, Expr};
use crate::lv2::*;

use super::{Module, ModuleBuilderSlot};

#[pyclass(unsendable)]
pub struct ModuleBuilder {
    name: String,
    slots: HashMap<String, ModuleBuilderSlot>,
    uses: Vec<String>,
}

#[pymethods]
impl ModuleBuilder {
    #[new]
    pub fn new(name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or(DEFAULT_MODULE_NAME.to_string()),
            slots: HashMap::new(),
            uses: vec![],
        }
    }

    pub fn add(&mut self, py: Python, name: String, args: Option<&PyList>) -> Py<BlockBuilder> {
        let slot = if let Some(args) = args {
            ModuleBuilderSlot::with_args(args)
        } else {
            ModuleBuilderSlot::new()
        };

        self.slots.insert(name.clone(), slot);
        let block = self.slots.get_mut(&name).unwrap().code().unwrap();

        Py::new(py, block).unwrap()
    }

    pub fn add_pyfn(&mut self, name: String, pyfn: PyObject) -> PyResult<()> {
        self.slots.insert(name, ModuleBuilderSlot::pyfn(pyfn));
        Ok(())
    }

    pub fn add_dependency(&mut self, name: String) {
        if !self.uses.contains(&name) {
            self.uses.push(name);
        }
    }

    // TODO: can we avoid duplicating the code here?
    pub fn build(&mut self, module_location: Option<String>) -> PyResult<Module> {
        let meta = ModuleMeta::new(self.name.clone(), module_location, self.uses.clone());
        let mut builder = Lovm2ModuleBuilder::with_meta(meta);
        let mut slots = Lovm2Slots::new();

        for (key, co_builder) in self.slots.drain() {
            let mut co_builder: ModuleBuilderSlot = co_builder;

            match &mut co_builder {
                ModuleBuilderSlot::Lovm2Hir(ref mut hir) => {
                    *builder.add(key) = hir.clone();
                }
                ModuleBuilderSlot::PyFn(ref mut pyfn) => {
                    let func = CodeObject::from(pyfn.clone());
                    slots.insert(key, Rc::new(func));
                }
            }
        }

        // TODO: correctly raise error
        let mut module = builder.build().unwrap();

        for (key, callable) in slots.iter() {
            module.slots.insert(key.clone(), callable.clone());
        }

        Ok(Module::from(module))
    }

    pub fn entry(&mut self, py: Python) -> Py<BlockBuilder> {
        let name = lovm2::module::ENTRY_POINT.to_string();
        if !self.slots.contains_key(&name) {
            self.add(py, name, None)
        } else {
            let block = self.slots.get_mut(&name).unwrap().code().unwrap();
            Py::new(py, block).unwrap()
        }
    }
}

#[pyclass(unsendable)]
pub struct BranchBuilder {
    inner: *mut Lovm2Branch,
}

#[pymethods]
impl BranchBuilder {
    pub fn add_condition(&mut self, condition: &PyAny) -> PyResult<BlockBuilder> {
        let condition = any_to_expr(condition)?;
        unsafe {
            let inner = (*self.inner).add_condition(condition) as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn default_condition(&mut self) -> PyResult<BlockBuilder> {
        unsafe {
            let inner = (*self.inner).default_condition() as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }
}

#[pyclass(unsendable)]
pub struct BlockBuilder {
    pub(super) inner: *mut Lovm2Block,
}

#[pymethods]
impl BlockBuilder {
    pub fn assign(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).step(Assign::local(&any_to_ident(n)?, any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn assign_global(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).step(Assign::global(&any_to_ident(n)?, any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn set(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).step(Assign::set(&any_to_access(n)?, any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn branch(&mut self) -> PyResult<BranchBuilder> {
        unsafe {
            let inner = (*self.inner).branch() as *mut Lovm2Branch;
            Ok(BranchBuilder { inner })
        }
    }

    #[args(args = "*")]
    pub fn call(&mut self, name: String, args: &PyTuple) -> PyResult<()> {
        use lovm2::prelude::*;
        let mut call = Call::new(name);
        for arg in args.into_iter() {
            call = call.arg(any_to_expr(arg)?);
        }
        unsafe {
            (*self.inner).step(call);
        }
        Ok(())
    }

    pub fn expr(&mut self, expr: &Expr) -> PyResult<()> {
        match &expr.inner {
            hir::Expr::Call(call) => unsafe {
                (*self.inner).step(call.clone());
                Ok(())
            },
            _ => Err(PyRuntimeError::new_err(
                "expression cannot be placed here.".to_string(),
            )),
        }
    }

    pub fn load(&mut self, name: &Expr) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).step(Include::import(name.inner.clone()));
        }
        Ok(())
    }

    pub fn interrupt(&mut self, id: u16) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).step(Interrupt::new(id));
        }
        Ok(())
    }

    pub fn repeat(&mut self) -> PyResult<BlockBuilder> {
        unsafe {
            let repeat = (*self.inner).repeat();
            let inner = repeat.block_mut() as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn repeat_break(&mut self) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).step(Break::new());
        }
        Ok(())
    }

    pub fn repeat_continue(&mut self) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).step(Continue::new());
        }
        Ok(())
    }

    pub fn repeat_until(&mut self, condition: &PyAny) -> PyResult<BlockBuilder> {
        let condition = any_to_expr(condition)?;
        unsafe {
            let repeat = (*self.inner).repeat_until(condition);
            let inner = repeat.block_mut() as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn ret(&mut self, val: &PyAny) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            let val = any_to_expr(val)?;
            (*self.inner).step(Return::value(val));
        }
        Ok(())
    }
}
