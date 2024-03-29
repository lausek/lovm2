//! Information for the process of lowering LIR to bytecode

use std::collections::{HashMap, HashSet};

use crate::bytecode::Instruction;
use crate::code::LV2CodeObject;
use crate::value::LV2Value;
use crate::var::LV2Variable;

use super::*;

fn patch_addrs(
    code: &mut Vec<Instruction>,
    joffs: Vec<usize>,
    coff: usize,
) -> LV2CompileResult<()> {
    let coff = coff as u16;

    for joff in joffs.into_iter() {
        let jmp = &mut code[joff];

        match jmp {
            Instruction::Jf(off) => *off = coff,
            Instruction::Jt(off) => *off = coff,
            Instruction::Jmp(off) => *off = coff,
            inx => return Err(format!("cannot patch address into {:?} instruction", inx).into()),
        }
    }

    Ok(())
}

enum Offset {
    Resolved(usize),
    Unresolved(Vec<usize>),
}

/// Information for the process of lowering LIR to bytecode
pub struct LirLoweringRuntime {
    meta: LV2ModuleMeta,
    entries: Vec<(usize, usize)>,
    consts: Vec<LV2Value>,
    idents: Vec<LV2Variable>,
    code: Vec<Instruction>,

    globals: HashSet<LV2Variable>,
    offsets: HashMap<LV2Label, Offset>,
}

impl LirLoweringRuntime {
    pub fn from(meta: LV2ModuleMeta) -> Self {
        Self {
            meta,
            entries: vec![],
            consts: vec![],
            idents: vec![],
            code: vec![],

            globals: HashSet::new(),
            offsets: HashMap::new(),
        }
    }

    pub fn lower(mut self, code: Vec<LirElement>) -> LV2CompileResult<LV2CodeObject> {
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

        self.postprocess();

        let mut co = LV2CodeObject::new();

        co.name = self.meta.name;
        co.loc = self.meta.loc;

        co.entries = self.entries;
        co.consts = self.consts;
        co.idents = self.idents;
        co.code = self.code;

        Ok(co)
    }

    fn emit(&mut self, lir_element: LirElement) -> LV2CompileResult<()> {
        match lir_element {
            LirElement::Append => self.code.push(Instruction::Append),
            LirElement::Call { argn, ident } => {
                let iidx = self.index_ident(ident) as u16;

                self.code.push(Instruction::Call(iidx, argn));
            }
            LirElement::Box => self.code.push(Instruction::Box),
            LirElement::Conv { ty } => self.code.push(Instruction::Conv(ty as u16)),
            LirElement::Entry { ident } => {
                let iidx = self.index_ident(ident);
                let off = self.code.len();

                self.entries.push((iidx, off));
            }
            LirElement::Jump { condition, label } => {
                let off = match self.offsets.get_mut(&label) {
                    Some(Offset::Resolved(off)) => *off as u16,
                    Some(Offset::Unresolved(uoffs)) => {
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
                let coff = self.code.len();

                if let Some(Offset::Unresolved(jmps)) =
                    self.offsets.insert(label, Offset::Resolved(coff))
                {
                    patch_addrs(&mut self.code, jmps, coff)?;
                }
            }
            LirElement::Operation(op) => {
                let inx = Instruction::from(op);

                self.code.push(inx);
            }
            LirElement::PushConstant { value } => {
                let cidx = self.index_const(&value) as u16;

                self.code.push(Instruction::CPush(cidx));
            }
            LirElement::ScopeGlobal { ident } => {
                self.globals.insert(ident.clone());
            }
            LirElement::ScopeLocal { ident } => {
                self.globals.remove(ident);
            }
            LirElement::PushDynamic { ident, .. } => {
                let iidx = self.index_ident(ident) as u16;

                if self.globals.contains(ident) {
                    self.code.push(Instruction::GPush(iidx));
                } else {
                    self.code.push(Instruction::LPush(iidx));
                }
            }
            LirElement::StoreDynamic { ident, .. } => {
                let iidx = self.index_ident(ident) as u16;

                if self.globals.contains(ident) {
                    self.code.push(Instruction::GMove(iidx));
                } else {
                    self.code.push(Instruction::LMove(iidx));
                }
            }

            LirElement::Drop => self.code.push(Instruction::Drop),
            LirElement::Duplicate => self.code.push(Instruction::Dup),
            LirElement::Get => self.code.push(Instruction::Get),
            LirElement::RGet => self.code.push(Instruction::RGet),
            LirElement::Interrupt { n } => self.code.push(Instruction::Interrupt(n)),
            LirElement::Import { namespaced } => {
                if namespaced {
                    self.code.push(Instruction::NImport);
                } else {
                    self.code.push(Instruction::Import);
                }
            }
            LirElement::Ret => self.code.push(Instruction::Ret),
            LirElement::Set => self.code.push(Instruction::Set),
            LirElement::Slice => self.code.push(Instruction::Slice),
            // TODO: implement this once reference logic was updated
            LirElement::Unbox => unimplemented!(),

            LirElement::IterCreate => self.code.push(Instruction::IterCreate),
            LirElement::IterCreateRanged => self.code.push(Instruction::IterCreateRanged),
            LirElement::IterHasNext => self.code.push(Instruction::IterHasNext),
            LirElement::IterNext => self.code.push(Instruction::IterNext),
            LirElement::IterReverse => self.code.push(Instruction::IterReverse),
        }

        Ok(())
    }

    fn postprocess(&mut self) {
        for inx in self.code.iter_mut() {
            if let Instruction::Call(iidx, argn) = inx {
                // if the calls target name was declared inside the current module
                if self.entries.iter().any(|(idx, _)| *idx == *iidx as usize) {
                    *inx = Instruction::LCall(*iidx, *argn);
                }
            }
        }
    }

    fn index_const(&mut self, val: &LV2Value) -> usize {
        match self.consts.iter().position(|item| item == val) {
            Some(pos) => pos,
            None => {
                self.consts.push(val.clone());
                self.consts.len() - 1
            }
        }
    }

    fn index_ident(&mut self, var: &LV2Variable) -> usize {
        match self.idents.iter().position(|item| item == var) {
            Some(pos) => pos,
            None => {
                self.idents.push(var.clone());
                self.idents.len() - 1
            }
        }
    }
}

impl From<Operator> for Instruction {
    fn from(op: Operator) -> Self {
        match op {
            Operator::Operator2(op) => match op {
                LV2Operator2::Add => Instruction::Add,
                LV2Operator2::Sub => Instruction::Sub,
                LV2Operator2::Mul => Instruction::Mul,
                LV2Operator2::Div => Instruction::Div,
                LV2Operator2::Pow => Instruction::Pow,
                LV2Operator2::Rem => Instruction::Rem,
                LV2Operator2::Shl => Instruction::Shl,
                LV2Operator2::Shr => Instruction::Shr,
                LV2Operator2::And => Instruction::And,
                LV2Operator2::Or => Instruction::Or,
                LV2Operator2::XOr => Instruction::XOr,
                LV2Operator2::Eq => Instruction::Eq,
                LV2Operator2::Ne => Instruction::Ne,
                LV2Operator2::Ge => Instruction::Ge,
                LV2Operator2::Gt => Instruction::Gt,
                LV2Operator2::Le => Instruction::Le,
                LV2Operator2::Lt => Instruction::Lt,
            },
            Operator::Operator1(op) => match op {
                LV2Operator1::Abs => Instruction::Abs,
                LV2Operator1::Not => Instruction::Not,
            },
        }
    }
}
