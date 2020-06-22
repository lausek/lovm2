use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::context::Context;
use crate::var::Variable;

pub type RuDict = HashMap<Variable, RuValue>;
pub type RuDictRef = Rc<RefCell<RuDict>>;
pub type RuList = Vec<RuValue>;
pub type RuListRef = Rc<RefCell<RuList>>;

pub enum CoValue {
    Int(i64),
    Float(f64),
    Str(String),
    Dict(HashMap<Variable, Box<CoValue>>),
    List(Vec<Box<CoValue>>),
}

pub enum RuValue {
    Int(i64),
    Float(f64),
    Str(String),
    Dict(RuDictRef),
    List(RuListRef),
}

pub fn instantiate(ctx: &mut Context, covalue: CoValue) -> RuValue {
    RuValue::Int(0)
}
