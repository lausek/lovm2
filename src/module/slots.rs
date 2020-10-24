use serde::{de::Visitor, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::rc::Rc;

use crate::code::{CallProtocol, CodeObject, CodeObjectRef};
use crate::var::Variable;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Slots(
    #[serde(serialize_with = "serialize_slots")]
    #[serde(deserialize_with = "deserialize_slots")]
    HashMap<Variable, CodeObjectRef>,
);

impl Slots {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(slots: HashMap<Variable, CodeObjectRef>) -> Self {
        Self(slots)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Variable, CodeObjectRef> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, Variable, CodeObjectRef> {
        self.0.iter_mut()
    }

    pub fn get(&self, var: &Variable) -> Option<&CodeObjectRef> {
        self.0.get(var)
    }

    pub fn insert(&mut self, var: Variable, val: CodeObjectRef) {
        self.0.insert(var, val);
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

    d.deserialize_map(Unslotter)
}
