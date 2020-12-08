use super::*;

pub struct HirLoweringRepeat {
    id: usize,
}

impl Jumpable for HirLoweringRepeat {
    fn new(counter: LabelCounterRef) -> Self {
        Self {
            id: counter.borrow_mut().create_repeat_id(),
        }
    }

    fn end(&self) -> Label {
        Label::Custom(format!("rep_{}_end", self.id))
    }

    fn start(&self) -> Label {
        Label::Custom(format!("rep_{}_start", self.id))
    }
}
