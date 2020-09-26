//! collections of runnable objects
//!
//! if a module gets loaded by the virtual machine, its code objects are not available by default.
//! code objects need to be added to the scope to be callable by lovm2 bytecode.

pub mod builder;
pub mod shared;
pub mod slots;
pub mod standard;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use lovm2_error::*;

use crate::code::CallProtocol;
use crate::var::Variable;

pub use self::builder::ModuleBuilder;
pub use self::shared::SharedObjectModule;
pub use self::slots::Slots;
pub use self::standard::create_standard_module;

pub const ENTRY_POINT: &str = "main";

/// generalization for loadable modules
/// - lovm2 bytecode ([Module](/latest/lovm2/module/struct.Module.html))
/// - shared objects `.so`
/// ([SharedObjectModule](/latest/lovm2/module/shared/struct.SharedObjectModule.html))
pub trait ModuleProtocol: std::fmt::Debug {
    fn slots(&self) -> &Slots {
        unimplemented!()
    }

    fn slot(&self, _name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        unimplemented!()
    }

    fn store_to_file(&self, _path: &str) -> Lovm2Result<()> {
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

    // TODO: could lead to errors when two threads serialize to the same file
    fn store_to_file(&self, path: &str) -> Lovm2Result<()> {
        use bincode::Options;
        let file = File::create(path).map_err(|e| e.to_string())?;
        bincode::options()
            .with_varint_encoding()
            .serialize_into(file, self)
            .map_err(|e| e.to_string().into())
    }
}

impl Module {
    pub fn new() -> Self {
        Self {
            slots: Slots::new(),
        }
    }

    /// tries to load the file as shared object first and tries regular deserialization if it failed
    pub fn load_from_file<T>(path: T) -> Lovm2Result<Box<dyn ModuleProtocol>>
    where
        T: AsRef<Path>,
    {
        // try loading module as shared object
        if let Ok(so_module) = SharedObjectModule::load_from_file(&path) {
            return Ok(Box::new(so_module) as Box<dyn ModuleProtocol>);
        }

        use bincode::Options;
        let file = File::open(path).map_err(|e| e.to_string())?;
        // avoid misinterpreting random bytes as length of buffer
        // this could lead to memory allocation faults
        let module: Module = bincode::options()
            .with_varint_encoding()
            .deserialize_from(file)
            .map_err(|e| e.to_string())?;
        Ok(Box::new(module) as Box<dyn ModuleProtocol>)
    }
}
