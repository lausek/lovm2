pub struct LoweringBranch {
    pub start: usize,
    pub end: Option<usize>,
}

impl LoweringBranch {
    pub fn from(start: usize) -> Self {
        Self {
            start,
            end: None,
        }
    }
}

pub struct LoweringCondition {}
