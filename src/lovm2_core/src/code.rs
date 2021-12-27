//! Runnable bytecode objects.

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::bytecode::Instruction;
use crate::error::*;
use crate::value::LV2Value;
use crate::var::LV2Variable;
use crate::vm::LV2Vm;

/// 4 bytes at the start of each serialized lovm2 module.
pub const LV2_MAGIC_NUMBER: &[u8] = &[0x7f, b'L', b'V', b'2'];

/// Generic object implementing the [LV2CallProtocol].
pub type LV2CallableRef = Rc<dyn LV2CallProtocol>;

/// Generalization for runnable objects.
/// - lovm2 bytecode ([LV2CodeObject])
/// - Statically linked functions (standard library is an example, [lv2_create_callable](crate::extend::lv2_create_callable))
/// - Dynamically linked functions ([LV2SharedObjectSlot](crate::module::LV2SharedObjectSlot))
///
/// Functions implementing this protocol can support variadic arguments by looking at
/// the amount of passed values on stack inside `ctx.frame_mut()?.argn`.
pub trait LV2CallProtocol: std::fmt::Debug {
    fn module(&self) -> Option<String> {
        None
    }

    fn run(&self, vm: &mut LV2Vm) -> LV2Result<()>;
}

/// `LV2CodeObject` contains the bytecode as well as all the data used by it.
///
/// The `entries` attribute is a vector of name-offset pairs where the first component is an
/// index into `idents`. This information is essential for the [run_bytecode](crate::vm::LV2Vm::run_bytecode) function used by
/// [LV2CodeObjectFunction]. It shouldn't be necessary to manually alter `entries`. By default,
/// the subprogram named [LV2_ENTRY_POINT](crate::module::LV2_ENTRY_POINT) will be at offset 0:
///
/// ``` ignored
/// main:
///     ...
///     ret
/// add:
///     ...
///     ret
/// ```
///
/// Values will be returned over the value stack. Every code object has
/// to return some value on termination. If no value is produced, `Nil` is implicitly returned.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LV2CodeObject {
    /// Name of the object. This is used as the modules name when imported.
    pub name: String,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    /// Location of objects origin.
    pub loc: Option<String>,
    /// Modules required for executing this object successfully.
    pub uses: Vec<String>,
    /// Entry points for the bytecode in the form Vec<(index_into_idents, bytecode_offset)>. These are the functions of the module.
    pub entries: Vec<(usize, usize)>,
    /// Necessary constants.
    pub consts: Vec<LV2Value>,
    /// Necessary identifiers.
    pub idents: Vec<LV2Variable>,
    /// Bytecode itself.
    pub code: Vec<Instruction>,
}

impl LV2CallProtocol for LV2CodeObject {
    fn module(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn run(&self, vm: &mut LV2Vm) -> LV2Result<()> {
        vm.run_bytecode(self, 0)
    }
}

impl LV2CodeObject {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tries to load the file as shared object first and falls back to regular deserialization if it failed.
    pub fn load_from_file<T>(path: T) -> LV2Result<Self>
    where
        T: AsRef<std::path::Path>,
    {
        use bincode::Options;
        use std::fs::File;
        use std::io::Read;

        let name = path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let loc = path.as_ref().to_str().unwrap().to_string();
        let mut file = File::open(path).or_else(err_from_string)?;
        let mut buffer = vec![];

        // avoid misinterpreting random bytes as length of buffer
        // this could lead to memory allocation faults
        file.read_to_end(&mut buffer).unwrap();
        let mut co: LV2CodeObject = bincode::options()
            .with_varint_encoding()
            .deserialize(&buffer[4..])
            .or_else(err_from_string)?;

        co.name = name;
        co.loc = Some(loc);

        Ok(co)
    }

    /// Return the objects representation as bytes.
    pub fn to_bytes(&self) -> LV2Result<Vec<u8>> {
        use bincode::Options;

        let mut buffer = Vec::from(LV2_MAGIC_NUMBER);
        let obj: Vec<u8> = bincode::options()
            .with_varint_encoding()
            .serialize(self)
            .or_else(err_from_string)?;

        buffer.extend(obj);

        Ok(buffer)
    }

    // TODO: could lead to errors when two threads serialize to the same file
    /// Write the object to a file at given path.
    pub fn store_to_file<T>(&self, path: T) -> LV2Result<()>
    where
        T: AsRef<std::path::Path>,
    {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path).or_else(err_from_string)?;
        let bytes = self.to_bytes()?;

        file.write_all(&bytes).or_else(err_from_string)
    }
}

/// Function of a [LV2CodeObject]. Implements [LV2CallProtocol] to allow execution of bytecode from a certain offset.
#[derive(Debug)]
pub struct LV2CodeObjectFunction {
    offset: usize,
    on: Rc<LV2CodeObject>,
}

impl LV2CodeObjectFunction {
    pub fn from(on: Rc<LV2CodeObject>, offset: usize) -> Self {
        Self { offset, on }
    }
}

impl LV2CallProtocol for LV2CodeObjectFunction {
    fn module(&self) -> Option<String> {
        Some(self.on.name.clone())
    }

    fn run(&self, vm: &mut LV2Vm) -> LV2Result<()> {
        vm.run_bytecode(&self.on, self.offset)
    }
}
