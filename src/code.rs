//! runnable bytecode objects

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::context::Context;
use crate::value::CoValue;
use crate::var::Variable;
use crate::vm::run_bytecode;

pub type CodeObjectRef = Rc<dyn CallProtocol>;
/// definition for statically linked function
pub type StaticFunction = fn(&mut Context) -> Lovm2Result<()>;
/// definition for dynamically linked function
pub type ExternFunction = unsafe extern "C" fn(&mut Context) -> Lovm2Result<()>;

/// generalization for runnable objects
/// - lovm2 bytecode ([CodeObject](/latest/lovm2/code/struct.CodeObject.html))
/// - statically linked functions (defined in `module::standard`, macro `lovm2_internals::lovm2_builtin`)
/// - dynamically linked functions ([SharedObjectSlot](/latest/lovm2/module/shared/struct.SharedObjectSlot.html))
pub trait CallProtocol: std::fmt::Debug {
    fn code_object(&self) -> Option<&CodeObject> {
        None
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()>;
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
    pub consts: Vec<CoValue>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,

    pub code: Vec<Instruction>,
}

impl CallProtocol for CodeObject {
    fn code_object(&self) -> Option<&CodeObject> {
        Some(self)
    }

    fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
        run_bytecode(self, ctx)
    }
}

#[derive(Debug)]
pub struct CodeObjectBuilder {
    consts: Vec<CoValue>,
    locals: Vec<Variable>,
    globals: Vec<Variable>,

    code: Option<Vec<Instruction>>,
}

impl CodeObjectBuilder {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],

            code: None,
        }
    }

    pub fn consts(mut self, consts: Vec<CoValue>) -> Self {
        self.consts = consts;
        self
    }

    pub fn locals(mut self, locals: Vec<Variable>) -> Self {
        self.locals = locals;
        self
    }

    pub fn globals(mut self, globals: Vec<Variable>) -> Self {
        self.globals = globals;
        self
    }

    pub fn code(mut self, code: Vec<Instruction>) -> Self {
        self.code = Some(code);
        self
    }

    pub fn build(self) -> Lovm2CompileResult<CodeObject> {
        if self.code.is_none() {
            return Err("code missing".into());
        }
        Ok(CodeObject {
            consts: self.consts,
            locals: self.locals,
            globals: self.globals,

            code: self.code.unwrap(),
        })
    }
}
