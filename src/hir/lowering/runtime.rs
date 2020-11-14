//! shared lowering state

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::code::CodeObject;
use crate::hir::HIR;
use crate::value::Value;
use crate::var::Variable;

use super::*;

// TODO: add ExprOptimizer field for improving Exprs
pub struct LoweringRuntime {
    pub name: String,
    pub loc: Option<String>,
    pub entries: Vec<(usize, usize)>,
    pub uses: Vec<String>,
    pub consts: Vec<Value>,
    pub idents: Vec<Variable>,
    pub code: Vec<Instruction>,

    branch_stack: Vec<LoweringBranch>,
    locals: Vec<Variable>,
    loop_stack: Vec<LoweringLoop>,
}

impl LoweringRuntime {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            loc: None,
            entries: vec![],
            uses: vec![],
            consts: vec![],
            idents: vec![],
            code: vec![],

            branch_stack: vec![],
            locals: vec![],
            loop_stack: vec![],
        }
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
        let mut co = CodeObject::new();
        co.name = self.name;
        co.loc = self.loc;
        co.entries = self.entries;
        co.uses = self.uses;
        co.consts = self.consts;
        co.idents = self.idents;
        co.code = self.code;
        Ok(co)
    }

    pub fn add_prelude(&mut self, args: Vec<Variable>) -> Lovm2CompileResult<()> {
        // read in code object parameters from value stack
        // read this in reverse, because last parameter is top of stack
        for arg in args.into_iter().rev() {
            let lidx = self.index_local(&arg);
            self.emit(Instruction::Movel(lidx as u16));
        }
        Ok(())
    }

    pub fn offset(&self) -> usize {
        let len = self.code.len();
        if len == 0 {
            0
        } else {
            len - 1
        }
    }

    pub fn emit(&mut self, inx: Instruction) {
        self.code.push(inx);
    }

    pub fn index_const(&mut self, val: &Value) -> usize {
        match self.consts.iter().position(|item| item == val) {
            Some(pos) => pos,
            None => {
                self.consts.push(val.clone());
                self.consts.len() - 1
            }
        }
    }

    pub fn index_local(&mut self, var: &Variable) -> usize {
        if !self.has_local(var) {
            self.locals.push(var.clone());
        }
        self.index_ident(var)
    }

    pub fn index_global(&mut self, var: &Variable) -> usize {
        self.index_ident(var)
    }

    pub fn index_ident(&mut self, var: &Variable) -> usize {
        match self.idents.iter().position(|item| item == var) {
            Some(pos) => pos,
            None => {
                self.idents.push(var.clone());
                self.idents.len() - 1
            }
        }
    }

    pub fn has_local(&self, var: &Variable) -> bool {
        self.locals.contains(var)
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
