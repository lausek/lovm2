use std::collections::HashMap;
use std::rc::Rc;

use crate::code::CodeObjectRef;
use crate::frame::Frame;
use crate::module::Module;
use crate::value::RuValue;
use crate::var::Variable;

pub type InterruptFn = dyn Fn(&mut Context) -> ();

pub struct Context {
    pub modules: Vec<Module>,
    pub globals: HashMap<Variable, RuValue>,
    pub scope: HashMap<Variable, CodeObjectRef>,
    pub interrupts: [Option<Rc<Box<InterruptFn>>>; 256],

    pub lstack: Vec<Frame>,
    pub vstack: Vec<RuValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            modules: vec![],
            globals: HashMap::new(),
            scope: HashMap::new(),
            interrupts: [None; 256],

            lstack: vec![],
            vstack: vec![],
        }
    }

    pub fn load_and_import_all(&mut self, module: Module) -> Result<(), String> {
        for (key, co_object) in module.slots.iter() {
            if let Some(_) = self.scope.insert(key.clone(), co_object.clone()) {
                return Err(format!("import conflict: `{}` is already defined", key));
            }
        }

        self.modules.push(module);

        Ok(())
    }

    pub fn lookup_code_object(&self, name: &Variable) -> Option<CodeObjectRef> {
        self.scope.get(name).cloned()
    }

    pub fn set_interrupt<T>(&mut self, n: u16, func: T)
        where T: Fn(&mut Context) -> () + Sized + 'static,
    {
        self.interrupts[n as usize] = Some(Rc::new(Box::new(func)));
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
