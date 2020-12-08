//! conversion of values

use lovm2_error::*;

use super::*;

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
            RUVALUE_BOOL_TY => self.as_bool(),
            RUVALUE_INT_TY => self.as_integer(),
            RUVALUE_FLOAT_TY => self.as_float(),
            RUVALUE_STR_TY => self.as_str(),
            _ => not_supported(),
        }
    }

    pub fn cast_inplace(&mut self, tid: u16) -> Lovm2Result<()> {
        match tid {
            RUVALUE_BOOL_TY => *self = self.as_bool()?,
            RUVALUE_INT_TY => *self = self.as_integer()?,
            RUVALUE_FLOAT_TY => *self = self.as_float()?,
            RUVALUE_STR_TY => *self = self.as_str()?,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn as_bool(&self) -> CastResult {
        self.as_bool_inner().map(Value::Bool)
    }

    pub fn as_float(&self) -> CastResult {
        self.as_float_inner().map(Value::Float)
    }

    pub fn as_integer(&self) -> CastResult {
        self.as_integer_inner().map(Value::Int)
    }

    pub fn as_integer_round(&self) -> CastResult {
        self.as_integer_round_inner().map(Value::Int)
    }

    pub fn as_str(&self) -> CastResult {
        self.as_str_inner().map(Value::Str)
    }

    pub fn as_bool_inner(&self) -> Lovm2Result<bool> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::Int(n) => Ok(*n != 0),
            Value::Str(s) => Ok(!s.is_empty()),
            Value::Dict(d) => Ok(!d.is_empty()),
            Value::List(ls) => Ok(!ls.is_empty()),
            Value::Ref(Some(r)) => r.borrow().as_bool_inner(),
            Value::Nil |
            // TODO: compare with 0
            Value::Float(_) |
            Value::Ref(_) => Ok(false),
        }
    }

    pub fn as_float_inner(&self) -> Lovm2Result<f64> {
        match self {
            Value::Nil => not_supported(),
            Value::Bool(b) => Ok(if *b { 1. } else { 0. }),
            Value::Int(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            Value::Str(s) => s.parse::<f64>().map_err(|_| "not a float".into()),
            Value::Dict(_) => not_supported(),
            Value::List(_) => not_supported(),
            _ => panic!("TODO: ref does not have a type"),
        }
    }

    pub fn as_integer_inner(&self) -> Lovm2Result<i64> {
        match self {
            Value::Nil => not_supported(),
            Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Value::Int(n) => Ok(*n),
            Value::Float(n) => Ok(*n as i64),
            Value::Str(s) => s.parse::<i64>().map_err(|_| "not an integer".into()),
            Value::Dict(_) => not_supported(),
            Value::List(_) => not_supported(),
            _ => panic!("TODO: ref does not have a type"),
        }
    }

    pub fn as_integer_round_inner(&self) -> Lovm2Result<i64> {
        if let Value::Float(n) = self {
            Ok(n.round() as i64)
        } else {
            self.as_integer_inner()
        }
    }

    pub fn as_str_inner(&self) -> Lovm2Result<String> {
        Ok(format!("{}", self))
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
