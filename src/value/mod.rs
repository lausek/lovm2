pub mod cast;
pub mod co_value;
pub mod operations;
pub mod ru_value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub use self::co_value::CoValue;
pub use self::ru_value::{box_ruvalue, RuValue, RuValueRef};

pub fn instantiate(covalue: &CoValue) -> RuValue {
    match covalue {
        CoValue::Nil => RuValue::Nil,
        CoValue::Bool(n) => RuValue::Bool(*n),
        CoValue::Int(n) => RuValue::Int(*n),
        CoValue::Float(n) => RuValue::Float(*n),
        CoValue::Str(n) => RuValue::Str(n.clone()),
        CoValue::Dict(map) => {
            let mut rumap = HashMap::new();
            for (key, value) in map.iter() {
                rumap.insert(instantiate(key), instantiate(value));
            }
            RuValue::Dict(Rc::new(RefCell::new(rumap)))
        }
        CoValue::List(ls) => {
            let ruls = ls.iter().map(|item| instantiate(&item)).collect();
            RuValue::List(Rc::new(RefCell::new(ruls)))
        }
    }
}
