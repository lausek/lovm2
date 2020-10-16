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
    ($name:ident, $key:expr $(, $rest:expr)* $(,)?) => {{
        Access::target(stringify!($name).into())
            .at($key)
            $(
                .at($rest)
            )*
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
        let mut dict = Initialize::dict();
        $(
            dict.add_by_key($key, $val);
        )*
        dict
    }};
}

#[macro_export]
macro_rules! co_list {
    ($($val:expr),* $(,)?) => {{
        let mut list = Initialize::list();
        $(
            list.add($val);
        )*
        list
    }};
}

#[macro_export]
macro_rules! var {
    ($name:ident) => {
        Variable::from(stringify!($name))
    };
}
