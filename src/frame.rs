use std::collections::HashMap;

use crate::value::Value;
use crate::var::Variable;

/// a stack frame used in `Context.lstack`
pub struct Frame {
    /// amount of parameters passed to the frame
    pub argn: u8,
    /// locals defined
    pub locals: HashMap<Variable, Value>,
}

impl Frame {
    pub fn new(argn: u8) -> Self {
        Self {
            argn,
            locals: HashMap::new(),
        }
    }

    pub fn value_of(&self, var: &Variable) -> Option<Value> {
        self.locals.get(var).cloned()
    }
}
