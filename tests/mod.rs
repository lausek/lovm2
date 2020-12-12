#![cfg(test)]

pub mod context;
pub mod flow;
pub mod hir;
pub mod lir;
pub mod module;
pub mod value;
pub mod vm;

pub use lovm2::context::Context;
pub use lovm2::module::Module;
pub use lovm2::prelude::*;
pub use lovm2::value::Value;
pub use lovm2::vm::Vm;
