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
pub type GenericModule = Rc<dyn ModuleProtocol>;

/// generalization for loadable modules
/// - lovm2 bytecode ([Module](/latest/lovm2/module/struct.Module.html))
/// - shared objects `.so`
/// ([SharedObjectModule](/latest/lovm2/module/shared/struct.SharedObjectModule.html))
pub trait ModuleProtocol: std::fmt::Debug {
    fn name(&self) -> &str {
        unimplemented!()
    }

    fn location(&self) -> Option<&String> {
        None
    }

    fn slots(&self) -> &Slots {
        unimplemented!()
    }

    fn slot(&self, _name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        unimplemented!()
    }

    fn store_to_file(&self, _path: &str) -> Lovm2Result<()> {
        unimplemented!()
    }

    fn uses(&self) -> &[String] {
        &[]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Module {
    // TODO: can we skip this?
    //#[serde(skip_deserializing)]
    //#[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub loc: Option<String>,

    pub slots: Slots,
    pub uses: Vec<String>,
}

impl Into<GenericModule> for Module {
    fn into(self) -> GenericModule {
        Rc::new(self) as GenericModule
    }
}

impl ModuleProtocol for Module {
    fn name(&self) -> &str {
        &self.name
    }

    fn location(&self) -> Option<&String> {
        self.loc.as_ref()
    }

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

    fn uses(&self) -> &[String] {
        self.uses.as_ref()
    }
}

impl Module {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            loc: None,
            slots: Slots::new(),
            uses: vec![],
        }
    }

    /// tries to load the file as shared object first and tries regular deserialization if it failed
    pub fn load_from_file<T>(path: T) -> Lovm2Result<GenericModule>
    where
        T: AsRef<Path>,
    {
        // try loading module as shared object
        if let Ok(so_module) = SharedObjectModule::load_from_file(&path) {
            return Ok(Rc::new(so_module) as GenericModule);
        }

        use bincode::Options;
        let name = path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let loc = path.as_ref().to_str().unwrap().to_string();
        let file = File::open(path).map_err(|e| e.to_string())?;
        // avoid misinterpreting random bytes as length of buffer
        // this could lead to memory allocation faults
        let mut module: Module = bincode::options()
            .with_varint_encoding()
            .deserialize_from(file)
            .map_err(|e| e.to_string())?;
        module.name = name;
        module.loc = Some(loc);

        for (_, slot) in module.slots.iter_mut() {
            Rc::get_mut(slot).unwrap().set_module(module.name.clone());
        }

        Ok(Rc::new(module) as GenericModule)
    }
}
