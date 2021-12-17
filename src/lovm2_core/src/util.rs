//! Helper functionality for solving small tasks

/// Creates an `Access` expression
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

/// Creates a `Call` expression
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

/// Creates a dict `Initialize` expression using `Expr` as items
#[macro_export]
macro_rules! lv2_dict {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut dict = Expr::dict();
        $(
            dict = dict.insert($key, $val);
        )*
        dict
    }};
}

/// Creates a list `Initialize` expression using `Expr` as items
#[macro_export]
macro_rules! lv2_list {
    ($($val:expr),* $(,)?) => {{
        let mut list = Expr::list();
        $(
            list = list.append($val);
        )*
        list
    }};
}

/// Creates a `Variable` from a rust identifier
#[macro_export]
macro_rules! lv2_var {
    ($name:ident) => {
        Variable::from(stringify!($name))
    };
    ($name1:ident, $name2:ident $(, $other:ident)*) => {
        (
            Variable::from(stringify!($name1)),
            Variable::from(stringify!($name2))
            $(, Variable::from(stringify!($other)) )*
        )
    };
}

/// Translate a name from `snake_case` (lovm2 default) to `lowerCamelCase`
///
/// ``` rust
/// # use lovm2_core::util::to_lower_camel_case;
/// assert_eq!("toCamelCase".to_string(), to_lower_camel_case("to_camel_case"));
/// ```
pub fn to_lower_camel_case(name: &str) -> String {
    let mut buffer = String::with_capacity(name.len());
    let mut needs_caps = false;

    for c in name.chars() {
        match c {
            '_' if buffer.is_empty() => buffer.push(c),
            '_' => needs_caps = true,
            _ => {
                let c = if needs_caps {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                };
                buffer.push(c);
                needs_caps = false;
            }
        }
    }

    buffer
}
