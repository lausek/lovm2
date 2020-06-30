pub mod builder;
pub mod standard;

use serde::{ser::SerializeMap, Serialize, Serializer};
use std::collections::HashMap;

use crate::code::CodeObjectRef;
use crate::var::Variable;

pub use self::builder::ModuleBuilder;
pub use self::standard::create_standard_module;

#[derive(Debug, Serialize)]
pub struct Module {
    #[serde(serialize_with = "serialize_slots")]
    pub slots: HashMap<Variable, CodeObjectRef>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
}

fn serialize_slots<S>(slots: &HashMap<Variable, CodeObjectRef>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut m = s.serialize_map(Some(slots.len()))?;
    for (key, value) in slots.iter() {
        if let Some(co) = value.code_object() {
            m.serialize_entry(key, co)?;
        }
    }
    m.end()
}
