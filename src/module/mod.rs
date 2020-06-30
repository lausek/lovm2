pub mod builder;
pub mod standard;

use serde::{ser::SerializeMap, Serialize, Serializer};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

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

    /*
    pub fn load_from_file<T>(path: T) -> Result<Module, String>
        where T: AsRef<Path>
    {
        let file = File::open(path)?;
        let module: Module = serde_cbor::from_reader(file)?;
        Ok(module)
    }
    */

    pub fn store_to_file<T>(&self, path: T) -> Result<(), String>
    where
        T: AsRef<Path>,
    {
        let file = File::create(path).map_err(|e| e.to_string())?;
        serde_cbor::to_writer(file, self).map_err(|e| e.to_string())?;
        Ok(())
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
