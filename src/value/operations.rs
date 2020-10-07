//! implementation of primitive operations on `RuValue` e.g. Add, Sub

use std::cmp::Ordering;

use crate::value::RuValue;

macro_rules! auto_implement_branch {
    ($v1:ident, $v2:ident; $ty1:ident, $ty2:ident => $func:tt) => {
        if let (RuValue::$ty1(a), RuValue::$ty2(b)) = (&$v1, &$v2) {
            return RuValue::$ty1($func(a.clone(), b.clone()));
        }
    };
    ($v1:ident, $v2:ident; $ty1:ident, $ty2:ident, $conv:ident => $func:tt) => {
        match ($v1, $v2) {
            (RuValue::$ty1(a), inc @ RuValue::$ty2(_))
            | (inc @ RuValue::$ty2(_), RuValue::$ty1(a)) => {
                if let Ok(RuValue::$ty1(b)) = inc.$conv() {
                    return RuValue::$ty1($func(a.clone(), b.clone()));
                }
            }
            _ => {}
        }
    };
}

macro_rules! auto_implement {
    {
        1, $tr:path, $method:ident;
        $( $($ty:ident),* => $func:tt; )*
    } => {
        impl $tr for RuValue {
            type Output = RuValue;

            fn $method(self) -> RuValue {
                match self {
                    $(
                        $(
                            RuValue::$ty(a) => RuValue::$ty($func(a)),
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
        impl $tr for RuValue {
            type Output = RuValue;

            fn $method(self, other: RuValue) -> RuValue {
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

impl RuValue {
    pub fn pow(&self, exp: RuValue) -> RuValue {
        if let RuValue::Int(exp) = exp.into_integer().unwrap() {
            return match self {
                RuValue::Int(base) => RuValue::Int(base.pow(exp as u32).into()),
                RuValue::Float(base) => RuValue::Float(base.powi(exp as i32).into()),
                _ => unimplemented!(),
            };
        }
        unimplemented!()
    }
}

impl std::cmp::PartialOrd for RuValue {
    fn partial_cmp(&self, other: &RuValue) -> Option<Ordering> {
        match (self, other) {
            (RuValue::Int(a), RuValue::Int(b)) => a.partial_cmp(b),
            (RuValue::Float(a), RuValue::Float(b)) => a.partial_cmp(b),
            (RuValue::Str(a), RuValue::Str(b)) => a.partial_cmp(b),
            // TODO: test this
            (inc @ RuValue::Int(_), RuValue::Float(b)) => {
                if let Ok(RuValue::Float(a)) = inc.clone().into_float() {
                    a.partial_cmp(b)
                } else {
                    None
                }
            }
            (RuValue::Float(a), inc @ RuValue::Int(_)) => {
                if let Ok(RuValue::Float(b)) = inc.clone().into_float() {
                    a.partial_cmp(&b)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn lt(&self, other: &RuValue) -> bool {
        self.partial_cmp(other) == Some(Ordering::Less)
    }

    fn le(&self, other: &RuValue) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn gt(&self, other: &RuValue) -> bool {
        self.partial_cmp(other) == Some(Ordering::Greater)
    }

    fn ge(&self, other: &RuValue) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}
