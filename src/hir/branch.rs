//! conditional execution

use crate::bytecode::Instruction;
use crate::hir::block::Block;
use crate::hir::expr::Expr;
use crate::hir::lowering::{patch_addr, HirLowering, HirLoweringRuntime};

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

        runtime.push_branch();

        for (condition, block) in self.branches.into_iter() {
            // adjust offset by one, because no condition instruction
            // was emitted yet
            let offset = runtime.offset() + 1;
            runtime.branch_mut().unwrap().add_condition(offset);

            condition.lower(runtime);

            runtime.emit(Instruction::Jf(std::u16::MAX));
            runtime.branch_mut().unwrap().condition_mut().unwrap().next = Some(runtime.offset());

            block.lower(runtime);

            runtime.emit(Instruction::Jmp(std::u16::MAX));
            runtime.branch_mut().unwrap().condition_mut().unwrap().term = Some(runtime.offset());
        }

        if let Some(default_block) = self.default {
            // adjust offset by one, because no default_block instruction
            // was emitted yet
            let offset = runtime.offset() + 1;
            runtime.branch_mut().unwrap().add_default(offset);
            default_block.lower(runtime);
        }

        let mut lowering_branch = runtime.pop_branch().unwrap();

        let lowering_branch_end = lowering_branch.end.unwrap();
        let mut jump_chain = lowering_branch
            .conditions
            .iter()
            .map(|cond| cond.start)
            .collect::<Vec<usize>>();

        if let Some(default_branch) = lowering_branch.default_mut() {
            jump_chain.push(default_branch.start);
        } else {
            jump_chain.push(lowering_branch_end);
        }

        let mut next_iter = jump_chain.into_iter().skip(1);

        for lowering_condition in lowering_branch.conditions {
            let next_addr = next_iter.next().unwrap();
            if let Some(next_offset) = lowering_condition.next {
                patch_addr(runtime, next_offset, next_addr);
            }
            if let Some(term_offset) = lowering_condition.term {
                patch_addr(runtime, term_offset, lowering_branch_end);
            }
        }
    }
}
