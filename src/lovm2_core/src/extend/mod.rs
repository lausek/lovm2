//! `lovm2::extend` bundles functionality for writing `lovm2` extensions using Rust. You can either statically import functions or produce a shared object that can be loaded at runtime.
//!
//! Shared object libraries created using this crate can be imported by `lovm2` and used like regular modules. You just need to add the shared object to `lovm2`s module search path e.g. `~/.local/lib/lovm2/`. When searching for a module, the file extension is stripped. This means that a file named `libmymodule.so` will only be imported if you have a `Import("libmymodule")` instruction.
//!
//! ## Examples
//!
//! - [shared-module](./examples/shared-module)
//! - [static-module](./examples/static-module)
//!
//! ## Shared Objects
//!
//! ### Setup
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
//! ### Usage
//!
//! This crate exports three macros:
//!
//! - `lovm2_module_init!()` - creates a module initializer. Required at end of file for all shared object modules.
//! - `#[lovm2_function]` - easy to use wrapper function in combination with `create_callable`.
//! - `#[lovm2_function(extern)]` - an attribute for exporting functions in shared object modules.
//! - `#[lovm2_object]` - a `struct` attribute for making data structures available to functions.
//!
//! ``` rust
//! // Import all required types for writing a module
//! use lovm2_core::extend::prelude::*;
//!
//! #[lovm2_function(extern)]
//! fn div(a: i64, b: i64) -> Lovm2Result<i64> {
//!     if b == 0 {
//!         return err_from_string("div by zero");
//!     }
//!     Ok(a / b)
//! }
//!
//! // This attribute generates wrapper code for Rust structures
//! #[lovm2_object]
//! pub struct Session {
//!     name: Option<String>,
//! }
//!
//! // Constructor for new values of `Session`
//! #[lovm2_function(extern)]
//! fn new() -> Session {
//!     Session { name: None }
//! }
//!
//! // Returning `Option`s is allowed
//! #[lovm2_function(extern)]
//! fn get_name(session: &Session) -> Option<String> {
//!     session.name.clone()
//! }
//!
//! // You can modify `Session`
//! #[lovm2_function(extern)]
//! fn set_name(session: &mut Session, name: String) {
//!     session.name = Some(name);
//! }
//!
//! // Generate module bloat (required)
//! lovm2_module_init!();
//! ```
//!
//! ## Supported Types
//!
//! - Functions can take the generic `Value` type as argument. `Value` is also allowed in return position. If `&Value` or `&mut Value` is used, `lovm2` references are **automatically dereferenced** so you never have to worry about them in your functions body. You probably want this behavior in most cases.
//! - `bool`, `i64`, `f64`, `String` support conversion from `lovm2` values and can be used for arguments and as return type.
//! - Functions are allowed to return nothing aka. `()`.
//! - Wrapping the types above in `Option<_>` or `Lovm2Result<_>` also produces an accepted return type.
//! - A function is allowed to have at most one argument taking a mutable or immutable reference to the virtual machine itself e.g. `vm: &mut Vm`.

pub mod prelude;
mod wrapper;

use crate::vm::Vm;

pub use lovm2_module::*;

pub use self::wrapper::create_callable;

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

    let mut vm = Vm::new();

    vm.add_load_path(build_dir);

    vm
}
