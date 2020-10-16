use std::collections::HashMap;

use crate::value::RuValue;
use crate::var::Variable;

/// a stack frame used in `Context.lstack`
pub struct Frame {
    /// amount of parameters passed to the frame
    pub argn: u8,
    /// locals defined
    pub locals: HashMap<Variable, RuValue>,
}

impl Frame {
    pub fn new(argn: u8) -> Self {
        Self {
            argn,
            locals: HashMap::new(),
        }
    }

    pub fn value_of(&self, var: &Variable) -> Option<RuValue> {
        self.locals.get(var).cloned()
    }
}
