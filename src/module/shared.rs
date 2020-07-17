//! implementation of shared objects as lovm2 modules
//!
//! shared objects must be specifically compiled for lovm2 and export a function named
//! after `EXTERN_LOVM2_INITIALIZER`. it is responsible for registering the exported
//! functions in the given `HashMap`.
//!
//! See [lovm2_extend](/lovm2_extend) for helper utilities and examples.

use libloading::{Error, Library, Symbol};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::code::{CallProtocol, CodeObjectRef, ExternFunction};
use crate::context::Context;
use crate::module::{ModuleProtocol, Slots};
use crate::var::Variable;

pub const EXTERN_LOVM2_INITIALIZER: &str = "lovm2_module_initializer";
pub type ExternInitializer = extern "C" fn(lib: Rc<Library>, &mut HashMap<Variable, CodeObjectRef>);

pub struct SharedObjectModule {
    lib: Rc<Library>,
    slots: Slots,
}

impl ModuleProtocol for SharedObjectModule {
    fn slots(&self) -> &Slots {
        &self.slots
    }

    fn slot(&self, name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        unsafe {
            let lookup: Result<Symbol<ExternFunction>, Error> = self.lib.get(name.as_bytes());
            match lookup {
                Ok(_) => Some(
                    Rc::new(SharedObjectSlot::new(self.lib.clone(), name.to_string()))
                        as Rc<dyn CallProtocol>,
                ),
                Err(_) => None,
            }
        }
    }
}

impl SharedObjectModule {
    pub fn from_library(lib: Library) -> Self {
        unsafe {
            let lib = Rc::new(lib);
            let lookup: Result<Symbol<ExternInitializer>, Error> =
                lib.get(EXTERN_LOVM2_INITIALIZER.as_bytes());
            match lookup {
                Ok(initializer) => {
                    let mut slots = HashMap::new();
                    initializer(lib.clone(), &mut slots);
                    Self {
                        lib,
                        slots: Slots::from(slots),
                    }
                }
                Err(_) => unimplemented!(),
            }
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

impl std::fmt::Debug for SharedObjectModule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<extern module>")
    }
}

impl Into<Box<dyn ModuleProtocol>> for SharedObjectModule {
    fn into(self) -> Box<dyn ModuleProtocol> {
        Box::new(self) as Box<dyn ModuleProtocol>
    }
}

pub struct SharedObjectSlot(Rc<Library>, String);

impl SharedObjectSlot {
    pub fn new(lib: Rc<Library>, name: String) -> Self {
        Self(lib, name)
    }
}

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
