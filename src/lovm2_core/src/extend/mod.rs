#[doc = include_str!("../../../../README-extend.md")]
pub mod prelude;
mod wrapper;

use crate::vm::LV2Vm;

pub use lovm2_module::*;

pub use self::wrapper::lv2_create_callable;

pub const LV2_EXTERN_INITIALIZER: &str = "lovm2_module_initialize";

/// Returns a virtual machine with the crates `target/debug` directory in the load path.
pub fn create_test_vm() -> LV2Vm {
    let cargo_root = std::env::var("CARGO_MANIFEST_DIR").expect("no cargo manifest");
    let build_dir = format!("{}/target/debug", cargo_root);

    assert!(std::path::Path::new(&build_dir).exists());

    let mut vm = LV2Vm::new();

    vm.add_load_path(build_dir);

    vm
}
