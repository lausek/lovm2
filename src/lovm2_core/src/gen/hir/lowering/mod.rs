//! Transform Hir into actual bytecode.

mod runtime;

use super::*;

pub use self::runtime::LV2HirLoweringRuntime;

/// Structures supporting transformation into LIR.
pub trait LV2HirLowering {
    fn lower<'lir, 'hir: 'lir>(&'hir self, runtime: &mut LV2HirLoweringRuntime<'lir>);
}

#[derive(Default)]
pub struct LV2LabelCounter {
    counter: usize,
}

impl LV2LabelCounter {
    fn create_id(&mut self) -> usize {
        let id = self.counter;
        self.counter += 1;
        id
    }

    pub fn create_new_label(&mut self) -> LV2Label {
        LV2Label {
            id: self.create_id(),
            is_start: true,
            ty: LV2LabelTy::Condition,
        }
    }

    pub fn create_section(&mut self, ty: LV2LabelTy) -> LV2Section {
        LV2Section {
            id: self.create_id(),
            ty,
        }
    }
}

/// Structures supporting custom jump targets.
pub struct LV2Section {
    id: usize,
    ty: LV2LabelTy,
}

impl LV2Section {
    pub fn new(id: usize, ty: LV2LabelTy) -> Self {
        Self { id, ty }
    }

    pub fn end(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: false,
            ty: self.ty.clone(),
        }
    }

    pub fn start(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: true,
            ty: self.ty.clone(),
        }
    }
}
