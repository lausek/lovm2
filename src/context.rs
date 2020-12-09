//! vm state

use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

use crate::code::{CallProtocol, CallableRef};
use crate::frame::Frame;
use crate::module::Module;
use crate::value::Value;
use crate::var::Variable;

/// the state of the virtual machine
///
/// this contains all necessary runtime data and gets shared with objects that
/// implement `CallProtocol` as well as interrupts.
pub struct Context {
    pub entry: Option<Rc<dyn CallProtocol>>,
    /// list of loaded modules: `Module` or `SharedObjectModule`
    pub modules: HashMap<String, Rc<Module>>,
    /// global variables that can be altered from every object
    pub globals: HashMap<Variable, Value>,
    /// entries in this map can directly be called from lovm2 bytecode
    pub scope: HashMap<Variable, CallableRef>,
    /// call stack that contains local variables
    pub lstack: Vec<Frame>,
    /// value stack. this is shared across `CallProtocol` functionality
    pub vstack: Vec<Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            entry: None,
            modules: HashMap::new(),
            globals: HashMap::new(),
            scope: HashMap::new(),
            lstack: vec![],
            vstack: vec![],
        }
    }

    pub fn lookup_code_object(&self, name: &Variable) -> Lovm2Result<CallableRef> {
        self.scope
            .get(name)
            .cloned()
            .ok_or_else(|| (Lovm2ErrorTy::LookupFailed, name).into())
    }

    pub fn stack_mut(&mut self) -> &mut Vec<Value> {
        &mut self.vstack
    }

    /// get last stack frame or raise error
    pub fn frame_mut(&mut self) -> Lovm2Result<&mut Frame> {
        self.lstack
            .last_mut()
            .ok_or_else(|| Lovm2ErrorTy::FrameStackEmpty.into())
    }

    pub fn push_frame(&mut self, argn: u8) {
        self.lstack.push(Frame::new(argn));
    }

    pub fn pop_frame(&mut self) {
        self.lstack.pop();
    }

    pub fn push_value(&mut self, value: Value) {
        self.vstack.push(value);
    }

    pub fn pop_value(&mut self) -> Lovm2Result<Value> {
        self.vstack
            .pop()
            .ok_or_else(|| Lovm2ErrorTy::ValueStackEmpty.into())
    }

    pub fn last_value_mut(&mut self) -> Lovm2Result<&mut Value> {
        self.vstack
            .last_mut()
            .ok_or_else(|| Lovm2ErrorTy::ValueStackEmpty.into())
    }

    pub fn value_of(&self, var: &Variable) -> Option<Value> {
        self.globals.get(var).cloned()
    }
}
