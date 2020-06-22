use crate::bytecode::Instruction;
use crate::code::{CodeObject, CodeObjectBuilder};
use crate::hir::{HIR, HIRElement};

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
    pub code: Vec<Instruction>,
    loop_stack: Vec<LoweringLoop>,
}

impl LoweringRuntime {
    pub fn new() -> Self {
        Self {
            code: vec![],
            loop_stack: vec![],
        }
    }

    pub fn complete(hir: HIR) -> Result<CodeObject, String> {
        let mut lowru = LoweringRuntime::new();
        let mut hir_elements = hir.code.into_iter();
        let mut code_builder = CodeObjectBuilder::new()
                                .consts(hir.consts)
                                .locals(hir.locals)
                                .globals(hir.globals);

        for element in hir_elements {
            match element {
                HIRElement::Assign(assign) => assign.lower(&mut lowru),
                HIRElement::Branch(branch) => branch.lower(&mut lowru),
                HIRElement::Repeat(repeat) => repeat.lower(&mut lowru),
            }
        }

        code_builder.code(lowru.code).build()
    }

    pub fn push_loop(&mut self) {
        self.loop_stack.push(LoweringLoop::from(self.code.len()));
    }

    pub fn pop_loop(&mut self) -> Option<LoweringLoop> {
        self.loop_stack.pop()
    }
}
