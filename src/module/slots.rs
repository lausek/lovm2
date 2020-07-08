use std::collections::HashMap;

use crate::code::CodeObjectRef;
use crate::var::Variable;

pub struct Slots(HashMap<Variable, CodeObjectRef>);

impl Slots {
    pub fn from(slots: HashMap<Variable, CodeObjectRef>) -> Self {
        Self(slots)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Variable, CodeObjectRef> {
        self.0.iter()
    }
}
