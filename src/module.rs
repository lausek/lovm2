use std::collections::HashMap;

use crate::code::CodeObject;
use crate::var::Variable;

pub struct Module {
    slots: HashMap<Variable, CodeObject>,
}
