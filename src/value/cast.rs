use lovm2_error::*;

use crate::value::RuValue;

pub type CastResult = Lovm2Result<RuValue>;

pub const RUVALUE_NIL_TY: u16 = 0;
pub const RUVALUE_BOOL_TY: u16 = 1;
pub const RUVALUE_INT_TY: u16 = 2;
pub const RUVALUE_FLOAT_TY: u16 = 3;
pub const RUVALUE_STR_TY: u16 = 4;
pub const RUVALUE_DICT_TY: u16 = 5;
pub const RUVALUE_LIST_TY: u16 = 6;

impl RuValue {
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
        unimplemented!()
    }

    pub fn into_float(self) -> CastResult {
        match self {
            RuValue::Nil => unimplemented!(),
            RuValue::Bool(b) => Ok(RuValue::Float(if b { 1. } else { 0. })),
            RuValue::Int(n) => Ok(RuValue::Float(n as f64)),
            RuValue::Float(_) => Ok(self),
            RuValue::Str(s) => s
                .parse::<f64>()
                .map(RuValue::from)
                .map_err(|_| "not a float".into()),
            RuValue::Dict(_) => unimplemented!(),
            RuValue::List(_) => unimplemented!(),
            _ => panic!("TODO: ref does not have a type"),
        }
    }

    pub fn into_integer(self) -> CastResult {
        match self {
            RuValue::Nil => unimplemented!(),
            RuValue::Bool(b) => Ok(RuValue::Int(if b { 1 } else { 0 })),
            RuValue::Int(_) => Ok(self),
            RuValue::Float(n) => Ok(RuValue::Int(n as i64)),
            RuValue::Str(s) => s
                .parse::<i64>()
                .map(RuValue::from)
                .map_err(|_| "not an integer".into()),
            RuValue::Dict(_) => unimplemented!(),
            RuValue::List(_) => unimplemented!(),
            _ => panic!("TODO: ref does not have a type"),
        }
    }

    pub fn into_integer_round(self) -> CastResult {
        match self {
            RuValue::Float(n) => Ok(RuValue::Int(n.round() as i64)),
            _ => self.into_integer(),
        }
    }

    pub fn into_str(self) -> CastResult {
        let s = format!("{}", self);
        Ok(RuValue::Str(s))
    }

    pub fn type_id(&self) -> u16 {
        match self {
            RuValue::Nil => RUVALUE_BOOL_TY,
            RuValue::Bool(_) => RUVALUE_BOOL_TY,
            RuValue::Int(_) => RUVALUE_INT_TY,
            RuValue::Float(_) => RUVALUE_FLOAT_TY,
            RuValue::Str(_) => RUVALUE_STR_TY,
            RuValue::Dict(_) => RUVALUE_DICT_TY,
            RuValue::List(_) => RUVALUE_LIST_TY,
            _ => panic!("TODO: ref does not have a type"),
        }
    }
}
