pub mod builder;
pub mod shared;
pub mod slots;
pub mod standard;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use crate::code::CallProtocol;
use crate::var::Variable;

pub use self::builder::ModuleBuilder;
pub use self::shared::SharedObjectModule;
pub use self::slots::Slots;
pub use self::standard::create_standard_module;

pub trait ModuleProtocol {
    fn slots(&self) -> &Slots {
        unimplemented!()
    }

    fn slot(&self, _name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Module {
    pub slots: Slots,
}

impl Into<Box<dyn ModuleProtocol>> for Module {
    fn into(self) -> Box<dyn ModuleProtocol> {
        Box::new(self) as Box<dyn ModuleProtocol>
    }
}

impl ModuleProtocol for Module {
    fn slots(&self) -> &Slots {
        &self.slots
    }

    fn slot(&self, name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        self.slots
            .get(name)
            .map(|co_ref| co_ref.clone() as Rc<dyn CallProtocol>)
    }
}

impl Module {
    pub fn new() -> Self {
        Self {
            slots: Slots::new(),
        }
    }

    pub fn load_from_file<T>(path: T) -> Result<Box<dyn ModuleProtocol>, String>
    where
        T: AsRef<Path>,
    {
        // try loading module as shared object
        if let Ok(so_module) = SharedObjectModule::load_from_file(&path) {
            return Ok(Box::new(so_module) as Box<dyn ModuleProtocol>);
        }

        let file = File::open(path).map_err(|e| e.to_string())?;
        let module: Module = serde_cbor::from_reader(file).map_err(|e| e.to_string())?;
        Ok(Box::new(module) as Box<dyn ModuleProtocol>)
    }

    // TODO: could lead to errors when two threads serialize to the same file
    pub fn store_to_file<T>(&self, path: T) -> Result<(), String>
    where
        T: AsRef<Path>,
    {
        let file = File::create(path).map_err(|e| e.to_string())?;
        serde_cbor::to_writer(file, self).map_err(|e| e.to_string())?;
        Ok(())
    }
}
