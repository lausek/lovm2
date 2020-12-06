pub mod hir;
pub mod lir;
pub mod lowering;
pub mod opt;
pub mod prelude;

pub use self::hir::*;
pub use self::lir::*;
pub use self::lowering::*;
pub use self::opt::*;

#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub optimize: bool,
}

impl std::default::Default for CompileOptions {
    fn default() -> Self {
        Self { optimize: true }
    }
}
