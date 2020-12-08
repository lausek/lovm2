#[cfg(lovm2_version = "0.4.6")]
pub type Module = lovm2::module::Module;
#[cfg(lovm2_version = "0.4.6")]
pub type Hir = lovm2::hir::HIR;

#[cfg(lovm2_version = "0.4.5")]
pub type Module = lovm2::module::Module;
#[cfg(lovm2_version = "0.4.5")]
pub type Hir = lovm2::hir::HIR;

#[cfg(lovm2_version = "0.3.7")]
pub type Hir = lovm2::hir::HIR;
#[cfg(lovm2_version = "0.3.7")]
pub type Module = lovm2::module::Module;
#[cfg(lovm2_version = "0.3.7")]
pub type Value = lovm2::value::RuValue;
#[cfg(lovm2_version = "0.3.7")]
#[macro_export]
macro_rules! lv2_var {
    ($name:ident) => {
        lovm2::var::Variable::from(stringify!($name))
    };
}
#[cfg(lovm2_version = "0.3.7")]
#[macro_export]
macro_rules! lv2_access {
    ($name:ident, $key:expr $(, $rest:expr)* $(,)?) => {
        lovm2::access!($name, $key, $($rest),*)
    };
}
#[cfg(lovm2_version = "0.3.7")]
#[macro_export]
macro_rules! lv2_list {
    ($($val:expr),* $(,)?) => {
        lovm2::co_list!($( $val ),*)
    };
}

#[cfg(lovm2_version = "0.3.7")]
pub(crate) fn create_vm() -> lovm2::vm::Vm {
    lovm2::vm::Vm::new()
}

#[cfg(not(lovm2_version = "0.3.7"))]
pub(crate) fn create_vm() -> lovm2::vm::Vm {
    lovm2::vm::Vm::with_std()
}
