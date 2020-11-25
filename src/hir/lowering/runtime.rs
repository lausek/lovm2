//! shared lowering state

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::code::CodeObject;
use crate::hir::HIR;
use crate::lir::{Label, LirElement, LirLoweringRuntime, Scope};
use crate::module::ModuleMeta;
use crate::value::Value;
use crate::var::Variable;

use super::*;

// TODO: add ExprOptimizer field for improving Exprs
pub struct HirLoweringRuntime {
    code: Vec<LirElement>,
    meta: ModuleMeta,

    branch_stack: Vec<HirLoweringBranch>,
    locals: Vec<Variable>,
    loop_stack: Vec<HirLoweringLoop>,
}

impl HirLoweringRuntime {
    pub fn new(meta: ModuleMeta) -> Self {
        Self {
            code: vec![],
            meta,

            branch_stack: vec![],
            locals: vec![],
            loop_stack: vec![],
        }
    }

    pub fn create_new_label(&mut self) -> Label {
        Label(0)
    }

    pub fn add_hir(&mut self, hir: HIR) -> Lovm2CompileResult<()> {
        let hir_elements = hir.code.into_iter();

        // before lowering a code object function, reset locals
        self.locals.clear();

        self.add_prelude(hir.args)?;

        for element in hir_elements {
            element.lower(self);
        }

        Ok(())
    }

    pub fn complete(self) -> Lovm2CompileResult<CodeObject> {
        let mut lo_runtime = LirLoweringRuntime::from(self.meta);
        lo_runtime.lower(self.code)
    }

    pub fn add_prelude(&mut self, args: Vec<Variable>) -> Lovm2CompileResult<()> {
        // read in code object parameters from value stack
        // read this in reverse, because last parameter is top of stack
        for arg in args.into_iter().rev() {
            self.emit(LirElement::store(Scope::Local, arg));
        }
        Ok(())
    }

    pub fn offset(&self) -> usize {
        todo!()
    }

    pub fn emit(&mut self, elem: LirElement) {
        if let LirElement::StoreDynamic {
            ident,
            scope: Scope::Local,
        } = &elem
        {
            if !self.has_local(ident) {
                self.locals.push(ident.clone());
            }
        }
        // TODO: optimize lir
        self.code.push(elem);
    }

    pub fn has_local(&self, var: &Variable) -> bool {
        self.locals.contains(var)
    }

    pub fn loop_mut(&mut self) -> Option<&mut HirLoweringLoop> {
        self.loop_stack.last_mut()
    }

    pub fn push_loop(&mut self) -> &mut HirLoweringLoop {
        self.loop_stack.push(HirLoweringLoop::from(self.code.len()));
        self.loop_stack.last_mut().unwrap()
    }

    pub fn pop_loop(&mut self) -> Option<HirLoweringLoop> {
        if let Some(mut lowering_loop) = self.loop_stack.pop() {
            // point at offset after block
            lowering_loop.end = Some(self.offset() + 1);
            Some(lowering_loop)
        } else {
            None
        }
    }

    pub fn branch_mut(&mut self) -> Option<&mut HirLoweringBranch> {
        self.branch_stack.last_mut()
    }

    pub fn push_branch(&mut self) -> &mut HirLoweringBranch {
        self.branch_stack
            .push(HirLoweringBranch::from(self.code.len()));
        self.branch_stack.last_mut().unwrap()
    }

    pub fn pop_branch(&mut self) -> Option<HirLoweringBranch> {
        if let Some(mut lowering_branch) = self.branch_stack.pop() {
            lowering_branch.end = Some(self.offset() + 1);
            Some(lowering_branch)
        } else {
            None
        }
    }
}
