use std::collections::HashSet;

use super::*;

pub(super) struct ValidPath {
    // TODO: can this be done more efficiently?
    offsets: HashSet<usize>,
}

impl ValidPath {
    pub fn scan(code: &[LirElement]) -> Self {
        let mut vp = ValidPath::default();

        vp.update(code, 0, false);

        vp
    }

    fn add(&mut self, offset: usize) {
        self.offsets.insert(offset);
    }

    pub fn is_valid(&self, offset: usize) -> bool {
        self.offsets.contains(&offset)
    }

    fn update(&mut self, code: &[LirElement], mut off: usize, mut scanning: bool) {
        while let Some(elem) = code.get(off) {
            // Stop adding offsets to valid path if the current offset is already known.
            if self.is_valid(off) {
                break;
            }

            // Entry labels must be reachable from extern calls and are always valid as such.
            // Everything that follows the entry will be added to the valid path.
            if let LirElement::Entry { .. } = elem {
                scanning = true;
            }

            if scanning {
                self.add(off);
            }

            match elem {
                LirElement::Jump { condition, label } => {
                    // Jumps change the programs execution location and must be handled carefully.
                    let (label_off, _) = code
                        .iter()
                        .enumerate()
                        .find(|(_, elem)| matches!(elem, LirElement::Label(l) if l == label))
                        .unwrap();

                    if condition.is_some() {
                        // If the jump is conditionally executed, we interrupt scanning and check
                        // the target offset first as it could be reached by the program.
                        // Once this scan is done we continue scanning at the current location.
                        self.update(code, label_off, true);
                    } else {
                        // If the jump is always executed (does not have a condition), we
                        // continue scanning at the jumps target offset.
                        off = label_off;
                        continue;
                    }
                }
                LirElement::Ret => {
                    // Return instructions mark a functions end. Instructions after return must be
                    // explicitly targeted by jumps or follow an entry label in order to be
                    // considered reachable.
                    scanning = false;
                }
                _ => {}
            }

            off += 1;
        }
    }
}

impl std::default::Default for ValidPath {
    fn default() -> Self {
        Self {
            offsets: HashSet::new(),
        }
    }
}
