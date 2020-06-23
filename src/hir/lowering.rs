use crate::bytecode::Instruction;
use crate::code::{CodeObject, CodeObjectBuilder};
use crate::hir::{HIR, HIRElement};
use crate::value::CoValue;
use crate::var::Variable;

pub trait Lowering {
    fn lower(self, runtime: &mut LoweringRuntime);
}

pub struct LoweringLoop {
    start: usize,
    breaks: Vec<usize>,
    continues: Vec<usize>,
}

impl LoweringLoop {
    pub fn from(start: usize) -> Self {
        Self {
            start,
            breaks: vec![],
            continues: vec![],
        }
    }

    pub fn complete(self) {
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

    pub fn complete(hir: HIR) -> Result<CodeObject, String> {
        let mut lowru = LoweringRuntime::new();
        let mut hir_elements = hir.code.into_iter();

        for element in hir_elements {
            match element {
                HIRElement::Assign(assign) => assign.lower(&mut lowru),
                HIRElement::Branch(branch) => branch.lower(&mut lowru),
                HIRElement::Break(cmd) => cmd.lower(&mut lowru),
                HIRElement::Call(call) => call.lower(&mut lowru),
                HIRElement::Continue(cmd) => cmd.lower(&mut lowru),
                HIRElement::Repeat(repeat) => repeat.lower(&mut lowru),
            }
        }

        CodeObjectBuilder::new()
            .consts(lowru.consts)
            .locals(lowru.locals)
            .globals(lowru.globals)
            .code(lowru.code)
            .build()
    }

    pub fn index_const(&mut self, val: &CoValue) -> usize {
        match self.consts.iter().position(|item| item == val) {
            Some(pos) => pos,
            None => {
                self.consts.push(val.clone());
                self.consts.len() - 1
            },
        }
    }

    pub fn index_local(&mut self, var: &Variable) -> usize {
        match self.locals.iter().position(|item| item == var) {
            Some(pos) => pos,
            None => {
                self.locals.push(var.clone());
                self.locals.len() - 1
            },
        }
    }

    pub fn index_global(&mut self, var: &Variable) -> usize {
        match self.globals.iter().position(|item| item == var) {
            Some(pos) => pos,
            None => {
                self.globals.push(var.clone());
                self.globals.len() - 1
            },
        }
    }

    pub fn push_loop(&mut self) {
        self.loop_stack.push(LoweringLoop::from(self.code.len()));
    }

    pub fn pop_loop(&mut self) -> Option<LoweringLoop> {
        self.loop_stack.pop()
    }
}
