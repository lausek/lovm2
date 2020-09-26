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

use lovm2_error::*;

use crate::code::{CallProtocol, CodeObjectRef, ExternFunction};
use crate::context::Context;
use crate::module::{GenericModule, ModuleProtocol, Slots};
use crate::var::Variable;

pub const EXTERN_LOVM2_INITIALIZER: &str = "lovm2_module_initializer";
pub type ExternInitializer = extern "C" fn(lib: Rc<Library>, &mut HashMap<Variable, CodeObjectRef>);

pub struct SharedObjectModule {
    name: String,
    lib: Rc<Library>,
    slots: Slots,
}

impl ModuleProtocol for SharedObjectModule {
    fn name(&self) -> &str {
        &self.name
    }

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
    fn from_library(lib: Library, name: String) -> Self {
        unsafe {
            let lib = Rc::new(lib);
            let lookup: Result<Symbol<ExternInitializer>, Error> =
                lib.get(EXTERN_LOVM2_INITIALIZER.as_bytes());
            match lookup {
                Ok(initializer) => {
                    let mut slots = HashMap::new();
                    initializer(lib.clone(), &mut slots);
                    Self {
                        name,
                        lib,
                        slots: Slots::from(slots),
                    }
                }
                Err(_) => unimplemented!(),
            }
        }
    }

    pub fn load_from_file<T>(path: T) -> Lovm2Result<SharedObjectModule>
    where
        T: AsRef<Path>,
    {
        let name = path.as_ref().file_stem().unwrap().to_str().unwrap();
        match Library::new(path.as_ref().as_os_str()) {
            Ok(lib) => Ok(SharedObjectModule::from_library(lib, name.to_string())),
            Err(err) => Err(format!("{}", err).into()),
        }
    }
}

impl std::fmt::Debug for SharedObjectModule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<extern module>")
    }
}

impl Into<GenericModule> for SharedObjectModule {
    fn into(self) -> GenericModule {
        Rc::new(self) as GenericModule
    }
}

pub struct SharedObjectSlot(Rc<Library>, String);

impl SharedObjectSlot {
    pub fn new(lib: Rc<Library>, name: String) -> Self {
        Self(lib, name)
    }
}

impl CallProtocol for SharedObjectSlot {
    fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
        unsafe {
            let (lib, name) = (&self.0, &self.1);
            let lookup: Result<Symbol<ExternFunction>, Error> = lib.get(name.as_bytes());
            match lookup {
                Ok(symbol) => symbol(ctx),
                Err(_) => {
                    Err(format!("symbol `{}` cannot be loaded from shared object", name).into())
                }
            }
        }
    }
}

impl std::fmt::Debug for SharedObjectSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<extern function>")
    }
}
