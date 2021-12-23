//! Conversion between [LV2Value].

use super::*;

/// Type of a value as integer.
#[derive(Clone, Debug)]
#[repr(u16)]
pub enum LV2ValueType {
    Nil = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    Str = 4,
    Dict = 5,
    List = 6,
}

impl LV2Value {
    /// Try converting the value into given type.
    pub fn conv(self, ty: LV2ValueType) -> LV2Result<LV2Value> {
        match ty {
            LV2ValueType::Bool => self.as_bool(),
            LV2ValueType::Int => self.as_integer(),
            LV2ValueType::Float => self.as_float(),
            LV2ValueType::Str => self.as_str(),
            _ => err_not_supported(),
        }
    }

    /// Try converting the value into given type and update inplace.
    pub fn cast_inplace(&mut self, ty: LV2ValueType) -> LV2Result<()> {
        match ty {
            LV2ValueType::Bool => *self = self.as_bool()?,
            LV2ValueType::Int => *self = self.as_integer()?,
            LV2ValueType::Float => *self = self.as_float()?,
            LV2ValueType::Str => *self = self.as_str()?,
            _ => err_not_supported()?,
        }
        Ok(())
    }

    /// Try turning a value into [Bool][LV2Value::Bool].
    pub fn as_bool(&self) -> LV2Result<LV2Value> {
        self.as_bool_inner().map(LV2Value::Bool)
    }

    /// Try turning a value into [Float][LV2Value::Float].
    pub fn as_float(&self) -> LV2Result<LV2Value> {
        self.as_float_inner().map(LV2Value::Float)
    }

    /// Try turning a value into [Int][LV2Value::Int].
    pub fn as_integer(&self) -> LV2Result<LV2Value> {
        self.as_integer_inner().map(LV2Value::Int)
    }

    /// Try turning a value into [Int][LV2Value::Int] while rounding.
    pub fn as_integer_round(&self) -> LV2Result<LV2Value> {
        self.as_integer_round_inner().map(LV2Value::Int)
    }

    /// Try turning a value into [Str][LV2Value::Str].
    pub fn as_str(&self) -> LV2Result<LV2Value> {
        self.as_str_inner().map(LV2Value::Str)
    }

    /// Try getting the contained [LV2AnyRef].
    pub fn as_any_inner(&self) -> LV2Result<LV2AnyRef> {
        match self {
            LV2Value::Any(r) => Ok(r.clone()),
            LV2Value::Ref(r) => r.borrow()?.as_any_inner(),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `bool`.
    pub fn as_bool_inner(&self) -> LV2Result<bool> {
        match self {
            LV2Value::Bool(b) => Ok(*b),
            LV2Value::Int(n) => Ok(*n != 0),
            LV2Value::Str(s) => Ok(!s.is_empty()),
            LV2Value::Dict(d) => Ok(!d.is_empty()),
            LV2Value::List(ls) => Ok(!ls.is_empty()),
            LV2Value::Ref(r) => if let Ok(r) = r.borrow() {
                r.as_bool_inner()
            } else {
                Ok(false)
            }
            LV2Value::Nil |
            // TODO: compare with 0
            LV2Value::Float(_) => Ok(false),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `f64`.
    pub fn as_float_inner(&self) -> LV2Result<f64> {
        match self {
            LV2Value::Nil => err_not_supported(),
            LV2Value::Bool(b) => Ok(if *b { 1. } else { 0. }),
            LV2Value::Int(n) => Ok(*n as f64),
            LV2Value::Float(n) => Ok(*n),
            LV2Value::Str(s) => s.parse::<f64>().or_else(|_| err_from_string("not a float")),
            LV2Value::Dict(_) => err_not_supported(),
            LV2Value::List(_) => err_not_supported(),
            LV2Value::Ref(r) => r.borrow()?.as_float_inner(),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `i64`.
    pub fn as_integer_inner(&self) -> LV2Result<i64> {
        match self {
            LV2Value::Nil => err_not_supported(),
            LV2Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
            LV2Value::Int(n) => Ok(*n),
            LV2Value::Float(n) => Ok(*n as i64),
            LV2Value::Str(s) => s
                .parse::<i64>()
                .or_else(|_| err_from_string("not an integer")),
            LV2Value::Dict(_) => err_not_supported(),
            LV2Value::List(_) => err_not_supported(),
            LV2Value::Ref(r) => r.borrow()?.as_integer_inner(),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `i64` while rounding.
    pub fn as_integer_round_inner(&self) -> LV2Result<i64> {
        if let LV2Value::Float(n) = self {
            Ok(n.round() as i64)
        } else {
            self.as_integer_inner()
        }
    }

    /// Try turning a value into a Rust `String`.
    pub fn as_str_inner(&self) -> LV2Result<String> {
        Ok(format!("{}", self))
    }

    /// Return the `ValueType` of this value. Used for converting between values.
    pub fn type_id(&self) -> LV2Result<LV2ValueType> {
        let tid = match self {
            LV2Value::Nil => LV2ValueType::Nil,
            LV2Value::Bool(_) => LV2ValueType::Bool,
            LV2Value::Int(_) => LV2ValueType::Int,
            LV2Value::Float(_) => LV2ValueType::Float,
            LV2Value::Str(_) => LV2ValueType::Str,
            LV2Value::Dict(_) => LV2ValueType::Dict,
            LV2Value::List(_) => LV2ValueType::List,
            LV2Value::Ref(r) => r.unref_to_value()?.borrow().type_id()?,
            _ => todo!(),
        };
        Ok(tid)
    }
}

impl LV2ValueType {
    pub fn from_raw(tid: u16) -> LV2Result<LV2ValueType> {
        match tid {
            0 => Ok(LV2ValueType::Nil),
            1 => Ok(LV2ValueType::Bool),
            2 => Ok(LV2ValueType::Int),
            3 => Ok(LV2ValueType::Float),
            4 => Ok(LV2ValueType::Str),
            5 => Ok(LV2ValueType::Dict),
            6 => Ok(LV2ValueType::List),
            _ => err_from_string("not a valid type"),
        }
    }
}
