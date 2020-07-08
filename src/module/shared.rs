use libloading::{Error, Library, Symbol};
use std::path::Path;
use std::rc::Rc;

use crate::code::{CallProtocol, ExternFunction};
use crate::context::Context;
use crate::module::{ModuleProtocol, Slots};
use crate::var::Variable;

pub struct SharedObjectModule {
    lib: Rc<Library>,
}

impl ModuleProtocol for SharedObjectModule {
    fn slots(&self) -> Slots {
        unimplemented!()
    }

    fn slot(&self, name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        unsafe {
            let lookup: Result<Symbol<ExternFunction>, Error> = self.lib.get(name.as_bytes());
            match lookup {
                Ok(_) => Some(
                    Rc::new(SharedObjectSlot(self.lib.clone(), name.to_string()))
                        as Rc<dyn CallProtocol>,
                ),
                Err(_) => None,
            }
        }
    }
}

impl SharedObjectModule {
    pub fn from_library(lib: Library) -> Self {
        Self { lib: Rc::new(lib) }
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

pub struct SharedObjectSlot(Rc<Library>, String);

impl CallProtocol for SharedObjectSlot {
    fn run(&self, ctx: &mut Context) -> Result<(), String> {
        unsafe {
            let (lib, name) = (&self.0, &self.1);
            let lookup: Result<Symbol<ExternFunction>, Error> = lib.get(name.as_bytes());
            match lookup {
                Ok(symbol) => symbol(ctx),
                Err(err) => Err(err.to_string()),
            }
        }
    }
}

impl std::fmt::Debug for SharedObjectSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<extern function>")
    }
}
