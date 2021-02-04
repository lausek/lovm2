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

impl Jumpable for HirLoweringBranch {
    fn new(counter: LabelCounterRef) -> Self {
        Self {
            counter: counter.clone(),
            id: counter.borrow_mut().create_condition_id(),
        }
    }

    fn end(&self) -> Label {
        Label::from(format!("branch_{}_end", self.id))
    }

    fn start(&self) -> Label {
        Label::from(format!("branch_{}_start", self.id))
    }
}

pub struct HirLoweringCondition {
    id: usize,
}

impl Jumpable for HirLoweringCondition {
    fn new(counter: LabelCounterRef) -> Self {
        Self {
            id: counter.borrow_mut().create_condition_id(),
        }
    }

    fn end(&self) -> Label {
        Label::from(format!("cond_{}_end", self.id))
    }

    fn start(&self) -> Label {
        Label::from(format!("cond_{}_start", self.id))
    }
}
