//! conditional execution

use crate::hir::block::Block;
use crate::hir::expr::Expr;
use crate::hir::lowering::{HirLowering, HirLoweringRuntime, Jumpable};
use crate::lir::LirElement;

#[derive(Clone)]
pub struct Branch {
    pub branches: Vec<(Expr, Block)>,
    pub default: Option<Block>,
}

impl Branch {
    pub fn new() -> Self {
        Self {
            branches: vec![],
            default: None,
        }
    }

    /// expects a condition that evaluates to boolean `true`
    pub fn add_condition(&mut self, condition: Expr) -> &mut Block {
        self.branches.push((condition, Block::new()));
        let (_, block) = self.branches.last_mut().unwrap();
        block
    }

    /// `Block` to execute if no condition evaluates to `true`
    pub fn default_condition(&mut self) -> &mut Block {
        self.default = Some(Block::new());
        self.default.as_mut().unwrap()
    }
}

// lowering for branches:
// - push a new HirLoweringBranch onto branches_stack
// - whenever a new condition gets lowered
//    - create a new HirLoweringCondition in top HirLoweringBranch
//    - set start offset of HirLoweringCondition: `cond_start`
//    - emit bytecode for condition
//    - emit Jf instruction, remember jump offset in HirLoweringCondition: `cond_next`
//        - this jump will be used for skipping to the next condition
//    - emit bytecode for block
//    - emit Jmp instruction, remember jump offset in HirLoweringCondition: `cond_term`
//        - this jump will be used for ending the branch / skipping all other conditions
//    - repeat until all conditions are registered
// - if it exists, emit default branch now
// - pop HirLoweringBranch from branches_stack
//    - determine the BRANCH_END offset, pointing after the whole branch
// - begin patching addresses
//    - patch all `cond_term`s with BRANCH_END
//    - iterate through `cond_next`s, take offsets using this logic:
//        - create a vector of all `cond_start`
//        - add BRANCH_END to vector, for correctly terminating the last condition
//        - skip the first element of vector, as we won't jump to the first condition
//        - take one offset from `cond_next`s, take one offset above vector; patch address
//        - repeat until all addresses are patched
impl HirLowering for Branch {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        if self.branches.is_empty() && self.default.is_some() {
            panic!("cannot lower branch: no conditions");
        }

        let branch = runtime.push_branch();
        let branch_start = branch.start();
        let branch_end = branch.end();

        runtime.emit(LirElement::Label(branch_start));

        for (condition, block) in self.branches.into_iter() {
            let cond = runtime.branch_mut().unwrap().add_condition();

            runtime.emit(LirElement::Label(cond.start()));

            condition.lower(runtime);

            runtime.emit(LirElement::jump_conditional(false, cond.end()));

            block.lower(runtime);

            runtime.emit(LirElement::jump(branch_end.clone()));
            runtime.emit(LirElement::Label(cond.end()));
        }

        if let Some(default_block) = self.default {
            // adjust offset by one, because no default_block instruction
            // was emitted yet
            default_block.lower(runtime);
        }

        runtime.emit(LirElement::Label(branch_end));

        runtime.pop_branch().unwrap();
    }
}
