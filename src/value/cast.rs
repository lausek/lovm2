//! Conversion of values

use lovm2_error::*;

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
    pub fn cast(self, ty: ValueType) -> Lovm2Result<Value> {
        match ty {
            ValueType::Bool => self.as_bool(),
            ValueType::Int => self.as_integer(),
            ValueType::Float => self.as_float(),
            ValueType::Str => self.as_str(),
            _ => not_supported(),
        }
    }

    pub fn cast_inplace(&mut self, ty: ValueType) -> Lovm2Result<()> {
        match ty {
            ValueType::Bool => *self = self.as_bool()?,
            ValueType::Int => *self = self.as_integer()?,
            ValueType::Float => *self = self.as_float()?,
            ValueType::Str => *self = self.as_str()?,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn as_bool(&self) -> Lovm2Result<Value> {
        self.as_bool_inner().map(Value::Bool)
    }

    pub fn as_float(&self) -> Lovm2Result<Value> {
        self.as_float_inner().map(Value::Float)
    }

    pub fn as_integer(&self) -> Lovm2Result<Value> {
        self.as_integer_inner().map(Value::Int)
    }

    pub fn as_integer_round(&self) -> Lovm2Result<Value> {
        self.as_integer_round_inner().map(Value::Int)
    }

    pub fn as_str(&self) -> Lovm2Result<Value> {
        self.as_str_inner().map(Value::Str)
    }

    pub fn as_any_inner(&self) -> Lovm2Result<AnyRef> {
        match self {
            Value::Any(r) => Ok(r.clone()),
            _ => not_supported(),
        }
    }

    pub fn as_any_ref(&self) -> Lovm2Result<AnyRef> {
        match self {
            Value::Any(ar) => Ok(ar.clone()),
            _ => not_supported(),
        }
    }

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
            _ => not_supported(),
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
            Value::Ref(r) => r.borrow()?.as_float_inner(),
            _ => not_supported(),
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
            Value::Ref(r) => r.borrow()?.as_integer_inner(),
            _ => not_supported(),
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

    pub fn type_id(&self) -> Lovm2Result<ValueType> {
        let tid = match self {
            Value::Nil => ValueType::Nil,
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::Str(_) => ValueType::Str,
            Value::Dict(_) => ValueType::Dict,
            Value::List(_) => ValueType::List,
            Value::Ref(r) => r.unref_total()?.type_id()?,
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
            _ => Err("not a valid type".to_string().into()),
        }
    }
}
