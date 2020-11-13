use std::collections::HashMap;
use std::rc::Rc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use lovm2::hir;
use lovm2::var;

use crate::expr::{any_to_access, any_to_expr, any_to_ident, Expr};

use super::{Lovm2Block, Lovm2Branch, Lovm2CallableModule, Lovm2Module, Module, ModuleBuilderSlot};

#[pyclass(unsendable)]
pub struct ModuleBuilder {
    name: String,
    slots: HashMap<String, Py<ModuleBuilderSlot>>,
    uses: Vec<String>,
}

#[pymethods]
impl ModuleBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            name: "<unknown>".to_string(),
            slots: HashMap::new(),
            uses: vec![],
        }
    }

    #[classmethod]
    pub fn named(_this: &PyAny, name: String) -> Self {
        Self {
            name,
            slots: HashMap::new(),
            uses: vec![],
        }
    }

    pub fn add(&mut self, py: Python, name: String) -> Py<ModuleBuilderSlot> {
        let inst = Py::new(py, ModuleBuilderSlot::new()).unwrap();
        self.slots.insert(name.clone(), inst);
        self.slots.get(&name).unwrap().clone_ref(py)
    }

    pub fn add_dependency(&mut self, name: String) {
        if !self.uses.contains(&name) {
            self.uses.push(name);
        }
    }

    pub fn add_slot(&mut self, py: Python, name: String, slot: ModuleBuilderSlot) -> PyResult<()> {
        let inst = Py::new(py, slot).unwrap();
        self.slots.insert(name.clone(), inst);
        Ok(())
    }

    // TODO: can we avoid duplicating the code here?
    pub fn build(&mut self, py: Python, module_location: Option<String>) -> PyResult<Module> {
        let mut module = Lovm2CallableModule::new();
        module.name = self.name.clone();
        module.loc = module_location;
        module.uses = self.uses.clone();

        for (key, co_builder) in self.slots.drain() {
            let mut co_builder: PyRefMut<ModuleBuilderSlot> = co_builder.as_ref(py).borrow_mut();
            match co_builder.complete() {
                Ok(mut co) => {
                    /*
                    {
                        use lovm2::code::CallProtocol;
                        co.set_module(module.name.clone());
                    }
                    */

                    module.slots.insert(var::Variable::from(key), Rc::new(co));
                }
                Err(msg) => return Err(msg),
            }
        }

        Ok(Module::from(module))
    }

    pub fn entry(&mut self, py: Python) -> Py<ModuleBuilderSlot> {
        let name = lovm2::module::ENTRY_POINT.to_string();
        if !self.slots.contains_key(&name) {
            let inst = Py::new(py, ModuleBuilderSlot::new()).unwrap();
            self.slots.insert(name.clone(), inst);
        }
        self.slots.get(&name).unwrap().clone_ref(py)
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
            (*self.inner).push(Assign::local(any_to_ident(n)?, any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn assign_global(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Assign::global(any_to_ident(n)?, any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn set(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Assign::set(any_to_access(n)?, any_to_expr(expr)?));
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
            (*self.inner).push(call);
        }
        Ok(())
    }

    pub fn expr(&mut self, expr: &Expr) -> PyResult<()> {
        match &expr.inner {
            hir::expr::Expr::Call(call) => unsafe {
                (*self.inner).push(call.clone());
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
            (*self.inner).push(Include::load(name.inner.clone()));
        }
        Ok(())
    }

    pub fn interrupt(&mut self, id: u16) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Interrupt::new(id));
        }
        Ok(())
    }

    pub fn repeat(&mut self) -> PyResult<BlockBuilder> {
        unsafe {
            let repeat = (*self.inner).repeat(None);
            let inner = &mut repeat.block as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn repeat_break(&mut self) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Break::new());
        }
        Ok(())
    }

    pub fn repeat_continue(&mut self) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Continue::new());
        }
        Ok(())
    }

    pub fn repeat_until(&mut self, condition: &PyAny) -> PyResult<BlockBuilder> {
        let condition = any_to_expr(condition)?;
        unsafe {
            let repeat = (*self.inner).repeat(Some(condition));
            let inner = &mut repeat.block as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn ret(&mut self, val: &PyAny) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            let val = any_to_expr(val)?;
            (*self.inner).push(Return::value(val));
        }
        Ok(())
    }
}
