pub(super) struct ValidPath {
    // TODO: this is inefficient
    offsets: Vec<usize>,
}

impl ValidPath {
    pub fn new() -> Self {
        Self { offsets: vec![] }
    }

    pub fn add(&mut self, offset: usize) -> bool {
        if !self.is_valid(offset) {
            self.offsets.push(offset);
            true
        } else {
            false
        }
    }

    pub fn is_valid(&self, offset: usize) -> bool {
        self.offsets.contains(&offset)
    }
}
