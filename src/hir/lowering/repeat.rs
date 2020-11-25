pub struct HirLoweringLoop {
    pub start: usize,
    pub end: Option<usize>,
    pub breaks: Vec<usize>,
    pub continues: Vec<usize>,
}

impl HirLoweringLoop {
    pub fn from(start: usize) -> Self {
        Self {
            start,
            end: None,
            breaks: vec![],
            continues: vec![],
        }
    }

    pub fn add_break(&mut self, idx: usize) {
        self.breaks.push(idx);
    }

    pub fn add_continue(&mut self, idx: usize) {
        self.continues.push(idx);
    }
}
