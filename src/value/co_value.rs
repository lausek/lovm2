use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// constant values
///
/// these values are better suited for serialization and deserialization than `RuValue`. they are
/// not used in the vm directly. transform `CoValue`s into `RuValue`s using `value::instantiate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum CoValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(HashMap<CoValue, Box<CoValue>>),
    List(Vec<CoValue>),
}

impl From<bool> for CoValue {
    fn from(b: bool) -> Self {
        CoValue::Bool(b)
    }
}

impl From<i64> for CoValue {
    fn from(n: i64) -> Self {
        CoValue::Int(n)
    }
}

impl From<f64> for CoValue {
    fn from(n: f64) -> Self {
        CoValue::Float(n)
    }
}

impl From<&str> for CoValue {
    fn from(s: &str) -> Self {
        CoValue::Str(s.to_string())
    }
}

impl std::cmp::Eq for CoValue {}

impl std::hash::Hash for CoValue {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            CoValue::Nil => unimplemented!(),
            CoValue::Bool(b) => hasher.write_u8(*b as u8),
            CoValue::Int(n) => hasher.write_i64(*n),
            CoValue::Float(_) => unimplemented!(),
            CoValue::Str(s) => hasher.write(s.as_bytes()),
            CoValue::Dict(_) => unimplemented!(),
            CoValue::List(_) => unimplemented!(),
        }
    }
}
