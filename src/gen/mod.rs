//! Tools for generating bytecode

pub mod hir;
pub mod lir;
pub mod opt;
pub mod prelude;

pub use self::hir::*;
pub use self::lir::*;
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
