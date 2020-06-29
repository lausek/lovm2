use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum CoValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(HashMap<CoValue, Box<CoValue>>),
    List(Vec<Box<CoValue>>),
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
    fn hash<H>(&self, hasher: &mut H) where H: std::hash::Hasher {
        match self {
            CoValue::Bool(b) => hasher.write_u8(*b as u8),
            CoValue::Int(n) => hasher.write_i64(*n),
            CoValue::Float(_) => unimplemented!(),
            CoValue::Str(s) => hasher.write(s.as_bytes()),
            CoValue::Dict(_) => unimplemented!(),
            CoValue::List(_) => unimplemented!(),
        }
    }
}
