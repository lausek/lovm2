use std::collections::HashMap;
use std::rc::Rc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use lovm2::gen::LV2AddStatements as _;

use crate::code::CodeObject;
use crate::expr::{any_to_expr, any_to_ident, Expr};
use crate::module::slot::LV2Block;

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
            name: name.unwrap_or(lovm2::prelude::LV2_DEFAULT_MODULE_NAME.to_string()),
            slots: HashMap::new(),
            uses: vec![],
        }
    }

    pub fn add(&mut self, py: Python, name: String, args: Option<&PyList>) -> Py<LV2Block> {
        let slot = if let Some(args) = args {
            ModuleBuilderSlot::with_args(args)
        } else {
            ModuleBuilderSlot::new()
        };

        self.slots.insert(name.clone(), slot);

        self.slots.get_mut(&name).unwrap().code().unwrap()
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
    pub fn build(&mut self, module_location: Option<String>, py: Python) -> PyResult<Module> {
        let meta =
            lovm2::gen::LV2ModuleMeta::new(self.name.clone(), module_location, self.uses.clone());
        let mut builder = lovm2::gen::LV2ModuleBuilder::with_meta(meta);
        let mut slots = lovm2::module::Slots::new();

        for (key, co_builder) in self.slots.drain() {
            let mut co_builder: ModuleBuilderSlot = co_builder;

            match &mut co_builder {
                ModuleBuilderSlot::Lovm2Hir(ref mut hir) => {
                    let mut f = lovm2::prelude::LV2Function::with_args(hir.0.clone());
                    f.extend(hir.1.as_ref(py).borrow().inner.clone());
                    *builder.add(key) = f;
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

    pub fn entry(&mut self, py: Python) -> Py<LV2Block> {
        let name = lovm2::module::LV2_ENTRY_POINT.to_string();
        if !self.slots.contains_key(&name) {
            self.add(py, name, None)
        } else {
            self.slots.get_mut(&name).unwrap().code().unwrap()
        }
    }
}

#[pyclass(unsendable)]
pub struct BranchBuilder {
    inner: Py<lovm2::prelude::LV2Branch>,
}

#[pymethods]
impl BranchBuilder {
    pub fn add_condition(&mut self, condition: &PyAny) -> PyResult<LV2Block> {
        let condition = any_to_expr(condition)?;
        todo!()

        // unsafe {
        //     let inner = (*self.inner).add_condition(condition) as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
    }

    pub fn default_condition(&mut self) -> PyResult<LV2Block> {
        todo!()
        // unsafe {
        //     let inner = (*self.inner).default_condition() as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
    }
}

/*
pub struct BlockBuilder {
    pub(super) inner: lovm2::prelude::LV2Block,
}
*/

#[pymethods]
impl LV2Block {
    pub fn assign(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here

        // unsafe {
        //     (*self.inner).assign(&any_to_ident(n)?, any_to_expr(expr)?);
        // }

        Ok(())
    }

    pub fn assign_global(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here

        let ident = &any_to_ident(n)?;

        self.inner.global(ident);
        self.inner.assign(ident, any_to_expr(expr)?);

        Ok(())
    }

    pub fn set(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here

        // unsafe {
        //     (*self.inner).set(any_to_expr(n)?, any_to_expr(expr)?);
        // }

        Ok(())
    }

    pub fn branch(&mut self) -> PyResult<BranchBuilder> {
        // unsafe {
        //     let inner = (*self.inner).branch() as *mut lovm2::prelude::LV2Branch;

        //     Ok(BranchBuilder { inner })
        // }
        todo!()
    }

    #[args(args = "*")]
    pub fn call(&mut self, name: String, args: &PyTuple) -> PyResult<()> {
        let mut call = lovm2::prelude::LV2Call::new(name);

        for arg in args.into_iter() {
            call = call.arg(any_to_expr(arg)?);
        }

        // unsafe {
        //     (*self.inner).step(call);
        // }

        Ok(())
    }

    pub fn expr(&mut self, expr: &Expr) -> PyResult<()> {
        match &expr.inner {
            lovm2::prelude::LV2Expr::Call(call) => unsafe {
                //(*self.inner).step(call.clone());
                Ok(())
            },
            _ => Err(PyRuntimeError::new_err(
                "expression cannot be placed here.".to_string(),
            )),
        }
    }

    pub fn load(&mut self, name: &Expr) -> PyResult<()> {
        // unsafe {
        //     (*self.inner).import(name.inner.clone());
        // }

        Ok(())
    }

    pub fn interrupt(&mut self, id: u16) -> PyResult<()> {
        self.inner.trigger(id);
        // unsafe {
        //     (*self.inner).trigger(id);
        // }

        Ok(())
    }

    pub fn repeat(&mut self) -> PyResult<LV2Block> {
        // unsafe {
        //     let repeat = (*self.inner).repeat();
        //     let inner = repeat.block_mut() as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
        todo!()
    }

    pub fn repeat_break(&mut self) -> PyResult<()> {
        // unsafe {
        //     (*self.inner).break_repeat();
        // }

        Ok(())
    }

    pub fn repeat_continue(&mut self) -> PyResult<()> {
        // unsafe {
        //     (*self.inner).continue_repeat();
        // }

        Ok(())
    }

    pub fn repeat_until(&mut self, condition: &PyAny) -> PyResult<LV2Block> {
        let condition = any_to_expr(condition)?;

        // unsafe {
        //     let repeat = (*self.inner).repeat_until(condition);
        //     let inner = repeat.block_mut() as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
        todo!()
    }

    pub fn repeat_iterating(&mut self, collection: &PyAny, item: String) -> PyResult<LV2Block> {
        let collection = any_to_expr(collection)?;

        // unsafe {
        //     let repeat = (*self.inner).repeat_iterating(collection, item);
        //     let inner = repeat.block_mut() as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
        todo!()
    }

    pub fn ret(&mut self, val: &PyAny) -> PyResult<()> {
        // unsafe {
        //     let val = any_to_expr(val)?;

        //     (*self.inner).return_value(val);
        // }

        Ok(())
    }
}
