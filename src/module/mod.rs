pub mod builder;
pub mod standard;

use std::collections::HashMap;

use crate::code::CodeObjectRef;
use crate::var::Variable;

pub use self::builder::ModuleBuilder;
pub use self::standard::create_standard_module;

#[derive(Debug)]
pub struct Module {
    pub slots: HashMap<Variable, CodeObjectRef>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
}
