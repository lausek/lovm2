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

use crate::code::{CallProtocol, CallableRef, CodeObject};
use crate::module::{Module, Slots};
use crate::var::Variable;
use crate::vm::Vm;

use super::ExternFunction;

/// name of the unmangled function name to call when initializing module slots
pub const EXTERN_LOVM2_INITIALIZER: &str = "lovm2_module_initializer";
pub type ExternInitializer = extern "C" fn(lib: Rc<Library>, &mut HashMap<Variable, CallableRef>);

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

    Ok(Module {
        code_object: Rc::new(code_object),
        slots: load_slots(library)?,
    })
}

// as the `Library` is always valid for this structure, it should be fine to
// call `into_raw` on the loaded symbol and then use the function pointer afterwards.
/// contains a function name, imported by `EXTERN_LOVM2_INITIALIZER`
pub struct SharedObjectSlot(
    Rc<Library>,
    #[cfg(unix)] ::libloading::os::unix::Symbol<ExternFunction>,
    #[cfg(windows)] ::libloading::os::windows::Symbol<ExternFunction>,
);

impl SharedObjectSlot {
    pub fn new<T>(lib: Rc<Library>, name: T) -> Lovm2Result<Self>
        where T: AsRef<str>
    {
        let name = name.as_ref();
        unsafe {
            let lookup: Result<Symbol<ExternFunction>, Error> = lib.get(name.as_bytes());
            match lookup {
                Ok(symbol) => Ok(Self(lib.clone(), symbol.into_raw())),
                Err(_) => {
                    Err(format!("symbol `{}` cannot be loaded from shared object", name).into())
                }
            }
        }
    }
}

impl CallProtocol for SharedObjectSlot {
    fn run(&self, vm: &mut Vm) -> Lovm2Result<()> {
        unsafe { self.1(vm) }
    }
}

impl std::fmt::Debug for SharedObjectSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<extern function>")
    }
}
