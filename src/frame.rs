use std::collections::HashMap;

use crate::value::RuValue;
use crate::var::Variable;

pub struct Frame {
    pub argn: u8,
    pub locals: HashMap<Variable, RuValue>,
}

impl Frame {
    pub fn new(argn: u8) -> Self {
        Self {
            argn,
            locals: HashMap::new(),
        }
    }
}
