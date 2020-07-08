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

pub use lovm2_internals::lovm2_builtin;

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
