//! implementation of primitive operations on `Value` e.g. Add, Sub

use std::cmp::Ordering;

use crate::value::Value;

macro_rules! auto_implement_branch {
    ($v1:ident, $v2:ident; $ty1:ident, $ty2:ident => $func:tt) => {
        if let (Value::$ty1(a), Value::$ty2(b)) = (&$v1, &$v2) {
            return Value::$ty1($func(a.clone(), b.clone()));
        }
    };
    ($v1:ident, $v2:ident; $ty1:ident, $ty2:ident, $conv:ident => $func:tt) => {
        match ($v1, $v2) {
            (Value::$ty1(a), inc @ Value::$ty2(_)) | (inc @ Value::$ty2(_), Value::$ty1(a)) => {
                if let Ok(Value::$ty1(b)) = inc.$conv() {
                    return Value::$ty1($func(a.clone(), b.clone()));
                }
            }
            _ => {}
        }
    };
}

// TODO: this creates a closure and calls it. not good
macro_rules! auto_implement {
    {
        1, $tr:path, $method:ident;
        $( $($ty:ident),* => $func:tt; )*
    } => {
        impl $tr for Value {
            type Output = Value;

            fn $method(self) -> Value {
                match self {
                    $(
                        $(
                            Value::$ty(a) => Value::$ty($func(a)),
                        )*
                    )*
                    _ => unimplemented!(),
                }
            }
        }
    };

    {
        2, $tr:path, $method:ident;
        $( $ty1:ident, $ty2:ident $(, $conv:ident)? => $func:tt; )*
    } => {
        impl $tr for Value {
            type Output = Value;

            fn $method(self, other: Value) -> Value {
                $(
                    auto_implement_branch!(self, other; $ty1, $ty2 $(, $conv)? => $func);
                )*
                unimplemented!()
            }
        }
    };
}

auto_implement! {
    2, std::ops::Add, add;
    Int, Int => (|a, b| a + b);
    Float, Float => (|a, b| a + b);
    Str, Str => (|a, b| format!("{}{}", a, b));
    Float, Int, into_float => (|a, b| a + b);
}

auto_implement! {
    2, std::ops::Sub, sub;
    Int, Int => (|a, b| a - b);
    Float, Float => (|a, b| a - b);
    Float, Int, into_float => (|a, b| a - b);
}

auto_implement! {
    2, std::ops::Mul, mul;
    Int, Int => (|a, b| a * b);
    Float, Float => (|a, b| a * b);
    Float, Int, into_float => (|a, b| a * b);
}

auto_implement! {
    2, std::ops::Div, div;
    Int, Int => (|a, b| a / b);
    Float, Float => (|a, b| a / b);
    Float, Int, into_float => (|a, b| a / b);
}

auto_implement! {
    2, std::ops::Rem, rem;
    Int, Int => (|a, b| a % b);
    Float, Float => (|a, b| a % b);
    Float, Int, into_float => (|a, b| a % b);
}

auto_implement! {
    2, std::ops::BitAnd, bitand;
    Bool, Bool => (|a, b| a && b);
    Int, Int => (|a, b| a & b);
}

auto_implement! {
    2, std::ops::BitOr, bitor;
    Bool, Bool => (|a, b| a || b);
    Int, Int => (|a, b| a | b);
}

auto_implement! {
    1, std::ops::Not, not;
    Bool => (|a: bool| !a);
    Int => (|a: i64| !a);
}

impl Value {
    pub fn pow(&self, exp: Value) -> Value {
        if let Value::Int(exp) = exp.into_integer().unwrap() {
            return match self {
                Value::Int(base) => Value::Int(base.pow(exp as u32)),
                Value::Float(base) => Value::Float(base.powi(exp as i32)),
                _ => unimplemented!(),
            };
        }
        unimplemented!()
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Str(a), Value::Str(b)) => a.partial_cmp(b),
            // TODO: test this
            (inc @ Value::Int(_), Value::Float(b)) => {
                if let Ok(Value::Float(a)) = inc.clone().into_float() {
                    a.partial_cmp(b)
                } else {
                    None
                }
            }
            (Value::Float(a), inc @ Value::Int(_)) => {
                if let Ok(Value::Float(b)) = inc.clone().into_float() {
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
