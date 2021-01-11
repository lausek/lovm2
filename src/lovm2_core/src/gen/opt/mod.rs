//! Optimization on LIR

pub(crate) mod standard;
pub(crate) mod valid;

use super::*;

pub use self::standard::StandardOptimizer;
pub(self) use self::valid::ValidPath;

/// General functions of an optimizer
pub trait Optimizer {
    fn postprocess(&mut self, _: &mut Vec<LirElement>) {}

    fn transform(&mut self, _: &mut Vec<LirElement>) {}

    fn scan_back_for(&self, _: &LirElement) -> usize {
        0
    }
}

/// Does no optimization at all
pub struct NoOptimizer;

impl Optimizer for NoOptimizer {}
