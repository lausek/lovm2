use std::collections::HashMap;

use crate::code::CodeObjectRef;
use crate::var::Variable;

pub struct Module {
    slots: HashMap<Variable, CodeObjectRef>,
}
