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

    pub fn add(&mut self, offset: usize) {
        self.offsets.insert(offset);
    }

    pub fn is_valid(&self, offset: usize) -> bool {
        self.offsets.contains(&offset)
    }
}
