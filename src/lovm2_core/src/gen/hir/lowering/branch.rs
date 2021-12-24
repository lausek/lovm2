use super::*;

pub struct HirLoweringBranch {
    counter: LabelCounterRef,
    id: usize,
}

impl HirLoweringBranch {
    pub fn new(counter: LabelCounterRef) -> Self {
        Self {
            counter: counter.clone(),
            id: counter.borrow_mut().create_branch_id(),
        }
    }

    pub fn add_condition(&mut self) -> HirLoweringCondition {
        HirLoweringCondition::new(self.counter.clone())
    }

    pub fn add_default(&mut self) -> HirLoweringCondition {
        HirLoweringCondition::new(self.counter.clone())
    }
}

impl LV2HirJumpable for HirLoweringBranch {
    fn new(counter: LabelCounterRef) -> Self {
        Self {
            counter: counter.clone(),
            id: counter.borrow_mut().create_condition_id(),
        }
    }

    fn end(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: false,
            ty: LV2LabelTy::Branch,
        }
    }

    fn start(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: true,
            ty: LV2LabelTy::Branch,
        }
    }
}

pub struct HirLoweringCondition {
    id: usize,
}

impl LV2HirJumpable for HirLoweringCondition {
    fn new(counter: LabelCounterRef) -> Self {
        Self {
            id: counter.borrow_mut().create_condition_id(),
        }
    }

    fn end(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: false,
            ty: LV2LabelTy::Condition,
        }
    }

    fn start(&self) -> LV2Label {
        LV2Label {
            id: self.id,
            is_start: true,
            ty: LV2LabelTy::Condition,
        }
    }
}
