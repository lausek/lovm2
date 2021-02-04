//! Implementation of primitive operations on `Value` e.g. Add, Sub

use std::cmp::Ordering;

use super::Value::*;
use super::*;

impl std::ops::Add for Value {
    type Output = Lovm2Result<Value>;

    fn add(self, other: Value) -> Self::Output {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Int(a + b)),
            (Float(a), Float(b)) => Ok(Float(a + b)),
            (Str(a), Str(b)) => Ok(Str(format!("{}{}", a, b))),

            // switching positions is okay, because add is commutative
            (Float(a), b @ Int(_)) | (b @ Int(_), Float(a)) => Ok(Float(a + b.as_float_inner()?)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Lovm2Result<Value>;

    fn sub(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Int(a - b)),
            (Float(a), Float(b)) => Ok(Float(a - b)),

            // sub is not commutative
            (Float(a), b @ Int(_)) => Ok(Float(a - b.as_float_inner()?)),
            (b @ Int(_), Float(a)) => Ok(Float(b.as_float_inner()? - a)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Lovm2Result<Value>;

    fn mul(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Int(a * b)),
            (Float(a), Float(b)) => Ok(Float(a * b)),

            // switching positions is okay, because mul is commutative
            (Float(a), b @ Int(_)) | (b @ Int(_), Float(a)) => Ok(Float(a * b.as_float_inner()?)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Lovm2Result<Value>;

    fn div(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Int(a / b)),
            (Float(a), Float(b)) => Ok(Float(a / b)),

            // div is not commutative
            (Float(a), b @ Int(_)) => Ok(Float(a / b.as_float_inner()?)),
            (b @ Int(_), Float(a)) => Ok(Float(b.as_float_inner()? / a)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::Rem for Value {
    type Output = Lovm2Result<Value>;

    fn rem(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Int(a % b)),
            (Float(a), Float(b)) => Ok(Float(a % b)),

            // rem is not commutative
            (Float(a), b @ Int(_)) => Ok(Float(a % b.as_float_inner()?)),
            (b @ Int(_), Float(a)) => Ok(Float(b.as_float_inner()? % a)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::Shl for Value {
    type Output = Lovm2Result<Value>;

    fn shl(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Int(a << b)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::Shr for Value {
    type Output = Lovm2Result<Value>;

    fn shr(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(Int(a >> b)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::BitAnd for Value {
    type Output = Lovm2Result<Value>;

    fn bitand(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Bool(a), Bool(b)) => Ok(Bool(a && b)),
            (Int(a), Int(b)) => Ok(Int(a & b)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::BitOr for Value {
    type Output = Lovm2Result<Value>;

    fn bitor(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Bool(a), Bool(b)) => Ok(Bool(a || b)),
            (Int(a), Int(b)) => Ok(Int(a | b)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::BitXor for Value {
    type Output = Lovm2Result<Value>;

    fn bitxor(self, other: Value) -> Lovm2Result<Value> {
        match (self, other) {
            (Bool(a), Bool(b)) => Ok(Bool(a ^ b)),
            (Int(a), Int(b)) => Ok(Int(a ^ b)),
            _ => err_not_supported(),
        }
    }
}

impl std::ops::Not for Value {
    type Output = Lovm2Result<Value>;

    fn not(self) -> Lovm2Result<Value> {
        match self {
            Bool(a) => Ok(Bool(!a)),
            Int(a) => Ok(Int(!a)),
            _ => err_not_supported(),
        }
    }
}

impl Value {
    pub fn pow(&self, exp: Value) -> Lovm2Result<Value> {
        let exp = exp.as_integer_inner()?;
        match self {
            Int(base) => Ok(Int(base.pow(exp as u32))),
            Float(base) => Ok(Float(base.powi(exp as i32))),
            _ => err_not_supported(),
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        match (self, rhs) {
            (Nil, Nil) => true,
            (Bool(a), Bool(b)) => a == b,
            (Int(a), Int(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (Str(a), Str(b)) => a == b,
            (Dict(a), Dict(b)) => a == b,
            (List(a), List(b)) => a == b,
            (Any(a), Any(b)) => a == b,
            (Ref(r), other) | (other, Ref(r)) => r.eq(other),
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Ref(a), b) => a.borrow().unwrap().partial_cmp(b),
            (a, Ref(b)) => a.partial_cmp(&b.borrow().unwrap()),

            (Int(a), Int(b)) => a.partial_cmp(b),
            (Float(a), Float(b)) => a.partial_cmp(b),
            (Str(a), Str(b)) => a.partial_cmp(b),
            (inc @ Int(_), Float(b)) => inc.as_float_inner().ok().and_then(|a| a.partial_cmp(b)),
            (Float(a), inc @ Int(_)) => inc.as_float_inner().ok().and_then(|b| a.partial_cmp(&b)),
            _ => None,
        }
    }
}

impl std::cmp::Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
