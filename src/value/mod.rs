pub mod co_value;
pub mod operations;
pub mod ru_value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::context::Context;

pub use self::co_value::CoValue;
pub use self::ru_value::RuValue;

pub fn instantiate(ctx: &mut Context, covalue: &CoValue) -> RuValue {
    match covalue {
        CoValue::Bool(n) => RuValue::Bool(*n),
        CoValue::Int(n) => RuValue::Int(*n),
        CoValue::Float(n) => RuValue::Float(*n),
        CoValue::Str(n) => RuValue::Str(n.clone()),
        CoValue::Dict(map) => {
            let mut rumap = HashMap::new();
            for (key, value) in map.iter() {
                rumap.insert(key.clone(), instantiate(ctx, value));
            }
            RuValue::Dict(Rc::new(RefCell::new(rumap)))
        }
        CoValue::List(ls) => {
            let ruls = ls.iter().map(|item| instantiate(ctx, &item)).collect();
            RuValue::List(Rc::new(RefCell::new(ruls)))
        }
    }
}
