//! `lovm2_extend` is a library for writing `lovm2` extensions using Rust. The output
//! is a shared object that is directly loadable by the virtual machine.
//!
//! ## Setup
//!
//! 1. Create a new library crate `cargo init <name> --lib`
//!
//! 2. Change your crate-type inside `Cargo.toml`
//!
//! ``` toml
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//! 3. Write your functions and use `cargo build --release` to produce
//! a shared object inside `target/release/`
//!
//! ## Example
//!
//! ``` rust
//! // Import all required types for writing a module
//! use lovm2_extend::prelude::*;
//!
//! // This attribute generates wrapper code for Rust structures
//! #[lovm2_object]
//! pub struct Session {
//!     name: Option<String>,
//! }
//!
//! // Constructor for new values of `Session`
//! #[lovm2_function]
//! fn new() -> Session {
//!     Session { name: None }
//! }
//!
//! // Returning `Option`s is allowed
//! #[lovm2_function]
//! fn get_name(session: &Session) -> Option<String> {
//!     session.name.clone()
//! }
//!
//! // You can modify `Session`
//! #[lovm2_function]
//! fn set_name(session: &mut Session, name: String) {
//!     session.name = Some(name);
//! }
//!
//! // Generate module bloat (required)
//! lovm2_module_init!();
//! ```

use lovm2::vm::Vm;

pub use lovm2_module::*;

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

/// Returns a virtual machine with the crates `target/debug` directory in the load path.
pub fn create_test_vm() -> Vm {
    let cargo_root = std::env::var("CARGO_MANIFEST_DIR").expect("no cargo manifest");
    let build_dir = format!("{}/target/debug", cargo_root);
    assert!(std::path::Path::new(&build_dir).exists());

    let mut vm = Vm::with_std();
    vm.add_load_path(build_dir);

    vm
}
