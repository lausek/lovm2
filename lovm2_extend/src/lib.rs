use lovm2::vm::Vm;

pub mod prelude;

pub const BASIC: u8 = 1;
pub const FRAME_STACK_EMPTY: u8 = 2;
pub const IMPORT_CONFLICT: u8 = 3;
pub const KEY_NOT_FOUND: u8 = 4;
pub const LOOKUP_FAILED: u8 = 5;
pub const MODULE_NOT_FOUND: u8 = 6;
pub const OPERATION_NOT_SUPPORTED: u8 = 7;
pub const VALUE_STACK_EMPTY: u8 = 8;

#[repr(C)]
pub struct Lovm2CError {
    pub ty: u8,
}

impl From<u8> for Lovm2CError {
    fn from(ty: u8) -> Self {
        Self { ty }
    }
}

pub fn create_test_vm() -> Vm {
    let cargo_root = std::env::var("CARGO_MANIFEST_DIR").expect("no cargo manifest");
    let build_dir = format!("{}/target/debug", cargo_root);
    assert!(std::path::Path::new(&build_dir).exists());

    let mut vm = Vm::with_std();
    vm.add_load_path(build_dir);

    vm
}
