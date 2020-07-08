pub mod builder;
pub mod shared;
pub mod standard;

use serde::{de::Visitor, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use crate::code::{CallProtocol, CodeObject, CodeObjectRef};
use crate::var::Variable;

pub use self::builder::ModuleBuilder;
pub use self::shared::SharedObjectModule;
pub use self::standard::create_standard_module;

pub trait ModuleProtocol {
    fn slot(&self, name: &Variable) -> Option<Rc<dyn CallProtocol>>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Module {
    #[serde(serialize_with = "serialize_slots")]
    #[serde(deserialize_with = "deserialize_slots")]
    pub slots: HashMap<Variable, CodeObjectRef>,
}

impl ModuleProtocol for Module {
    fn slot(&self, name: &Variable) -> Option<Rc<dyn CallProtocol>> {
        self.slots
            .get(name)
            .map(|co_ref| co_ref.clone() as Rc<dyn CallProtocol>)
    }
}

impl Module {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn load_from_file<T>(path: T) -> Result<Module, String>
    where
        T: AsRef<Path>,
    {
        // try loading module as shared object
        if let Ok(so_module) = SharedObjectModule::load_from_file(path) {
            return Ok(so_module);
        }

        let file = File::open(path).map_err(|e| e.to_string())?;
        let module: Module = serde_cbor::from_reader(file).map_err(|e| e.to_string())?;
        Ok(module)
    }

    // TODO: could lead to errors when two threads serialize to the same file
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

fn deserialize_slots<'d, D>(d: D) -> Result<HashMap<Variable, CodeObjectRef>, D::Error>
where
    D: Deserializer<'d>,
{
    struct Unslotter;

    impl<'de> Visitor<'de> for Unslotter {
        type Value = HashMap<Variable, CodeObjectRef>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "do things")
        }

        fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((key, value)) = access.next_entry::<Variable, CodeObject>()? {
                map.insert(key, Rc::new(value) as Rc<dyn CallProtocol>);
            }
            Ok(map)
        }
    }

    d.deserialize_any(Unslotter)
}
