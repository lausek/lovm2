//! VM state

use std::collections::HashMap;
use std::rc::Rc;

use crate::code::{CallProtocol, CallableRef};
use crate::value::Value;
use crate::var::LV2Variable;

use super::*;

pub const DEFAULT_LSTACK_SIZE: usize = 256;
pub const DEFAULT_VSTACK_SIZE: usize = 256;

/// The state of the virtual machine
///
/// This contains all necessary runtime data and gets shared with objects that
/// implement `CallProtocol` as well as interrupts.
#[derive(Debug)]
pub struct Context {
    /// Starting point of execution.
    pub(super) entry: Option<Rc<dyn CallProtocol>>,
    /// Global variables that can be altered from every object.
    pub(super) globals: HashMap<String, Value>,
    /// Entries in this map can directly be called from `lovm2` bytecode.
    pub(super) scope: HashMap<LV2Variable, CallableRef>,
    /// Call stack that contains local variables and the amount of arguments passed.
    pub(super) lstack: Vec<Frame>,
    /// Value stack. This is where the computation happens.
    pub(super) vstack: Vec<Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            entry: None,
            globals: HashMap::new(),
            scope: HashMap::new(),
            lstack: Vec::with_capacity(DEFAULT_LSTACK_SIZE),
            vstack: Vec::with_capacity(DEFAULT_VSTACK_SIZE),
        }
    }

    /// Try to resolve the given name to a callable
    pub fn lookup_code_object(&self, name: &LV2Variable) -> Lovm2Result<CallableRef> {
        self.scope
            .get(name)
            .cloned()
            .ok_or_else(|| (Lovm2ErrorTy::LookupFailed, name).into())
    }

    /// Get a mutable reference to the value stack itself
    pub fn stack_mut(&mut self) -> &mut Vec<Value> {
        &mut self.vstack
    }

    /// Get a mutable reference to the value lstack itself
    pub fn lstack_mut(&mut self) -> &mut Vec<Frame> {
        &mut self.lstack
    }

    /// Get a mutable reference to the last stack frame
    pub fn frame_mut(&mut self) -> Lovm2Result<&mut Frame> {
        self.lstack
            .last_mut()
            .ok_or_else(|| Lovm2ErrorTy::FrameStackEmpty.into())
    }

    /// Set value of a global variable
    pub fn set_global<T>(&mut self, var: T, val: Value)
    where
        T: AsRef<str>,
    {
        self.globals.insert(var.as_ref().to_string(), val);
    }

    /// Create a frame on the callstack
    pub fn push_frame(&mut self, argn: u8) {
        self.lstack.push(Frame::new(argn));
    }

    /// Remove a frame from the callstack
    pub fn pop_frame(&mut self) {
        self.lstack.pop();
    }

    /// Put a new value on the stack
    pub fn push_value(&mut self, value: Value) {
        self.vstack.push(value);
    }

    /// Remove the last value on stack
    pub fn pop_value(&mut self) -> Lovm2Result<Value> {
        self.vstack
            .pop()
            .ok_or_else(|| Lovm2ErrorTy::ValueStackEmpty.into())
    }

    /// Get a mutable reference to the last value on stack
    pub fn last_value_mut(&mut self) -> Lovm2Result<&mut Value> {
        self.vstack
            .last_mut()
            .ok_or_else(|| Lovm2ErrorTy::ValueStackEmpty.into())
    }

    /// Lookup a global value
    pub fn value_of<T>(&self, var: T) -> Lovm2Result<&Value>
    where
        T: AsRef<str>,
    {
        self.globals
            .get(var.as_ref())
            .ok_or_else(|| (Lovm2ErrorTy::LookupFailed, var).into())
    }
}
