//! implementation of primitive operations on `Value` e.g. Add, Sub

use std::cmp::Ordering;

use lovm2_error::*;

use super::Value::*;
use super::*;

impl std::ops::Add for Value {
    type Output = Lovm2Result<Value>;

    fn add(self, other: Value) -> Self::Output {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Value::Int(a + b)),
            (Float(a), Float(b)) => Ok(Value::Float(a + b)),
            (Str(a), Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
            (Float(a), b @ Int(_)) | (b @ Int(_), Float(a)) => {
                if let Float(b) = b.into_float()? {
                    Ok(Value::Float(a + b))
                } else {
                    unreachable!()
                }
            }
            _ => not_supported(),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Lovm2Result<Value>;

    fn sub(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Value::Int(a - b)),
            (Float(a), Float(b)) => Ok(Value::Float(a - b)),
            (Float(a), b @ Int(_)) | (b @ Int(_), Float(a)) => {
                if let Float(b) = b.into_float()? {
                    Ok(Value::Float(a - b))
                } else {
                    unreachable!()
                }
            }
            _ => not_supported(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Lovm2Result<Value>;

    fn mul(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Value::Int(a * b)),
            (Float(a), Float(b)) => Ok(Value::Float(a * b)),
            (Float(a), b @ Int(_)) | (b @ Int(_), Float(a)) => {
                if let Float(b) = b.into_float()? {
                    Ok(Value::Float(a * b))
                } else {
                    unreachable!()
                }
            }
            _ => not_supported(),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Lovm2Result<Value>;

    fn div(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Value::Int(a / b)),
            (Float(a), Float(b)) => Ok(Value::Float(a / b)),
            (Float(a), b @ Int(_)) | (b @ Int(_), Float(a)) => {
                if let Float(b) = b.into_float()? {
                    Ok(Value::Float(a / b))
                } else {
                    unreachable!()
                }
            }
            _ => not_supported(),
        }
    }
}

impl std::ops::Rem for Value {
    type Output = Lovm2Result<Value>;

    fn rem(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Value::Int(a % b)),
            (Float(a), Float(b)) => Ok(Value::Float(a % b)),
            (Float(a), b @ Int(_)) | (b @ Int(_), Float(a)) => {
                if let Float(b) = b.into_float()? {
                    Ok(Value::Float(a % b))
                } else {
                    unreachable!()
                }
            }
            _ => not_supported(),
        }
    }
}

impl std::ops::BitAnd for Value {
    type Output = Lovm2Result<Value>;

    fn bitand(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Bool(a), Bool(b)) => Ok(Value::Bool(a && b)),
            (Int(a), Int(b)) => Ok(Value::Int(a & b)),
            _ => not_supported(),
        }
    }
}

impl std::ops::BitOr for Value {
    type Output = Lovm2Result<Value>;

    fn bitor(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Bool(a), Bool(b)) => Ok(Value::Bool(a || b)),
            (Int(a), Int(b)) => Ok(Value::Int(a | b)),
            _ => not_supported(),
        }
    }
}

impl std::ops::Not for Value {
    type Output = Lovm2Result<Value>;

    fn not(self) -> Lovm2Result<Value> {
        match self {
            Bool(a) => Ok(Value::Bool(!a)),
            Int(a) => Ok(Value::Int(!a)),
            _ => not_supported(),
        }
    }
}

impl Value {
    pub fn pow(&self, exp: Value) -> Lovm2Result<Value> {
        if let Int(exp) = exp.into_integer()? {
            return match self {
                Int(base) => Ok(Int(base.pow(exp as u32))),
                Float(base) => Ok(Float(base.powi(exp as i32))),
                _ => not_supported(),
            };
        }
        not_supported()
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Int(a), Int(b)) => a.partial_cmp(b),
            (Float(a), Float(b)) => a.partial_cmp(b),
            (Str(a), Str(b)) => a.partial_cmp(b),
            (inc @ Int(_), Float(b)) => {
                if let Ok(Float(a)) = inc.clone().into_float() {
                    a.partial_cmp(b)
                } else {
                    None
                }
            }
            (Float(a), inc @ Int(_)) => {
                if let Ok(Float(b)) = inc.clone().into_float() {
                    a.partial_cmp(&b)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn lt(&self, other: &Value) -> bool {
        self.partial_cmp(other) == Some(Ordering::Less)
    }

    fn le(&self, other: &Value) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Less) | Some(Ordering::Equal)
        )
    }

    fn gt(&self, other: &Value) -> bool {
        self.partial_cmp(other) == Some(Ordering::Greater)
    }

    fn ge(&self, other: &Value) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater) | Some(Ordering::Equal)
        )
    }
}
