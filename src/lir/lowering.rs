use std::collections::HashMap;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::code::CodeObject;
use crate::hir::expr::{Operator1, Operator2};
use crate::prelude::ModuleMeta;
use crate::value::Value;
use crate::var::Variable;

use super::{Label, LirElement, Operator, Scope};

fn patch_addrs(
    code: &mut Vec<Instruction>,
    joffs: Vec<usize>,
    coff: usize,
) -> Lovm2CompileResult<()> {
    let coff = coff as u16;
    for joff in joffs.into_iter() {
        let jmp = &mut code[joff];
        match jmp {
            Instruction::Jf(off) => *off = coff,
            Instruction::Jt(off) => *off = coff,
            Instruction::Jmp(off) => *off = coff,
            _ => unreachable!(),
        }
    }

    Ok(())
}

enum Offset {
    Resolved(usize),
    Unresolved(Vec<usize>),
}

pub struct LirLoweringRuntime {
    meta: ModuleMeta,
    pub entries: Vec<(usize, usize)>,
    pub consts: Vec<Value>,
    pub idents: Vec<Variable>,
    pub code: Vec<Instruction>,

    offsets: HashMap<Label, Offset>,
}

impl LirLoweringRuntime {
    pub fn from(meta: ModuleMeta) -> Self {
        Self {
            meta,
            entries: vec![],
            consts: vec![],
            idents: vec![],
            code: vec![],

            offsets: HashMap::new(),
        }
    }

    pub fn lower(mut self, code: Vec<LirElement>) -> Lovm2CompileResult<CodeObject> {
        if cfg!(debug_assertions) {
            println!(">>> LIR");
            for lir_element in code.iter() {
                println!("{}", lir_element);
            }
            println!();
        }

        for lir_element in code.into_iter() {
            self.emit(lir_element)?;
        }

        let mut co = CodeObject::new();

        co.name = self.meta.name;
        co.loc = self.meta.loc;
        co.uses = self.meta.uses;

        co.entries = self.entries;
        co.consts = self.consts;
        co.idents = self.idents;
        co.code = self.code;

        Ok(co)
    }

    fn emit(&mut self, lir_element: LirElement) -> Lovm2CompileResult<()> {
        match lir_element {
            LirElement::Call { argn, ident } => {
                let iidx = self.index_ident(&ident) as u16;
                self.code.push(Instruction::Call(argn, iidx));
            }
            LirElement::Cast { tyid } => self.code.push(Instruction::Cast(tyid)),
            LirElement::Entry { ident } => {
                let iidx = self.index_ident(&ident);
                // TODO: is this correct?
                let off = self.code.len();
                self.entries.push((iidx, off));
            }
            LirElement::Jump { condition, label } => {
                let off = match self.offsets.get_mut(&label) {
                    Some(Offset::Resolved(off)) => *off as u16,
                    Some(Offset::Unresolved(uoffs)) => {
                        // TODO: is this correct?
                        uoffs.push(self.code.len());
                        u16::MAX
                    }
                    _ => {
                        self.offsets
                            .insert(label, Offset::Unresolved(vec![self.code.len()]));
                        u16::MAX
                    }
                };

                let inx = match condition {
                    Some(true) => Instruction::Jt(off),
                    Some(false) => Instruction::Jf(off),
                    _ => Instruction::Jmp(off),
                };

                self.code.push(inx)
            }
            LirElement::Label(label) => {
                // TODO: is this correct?
                let coff = self.code.len();

                if let Some(Offset::Unresolved(jmps)) =
                    self.offsets.insert(label, Offset::Resolved(coff))
                {
                    patch_addrs(&mut self.code, jmps, coff)?;
                }
            }
            LirElement::Operation(op) => {
                let inx = match op {
                    Operator::Operator2(op) => match op {
                        Operator2::Add => Instruction::Add,
                        Operator2::Sub => Instruction::Sub,
                        Operator2::Mul => Instruction::Mul,
                        Operator2::Div => Instruction::Div,
                        Operator2::Pow => Instruction::Pow,
                        Operator2::Rem => Instruction::Rem,
                        Operator2::And => Instruction::And,
                        Operator2::Or => Instruction::Or,
                        Operator2::Equal => Instruction::Eq,
                        Operator2::NotEqual => Instruction::Ne,
                        Operator2::GreaterEqual => Instruction::Ge,
                        Operator2::GreaterThan => Instruction::Gt,
                        Operator2::LessEqual => Instruction::Le,
                        Operator2::LessThan => Instruction::Lt,
                    },
                    Operator::Operator1(op) => match op {
                        Operator1::Not => Instruction::Not,
                    },
                };
                self.code.push(inx);
            }
            LirElement::PushConstant { value } => {
                let cidx = self.index_const(&value) as u16;
                self.code.push(Instruction::Pushc(cidx));
            }
            LirElement::PushDynamic { ident, scope } => {
                let iidx = self.index_ident(&ident) as u16;
                match scope {
                    Scope::Global => self.code.push(Instruction::Pushg(iidx)),
                    Scope::Local => self.code.push(Instruction::Pushl(iidx)),
                }
            }
            LirElement::StoreDynamic { ident, scope } => {
                let iidx = self.index_ident(&ident) as u16;
                match scope {
                    Scope::Global => self.code.push(Instruction::Moveg(iidx)),
                    Scope::Local => self.code.push(Instruction::Movel(iidx)),
                }
            }

            LirElement::Box => self.code.push(Instruction::Box),
            LirElement::Discard => self.code.push(Instruction::Discard),
            LirElement::Duplicate => self.code.push(Instruction::Dup),
            LirElement::Get => self.code.push(Instruction::Get),
            LirElement::Getr => self.code.push(Instruction::Getr),
            LirElement::Interrupt(n) => self.code.push(Instruction::Interrupt(n)),
            LirElement::Load => self.code.push(Instruction::Load),
            LirElement::Ret => self.code.push(Instruction::Ret),
            LirElement::Set => self.code.push(Instruction::Set),
            LirElement::Slice => self.code.push(Instruction::Slice),
        }

        Ok(())
    }

    fn index_const(&mut self, val: &Value) -> usize {
        match self.consts.iter().position(|item| item == val) {
            Some(pos) => pos,
            None => {
                self.consts.push(val.clone());
                self.consts.len() - 1
            }
        }
    }

    fn index_ident(&mut self, var: &Variable) -> usize {
        match self.idents.iter().position(|item| item == var) {
            Some(pos) => pos,
            None => {
                self.idents.push(var.clone());
                self.idents.len() - 1
            }
        }
    }
}
