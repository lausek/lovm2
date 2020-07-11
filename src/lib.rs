//! lovm2 is a lightweight virtual machine with a focus on simplicity and extendability.
//!
//! ## Features
//!
//! - dynamic typing
//! - generate bytecode using highlevel intermediate representation
//! - call into shared objects: [lovm2_extend](lovm2_extend/README.md)
//! - python bindings: [pylovm2](pylovm2/README.md)
//! - define own callbacks for interrupts
//!
//! ## Example
//!
//! ``` rust
//! use lovm2::prelude::*;
//! use lovm2::vm::Vm;
//!
//! fn main() {
//!     let mut main_hir = HIR::new();
//!
//!     // set the local variable n to 10
//!     main_hir.push(Assign::local(var!(n), 10));
//!
//!     // `print` is a builtin function. the `var!` macro
//!     // ensures that the given identifier is not confused
//!     // with a string.
//!     main_hir.push(Call::new("print").arg(var!(n)).arg("Hello World"));
//!     // ... this is equivalent to the developer-friendly version:
//!     main_hir.push(call!(print, n, "Hello World"));
//!
//!     let mut module = ModuleBuilder::new();
//!
//!     // a module needs a code object called `main`
//!     // if you want to make it runnable
//!     module.add("main").hir(main_hir);
//!
//!     // consumes the `ModuleBuilder` and transforms
//!     // it into a `Module`
//!     let module = module.build().unwrap();
//!     println!("{:#?}", module);
//!
//!     // load the module and run it
//!     let mut vm = Vm::new();
//!     vm.load_and_import_all(module).expect("load error");
//!     vm.run().expect("run error");
//! }
//! ```

#![feature(const_in_array_repeat_expressions)]
#![allow(clippy::new_without_default)]
#![allow(clippy::wrong_self_convention)]

extern crate lovm2_internals;

pub mod bytecode;
pub mod code;
pub mod context;
pub mod frame;
pub mod hir;
pub mod module;
pub mod util;
pub mod value;
pub mod var;
pub mod vm;

/// used for generating wrappers of statically linked functions to be called from lovm2
pub use lovm2_internals::lovm2_builtin;

// TODO: remove these
pub use self::bytecode::*;
pub use self::code::*;
pub use self::context::*;
pub use self::frame::*;
pub use self::hir::*;
pub use self::module::*;
pub use self::util::*;
pub use self::value::*;
pub use self::var::*;
pub use self::vm::*;
