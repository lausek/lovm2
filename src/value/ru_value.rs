use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::var::Variable;

pub type RuDict = HashMap<Variable, RuValue>;
pub type RuDictRef = Rc<RefCell<RuDict>>;
pub type RuList = Vec<RuValue>;
pub type RuListRef = Rc<RefCell<RuList>>;

#[derive(Clone, Debug, PartialEq)]
pub enum RuValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Dict(RuDictRef),
    List(RuListRef),
}

impl std::fmt::Display for RuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuValue::Bool(b) => write!(f, "{}", b),
            RuValue::Int(n) => write!(f, "{}", n),
            RuValue::Float(n) => write!(f, "{}", n),
            RuValue::Str(s) => write!(f, "{}", s),
            _ => unimplemented!(),
            // RuValue::Dict(RuDictRef),
            // RuValue::List(RuListRef),
        }
    }
}
