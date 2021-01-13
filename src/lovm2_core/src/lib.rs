#![allow(clippy::new_without_default)]
#![allow(clippy::wrong_self_convention)]
//#![feature(result_cloned)]

pub mod code;
pub mod error;
pub mod gen;
pub mod module;
pub mod prelude;
pub mod util;
pub mod value;
pub mod vm;

pub(crate) mod bytecode;
pub(crate) mod var;

pub use crate::bytecode::Instruction;
pub use crate::var::Variable;
