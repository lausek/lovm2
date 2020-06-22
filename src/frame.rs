use std::collections::HashMap;

use crate::var::Variable;
use crate::value::RuValue;

pub struct Frame {
    argn: u8,
    locals: HashMap<Variable, RuValue>,
}
