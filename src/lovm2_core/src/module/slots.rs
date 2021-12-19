//! Own type for maintaining callable functions inside a module

use std::collections::HashMap;

use crate::code::CallableRef;
use crate::var::LV2Variable;

/// Own type for maintaining callable functions inside a module
#[derive(Clone, Debug)]
pub struct Slots(HashMap<LV2Variable, CallableRef>);

impl Slots {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(slots: HashMap<LV2Variable, CallableRef>) -> Self {
        Self(slots)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, LV2Variable, CallableRef> {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<'_, LV2Variable, CallableRef> {
        self.0.iter_mut()
    }

    pub fn get(&self, var: &LV2Variable) -> Option<&CallableRef> {
        self.0.get(var)
    }

    pub fn insert<T>(&mut self, var: T, val: CallableRef)
    where
        T: Into<LV2Variable>,
    {
        self.0.insert(var.into(), val);
    }
}
