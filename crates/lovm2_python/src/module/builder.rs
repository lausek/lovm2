use std::collections::HashMap;
use std::rc::Rc;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use lovm2::gen::LV2AddStatements as _;

use crate::code::LV2CodeObject;
use crate::expr::{any_to_expr, any_to_ident};
use crate::module::slot::LV2Block;

use super::{LV2Module, ModuleBuilderSlot};

#[pyclass(unsendable)]
pub struct LV2ModuleBuilder {
    name: String,
    slots: HashMap<String, ModuleBuilderSlot>,
}

#[pymethods]
impl LV2ModuleBuilder {
    #[new]
    pub fn new(name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or(lovm2::prelude::LV2_DEFAULT_MODULE_NAME.to_string()),
            slots: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, args: Option<&PyList>) -> PyResult<LV2Block> {
        let slot = if let Some(args) = args {
            ModuleBuilderSlot::with_args(args)?
        } else {
            ModuleBuilderSlot::new()
        };

        self.slots.insert(name.clone(), slot);
        let block = self.slots.get_mut(&name).unwrap().code().unwrap();

        Ok(block)
    }

    pub fn add_pyfn(&mut self, name: String, pyfn: PyObject) -> PyResult<()> {
        self.slots.insert(name, ModuleBuilderSlot::pyfn(pyfn));

        Ok(())
    }

    // TODO: can we avoid duplicating the code here?
    pub fn build(&mut self, module_location: Option<String>) -> PyResult<LV2Module> {
        let meta =
            lovm2::gen::LV2ModuleMeta::new(self.name.clone(), module_location);
        let mut builder = lovm2::gen::LV2ModuleBuilder::with_meta(meta);
        let mut slots = lovm2::module::LV2ModuleSlots::new();

        for (key, co_builder) in self.slots.drain() {
            let mut co_builder: ModuleBuilderSlot = co_builder;

            match &mut co_builder {
                ModuleBuilderSlot::LV2Function(ref mut f) => {
                    *builder.add(key) = f.clone();
                }
                ModuleBuilderSlot::PyFn(ref mut pyfn) => {
                    let func = LV2CodeObject::from(pyfn.clone());

                    slots.insert(key, Rc::new(func));
                }
            }
        }

        // TODO: correctly raise error
        let mut module = builder.build().unwrap();

        for (key, callable) in slots.iter() {
            module.slots.insert(key.clone(), callable.clone());
        }

        Ok(LV2Module::from(module))
    }

    pub fn entry(&mut self) -> PyResult<LV2Block> {
        let name = lovm2::module::LV2_ENTRY_POINT.to_string();
        if !self.slots.contains_key(&name) {
            self.add(name, None)
        } else {
            let block = self.slots.get_mut(&name).unwrap().code().unwrap();
            Ok(block)
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
        unsafe { &mut *self.inner }
    }
}

#[pymethods]
impl LV2Branch {
    pub fn add_condition(&mut self, condition: &PyAny) -> PyResult<LV2Block> {
        let condition = any_to_expr(condition)?;
        let block = self.branch().add_condition(condition) as *mut lovm2::prelude::LV2Block;
        Ok(LV2Block::from_ptr(block))
    }

    pub fn default_condition(&mut self) -> PyResult<LV2Block> {
        let block = self.branch().default_condition() as *mut lovm2::prelude::LV2Block;
        Ok(LV2Block::from_ptr(block))
    }
}

#[pymethods]
impl LV2Block {
    pub fn assign(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        let (target, source) = (&any_to_ident(target)?, any_to_expr(source)?);
        self.block().assign(target, source);
        Ok(())
    }

    /// This temporarily set the scope of `target` to global.
    pub fn assign_global(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        let (target, source) = (&any_to_ident(target)?, any_to_expr(source)?);

        self.block().global(target);
        self.block().assign(target, source);
        self.block().local(target);

        Ok(())
    }

    pub fn branch(&mut self) -> PyResult<LV2Branch> {
        let branch = self.block().branch() as *mut lovm2::prelude::LV2Branch;
        Ok(LV2Branch::from_ptr(branch))
    }

    // TODO: this is actually not part of `LV2Block`
    #[args(args = "*")]
    pub fn call(&mut self, name: String, args: &PyTuple) -> PyResult<()> {
        let mut call = lovm2::prelude::LV2Call::new(name);

        for arg in args.into_iter() {
            call = call.arg(any_to_expr(arg)?);
        }

        self.block().step(call);

        Ok(())
    }

    pub fn global_(&mut self, target: &PyAny) -> PyResult<()> {
        let target = any_to_ident(target)?;
        self.block().global(&target);
        Ok(())
    }

    pub fn import_(&mut self, name: &PyAny) -> PyResult<()> {
        let name = any_to_expr(name)?;
        self.block().import(name);
        Ok(())
    }

    pub fn import_from(&mut self, name: &PyAny) -> PyResult<()> {
        let name = any_to_expr(name)?;
        self.block().import_from(name);
        Ok(())
    }

    pub fn local(&mut self, target: &PyAny) -> PyResult<()> {
        let target = any_to_ident(target)?;
        self.block().local(&target);
        Ok(())
    }

    pub fn repeat(&mut self) -> PyResult<LV2Block> {
        let repeat = self.block().repeat();
        let block = repeat.block_mut() as *mut lovm2::prelude::LV2Block;
        Ok(LV2Block::from_ptr(block))
    }

    pub fn repeat_break(&mut self) -> PyResult<()> {
        self.block().break_repeat();
        Ok(())
    }

    pub fn repeat_continue(&mut self) -> PyResult<()> {
        self.block().continue_repeat();
        Ok(())
    }

    pub fn repeat_until(&mut self, condition: &PyAny) -> PyResult<LV2Block> {
        let condition = any_to_expr(condition)?;
        let repeat = self.block().repeat_until(condition);
        let block = repeat.block_mut() as *mut lovm2::prelude::LV2Block;
        Ok(LV2Block::from_ptr(block))
    }

    pub fn repeat_iterating(&mut self, collection: &PyAny, item: &PyAny) -> PyResult<LV2Block> {
        let (collection, item) = (any_to_expr(collection)?, any_to_ident(item)?);
        let repeat = self.block().repeat_iterating(collection, item);
        let block = repeat.block_mut() as *mut lovm2::prelude::LV2Block;
        Ok(LV2Block::from_ptr(block))
    }

    pub fn ret(&mut self, val: &PyAny) -> PyResult<()> {
        let val = any_to_expr(val)?;
        self.block().return_value(val);
        Ok(())
    }

    pub fn set(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        let (target, source) = (any_to_expr(target)?, any_to_expr(source)?);
        self.block().set(target, source);
        Ok(())
    }

    pub fn step(&mut self, expr: &PyAny) -> PyResult<()> {
        let expr = any_to_expr(expr)?;
        self.block().step(expr);
        Ok(())
    }

    pub fn trigger(&mut self, id: u16) -> PyResult<()> {
        self.block().trigger(id);
        Ok(())
    }
}
