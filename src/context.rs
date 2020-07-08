use std::collections::HashMap;
use std::rc::Rc;

use crate::code::CodeObjectRef;
use crate::frame::Frame;
use crate::module::{Module, ModuleProtocol};
use crate::value::{RuValue, RuValueRef};
use crate::var::Variable;

// TODO: this should also return some Result
pub type InterruptFn = dyn Fn(&mut Context);

fn find_module(name: &str, load_paths: &[String]) -> Result<String, String> {
    use std::fs::read_dir;
    for path in load_paths.iter() {
        let dir = read_dir(path).map_err(|e| e.to_string())?;
        for entry in dir {
            let entry = entry.map_err(|e| e.to_string())?;
            let fname = entry.path();
            if fname.file_stem().unwrap() == name {
                let abspath = std::fs::canonicalize(fname).unwrap();
                let abspath = abspath.to_string_lossy();
                return Ok(abspath.into_owned());
            }
        }
    }
    Err(format!("{} not found", name))
}

pub struct Context {
    pub modules: Vec<Box<dyn ModuleProtocol>>,
    pub globals: HashMap<Variable, RuValueRef>,
    pub scope: HashMap<Variable, CodeObjectRef>,
    pub interrupts: [Option<Rc<InterruptFn>>; 256],
    pub load_paths: Vec<String>,

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
            load_paths: vec![format!(
                "{}/.local/lib/lovm2/",
                dirs::home_dir().unwrap().to_str().unwrap()
            )],

            lstack: vec![],
            vstack: vec![],
        }
    }

    pub fn load_and_import_by_name(&mut self, name: &str) -> Result<(), String> {
        let path = find_module(name, &self.load_paths)?;
        let module = Module::load_from_file(path)?;
        self.load_and_import_all(module)
    }

    pub fn load_and_import_all(&mut self, module: Box<dyn ModuleProtocol>) -> Result<(), String> {
        for (key, co_object) in module.slots().iter() {
            if self.scope.insert(key.clone(), co_object.clone()).is_some() {
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
    where
        T: Fn(&mut Context) + Sized + 'static,
    {
        self.interrupts[n as usize] = Some(Rc::new(func));
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
