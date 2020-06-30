use crate::value::RuValue;

pub type CastResult = Result<RuValue, String>;

pub const RUVALUE_BOOL_TY: u16 = 1;
pub const RUVALUE_INT_TY: u16 = 2;
pub const RUVALUE_FLOAT_TY: u16 = 3;
pub const RUVALUE_STR_TY: u16 = 4;
pub const RUVALUE_DICT_TY: u16 = 5;
pub const RUVALUE_LIST_TY: u16 = 6;

impl RuValue {
    pub fn cast(self, tid: u16) -> CastResult {
        match tid {
            RUVALUE_BOOL_TY => self.to_bool(),
            RUVALUE_INT_TY => self.to_integer(),
            RUVALUE_FLOAT_TY => self.to_float(),
            RUVALUE_STR_TY => self.to_str(),
            RUVALUE_DICT_TY => unimplemented!(),
            RUVALUE_LIST_TY => unimplemented!(),
            _ => unimplemented!(),
        }
    }

    pub fn to_bool(self) -> CastResult {
        unimplemented!()
    }

    pub fn to_float(self) -> CastResult {
        match self {
            RuValue::Bool(b) => Ok(RuValue::Float(if b {1.} else {0.})),
            RuValue::Int(n) => Ok(RuValue::Float(n as f64)),
            RuValue::Float(_) => Ok(self),
            RuValue::Str(_) => unimplemented!(),
            RuValue::Dict(_) => unimplemented!(),
            RuValue::List(_) => unimplemented!(),
        }
    }

    pub fn to_integer(self) -> CastResult {
        match self {
            RuValue::Bool(b) => Ok(RuValue::Int(if b {1} else {0})),
            RuValue::Int(_) => Ok(self),
            RuValue::Float(n) => Ok(RuValue::Int(n as i64)),
            RuValue::Str(_) => unimplemented!(),
            RuValue::Dict(_) => unimplemented!(),
            RuValue::List(_) => unimplemented!(),
        }
    }

    pub fn to_str(self) -> CastResult {
        unimplemented!()
    }

    pub fn type_id(&self) -> u16 {
        match self {
            RuValue::Bool(_) => RUVALUE_BOOL_TY,
            RuValue::Int(_) => RUVALUE_INT_TY,
            RuValue::Float(_) => RUVALUE_FLOAT_TY,
            RuValue::Str(_) => RUVALUE_STR_TY,
            RuValue::Dict(_) => RUVALUE_DICT_TY,
            RuValue::List(_) => RUVALUE_LIST_TY,
        }
    }
}