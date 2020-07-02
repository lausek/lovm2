use libloading::{Error, Library, Symbol};
use std::ops::Deref;
use std::path::Path;

use crate::code::{CallProtocol, ExternFunction};
use crate::module::ModuleProtocol;
use crate::var::Variable;

pub struct SharedObjectModule {
    lib: Library,
}

impl ModuleProtocol for SharedObjectModule {
    fn slot(&self, name: &Variable) -> Option<Box<dyn Deref<Target = dyn CallProtocol>>> {
        unsafe {
            let lookup: Result<Symbol<ExternFunction>, Error> = self.lib.get(name.as_bytes());
            match lookup {
                Ok(symbol) => {
                    None
                }
                Err(_) => None,
            }
        }
    }
}

impl SharedObjectModule {
    pub fn from_library(lib: Library) -> Self {
        Self {
            lib,
        }
    }

    pub fn load_from_file<T>(path: T) -> Result<SharedObjectModule, String>
    where
        T: AsRef<Path>,
    {
        match Library::new(path.as_ref().as_os_str()) {
            Ok(lib) => Ok(SharedObjectModule::from_library(lib)),
            Err(err) => Err(err.to_string()),
        }
    }
}
