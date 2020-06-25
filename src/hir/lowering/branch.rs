pub struct LoweringBranch {
    pub start: usize,
    pub end: Option<usize>,

    pub conditions: Vec<LoweringCondition>,
}

impl LoweringBranch {
    pub fn from(start: usize) -> Self {
        Self {
            start,
            end: None,

            conditions: vec![],
        }
    }

    pub fn condition_mut(&mut self) -> Option<&mut LoweringCondition> {
        self.conditions.last_mut()
    }

    pub fn default_mut(&mut self) -> Option<&mut LoweringCondition> {
        self.conditions.iter_mut().find(|cond| cond.is_default)
    }

    pub fn add_condition(&mut self, start: usize) -> &mut LoweringCondition {
        self.conditions.push(LoweringCondition::from(start));
        self.conditions.last_mut().unwrap()
    }

    pub fn add_default(&mut self, start: usize) -> &mut LoweringCondition {
        self.conditions.push(LoweringCondition::default(start));
        self.conditions.last_mut().unwrap()
    }
}

pub struct LoweringCondition {
    pub start: usize,
    pub next: Option<usize>,
    pub term: Option<usize>,
    pub is_default: bool,
}

impl LoweringCondition {
    pub fn from(start: usize) -> Self {
        Self {
            start,
            next: None,
            term: None,
            is_default: false,
        }
    }

    pub fn default(start: usize) -> Self {
        let mut new = Self::from(start);
        new.is_default = true;
        new
    }
}
