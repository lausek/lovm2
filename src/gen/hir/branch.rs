//! Conditional execution

use super::*;

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

impl HirLowering for Branch {
    fn lower(self, runtime: &mut HirLoweringRuntime) {
        // branches without conditions but a default block make no sense
        if self.branches.is_empty() && self.default.is_some() {
            panic!("cannot lower branch: no conditions");
        }

        // push a new branch section. the stack is not really
        // required (?) but it's quite handy to always have access
        // to the current branch
        let branch = runtime.push_branch();
        let branch_start = branch.start();
        let branch_end = branch.end();

        // the branch section starts now
        runtime.emit(LirElement::Label(branch_start));

        for (condition, block) in self.branches.into_iter() {
            let cond = runtime.branch_mut().unwrap().add_condition();

            // declare the start of a new condition
            runtime.emit(LirElement::Label(cond.start()));

            // lower condition expression. if this evaluates to false at runtime, jump
            // to the condition section's end - which is also the starting offset
            // for the next condition section.
            condition.lower(runtime);
            runtime.emit(LirElement::jump_conditional(false, cond.end()));

            // lower the actual code that should be executed if the condition is true
            block.lower(runtime);

            // if above block was executed, terminate the whole branch by jumping to its
            // end label
            runtime.emit(LirElement::jump(branch_end.clone()));
            runtime.emit(LirElement::Label(cond.end()));
        }

        if let Some(default_block) = self.default {
            default_block.lower(runtime);
        }

        // the branche's end label is a jump target for every sucessfully
        // executed condition block
        runtime.emit(LirElement::Label(branch_end));

        // branch has been lowered -> cleanup
        runtime.pop_branch().unwrap();
    }
}
