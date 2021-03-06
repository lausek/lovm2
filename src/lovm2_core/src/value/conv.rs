//! Conversion of values

use super::*;

/// Type of a value as integer
#[derive(Clone, Debug)]
#[repr(u16)]
pub enum ValueType {
    Nil = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    Str = 4,
    Dict = 5,
    List = 6,
}

impl Value {
    /// Try converting the value into given type.
    pub fn conv(self, ty: ValueType) -> Lovm2Result<Value> {
        match ty {
            ValueType::Bool => self.as_bool(),
            ValueType::Int => self.as_integer(),
            ValueType::Float => self.as_float(),
            ValueType::Str => self.as_str(),
            _ => err_not_supported(),
        }
    }

    /// Try converting the value into given type and update inplace.
    pub fn cast_inplace(&mut self, ty: ValueType) -> Lovm2Result<()> {
        match ty {
            ValueType::Bool => *self = self.as_bool()?,
            ValueType::Int => *self = self.as_integer()?,
            ValueType::Float => *self = self.as_float()?,
            ValueType::Str => *self = self.as_str()?,
            _ => err_not_supported()?,
        }
        Ok(())
    }

    /// Try turning a value into `Bool`.
    pub fn as_bool(&self) -> Lovm2Result<Value> {
        self.as_bool_inner().map(Value::Bool)
    }

    /// Try turning a value into `Float`.
    pub fn as_float(&self) -> Lovm2Result<Value> {
        self.as_float_inner().map(Value::Float)
    }

    /// Try turning a value into `Int`.
    pub fn as_integer(&self) -> Lovm2Result<Value> {
        self.as_integer_inner().map(Value::Int)
    }

    /// Try turning a value into `Int` while doing correct float rounding.
    pub fn as_integer_round(&self) -> Lovm2Result<Value> {
        self.as_integer_round_inner().map(Value::Int)
    }

    /// Try turning a value into `Str`.
    pub fn as_str(&self) -> Lovm2Result<Value> {
        self.as_str_inner().map(Value::Str)
    }

    /// Try getting the contained `AnyRef`.
    pub fn as_any_inner(&self) -> Lovm2Result<AnyRef> {
        match self {
            Value::Any(r) => Ok(r.clone()),
            Value::Ref(r) => r.borrow()?.as_any_inner(),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `bool`.
    pub fn as_bool_inner(&self) -> Lovm2Result<bool> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::Int(n) => Ok(*n != 0),
            Value::Str(s) => Ok(!s.is_empty()),
            Value::Dict(d) => Ok(!d.is_empty()),
            Value::List(ls) => Ok(!ls.is_empty()),
            Value::Ref(r) => if let Ok(r) = r.borrow() {
                r.as_bool_inner()
            } else {
                Ok(false)
            }
            Value::Nil |
            // TODO: compare with 0
            Value::Float(_) => Ok(false),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `f64`.
    pub fn as_float_inner(&self) -> Lovm2Result<f64> {
        match self {
            Value::Nil => err_not_supported(),
            Value::Bool(b) => Ok(if *b { 1. } else { 0. }),
            Value::Int(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            Value::Str(s) => s.parse::<f64>().or_else(|_| err_from_string("not a float")),
            Value::Dict(_) => err_not_supported(),
            Value::List(_) => err_not_supported(),
            Value::Ref(r) => r.borrow()?.as_float_inner(),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `i64`.
    pub fn as_integer_inner(&self) -> Lovm2Result<i64> {
        match self {
            Value::Nil => err_not_supported(),
            Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Value::Int(n) => Ok(*n),
            Value::Float(n) => Ok(*n as i64),
            Value::Str(s) => s
                .parse::<i64>()
                .or_else(|_| err_from_string("not an integer")),
            Value::Dict(_) => err_not_supported(),
            Value::List(_) => err_not_supported(),
            Value::Ref(r) => r.borrow()?.as_integer_inner(),
            _ => err_not_supported(),
        }
    }

    /// Try turning a value into a Rust `i64` while doing correct float rounding.
    pub fn as_integer_round_inner(&self) -> Lovm2Result<i64> {
        if let Value::Float(n) = self {
            Ok(n.round() as i64)
        } else {
            self.as_integer_inner()
        }
    }

    /// Try turning a value into a Rust `String`.
    pub fn as_str_inner(&self) -> Lovm2Result<String> {
        Ok(format!("{}", self))
    }

    /// Return the `ValueType` of this value. Used for converting between values.
    pub fn type_id(&self) -> Lovm2Result<ValueType> {
        let tid = match self {
            Value::Nil => ValueType::Nil,
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::Str(_) => ValueType::Str,
            Value::Dict(_) => ValueType::Dict,
            Value::List(_) => ValueType::List,
            Value::Ref(r) => r.unref_to_value()?.borrow().type_id()?,
            _ => todo!(),
        };
        Ok(tid)
    }
}

impl ValueType {
    pub fn from_raw(tid: u16) -> Lovm2Result<ValueType> {
        match tid {
            0 => Ok(ValueType::Nil),
            1 => Ok(ValueType::Bool),
            2 => Ok(ValueType::Int),
            3 => Ok(ValueType::Float),
            4 => Ok(ValueType::Str),
            5 => Ok(ValueType::Dict),
            6 => Ok(ValueType::List),
            _ => err_from_string("not a valid type"),
        }
    }
}
