pub mod standard;
pub mod valid;

use crate::lir::LirElement;

pub use self::standard::StandardOptimizer;
pub(self) use self::valid::ValidPath;

pub trait Optimizer {
    fn postprocess(&mut self, _: &mut Vec<LirElement>) {}

    fn transform(&mut self, _: &mut Vec<LirElement>) {}

    fn scan_back_for(&self, _: &LirElement) -> usize {
        0
    }
}

pub struct NoOptimizer;

impl Optimizer for NoOptimizer {}
