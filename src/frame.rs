use std::collections::HashMap;

use crate::value::RuValueRef;
use crate::var::Variable;

/// a stack frame used in `Context.lstack`
pub struct Frame {
    /// amount of parameters passed to the frame
    pub argn: u8,
    /// locals defined
    pub locals: HashMap<Variable, RuValueRef>,
}

impl Frame {
    pub fn new(argn: u8) -> Self {
        Self {
            argn,
            locals: HashMap::new(),
        }
    }
}
