//! shared lowering state

use std::cell::RefCell;
use std::rc::Rc;

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
    counter: LabelCounterRef,
    meta: ModuleMeta,
    optimize: bool,

    branch_stack: Vec<HirLoweringBranch>,
    locals: Vec<Variable>,
    loop_stack: Vec<HirLoweringRepeat>,
}

impl HirLoweringRuntime {
    pub fn new(meta: ModuleMeta) -> Self {
        Self {
            code: vec![],
            counter: Rc::new(RefCell::new(LabelCounter::default())),
            meta,
            optimize: true,

            branch_stack: vec![],
            locals: vec![],
            loop_stack: vec![],
        }
    }

    pub fn create_new_label(&mut self) -> Label {
        self.counter.borrow_mut().create_new_label()
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
        LirLoweringRuntime::from(self.meta).lower(self.code)
    }

    pub fn add_prelude(&mut self, args: Vec<Variable>) -> Lovm2CompileResult<()> {
        // read in code object parameters from value stack
        // read this in reverse, because last parameter is top of stack
        for arg in args.into_iter().rev() {
            self.emit(LirElement::store(Scope::Local, arg));
        }
        Ok(())
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

        self.code.push(elem);

        if self.optimize {
            use crate::hir::expr::Operator1::*;
            use crate::lir::LirElement::*;
            use crate::lir::Operator::*;

            loop {
                let l = self.code.len().checked_sub(3).unwrap_or(0);
                let view = &mut self.code[l..];

                match view {
                    [_, Operation(Operator1(Not)), Jump {
                        condition: Some(cond),
                        ..
                    }] => {
                        *cond = !*cond;
                        view.swap(1, 2);
                        self.code.pop();
                    }
                    _ => break,
                }
            }
        }
    }

    pub fn has_local(&self, var: &Variable) -> bool {
        self.locals.contains(var)
    }

    pub fn loop_mut(&mut self) -> Option<&mut HirLoweringRepeat> {
        self.loop_stack.last_mut()
    }

    pub fn push_loop(&mut self) -> &mut HirLoweringRepeat {
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

    pub fn push_branch(&mut self) -> &mut HirLoweringBranch {
        self.branch_stack
            .push(HirLoweringBranch::new(self.counter.clone()));
        self.branch_stack.last_mut().unwrap()
    }

    pub fn pop_branch(&mut self) -> Option<HirLoweringBranch> {
        self.branch_stack.pop()
    }
}
