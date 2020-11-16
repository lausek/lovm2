//! runnable bytecode objects

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::context::Context;
use crate::value::Value;
use crate::var::Variable;
use crate::vm::run_bytecode;

/// generic object implementing the `CallProtocol`
pub type CallableRef = Rc<dyn CallProtocol>;
/// definition for statically linked function
pub type StaticFunction = fn(&mut Context) -> Lovm2Result<()>;
/// definition for dynamically linked function
pub type ExternFunction = unsafe extern "C" fn(&mut Context) -> Lovm2Result<()>;

/// generalization for runnable objects
/// - lovm2 bytecode ([CodeObject])
/// - statically linked functions (defined in `module::standard`, macro `lovm2_internals::lovm2_builtin`)
/// - dynamically linked functions ([SharedObjectSlot](crate::module::shared::SharedObjectSlot))
///
/// functions implementing this protocol can support variadic arguments by looking at
/// the amount of passed values on stack inside `ctx.frame_mut()?.argn`
pub trait CallProtocol: std::fmt::Debug {
    fn module(&self) -> Option<String> {
        None
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()>;
}

/// `CodeObject` contains the bytecode as well as all the data used by it.
///
/// the `entries` attribute is a vector of name-offset pairs where the first component is an
/// index into `idents`. this information is essential for the [run_bytecode] function used by
/// [CodeObjectFunction]. it shouldn't be necessary to manually alter `entries`. by default,
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
/// values will be returned over the value stack. every code object has
/// to return some value on termination. if no value is produced, `Nil` is implicitly returned.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeObject {
    pub name: String,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub loc: Option<String>,
    pub uses: Vec<String>,
    pub entries: Vec<(usize, usize)>,
    pub consts: Vec<Value>,
    pub idents: Vec<Variable>,
    pub code: Vec<Instruction>,
}

impl CallProtocol for CodeObject {
    fn module(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
        run_bytecode(&self, ctx, 0)
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

    /// tries to load the file as shared object first and tries regular deserialization if it failed
    pub fn load_from_file<T>(path: T) -> Lovm2Result<Self>
    where
        T: AsRef<std::path::Path>,
    {
        use bincode::Options;
        use std::fs::File;
        let name = path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let loc = path.as_ref().to_str().unwrap().to_string();
        let file = File::open(path).map_err(|e| e.to_string())?;
        // avoid misinterpreting random bytes as length of buffer
        // this could lead to memory allocation faults
        let mut co: CodeObject = bincode::options()
            .with_varint_encoding()
            .deserialize_from(file)
            .map_err(|e| e.to_string())?;
        co.name = name;
        co.loc = Some(loc);

        Ok(co)
    }

    pub fn to_bytes(&self) -> Lovm2Result<Vec<u8>> {
        use bincode::Options;
        bincode::options()
            .with_varint_encoding()
            .serialize(self)
            .map_err(|e| e.to_string().into())
    }

    // TODO: could lead to errors when two threads serialize to the same file
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

/// implements `CallProtocol` to allow execution of bytecode from a certain offset.
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

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
        run_bytecode(&self.on, ctx, self.offset)
    }
}
