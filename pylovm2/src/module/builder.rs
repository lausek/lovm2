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

    pub fn add(&mut self, py: Python, name: String, args: Option<&PyList>) -> LV2Block {
        let slot = if let Some(args) = args {
            ModuleBuilderSlot::with_args(args)
        } else {
            ModuleBuilderSlot::new()
        };

        self.slots.insert(name.clone(), slot);
        
        self.slots.get_mut(&name).unwrap().code().unwrap()
        //let block = unsafe { block as *mut lovm2::prelude::LV2Block };
        //LV2Block::from_ptr(block)
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

    pub fn entry(&mut self, py: Python) -> LV2Block {
        let name = lovm2::module::LV2_ENTRY_POINT.to_string();
        if !self.slots.contains_key(&name) {
            self.add(py, name, None)
        } else {
            self.slots.get_mut(&name).unwrap().code().unwrap()
        }
    }
}

#[pyclass(unsendable)]
pub struct LV2Branch {
    inner: *mut lovm2::prelude::LV2Branch,
}

impl LV2Branch {
    pub fn from_ptr(inner: *mut lovm2::prelude::LV2Branch) -> Self {
        Self { inner }
    }

    pub(crate) fn branch(&mut self) -> &mut lovm2::prelude::LV2Branch {
        unsafe {
            &mut *self.inner
        }
    }
}

#[pymethods]
impl LV2Branch {
    pub fn add_condition(&mut self, condition: &PyAny) -> PyResult<LV2Block> {
        let condition = any_to_expr(condition)?;
        let block = self.branch().add_condition(condition) as *mut lovm2::prelude::LV2Block;
        Ok(LV2Block::from_ptr(block))

        // unsafe {
        //     let inner = (*self.inner).add_condition(condition) as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
    }

    pub fn default_condition(&mut self) -> PyResult<LV2Block> {
        let block = self.branch().default_condition() as *mut lovm2::prelude::LV2Block;
        Ok(LV2Block::from_ptr(block))
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
    pub fn assign(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        let (target, source) = (&any_to_ident(target)?, any_to_expr(source)?);
        self.block().assign(target, source);
        // TODO: allow usage of Expr::Variable here

        // unsafe {
        //     (*self.inner).assign(&any_to_ident(n)?, any_to_expr(expr)?);
        // }

        Ok(())
    }

    pub fn assign_global(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here

        let (target, source) = (&any_to_ident(target)?, any_to_expr(source)?);

        self.block().global(target);
        self.block().assign(target, source);

        Ok(())
    }

    pub fn set(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        let (target, source) = (any_to_expr(target)?, any_to_expr(source)?);
        self.block().set(target, source);
        // TODO: allow usage of Expr::Variable here

        // unsafe {
        //     (*self.inner).set(any_to_expr(n)?, any_to_expr(expr)?);
        // }

        Ok(())
    }

    pub fn branch(&mut self) -> PyResult<LV2Branch> {
        let branch = self.block().branch() as *mut lovm2::prelude::LV2Branch;
        Ok(LV2Branch::from_ptr(branch))
    }

    #[args(args = "*")]
    pub fn call(&mut self, name: String, args: &PyTuple) -> PyResult<()> {
        let mut call = lovm2::prelude::LV2Call::new(name);

        for arg in args.into_iter() {
            call = call.arg(any_to_expr(arg)?);
        }

        self.block().step(call);

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
        self.block().import(name.inner.clone());

        Ok(())
    }

    pub fn interrupt(&mut self, id: u16) -> PyResult<()> {
        self.block().trigger(id);
        // unsafe {
        //     (*self.inner).trigger(id);
        // }

        Ok(())
    }

    pub fn repeat(&mut self, py: Python) -> PyResult<LV2Block> {
        let repeat = self.block().repeat();
        let block = unsafe { repeat.block_mut() as *mut lovm2::prelude::LV2Block };
        Ok(LV2Block::from_ptr(block))
        // unsafe {
        //     let repeat = (*self.inner).repeat();
        //     let inner = repeat.block_mut() as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
    }

    pub fn repeat_break(&mut self) -> PyResult<()> {
        self.block().break_repeat();
        // unsafe {
        //     (*self.inner).break_repeat();
        // }

        Ok(())
    }

    pub fn repeat_continue(&mut self) -> PyResult<()> {
        self.block().continue_repeat();
        // unsafe {
        //     (*self.inner).continue_repeat();
        // }

        Ok(())
    }

    pub fn repeat_until(&mut self, condition: &PyAny, py: Python) -> PyResult<LV2Block> {
        let condition = any_to_expr(condition)?;
        let repeat = self.block().repeat_until(condition);
        let block = unsafe { repeat.block_mut() as *mut lovm2::prelude::LV2Block };
        Ok(LV2Block::from_ptr(block))
        /*
        let repeat = LV2Repeat {
            ty: lovm2::prelude::LV2RepeatType::Until { condition },
            block,
        };
        */
        //let ret = repeat.block.clone();

        //self.inner.step(lovm2::prelude::LV2Statement::embed(std::rc::Rc::new(repeat)));

        // unsafe {
        //     let repeat = (*self.inner).repeat_until(condition);
        //     let inner = repeat.block_mut() as *mut lovm2::prelude::LV2Block;

        //     Ok(BlockBuilder { inner })
        // }
        //todo!()
    }

    pub fn repeat_iterating(&mut self, collection: &PyAny, item: &PyAny, py: Python) -> PyResult<LV2Block> {
        let collection = any_to_expr(collection)?;
        let item = any_to_ident(item)?;
        /*
        let repeat = LV2Repeat {
            ty: lovm2::prelude::LV2RepeatType::Iterating { collection, item },
            block: Py::new(py, LV2Block::new()).unwrap(),
        };
        let ret = repeat.block.clone();
        */

        //self.inner.step(lovm2::prelude::LV2Statement::embed(std::rc::Rc::new(repeat)));

        unsafe {
            let repeat = self.block().repeat_iterating(collection, item);
            let block = repeat.block_mut() as *mut lovm2::prelude::LV2Block;
            Ok(LV2Block::from_ptr(block))
        }
    }

    pub fn ret(&mut self, val: &PyAny) -> PyResult<()> {
        // unsafe {
        //     let val = any_to_expr(val)?;
        //     (*self.inner).return_value(val);
        // }
        //     (*self.inner).return_value(val);
        // }
        let val = any_to_expr(val)?;
        self.block().return_value(val);

        Ok(())
    }
}
