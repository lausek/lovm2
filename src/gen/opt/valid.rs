use std::collections::HashSet;

pub(super) struct ValidPath {
    // TODO: can this be done more efficiently?
    offsets: HashSet<usize>,
}

impl ValidPath {
    pub fn new() -> Self {
        Self {
            offsets: HashSet::new(),
        }
    }

    pub fn add(&mut self, offset: usize) -> bool {
        if !self.is_valid(offset) {
            self.offsets.insert(offset);
            true
        } else {
            false
        }
    }

    pub fn is_valid(&self, offset: usize) -> bool {
        self.offsets.contains(&offset)
    }
}
