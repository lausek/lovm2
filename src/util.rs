#[macro_export]
macro_rules! define_code {
    {
        $(consts {$($cval:expr),*})?
        $(globals {$($gname:ident),*})?
        $(locals {$($lname:ident),*})?
        {
            $( $inx:ident $($args:expr),* ; )*
        }
    } => {{
        let builder = CodeObjectBuilder::new()
            $(.consts(vec![$( CoValue::from($cval) ),*]))?
            $(.locals(vec![$( Variable::from(stringify!($lname)) ),*]))?
            $(.globals(vec![$( Variable::from(stringify!($gname)) ),*]))?
            ;

        let mut c = vec![
            $(
                define_code! { compile_inx $inx $(, $args)* },
            )*
        ];

        builder.code(c).build().unwrap()
    }};

    { compile_inx $inx:ident } => { Instruction::$inx };
    { compile_inx $inx:ident $(, $args:expr)+ } => { Instruction::$inx($($args),*) };
}

#[macro_export]
macro_rules! access {
    ($name:ident, $key:ident $(, $rest:ident)* $(,)?) => {{
        let mut v = vec![];
        v.push(stringify!($key).into());
        $(
            v.push(stringify!($rest).into());
        )*
        Access::new(stringify!($name).into(), v)
    }};
}

#[macro_export]
macro_rules! call {
    ($name:ident $(, $arg:tt)* $(,)?) => {{
        Call::new(stringify!($name))
            $(
                .arg(call!(1, $arg))
            )*
    }};
    (1, $arg:ident) => {{ var!($arg) }};
    (1, $arg:expr) => {{ $arg }};
}

#[macro_export]
macro_rules! co_dict {
    ($($key:expr => $val:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(
            map.insert(CoValue::from($key), Box::new(CoValue::from($val)));
        )*
        CoValue::Dict(map)
    }};
}

#[macro_export]
macro_rules! co_list {
    ($($entry:expr),* $(,)?) => {{
        CoValue::List(vec![
            $(
                CoValue::from($entry)
            ),*
        ])
    }};
}

#[macro_export]
macro_rules! var {
    ($name:ident) => {
        Variable::from(stringify!($name))
    };
}
