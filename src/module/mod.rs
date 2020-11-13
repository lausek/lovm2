//! generic protocol for module like objects
//!
//! if a module gets loaded by the virtual machine, its code objects are not available by default.
//! code objects need to be added to the scope to be callable by lovm2 bytecode.

pub mod builder;
pub mod shared;
pub mod slots;
pub mod standard;

use std::rc::Rc;

use lovm2_error::*;

use crate::code::CallProtocol;
use crate::code::{CodeObject, CodeObjectFunction};
use crate::var::Variable;

pub use self::builder::ModuleBuilder;
pub use self::slots::Slots;
pub use self::standard::create_standard_module;

/// name of the `CodeObject` that is used as a programs starting point inside `vm.run()`
pub const ENTRY_POINT: &str = "main";

/// generalization for loadable modules
/// - lovm2 bytecode ([Module](/latest/lovm2/module/struct.Module.html))
/// - shared objects `.so`
/// ([SharedObjectModule](/latest/lovm2/module/shared/struct.SharedObjectModule.html))
#[derive(Clone, Debug)]
pub struct Module {
    pub code_object: Rc<CodeObject>,
    pub slots: Slots,
}

impl Module {
    pub fn load_from_file<T>(path: T) -> Lovm2Result<Self>
    where
        T: AsRef<std::path::Path>,
    {
        // try loading module as shared object
        if let Ok(so_module) = shared::load_from_file(&path) {
            Ok(so_module)
        } else {
            let co = CodeObject::load_from_file(path)?;
            Ok(co.into())
        }
    }

    pub fn name(&self) -> &str {
        self.code_object.name.as_ref()
    }

    pub fn location(&self) -> Option<&String> {
        self.code_object.loc.as_ref()
    }

    pub fn slots(&self) -> &Slots {
        &self.slots
    }

    pub fn slot(&self, name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        self.slots.get(name).cloned()
    }

    pub fn store_to_file(&self, path: &str) -> Lovm2Result<()> {
        self.code_object.store_to_file(path)
    }

    pub fn to_bytes(&self) -> Lovm2Result<Vec<u8>> {
        self.code_object.to_bytes()
    }

    pub fn uses(&self) -> &[String] {
        &self.code_object.uses
    }
}

impl From<CodeObject> for Module {
    fn from(code_object: CodeObject) -> Self {
        let code_object = Rc::new(code_object);
        let mut slots = Slots::new();

        for (iidx, offset) in code_object.entries.iter() {
            let var = &code_object.idents[*iidx];
            let func = CodeObjectFunction::from(code_object.clone(), *offset);
            slots.insert(var.clone(), Rc::new(func));
        }

        Self { code_object, slots }
    }
}
