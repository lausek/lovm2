//! runnable bytecode objects

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::rc::Rc;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::context::Context;
use crate::module::{LoadableModule, ModuleProtocol, SharedObjectModule};
use crate::value::Value;
use crate::var::Variable;
use crate::vm::run_bytecode;

/// generic object implementing the `CallProtocol`
pub type CodeObjectRef = Rc<dyn CallProtocol>;
/// definition for statically linked function
pub type StaticFunction = fn(&mut Context) -> Lovm2Result<()>;
/// definition for dynamically linked function
pub type ExternFunction = unsafe extern "C" fn(&mut Context) -> Lovm2Result<()>;

/// generalization for runnable objects
/// - lovm2 bytecode ([CodeObject](/latest/lovm2/code/struct.CodeObject.html))
/// - statically linked functions (defined in `module::standard`, macro `lovm2_internals::lovm2_builtin`)
/// - dynamically linked functions ([SharedObjectSlot](/latest/lovm2/module/shared/struct.SharedObjectSlot.html))
///
/// functions implementing this protocol can support variadic arguments by looking at
/// the amount of passed values on stack inside `ctx.frame_mut()?.argn`
pub trait CallProtocol: std::fmt::Debug {
    fn code_object(&self) -> Option<&CodeObject> {
        None
    }

    fn module(&self) -> Option<String> {
        None
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()>;

    fn set_module(&mut self, _module: String) {}
}

/// `CodeObject` contains the bytecode as well as all the data used by it.
///
/// identifiers for called functions will end up in the `globals` vector.
///
/// values will be returned over the value stack. every code object has
/// to return some value on termination. if no value is produced,
/// `Nil` is implicitly returned.
#[derive(Debug, Deserialize, Serialize)]
pub struct CodeObject {
    pub consts: Vec<Value>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,

    pub code: Vec<Instruction>,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    module: Option<String>,
}

impl CallProtocol for CodeObject {
    fn code_object(&self) -> Option<&CodeObject> {
        Some(self)
    }

    fn module(&self) -> Option<String> {
        self.module.as_ref().cloned()
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
        todo!()
        //run_bytecode(self, ctx)
    }

    fn set_module(&mut self, module: String) {
        self.module = Some(module);
    }
}

/// structure for building `CodeObjects`. only used by `LoweringRuntime`
#[derive(Debug)]
pub struct CodeObjectBuilder {
    pub name: Option<String>,
    loc: Option<String>,
    entries: Vec<(usize, usize)>,
    consts: Vec<Value>,
    idents: Vec<Variable>,
    code: Vec<Instruction>,
}

impl CodeObjectBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            loc: None,
            entries: vec![],
            consts: vec![],
            idents: vec![],
            code: vec![],
        }
    }

    pub fn entries(mut self, entries: Vec<(usize, usize)>) -> Self {
        self.entries = entries;
        self
    }

    pub fn code(mut self, code: Vec<Instruction>) -> Self {
        self.code = code;
        self
    }

    pub fn consts(mut self, consts: Vec<Value>) -> Self {
        self.consts = consts;
        self
    }

    pub fn idents(mut self, idents: Vec<Variable>) -> Self {
        self.idents = idents;
        self
    }

    pub fn location(mut self, loc: String) -> Self {
        self.loc = Some(loc);
        self
    }

    /*
    pub fn globals(mut self, globals: Vec<Variable>) -> Self {
        self.globals = globals;
        self
    }
    */

    pub fn build(self) -> Lovm2CompileResult<NewCodeObject> {
        Ok(NewCodeObject {
            name: self.name,
            loc: self.loc,
            entries: self.entries,
            consts: self.consts,
            idents: self.idents,
            code: self.code,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewCodeObject {
    pub name: Option<String>,
    pub loc: Option<String>,
    pub entries: Vec<(usize, usize)>,
    pub consts: Vec<Value>,
    pub idents: Vec<Variable>,
    pub code: Vec<Instruction>,
}

impl CallProtocol for NewCodeObject {
    fn module(&self) -> Option<String> {
        self.name.as_ref().cloned()
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
        run_bytecode(&self, ctx, 0)
    }
}

impl ModuleProtocol for NewCodeObject {
    fn name(&self) -> &str {
        // TODO: don't unwrap here
        self.name.as_ref().unwrap()
    }

    fn location(&self) -> Option<&String> {
        self.loc.as_ref()
    }

    /*
    fn slots(&self) -> &Slots {
        &self.slots
    }

    fn slot(&self, name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        self.slots
            .get(name)
            .map(|co_ref| co_ref.clone() as Rc<dyn CallProtocol>)
    }
    */

    fn to_bytes(&self) -> Lovm2Result<Vec<u8>> {
        use bincode::Options;
        bincode::options()
            .with_varint_encoding()
            .serialize(self)
            .map_err(|e| e.to_string().into())
    }

    // TODO: could lead to errors when two threads serialize to the same file
    fn store_to_file(&self, path: &str) -> Lovm2Result<()> {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        let bytes = self.to_bytes()?;
        file.write_all(&bytes).map_err(|e| e.to_string().into())
    }

    /*
    fn uses(&self) -> &[String] {
        self.uses.as_ref()
    }
    */
}

impl NewCodeObject {
    pub fn new() -> Self {
        Self {
            name: None,
            loc: None,
            entries: vec![],
            consts: vec![],
            idents: vec![],
            code: vec![],
        }
    }

    /// tries to load the file as shared object first and tries regular deserialization if it failed
    pub fn load_from_file<T>(path: T) -> Lovm2Result<LoadableModule>
    where
        T: AsRef<Path>,
    {
        use std::fs::File;
        // try loading module as shared object
        if let Ok(so_module) = SharedObjectModule::load_from_file(&path) {
            return Ok(so_module.into());
        }

        use bincode::Options;
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
        let mut module: NewCodeObject = bincode::options()
            .with_varint_encoding()
            .deserialize_from(file)
            .map_err(|e| e.to_string())?;
        //module.name = name;
        module.loc = Some(loc);

        /*
        for (_, slot) in module.slots.iter_mut() {
            Rc::get_mut(slot).unwrap().set_module(module.name.clone());
        }
        */

        Ok(module.into())
    }
}

#[derive(Debug)]
pub struct CodeObjectFunction {
    offset: usize,
    on: Rc<NewCodeObject>,
}

impl CodeObjectFunction {
    pub fn from(on: Rc<NewCodeObject>, offset: usize) -> Self {
        Self { offset, on }
    }
}

impl CallProtocol for CodeObjectFunction {
    fn module(&self) -> Option<String> {
        self.on.name.as_ref().cloned()
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
        run_bytecode(&self.on, ctx, self.offset)
    }
}
