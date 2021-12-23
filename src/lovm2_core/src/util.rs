//! Helper functionality for solving small tasks.

/// Creates an [LV2Expr::Get] expression recursively.
#[macro_export]
macro_rules! lv2_access {
    ($name:expr, $key:expr $(, $rest:expr)* $(,)?) => {{
        LV2Expr::from($name)
            .get($key)
            $(
                .get($rest)
            )*
    }};
}

/// Creates a [LV2Expr::Call] expression.
#[macro_export]
macro_rules! lv2_call {
    ($name:ident $(, $arg:tt)* $(,)?) => {{
        LV2Expr::from(
            LV2Call::new(stringify!($name))
                $(
                    .arg(lv2_call!(1, $arg))
                )*
        )
    }};
    (1, $arg:ident) => {{ lv2_var!($arg) }};
    (1, $arg:expr) => {{ $arg }};
}

/// Creates a dict expression using [LV2Expr] as items.
#[macro_export]
macro_rules! lv2_dict {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut dict = LV2Expr::dict();
        $(
            dict = dict.set($key, $val);
        )*
        dict
    }};
}

/// Creates a list expression using [LV2Expr] as items.
#[macro_export]
macro_rules! lv2_list {
    ($($val:expr),* $(,)?) => {{
        let mut list = LV2Expr::list();
        $(
            list = list.append($val);
        )*
        list
    }};
}

/// Creates a [LV2Variable] from a rust identifier.
#[macro_export]
macro_rules! lv2_var {
    ($name:ident) => {
        LV2Variable::from(stringify!($name))
    };
    ($name1:ident, $name2:ident $(, $other:ident)*) => {
        (
            LV2Variable::from(stringify!($name1)),
            LV2Variable::from(stringify!($name2))
            $(, LV2Variable::from(stringify!($other)) )*
        )
    };
}

/// Translate a name from `snake_case` (lovm2 default) to `lowerCamelCase`.
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
