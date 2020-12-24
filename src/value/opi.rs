//! Implementation of primitive inplace operations on `Value`

use lovm2_error::*;

use super::Value::*;
use super::*;

impl Value {
    pub fn add_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Int(ref mut a), Int(b)) => *a += b,
            (Float(ref mut a), Float(b)) => *a += b,
            (Str(ref mut a), Str(b)) => *a = format!("{}{}", a, b),
            (Float(ref mut a), b @ Int(_)) => *a += b.as_float_inner()?,
            (a @ Int(_), b @ Float(_)) => *a = (a.clone() + b)?,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn sub_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Int(ref mut a), Int(b)) => *a -= b,
            (Float(ref mut a), Float(b)) => *a -= b,
            (Float(ref mut a), b @ Int(_)) => *a -= b.as_float_inner()?,
            (a @ Int(_), b @ Float(_)) => *a = (a.clone() - b)?,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn mul_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Int(ref mut a), Int(b)) => *a *= b,
            (Float(ref mut a), Float(b)) => *a *= b,
            (Float(ref mut a), b @ Int(_)) => *a *= b.as_float_inner()?,
            (a @ Int(_), b @ Float(_)) => *a = (a.clone() * b)?,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn div_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Int(ref mut a), Int(b)) => *a /= b,
            (Float(ref mut a), Float(b)) => *a /= b,
            (Float(ref mut a), b @ Int(_)) => *a /= b.as_float_inner()?,
            (a @ Int(_), b @ Float(_)) => *a = (a.clone() / b)?,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn rem_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Int(ref mut a), Int(b)) => *a %= b,
            (Float(ref mut a), Float(b)) => *a %= b,
            (Float(ref mut a), b @ Int(_)) => *a %= b.as_float_inner()?,
            (a @ Int(_), b @ Float(_)) => *a = (a.clone() % b)?,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn pow_inplace(&mut self, exp: Value) -> Lovm2Result<()> {
        let exp = exp.as_integer_inner()?;
        match self {
            Int(ref mut base) => *base = base.pow(exp as u32),
            Float(ref mut base) => *base = base.powi(exp as i32),
            _ => not_supported()?,
        };
        Ok(())
    }

    pub fn and_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Bool(a), Bool(b)) => *a &= b,
            (Int(a), Int(b)) => *a &= b,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn or_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Bool(a), Bool(b)) => *a |= b,
            (Int(a), Int(b)) => *a |= b,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn xor_inplace(&mut self, other: Value) -> Lovm2Result<()> {
        match (self, other) {
            (Bool(a), Bool(b)) => *a ^= b,
            (Int(a), Int(b)) => *a ^= b,
            _ => not_supported()?,
        }
        Ok(())
    }

    pub fn not_inplace(&mut self) -> Lovm2Result<()> {
        match self {
            Bool(ref mut a) => *a = !*a,
            Int(ref mut a) => *a = !*a,
            _ => not_supported()?,
        }
        Ok(())
    }
}
