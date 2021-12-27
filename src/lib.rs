#[doc = include_str!("../README.md")]
pub use lovm2_core::*;

/// Create a new instance with standard functions already imported.
#[cfg(feature = "stdlib")]
pub fn create_vm_with_std() -> lovm2_core::vm::LV2Vm {
    let module = lovm2_std::create_std_module();
    let mut vm = lovm2_core::vm::LV2Vm::new();
    vm.add_module(module, false).unwrap();
    vm
}
