//! Implementation of shared objects as lovm2 modules
//!
//! Shared objects must be specifically compiled for lovm2 and export a function named
//! after `EXTERN_LOVM2_INITIALIZER`. It is responsible for registering the exported
//! functions in the given `HashMap`.
//!
//! See module [extend](lovm2_extend) for helper utilities and examples.

use libloading::{Error, Library, Symbol};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::code::{LV2CallProtocol, LV2CallableRef, LV2CodeObject};
use crate::error::*;
use crate::module::{LV2Module, LV2ModuleSlots};
use crate::var::LV2Variable;
use crate::vm::LV2Vm;

/// Name of the unmangled function name to call when initializing module slots.
pub const LV2_EXTERN_INITIALIZER: &str = "lovm2_module_initialize";

/// Definition for dynamically linked function.
pub type LV2ExternFunction = unsafe extern "C" fn(&mut LV2Vm) -> LV2Result<()>;
/// Function signature of the extern module initializer.
pub type LV2ExternInitializer =
    extern "C" fn(lib: Rc<Library>, &mut HashMap<LV2Variable, LV2CallableRef>);

fn load_slots(_name: &str, lib: Library) -> LV2Result<LV2ModuleSlots> {
    unsafe {
        let lib = Rc::new(lib);

        // try to lookup named initializer first, fallback to initializer without name
        let lookup: Result<Symbol<LV2ExternInitializer>, Error> =
            lib.get(LV2_EXTERN_INITIALIZER.as_bytes());

        match lookup {
            Ok(initializer) => {
                let mut slots = HashMap::new();

                initializer(lib.clone(), &mut slots);

                Ok(LV2ModuleSlots::from(slots))
            }
            Err(_) => Err(LV2ErrorTy::Basic.into()),
        }
    }
}

/// Try loading a shared object from a file.
pub fn load_library_from_file<T>(path: T) -> LV2Result<Library>
where
    T: AsRef<Path>,
{
    // this fixes some segfault errors. https://github.com/nagisa/rust_libloading/issues/41
    // load and initialize library
    #[cfg(target_os = "linux")]
    let library: Result<Library, libloading::Error> = {
        // load library with `RTLD_NOW | RTLD_NODELETE` to fix a SIGSEGV
        ::libloading::os::unix::Library::open(Some(path.as_ref()), 0x2 | 0x1000).map(Library::from)
    };
    #[cfg(not(target_os = "linux"))]
    let library = Library::new(path.as_ref());

    library.or_else(err_from_string)
}

/// Turn the loaded shared object into a `lovm2` [LV2Module].
pub fn module_from_library<T>(path: T, lib: Library) -> LV2Result<LV2Module>
where
    T: AsRef<Path>,
{
    let name = path.as_ref().file_stem().unwrap().to_str().unwrap();

    let code_object = LV2CodeObject {
        name: name.to_string(),
        loc: Some(path.as_ref().display().to_string()),
        ..LV2CodeObject::default()
    };

    Ok(LV2Module {
        code_object: Rc::new(code_object),
        slots: load_slots(name, lib)?,
    })
}

// As the `Library` is always valid for this structure, it should be fine to
// call `into_raw` on the loaded symbol and then use the function pointer afterwards.
/// Contains a function name, imported by [LV2_EXTERN_INITIALIZER](lovm2_extend::LV2_EXTERN_INITIALIZER).
pub struct LV2SharedObjectSlot(
    Rc<Library>,
    #[cfg(unix)] ::libloading::os::unix::Symbol<LV2ExternFunction>,
    #[cfg(windows)] ::libloading::os::windows::Symbol<LV2ExternFunction>,
);

impl LV2SharedObjectSlot {
    pub fn new<T>(lib: Rc<Library>, name: T) -> LV2Result<Self>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();

        unsafe {
            let lookup: Result<Symbol<LV2ExternFunction>, Error> = lib.get(name.as_bytes());

            match lookup {
                Ok(symbol) => Ok(Self(lib.clone(), symbol.into_raw())),
                Err(_) => err_symbol_not_found(name),
            }
        }
    }
}

impl LV2CallProtocol for LV2SharedObjectSlot {
    fn run(&self, vm: &mut LV2Vm) -> LV2Result<()> {
        unsafe { self.1(vm) }
    }
}

impl std::fmt::Debug for LV2SharedObjectSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<extern function>")
    }
}
