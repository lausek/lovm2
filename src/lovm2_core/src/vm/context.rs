//! VM state

use std::collections::HashMap;
use std::rc::Rc;

use crate::code::{LV2CallProtocol, LV2CallableRef};
use crate::value::LV2Value;
use crate::var::LV2Variable;

use super::*;

pub const DEFAULT_LSTACK_SIZE: usize = 256;
pub const DEFAULT_VSTACK_SIZE: usize = 256;

/// The state of the virtual machine
///
/// This contains all necessary runtime data and gets shared with objects that
/// implement `CallProtocol` as well as interrupts.
#[derive(Debug)]
pub struct LV2Context {
    /// Starting point of execution.
    pub(super) entry: Option<Rc<dyn LV2CallProtocol>>,
    /// Global variables that can be altered from every object.
    pub(super) globals: HashMap<String, LV2Value>,
    /// Entries in this map can directly be called from `lovm2` bytecode.
    pub(super) scope: HashMap<LV2Variable, LV2CallableRef>,
    /// Call stack that contains local variables and the amount of arguments passed.
    pub(super) lstack: Vec<LV2StackFrame>,
    /// Value stack. This is where the computation happens.
    pub(super) vstack: Vec<LV2Value>,
}

impl LV2Context {
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
    pub fn lookup_code_object(&self, name: &LV2Variable) -> LV2Result<LV2CallableRef> {
        self.scope
            .get(name)
            .cloned()
            .ok_or_else(|| (LV2ErrorTy::LookupFailed, name).into())
    }

    /// Get a mutable reference to the value stack itself
    pub fn stack_mut(&mut self) -> &mut Vec<LV2Value> {
        &mut self.vstack
    }

    /// Get a mutable reference to the value lstack itself
    pub fn lstack_mut(&mut self) -> &mut Vec<LV2StackFrame> {
        &mut self.lstack
    }

    /// Get a mutable reference to the last stack frame
    pub fn frame_mut(&mut self) -> LV2Result<&mut LV2StackFrame> {
        self.lstack
            .last_mut()
            .ok_or_else(|| LV2ErrorTy::FrameStackEmpty.into())
    }

    /// Set value of a global variable
    pub fn set_global<T>(&mut self, var: T, val: LV2Value)
    where
        T: AsRef<str>,
    {
        self.globals.insert(var.as_ref().to_string(), val);
    }

    /// Create a frame on the callstack
    pub fn push_frame(&mut self, argn: u8) {
        self.lstack.push(LV2StackFrame::new(argn));
    }

    /// Remove a frame from the callstack
    pub fn pop_frame(&mut self) {
        self.lstack.pop();
    }

    /// Put a new value on the stack
    pub fn push_value(&mut self, value: LV2Value) {
        self.vstack.push(value);
    }

    /// Remove the last value on stack
    pub fn pop_value(&mut self) -> LV2Result<LV2Value> {
        self.vstack
            .pop()
            .ok_or_else(|| LV2ErrorTy::ValueStackEmpty.into())
    }

    /// Get a mutable reference to the last value on stack
    pub fn last_value_mut(&mut self) -> LV2Result<&mut LV2Value> {
        self.vstack
            .last_mut()
            .ok_or_else(|| LV2ErrorTy::ValueStackEmpty.into())
    }

    /// Lookup a global value
    pub fn value_of<T>(&self, var: T) -> LV2Result<&LV2Value>
    where
        T: AsRef<str>,
    {
        self.globals
            .get(var.as_ref())
            .ok_or_else(|| (LV2ErrorTy::LookupFailed, var).into())
    }
}
