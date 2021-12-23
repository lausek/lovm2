use std::collections::HashMap;
use std::rc::Rc;

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

    pub fn add(&mut self, name: String, args: Option<&PyList>) -> LV2Block {
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
    pub fn build(&mut self, module_location: Option<String>) -> PyResult<Module> {
        let meta =
            lovm2::gen::LV2ModuleMeta::new(self.name.clone(), module_location, self.uses.clone());
        let mut builder = lovm2::gen::LV2ModuleBuilder::with_meta(meta);
        let mut slots = lovm2::module::Slots::new();

        for (key, co_builder) in self.slots.drain() {
            let mut co_builder: ModuleBuilderSlot = co_builder;

            match &mut co_builder {
                ModuleBuilderSlot::LV2Function(ref mut f) => {
                    *builder.add(key) = f.clone();
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

    pub fn entry(&mut self) -> LV2Block {
        let name = lovm2::module::LV2_ENTRY_POINT.to_string();
        if !self.slots.contains_key(&name) {
            self.add(name, None)
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

    pub fn assign_global(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        let (target, source) = (&any_to_ident(target)?, any_to_expr(source)?);
        self.block().global(target);
        self.block().assign(target, source);
        Ok(())
    }

    pub fn set(&mut self, target: &PyAny, source: &PyAny) -> PyResult<()> {
        let (target, source) = (any_to_expr(target)?, any_to_expr(source)?);
        self.block().set(target, source);
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

    pub fn load(&mut self, name: &Expr) -> PyResult<()> {
        self.block().import(name.inner.clone());
        Ok(())
    }

    pub fn interrupt(&mut self, id: u16) -> PyResult<()> {
        self.block().trigger(id);
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
}
