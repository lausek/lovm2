//! Conditional execution

use super::*;

/// Conditional execution
#[derive(Clone)]
pub struct LV2Branch {
    branches: Vec<(LV2Expr, LV2Block)>,
    default: Option<LV2Block>,
}

impl LV2Branch {
    pub fn new() -> Self {
        Self {
            branches: vec![],
            default: None,
        }
    }

    /// Expects a condition that evaluates to boolean `true`
    pub fn add_condition(&mut self, condition: LV2Expr) -> &mut LV2Block {
        self.branches.push((condition, LV2Block::new()));

        let (_, block) = self.branches.last_mut().unwrap();

        block
    }

    /// `Block` to execute if no condition evaluates to `true`
    pub fn default_condition(&mut self) -> &mut LV2Block {
        self.default = Some(LV2Block::new());
        self.default.as_mut().unwrap()
    }
}

impl HirLowering for LV2Branch {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut HirLoweringRuntime<'lir>) {
        lower_map_structure(runtime, &self.branches, &self.default);
    }
}

// generic lowering for control structures that evaluate expressions in order to execute code
pub(crate) fn lower_map_structure<'hir, 'lir, T>(
    runtime: &mut HirLoweringRuntime<'lir>,
    branches: &'hir Vec<(LV2Expr, T)>,
    default: &'hir Option<T>,
) where
    T: HirLowering,
    'hir: 'lir,
{
    // push a new branch section. the stack is not really
    // required (?) but it's quite handy to always have access
    // to the current branch
    let branch = runtime.push_branch();
    let branch_start = branch.start();
    let branch_end = branch.end();

    // the branch section starts now
    runtime.emit(LirElement::Label(branch_start));

    for (condition, block) in branches.iter() {
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

    if let Some(default_block) = &default {
        default_block.lower(runtime);
    }

    // the branche's end label is a jump target for every sucessfully
    // executed condition block
    runtime.emit(LirElement::Label(branch_end));

    // branch has been lowered -> cleanup
    runtime.pop_branch().unwrap();
}
