//! conversion of values

use lovm2_error::*;

use crate::value::Value;

pub type CastResult = Lovm2Result<Value>;

pub const RUVALUE_NIL_TY: u16 = 0;
pub const RUVALUE_BOOL_TY: u16 = 1;
pub const RUVALUE_INT_TY: u16 = 2;
pub const RUVALUE_FLOAT_TY: u16 = 3;
pub const RUVALUE_STR_TY: u16 = 4;
pub const RUVALUE_DICT_TY: u16 = 5;
pub const RUVALUE_LIST_TY: u16 = 6;

impl Value {
    pub fn cast(self, tid: u16) -> CastResult {
        match tid {
            RUVALUE_NIL_TY => unimplemented!(),
            RUVALUE_BOOL_TY => self.into_bool(),
            RUVALUE_INT_TY => self.into_integer(),
            RUVALUE_FLOAT_TY => self.into_float(),
            RUVALUE_STR_TY => self.into_str(),
            RUVALUE_DICT_TY => unimplemented!(),
            RUVALUE_LIST_TY => unimplemented!(),
            _ => unimplemented!(),
        }
    }

    pub fn into_bool(self) -> CastResult {
        match self {
            Value::Bool(_) => Ok(self),
            Value::Int(n) => Ok(Value::Bool(n != 0)),
            Value::Str(s) => Ok(Value::Bool(!s.is_empty())),
            Value::Dict(d) => Ok(Value::Bool(!d.is_empty())),
            Value::List(ls) => Ok(Value::Bool(!ls.is_empty())),
            // TODO: avoid clone
            Value::Ref(Some(r)) => r.borrow().clone().into_bool(),
            Value::Nil |
            // TODO: compare with 0
            Value::Float(_) |
            Value::Ref(_) => Ok(Value::Bool(false)),
        }
    }

    pub fn into_float(self) -> CastResult {
        match self {
            Value::Nil => unimplemented!(),
            Value::Bool(b) => Ok(Value::Float(if b { 1. } else { 0. })),
            Value::Int(n) => Ok(Value::Float(n as f64)),
            Value::Float(_) => Ok(self),
            Value::Str(s) => s
                .parse::<f64>()
                .map(Value::from)
                .map_err(|_| "not a float".into()),
            Value::Dict(_) => unimplemented!(),
            Value::List(_) => unimplemented!(),
            _ => panic!("TODO: ref does not have a type"),
        }
    }

    pub fn into_integer(self) -> CastResult {
        match self {
            Value::Nil => unimplemented!(),
            Value::Bool(b) => Ok(Value::Int(if b { 1 } else { 0 })),
            Value::Int(_) => Ok(self),
            Value::Float(n) => Ok(Value::Int(n as i64)),
            Value::Str(s) => s
                .parse::<i64>()
                .map(Value::from)
                .map_err(|_| "not an integer".into()),
            Value::Dict(_) => unimplemented!(),
            Value::List(_) => unimplemented!(),
            _ => panic!("TODO: ref does not have a type"),
        }
    }

    pub fn into_integer_round(self) -> CastResult {
        match self {
            Value::Float(n) => Ok(Value::Int(n.round() as i64)),
            _ => self.into_integer(),
        }
    }

    pub fn into_str(self) -> CastResult {
        let s = format!("{}", self);
        Ok(Value::Str(s))
    }

    pub fn type_id(&self) -> u16 {
        match self {
            Value::Nil => RUVALUE_BOOL_TY,
            Value::Bool(_) => RUVALUE_BOOL_TY,
            Value::Int(_) => RUVALUE_INT_TY,
            Value::Float(_) => RUVALUE_FLOAT_TY,
            Value::Str(_) => RUVALUE_STR_TY,
            Value::Dict(_) => RUVALUE_DICT_TY,
            Value::List(_) => RUVALUE_LIST_TY,
            _ => panic!("TODO: ref does not have a type"),
        }
    }
}
