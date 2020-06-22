use std::collections::HashMap;

use crate::frame::Frame;
use crate::module::Module;
use crate::value::RuValue;
use crate::var::Variable;

pub struct Context {
    pub modules: Vec<Module>,
    pub globals: HashMap<Variable, RuValue>,

    pub lstack: Vec<Frame>,
    pub vstack: Vec<RuValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            modules: vec![],
            globals: HashMap::new(),

            lstack: vec![],
            vstack: vec![],
        }
    }

    pub fn stack_mut(&mut self) -> &mut Vec<RuValue> {
        &mut self.vstack
    }

    pub fn frame_mut(&mut self) -> Option<&mut Frame> {
        self.lstack.last_mut()
    }

    pub fn push_frame(&mut self, argn: u8) {
        self.lstack.push(Frame::new(argn));
    }

    pub fn pop_frame(&mut self) {
        self.lstack.pop();
    }

    pub fn push_value(&mut self, value: RuValue) {
        self.vstack.push(value);
    }

    pub fn pop_value(&mut self) -> Option<RuValue> {
        self.vstack.pop()
    }
}
