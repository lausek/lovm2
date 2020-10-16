#![cfg(test)]

pub mod context;
pub mod flow;
pub mod hir;
pub mod module;

#[macro_export]
macro_rules! local_value {
    ($frame:expr, $name:ident) => {{
        match *$frame.locals.get(&var!($name)).unwrap() {
            lovm2::value::RuValue::Ref(Some(r)) => r.borrow().clone(),
            value => value,
        }
    }};
}

#[macro_export]
macro_rules! global_value {
    ($ctx:expr, $name:ident) => {{
        match *$ctx.globals.get(&var!($name)).unwrap() {
            lovm2::value::RuValue::Ref(Some(r)) => r.borrow().clone(),
            value => value,
        }
    }};
}
