use crate::value::RuValue;

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
        $( $($ty:ident),* => $func:tt; )*
    } => {
        impl $tr for RuValue {
            type Output = RuValue;
        
            fn $method(self, other: RuValue) -> RuValue {
                match (self, other) {
                    $(
                        $(
                            (RuValue::$ty(a), RuValue::$ty(b)) => RuValue::$ty($func(a, b)),
                        )*
                    )*
                    _ => unimplemented!(),
                }
            }
        }
    };
}

auto_implement!{
    2, std::ops::Add, add;
    Int, Float => (|a, b| a + b);
    Str => (|a, b| format!("{}{}", a, b));
}

auto_implement!{
    2, std::ops::Sub, sub;
    Int, Float => (|a, b| a - b);
}

auto_implement!{
    2, std::ops::Mul, mul;
    Int, Float => (|a, b| a * b);
}

auto_implement!{
    2, std::ops::Div, div;
    Int, Float => (|a, b| a / b);
}

auto_implement!{
    2, std::ops::Rem, rem;
    Int, Float => (|a, b| a % b);
}

auto_implement!{
    2, std::ops::BitAnd, bitand;
    Bool => (|a, b| a && b);
    Int => (|a, b| a & b);
}

auto_implement!{
    2, std::ops::BitOr, bitor;
    Bool => (|a, b| a || b);
    Int => (|a, b| a | b);
}

auto_implement!{
    1, std::ops::Not, not;
    Bool => (|a: bool| !a);
    Int => (|a: i64| !a);
}
