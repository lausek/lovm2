use crate::bytecode::Instruction;
use crate::code::{CodeObject, CodeObjectBuilder};
use crate::hir::{HIRElement, HIR};
use crate::value::CoValue;
use crate::var::Variable;

pub trait Lowering {
    fn lower(self, runtime: &mut LoweringRuntime);
}

pub struct LoweringLoop {
    pub start: usize,
    pub end: Option<usize>,
    pub breaks: Vec<usize>,
    pub continues: Vec<usize>,
}

impl LoweringLoop {
    pub fn from(start: usize) -> Self {
        Self {
            start,
            end: None,
            breaks: vec![],
            continues: vec![],
        }
    }

    pub fn add_break(&mut self, idx: usize) {
        self.breaks.push(idx);
    }

    pub fn add_continue(&mut self, idx: usize) {
        self.continues.push(idx);
    }
}

pub struct LoweringRuntime {
    pub consts: Vec<CoValue>,
    pub locals: Vec<Variable>,
    pub globals: Vec<Variable>,
    pub code: Vec<Instruction>,
    loop_stack: Vec<LoweringLoop>,
}

impl LoweringRuntime {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],
            code: vec![],
            loop_stack: vec![],
        }
    }

    pub fn offset(&self) -> usize {
        self.code.len() - 1
    }

    pub fn complete(hir: HIR) -> Result<CodeObject, String> {
        let mut lowru = LoweringRuntime::new();
        let hir_elements = hir.code.into_iter();

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
}
