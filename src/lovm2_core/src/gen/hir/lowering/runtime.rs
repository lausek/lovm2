//! Shared lowering state

use std::cell::RefCell;
use std::rc::Rc;

use crate::code::CodeObject;
use crate::gen::LV2ModuleMeta;
use crate::var::LV2Variable;

use super::*;

/// Information for the process of lowering HIR to LIR
pub struct HirLoweringRuntime<'lir> {
    code: Vec<LirElement<'lir>>,
    counter: LabelCounterRef,
    meta: LV2ModuleMeta,
    optimizer: &'lir mut dyn Optimizer,

    branch_stack: Vec<HirLoweringBranch>,
    locals: Vec<&'lir LV2Variable>,
    loop_stack: Vec<HirLoweringRepeat>,
}

impl<'lir> HirLoweringRuntime<'lir> {
    pub fn new(
        meta: LV2ModuleMeta,
        _options: CompileOptions,
        optimizer: &'lir mut impl Optimizer,
    ) -> Self {
        Self {
            code: vec![],
            counter: Rc::new(RefCell::new(LabelCounter::default())),
            meta,
            optimizer,

            branch_stack: vec![],
            locals: vec![],
            loop_stack: vec![],
        }
    }

    pub fn create_new_label(&mut self) -> Label {
        self.counter.borrow_mut().create_new_label()
    }

    pub fn add_hir(&mut self, hir: &'lir LV2Function) -> LV2CompileResult<()> {
        // before lowering a code object function, reset locals
        self.locals.clear();

        // read in code object parameters from value stack
        // read this in reverse, because last parameter is top of stack
        for arg in hir.args.iter().rev() {
            self.emit(LirElement::store(arg));
        }

        hir.block.lower(self);

        // automatically add a `return nil` if not present already
        match self.code.last_mut() {
            Some(LirElement::Ret) => {}
            _ => {
                self.emit(LirElement::push_constant_owned(LV2Value::Nil));
                self.emit(LirElement::Ret);
            }
        }

        Ok(())
    }

    pub fn complete(mut self) -> LV2CompileResult<CodeObject> {
        let lir_runtime = LirLoweringRuntime::from(self.meta);

        self.optimizer.postprocess(&mut self.code);

        lir_runtime.lower(self.code)
    }

    pub fn emit(&mut self, elem: LirElement<'lir>) {
        self.code.push(elem);
        self.optimizer.transform(&mut self.code);
    }

    pub fn has_local(&self, var: &LV2Variable) -> bool {
        self.locals.contains(&var)
    }

    pub fn loop_mut(&mut self) -> Option<&mut HirLoweringRepeat> {
        self.loop_stack.last_mut()
    }

    pub fn push_loop(&mut self) -> &HirLoweringRepeat {
        self.loop_stack
            .push(HirLoweringRepeat::new(self.counter.clone()));
        self.loop_stack.last_mut().unwrap()
    }

    pub fn pop_loop(&mut self) -> Option<HirLoweringRepeat> {
        self.loop_stack.pop()
    }

    pub fn branch_mut(&mut self) -> Option<&mut HirLoweringBranch> {
        self.branch_stack.last_mut()
    }

    pub fn push_branch(&mut self) -> &HirLoweringBranch {
        self.branch_stack
            .push(HirLoweringBranch::new(self.counter.clone()));
        self.branch_stack.last_mut().unwrap()
    }

    pub fn pop_branch(&mut self) -> Option<HirLoweringBranch> {
        self.branch_stack.pop()
    }
}
