use crate::block::Block;
use crate::expr::Expr;
use crate::hir::lowering::{Lowering, LoweringRuntime};

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

// TODO: lowering for branches:
// - push a new LoweringBranch onto branches_stack
// - whenever a new condition gets lowered
//    - create a new LoweringCondition in top LoweringBranch
//    - set start offset of LoweringCondition: `cond_start`
//    - emit bytecode for condition
//    - emit Jf instruction, remember jump offset in LoweringCondition: `cond_next`
//        - this jump will be used for skipping to the next condition
//    - emit bytecode for block
//    - emit Jmp instruction, remember jump offset in LoweringCondition: `cond_term`
//        - this jump will be used for ending the branch / skipping all other conditions
//    - repeat until all conditions are registered
// - if it exists, emit default branch now
// - pop LoweringBranch from branches_stack
//    - determine the BRANCH_END offset, pointing after the whole branch
// - begin patching addresses
//    - patch all `cond_term`s with BRANCH_END
//    - iterate through `cond_next`s, take offsets using this logic:
//        - create a vector of all `cond_start`
//        - add BRANCH_END to vector, for correctly terminating the last condition
//        - skip the first element of vector, as we won't jump to the first condition
//        - take one offset from `cond_next`s, take one offset above vector; patch address
//        - repeat until all addresses are patched
impl Lowering for Branch {
    fn lower(self, _runtime: &mut LoweringRuntime) {
    }
}
