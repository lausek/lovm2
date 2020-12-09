//! generic protocol for module like objects
//!
//! a `Module` can be created from a `CodeObject` or by loading a lovm2 compatible shared object
//! library.  it maintains an internal map of callable objects, meaning that everything
//! implementing the `CallProtocol` can be added and executed from inside the vm. on load, all
//! entries from `Slots` will then be added to the context making them runnable from bytecode.

pub mod builder;
pub mod meta;
pub mod shared;
pub mod slots;
pub mod standard;

use std::rc::Rc;

use lovm2_error::*;

use crate::code::CallProtocol;
use crate::code::{CodeObject, CodeObjectFunction};
use crate::var::Variable;
use crate::vm::Vm;

pub use self::builder::ModuleBuilder;
pub use self::meta::ModuleMeta;
pub use self::slots::Slots;
pub use self::standard::{create_standard_module, BUILTIN_FUNCTIONS};

/// name of the `CodeObject` entry that is used as a programs starting point inside `vm.run()`
pub const ENTRY_POINT: &str = "main";

/// definition for dynamically linked function
pub type ExternFunction = unsafe extern "C" fn(&mut Vm) -> Lovm2Result<()>;

/// main runtime representation for loadable modules.
#[derive(Clone, Debug)]
pub struct Module {
    /// always required. shared object libraries will only fill the `name` and `loc` attribute
    pub code_object: Rc<CodeObject>,
    /// contains `CallProtocol` entries that will be added to the context
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

    pub fn store_to_file<T>(&self, path: T) -> Lovm2Result<()>
    where
        T: AsRef<std::path::Path>,
    {
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

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "module({:?}, {:?}):",
            self.code_object.name, self.code_object.loc
        )?;
        if !self.code_object.uses.is_empty() {
            writeln!(f, "- uses: {:?}", self.code_object.uses)?;
        }

        if !self.code_object.consts.is_empty() {
            writeln!(f, "- consts: {:?}", self.code_object.consts)?;
        }

        if !self.code_object.idents.is_empty() {
            write!(f, "- idents: [")?;
            for (i, ident) in self.code_object.idents.iter().enumerate() {
                if i == 0 {
                    write!(f, "{}", ident)?;
                } else {
                    write!(f, ", {}", ident)?;
                }
            }
            writeln!(f, "]")?;
        }

        if !self.code_object.code.is_empty() {
            use crate::bytecode::Instruction::*;

            let mut entry_iter = self.code_object.entries.iter();
            let mut entry_current = entry_iter.next();

            writeln!(f, "- code:")?;
            for (off, inx) in self.code_object.code.iter().enumerate() {
                match entry_current {
                    Some(current) if current.1 == off => {
                        let entry_name = &self.code_object.idents[current.0];
                        writeln!(f, "{}:", entry_name)?;
                        entry_current = entry_iter.next();
                    }
                    _ => {}
                }

                write!(f, "\t{: >4}. {:<16}", off, format!("{:?}", inx))?;

                match inx {
                    Pushl(idx) | Pushg(idx) | Movel(idx) | Moveg(idx) => {
                        write!(f, "{}", self.code_object.idents[*idx as usize])?;
                    }
                    Call(_, idx) => {
                        write!(f, "{}", self.code_object.idents[*idx as usize])?;
                    }
                    Pushc(idx) => {
                        write!(f, "{}", self.code_object.consts[*idx as usize])?;
                    }
                    _ => {}
                }

                writeln!(f)?;
            }
        }

        Ok(())
    }
}
