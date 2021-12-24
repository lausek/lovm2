use super::*;

pub struct HirLoweringRepeat {
    id: usize,
}

impl LV2HirJumpable for HirLoweringRepeat {
    fn new(counter: LabelCounterRef) -> Self {
        Self {
            id: counter.borrow_mut().create_repeat_id(),
        }
    }

    fn end(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: false,
            ty: LV2LabelTy::Repeat,
        }
    }

    fn start(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: true,
            ty: LV2LabelTy::Repeat,
        }
    }
}
