use crate::bytecode::Instruction;
use crate::code::{CodeObject, CodeObjectBuilder};
use crate::hir::HIR;
use crate::value::CoValue;
use crate::var::Variable;

use super::*;

// TODO: add ExprOptimizer field for improving Exprs
pub struct LoweringRuntime {
    pub consts: Vec<CoValue>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,
    pub code: Vec<Instruction>,

    branch_stack: Vec<LoweringBranch>,
    loop_stack: Vec<LoweringLoop>,
}

impl LoweringRuntime {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],
            code: vec![],

            branch_stack: vec![],
            loop_stack: vec![],
        }
    }

    pub fn complete(hir: HIR) -> Result<CodeObject, String> {
        let mut lowru = LoweringRuntime::new();
        let hir_elements = hir.code.into_iter();

        lowru.add_prelude(hir.args)?;

        for element in hir_elements {
            element.lower(&mut lowru);
        }

        CodeObjectBuilder::new()
            .consts(lowru.consts)
            .locals(lowru.locals)
            .globals(lowru.globals)
            .code(lowru.code)
            .build()
    }

    pub fn add_prelude(&mut self, args: Vec<Variable>) -> Result<(), String> {
        // read in code object parameters from value stack
        // read this in reverse, because last parameter is top of stack
        for arg in args.into_iter().rev() {
            let lidx = self.index_local(&arg);
            self.emit(Instruction::Movel(lidx as u16));
        }
        Ok(())
    }

    pub fn offset(&self) -> usize {
        // TODO: avoid 0 - 1
        self.code.len() - 1
    }

    pub fn emit(&mut self, inx: Instruction) {
        self.code.push(inx);
    }

    pub fn index_const(&mut self, val: &CoValue) -> usize {
        match self.consts.iter().position(|item| item == val) {
            Some(pos) => pos,
            None => {
                self.consts.push(val.clone());
                self.consts.len() - 1
            }
        }
    }

    pub fn index_local(&mut self, var: &Variable) -> usize {
        match self.locals.iter().position(|item| item == var) {
            Some(pos) => pos,
            None => {
                self.locals.push(var.clone());
                self.locals.len() - 1
            }
        }
    }

    pub fn index_global(&mut self, var: &Variable) -> usize {
        match self.globals.iter().position(|item| item == var) {
            Some(pos) => pos,
            None => {
                self.globals.push(var.clone());
                self.globals.len() - 1
            }
        }
    }

    pub fn loop_mut(&mut self) -> Option<&mut LoweringLoop> {
        self.loop_stack.last_mut()
    }

    pub fn push_loop(&mut self) -> &mut LoweringLoop {
        self.loop_stack.push(LoweringLoop::from(self.code.len()));
        self.loop_stack.last_mut().unwrap()
    }

    pub fn pop_loop(&mut self) -> Option<LoweringLoop> {
        if let Some(mut lowering_loop) = self.loop_stack.pop() {
            // point at offset after block
            lowering_loop.end = Some(self.offset() + 1);
            Some(lowering_loop)
        } else {
            None
        }
    }

    pub fn branch_mut(&mut self) -> Option<&mut LoweringBranch> {
        self.branch_stack.last_mut()
    }

    pub fn push_branch(&mut self) -> &mut LoweringBranch {
        self.branch_stack
            .push(LoweringBranch::from(self.code.len()));
        self.branch_stack.last_mut().unwrap()
    }

    pub fn pop_branch(&mut self) -> Option<LoweringBranch> {
        if let Some(mut lowering_branch) = self.branch_stack.pop() {
            lowering_branch.end = Some(self.offset() + 1);
            Some(lowering_branch)
        } else {
            None
        }
    }
}
