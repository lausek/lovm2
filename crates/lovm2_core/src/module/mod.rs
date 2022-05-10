//! Generic protocol for module like objects.
//!
//! A `LV2Module` can be created from a `LV2CodeObject` or by loading a lovm2 compatible shared object
//! library. It maintains an internal map of callable objects, meaning that everything
//! implementing the `LV2CallProtocol` can be added and executed from inside the VM. On load, all
//! entries from `Slots` will then be added to the context making them runnable from bytecode.

mod shared;
mod slots;

use std::rc::Rc;

use crate::code::LV2CallProtocol;
use crate::code::{LV2CodeObject, LV2CodeObjectFunction, LV2_MAGIC_NUMBER};
use crate::error::*;
use crate::var::LV2Variable;

pub use self::shared::{LV2SharedObjectSlot, LV2_EXTERN_INITIALIZER};
pub use self::slots::LV2ModuleSlots;

/// Name of the [LV2CodeObject] entry that is used as a programs starting point inside
/// [LV2Vm::run](crate::vm::LV2Vm::run).
pub const LV2_ENTRY_POINT: &str = "main";

/// Main runtime representation for loadable modules.
#[derive(Clone, Debug)]
pub struct LV2Module {
    /// Always required. Shared object libraries will only fill the `name` and `loc` attribute.
    pub code_object: Rc<LV2CodeObject>,
    /// Contains `CallProtocol` entries that will be added to the context.
    pub slots: LV2ModuleSlots,
}

impl LV2Module {
    /// A module is loadable if the first four bytes of the file are either the ELF
    /// magic number (shared object) or the `lovm2` magic number [LV2_MAGIC_NUMBER].
    pub fn is_loadable<T>(path: T) -> LV2Result<bool>
    where
        T: AsRef<std::path::Path>,
    {
        use std::fs::File;
        use std::io::Read;

        const ELF_MAGIC_NUMBER: &[u8] = &[0x7F, b'E', b'L', b'F'];

        let mut mark = [0; 4];
        let mut file = File::open(path).unwrap();

        file.read_exact(&mut mark).unwrap();

        Ok(mark == LV2_MAGIC_NUMBER || mark == ELF_MAGIC_NUMBER)
    }

    /// Checks if the file is loadable and tries creating a module from it.
    pub fn load_from_file<T>(path: T) -> LV2Result<Self>
    where
        T: AsRef<std::path::Path>,
    {
        // try loading module as shared object
        if let Ok(lib) = shared::load_library_from_file(&path) {
            Ok(shared::module_from_library(path, lib)?)
        } else {
            let co = LV2CodeObject::load_from_file(path)?;

            Ok(co.into())
        }
    }

    /// Returns the modules name.
    pub fn name(&self) -> &str {
        self.code_object.name.as_ref()
    }

    /// Returns the filesystem path from which the module was created.
    pub fn location(&self) -> Option<&String> {
        self.code_object.loc.as_ref()
    }

    pub fn slots(&self) -> &LV2ModuleSlots {
        &self.slots
    }

    /// Try looking up a [LV2CallProtocol] object by name.
    pub fn slot(&self, name: &LV2Variable) -> Option<Rc<dyn LV2CallProtocol>> {
        self.slots.get(name).cloned()
    }

    /// Write the contained [LV2CodeObject] into a file. This wil do nothing
    /// for shared object modules.
    pub fn store_to_file<T>(&self, path: T) -> LV2Result<()>
    where
        T: AsRef<std::path::Path>,
    {
        self.code_object.store_to_file(path)
    }

    /// Returns the `LV2CodeObject` representation as bytes.
    pub fn to_bytes(&self) -> LV2Result<Vec<u8>> {
        self.code_object.to_bytes()
    }
}

impl From<LV2CodeObject> for LV2Module {
    fn from(code_object: LV2CodeObject) -> Self {
        let code_object = Rc::new(code_object);
        let mut slots = LV2ModuleSlots::new();

        // make code object functions available in module
        for (iidx, offset) in code_object.entries.iter() {
            let var = &code_object.idents[*iidx];
            let func = LV2CodeObjectFunction::from(code_object.clone(), *offset);

            slots.insert(var.clone(), Rc::new(func));
        }

        Self { code_object, slots }
    }
}

impl std::fmt::Display for LV2Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "module({:?}, {:?}):",
            self.code_object.name, self.code_object.loc
        )?;

        // print constants
        if !self.code_object.consts.is_empty() {
            writeln!(f, "- consts: {:?}", self.code_object.consts)?;
        }

        // print identifiers
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

        // print bytecode
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
                    LPush(idx) | GPush(idx) | LMove(idx) | GMove(idx) => {
                        write!(f, "{}", self.code_object.idents[*idx as usize])?;
                    }
                    LCall(idx, _) | Call(idx, _) => {
                        write!(f, "{}", self.code_object.idents[*idx as usize])?;
                    }
                    CPush(idx) => {
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
