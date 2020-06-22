use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::context::Context;
use crate::var::Variable;

pub type RuDict = HashMap<Variable, RuValue>;
pub type RuDictRef = Rc<RefCell<RuDict>>;
pub type RuList = Vec<RuValue>;
pub type RuListRef = Rc<RefCell<RuList>>;

#[derive(Clone, Debug, PartialEq)]
pub enum CoValue {
    Int(i64),
    Float(f64),
    Str(String),
    Dict(HashMap<Variable, Box<CoValue>>),
    List(Vec<Box<CoValue>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuValue {
    Int(i64),
    Float(f64),
    Str(String),
    Dict(RuDictRef),
    List(RuListRef),
}

pub fn instantiate(ctx: &mut Context, covalue: &CoValue) -> RuValue {
    match covalue {
        CoValue::Int(n) => RuValue::Int(*n),
        CoValue::Float(n) => RuValue::Float(*n),
        CoValue::Str(n) => RuValue::Str(n.clone()),
        CoValue::Dict(map) => {
            let mut rumap = HashMap::new();
            for (key, value) in map.iter() {
                rumap.insert(key.clone(), instantiate(ctx, value));
            }
            RuValue::Dict(Rc::new(RefCell::new(rumap)))
        },
        CoValue::List(ls) => {
            let ruls = ls.iter().map(|item| instantiate(ctx, &item)).collect();
            RuValue::List(Rc::new(RefCell::new(ruls)))
        },
    }
}
