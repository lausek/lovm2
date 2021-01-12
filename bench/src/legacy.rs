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

pub(crate) fn create_vm() -> lovm2::vm::Vm {
    if cfg!(lovm2_version = "0.4.8") {
        return lovm2::create_vm_with_std()
    }

    #[cfg(not(any(lovm2_version = "0.3.7", lovm2_version = "0.4.8")))]
    return lovm2::vm::Vm::with_std();

    lovm2::vm::Vm::new()
}
