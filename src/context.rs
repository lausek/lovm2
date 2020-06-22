use crate::frame::Frame;
use crate::module::Module;
use crate::value::RuValue;

pub struct Context {
    pub(crate) modules: Vec<Module>,

    lstack: Vec<Frame>,
    vstack: Vec<RuValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            modules: vec![],

            lstack: vec![],
            vstack: vec![],
        }
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
