pub struct HirLoweringBranch {
    pub start: usize,
    pub end: Option<usize>,

    pub conditions: Vec<HirLoweringCondition>,
}

impl HirLoweringBranch {
    pub fn from(start: usize) -> Self {
        Self {
            start,
            end: None,

            conditions: vec![],
        }
    }

    pub fn condition_mut(&mut self) -> Option<&mut HirLoweringCondition> {
        self.conditions.last_mut()
    }

    pub fn default_mut(&mut self) -> Option<&mut HirLoweringCondition> {
        self.conditions.iter_mut().find(|cond| cond.is_default)
    }

    pub fn add_condition(&mut self, start: usize) -> &mut HirLoweringCondition {
        self.conditions.push(HirLoweringCondition::from(start));
        self.conditions.last_mut().unwrap()
    }

    pub fn add_default(&mut self, start: usize) -> &mut HirLoweringCondition {
        self.conditions.push(HirLoweringCondition::default(start));
        self.conditions.last_mut().unwrap()
    }
}

pub struct HirLoweringCondition {
    pub start: usize,
    pub next: Option<usize>,
    pub term: Option<usize>,
    pub is_default: bool,
}

impl HirLoweringCondition {
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
