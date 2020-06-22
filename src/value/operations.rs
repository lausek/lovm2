use crate::value::RuValue;

macro_rules! auto_implement {
    {
        $tr:path, $method:ident;
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
    std::ops::Add, add;
    Int, Float => (|a, b| a + b);
    Str => (|a, b| format!("{}{}", a, b));
}

auto_implement!{
    std::ops::Sub, sub;
    Int, Float => (|a, b| a - b);
}

auto_implement!{
    std::ops::Mul, mul;
    Int, Float => (|a, b| a * b);
}

auto_implement!{
    std::ops::Div, div;
    Int, Float => (|a, b| a / b);
}
