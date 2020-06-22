use std::collections::HashMap;

use crate::var::Variable;
use crate::value::RuValue;

pub struct Frame {
    pub(crate) argn: u8,
    pub(crate) locals: HashMap<Variable, RuValue>,
}

impl Frame {
    pub fn new(argn: u8) -> Self {
        Self {
            argn,
            locals: HashMap::new(),
        }
    }
}
