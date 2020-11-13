/// define `CodeObject` on a low-level basis
#[macro_export]
macro_rules! define_code {
    {
        $(consts {$($cval:expr),*})?
        $(idents {$($name:ident),*})?
        {
            $( $inx:ident $($args:expr),* ; )*
        }
    } => {{
        let mut co = lovm2::code::CodeObject::new();
        $( co.idents = vec![$( Variable::from(stringify!($name)) ),*]; )?
        $( co.consts = vec![$( Value::from($cval) ),*]; )?

        let mut c = vec![
            $(
                define_code! { compile_inx $inx $(, $args)* },
            )*
        ];

        co.code = c;
        co
    }};

    { compile_inx $inx:ident } => { Instruction::$inx };
    { compile_inx $inx:ident $(, $args:expr)+ } => { Instruction::$inx($($args),*) };
}

/// creates an `Access` expression
#[macro_export]
macro_rules! lv2_access {
    ($name:ident, $key:expr $(, $rest:expr)* $(,)?) => {{
        Access::target(stringify!($name).into())
            .at($key)
            $(
                .at($rest)
            )*
    }};
}

/// creates a `Call` expression
#[macro_export]
macro_rules! lv2_call {
    ($name:ident $(, $arg:tt)* $(,)?) => {{
        Call::new(stringify!($name))
            $(
                .arg(lv2_call!(1, $arg))
            )*
    }};
    (1, $arg:ident) => {{ lv2_var!($arg) }};
    (1, $arg:expr) => {{ $arg }};
}

/// creates a dict `Initialize` expression using `Expr` as items
#[macro_export]
macro_rules! lv2_dict {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut dict = Initialize::dict();
        $(
            dict.add_by_key($key, $val);
        )*
        dict
    }};
}

/// creates a list `Initialize` expression using `Expr` as items
#[macro_export]
macro_rules! lv2_list {
    ($($val:expr),* $(,)?) => {{
        let mut list = Initialize::list();
        $(
            list.add($val);
        )*
        list
    }};
}

/// creates a `Variable` from a rust identifier
#[macro_export]
macro_rules! lv2_var {
    ($name:ident) => {
        Variable::from(stringify!($name))
    };
}
