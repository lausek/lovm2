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

use crate::code::{CallProtocol, CodeObject, CodeObjectRef, ExternFunction};
use crate::context::Context;
use crate::module::{/* GenericModule, */ Module, /* ModuleProtocol, */ Slots};
use crate::var::Variable;

/// name of the unmangled function name to call when initializing module slots
pub const EXTERN_LOVM2_INITIALIZER: &str = "lovm2_module_initializer";
pub type ExternInitializer = extern "C" fn(lib: Rc<Library>, &mut HashMap<Variable, CodeObjectRef>);

/// contains the loaded shared object
/*
#[derive(Clone)]
pub struct SharedObjectModule {
    name: String,
    lib: Rc<Library>,
    slots: Slots,
}

// TODO: add module name to SharedObjectSlot
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
*/

fn load_slots(lib: Library) -> Lovm2Result<Slots> {
    unsafe {
        let lib = Rc::new(lib);
        let lookup: Result<Symbol<ExternInitializer>, Error> =
            lib.get(EXTERN_LOVM2_INITIALIZER.as_bytes());
        match lookup {
            Ok(initializer) => {
                let mut slots = HashMap::new();
                initializer(lib.clone(), &mut slots);
                Ok(Slots::from(slots))
            }
            Err(_) => Err(Lovm2ErrorTy::Basic.into()),
        }
    }
}

pub fn load_from_file<T>(path: T) -> Lovm2Result<Module>
where
    T: AsRef<Path>,
{
    let name = path.as_ref().file_stem().unwrap().to_str().unwrap();

    // this fixes some segfault errors. https://github.com/nagisa/rust_libloading/issues/41
    // load and initialize library
    #[cfg(target_os = "linux")]
    let library: Result<Library, libloading::Error> = {
        // load library with `RTLD_NOW | RTLD_NODELETE` to fix a SIGSEGV
        ::libloading::os::unix::Library::open(Some(path.as_ref()), 0x2 | 0x1000).map(Library::from)
    };
    #[cfg(not(target_os = "linux"))]
    let library = Library::new(path.as_ref());

    let library = library.map_err(|e| Lovm2Error::from(format!("{}", e)))?;

    let code_object = CodeObject {
        name: name.to_string(),
        loc: Some(path.as_ref().display().to_string()),
        ..CodeObject::default()
    };

    /*
    let slots = match library {
        Ok(lib) => ,
        Err(err) => Err(format!("{}", err).into()),
    }?;
    */

    Ok(Module {
        code_object: Rc::new(code_object),
        slots: load_slots(library)?,
    })
}

/*
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
*/

/// contains a function name, imported by `EXTERN_LOVM2_INITIALIZER`
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
