//! Tools for generating bytecode

pub mod hir;
pub mod lir;
mod module;
pub mod opt;
pub mod prelude;

pub use self::hir::*;
pub use self::lir::*;
pub use self::module::{ModuleBuilder, ModuleMeta, DEFAULT_MODULE_NAME};
pub use self::opt::*;

/// Settings for lowering to bytecode
#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub optimize: bool,
}

impl std::default::Default for CompileOptions {
    fn default() -> Self {
        Self { optimize: true }
    }
}
