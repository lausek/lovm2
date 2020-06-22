use std::collections::HashMap;
use std::cell::RefCell;
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
