pub mod standard;

use crate::lir::LirElement;

pub use self::standard::StandardOptimizer;

pub trait Optimizer {
    fn scan_back_for(&self, _: &LirElement) -> usize {
        0
    }

    fn transform(&mut self, _: &mut Vec<LirElement>) {}
}

pub struct NoOptimizer;

impl Optimizer for NoOptimizer {}
