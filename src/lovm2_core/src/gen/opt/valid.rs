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
            if self.is_valid(off) {
                break;
            }

            if let LirElement::Entry { .. } = elem {
                scanning = true;
            }

            if scanning {
                self.add(off);
            }

            match elem {
                LirElement::Jump { condition, label } => {
                    let (label_off, _) = code
                        .iter()
                        .enumerate()
                        .find(|(_, elem)| matches!(elem, LirElement::Label(l) if l == label))
                        .unwrap();

                    if condition.is_some() {
                        self.update(code, label_off, true);
                    } else {
                        off = label_off;
                        continue;
                    }
                }
                LirElement::Ret => {
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
