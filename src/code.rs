use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::bytecode::Instruction;
use crate::context::Context;
use crate::value::CoValue;
use crate::var::Variable;
use crate::vm::run_bytecode;

pub type CodeObjectRef = Rc<dyn CallProtocol>;
pub type ExternFunction = unsafe extern fn(&mut Context) -> Result<(), String>;

pub trait CallProtocol: std::fmt::Debug {
    fn code_object(&self) -> Option<&CodeObject> {
        None
    }

    fn run(&self, ctx: &mut Context) -> Result<(), String>;
}

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

    fn run(&self, ctx: &mut Context) -> Result<(), String> {
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

    pub fn build(self) -> Result<CodeObject, String> {
        if self.code.is_none() {
            return Err("code missing".to_string());
        }
        Ok(CodeObject {
            consts: self.consts,
            locals: self.locals,
            globals: self.globals,

            code: self.code.unwrap(),
        })
    }
}
