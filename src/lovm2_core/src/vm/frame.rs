//! Stores local function values
//!
//! Each frame contains the amount of arguments passed to the `CodeObject` on stack. Make
//! sure to pop exactly `argn` values from stack and leave one value as return.

use std::collections::HashMap;

use crate::value::Value;

use super::*;

/// A stack frame used in `Context.lstack`
#[derive(Debug)]
pub struct Frame {
    /// Amount of parameters passed to the frame
    pub argn: u8,
    /// Local variables
    locals: HashMap<String, Value>,
}

impl Frame {
    pub fn new(argn: u8) -> Self {
        Self {
            argn,
            locals: HashMap::new(),
        }
    }

    pub fn set_local<T>(&mut self, var: T, value: Value)
    where
        T: AsRef<str>,
    {
        self.locals.insert(var.as_ref().to_string(), value);
    }

    pub fn value_of<T>(&self, var: T) -> Lovm2Result<&Value>
    where
        T: AsRef<str>,
    {
        self.locals
            .get(var.as_ref())
            .ok_or_else(|| (Lovm2ErrorTy::LookupFailed, var).into())
    }
}
