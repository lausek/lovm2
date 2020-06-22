use std::collections::HashMap;

use crate::var::Variable;

#[derive(Clone, Debug, PartialEq)]
pub enum CoValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(HashMap<Variable, Box<CoValue>>),
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
