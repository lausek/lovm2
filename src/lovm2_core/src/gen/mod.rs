//! Tools for generating bytecode

pub mod hir;
pub mod lir;
mod module;
pub mod opt;
pub mod prelude;

use crate::error::*;

pub use self::hir::*;
pub(crate) use self::lir::*;
pub use self::module::{LV2ModuleBuilder, LV2ModuleMeta, DEFAULT_MODULE_NAME};
pub(crate) use self::opt::*;

/// Settings for lowering to bytecode
#[derive(Clone, Debug)]
pub struct CompileOptions {
    /// If this is `false`, do not run any optimization.
    pub optimize: bool,
}

impl std::default::Default for CompileOptions {
    fn default() -> Self {
        Self { optimize: true }
    }
}
