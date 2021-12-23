//! Own type for maintaining callable functions inside a module.

use std::collections::HashMap;

use crate::code::LV2CallableRef;
use crate::var::LV2Variable;

/// Own type for maintaining callable functions inside a module.
#[derive(Clone, Debug)]
pub struct LV2ModuleSlots(HashMap<LV2Variable, LV2CallableRef>);

impl LV2ModuleSlots {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(slots: HashMap<LV2Variable, LV2CallableRef>) -> Self {
        Self(slots)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, LV2Variable, LV2CallableRef> {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<'_, LV2Variable, LV2CallableRef> {
        self.0.iter_mut()
    }

    pub fn get(&self, var: &LV2Variable) -> Option<&LV2CallableRef> {
        self.0.get(var)
    }

    pub fn insert<T>(&mut self, var: T, val: LV2CallableRef)
    where
        T: Into<LV2Variable>,
    {
        self.0.insert(var.into(), val);
    }
}
