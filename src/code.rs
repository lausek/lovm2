//! Runnable bytecode objects

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::value::Value;
use crate::var::Variable;
use crate::vm::Vm;

pub const LV2_MAGIC_NUMBER: &[u8] = &[0x7f, 'L' as u8, 'V' as u8, '2' as u8];

/// Generic object implementing the `CallProtocol`
pub type CallableRef = Rc<dyn CallProtocol>;

/// Generalization for runnable objects
/// - lovm2 bytecode ([CodeObject])
/// - Statically linked functions (defined in `module::standard`, macro `lovm2_internals::lovm2_builtin`)
/// - Dynamically linked functions ([SharedObjectSlot](crate::module::SharedObjectSlot))
///
/// Functions implementing this protocol can support variadic arguments by looking at
/// the amount of passed values on stack inside `ctx.frame_mut()?.argn`
pub trait CallProtocol: std::fmt::Debug {
    fn module(&self) -> Option<String> {
        None
    }

    fn run(&self, vm: &mut Vm) -> Lovm2Result<()>;
}

/// `CodeObject` contains the bytecode as well as all the data used by it.
///
/// The `entries` attribute is a vector of name-offset pairs where the first component is an
/// index into `idents`. This information is essential for the [run_bytecode](Vm::run_bytecode) function used by
/// [CodeObjectFunction]. It shouldn't be necessary to manually alter `entries`. By default,
/// the subprogram named [ENTRY_POINT](crate::module::ENTRY_POINT) will be at offset 0:
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeObject {
    /// Name of the object. This is used as the modules name when imported
    pub name: String,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    /// Location of objects origin
    pub loc: Option<String>,
    /// Modules required for executing this object successfully
    pub uses: Vec<String>,
    /// Entry points for the bytecode in the form Vec<(index_into_idents, bytecode_offset)>. These are the functions of the module
    pub entries: Vec<(usize, usize)>,
    /// Necessary constants
    pub consts: Vec<Value>,
    /// Necessary identifiers
    pub idents: Vec<Variable>,
    /// Bytecode
    pub code: Vec<Instruction>,
}

impl CallProtocol for CodeObject {
    fn module(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn run(&self, vm: &mut Vm) -> Lovm2Result<()> {
        vm.run_bytecode(&self, 0)
    }
}

impl std::default::Default for CodeObject {
    fn default() -> Self {
        Self {
            name: String::new(),
            loc: None,
            uses: vec![],
            entries: vec![],
            consts: vec![],
            idents: vec![],
            code: vec![],
        }
    }
}

impl CodeObject {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tries to load the file as shared object first and falls back to regular deserialization if it failed
    pub fn load_from_file<T>(path: T) -> Lovm2Result<Self>
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
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = vec![];

        // avoid misinterpreting random bytes as length of buffer
        // this could lead to memory allocation faults
        file.read_to_end(&mut buffer).unwrap();
        let mut co: CodeObject = bincode::options()
            .with_varint_encoding()
            .deserialize(&buffer[4..])
            .map_err(|e| e.to_string())?;

        co.name = name;
        co.loc = Some(loc);

        Ok(co)
    }

    /// Return the objects representation as bytes
    pub fn to_bytes(&self) -> Lovm2Result<Vec<u8>> {
        use bincode::Options;

        let mut buffer = Vec::from(LV2_MAGIC_NUMBER);
        let obj: Lovm2Result<Vec<u8>> = bincode::options()
            .with_varint_encoding()
            .serialize(self)
            .map_err(|e| e.to_string().into());
        let obj = obj?;

        buffer.extend(obj);

        Ok(buffer)
    }

    // TODO: could lead to errors when two threads serialize to the same file
    /// Write the object to a file at given path
    pub fn store_to_file<T>(&self, path: T) -> Lovm2Result<()>
    where
        T: AsRef<std::path::Path>,
    {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        let bytes = self.to_bytes()?;
        file.write_all(&bytes).map_err(|e| e.to_string().into())
    }
}

/// Function of a [CodeObject]. Implements [CallProtocol] to allow execution of bytecode from a certain offset.
#[derive(Debug)]
pub struct CodeObjectFunction {
    offset: usize,
    on: Rc<CodeObject>,
}

impl CodeObjectFunction {
    pub fn from(on: Rc<CodeObject>, offset: usize) -> Self {
        Self { offset, on }
    }
}

impl CallProtocol for CodeObjectFunction {
    fn module(&self) -> Option<String> {
        Some(self.on.name.clone())
    }

    fn run(&self, vm: &mut Vm) -> Lovm2Result<()> {
        vm.run_bytecode(&self.on, self.offset)
    }
}
