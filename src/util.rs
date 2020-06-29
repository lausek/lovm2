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
macro_rules! call {
    ($name:ident $(, $arg:expr)* $(,)?) => {{
        Call::new(stringify!($name))
            $(
                .arg($arg)
            )*
    }};
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
                Box::new(CoValue::from($entry))
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
