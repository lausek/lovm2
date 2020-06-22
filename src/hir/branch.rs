use crate::block::Block;
use crate::expr::Expr;

pub struct Branch {
    branches: Vec<(Expr, Block)>,
    default: Option<Block>,
}

impl Branch {
    pub fn new() -> Self {
        Self {
            branches: vec![],
            default: None,
        }
    }
}
