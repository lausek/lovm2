//! vm state

use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

use crate::code::CodeObjectRef;
use crate::frame::Frame;
use crate::module::{Module, ModuleProtocol};
use crate::value::{RuValue, RuValueRef};
use crate::var::Variable;

pub type LoadHookFn = dyn Fn(&mut Context) -> Lovm2Result<Option<Box<dyn ModuleProtocol>>>;
// TODO: this should also return some Result
pub type InterruptFn = dyn Fn(&mut Context);

fn find_module(name: &str, load_paths: &[String]) -> Lovm2Result<String> {
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
    Err(format!("{} not found", name).into())
}

/// the state of the virtual machine
///
/// this contains all necessary runtime data and gets shared with objects that
/// implement `CallProtocol` as well as interrupts.
pub struct Context {
    /// list of loaded modules: `Module` or `SharedObjectModule`
    pub modules: Vec<Box<dyn ModuleProtocol>>,
    /// global variables that can be altered from every object
    pub globals: HashMap<Variable, RuValueRef>,
    /// entries in this map can directly be called from lovm2 bytecode
    pub scope: HashMap<Variable, CodeObjectRef>,
    /// interrupt table. these functions can be triggered using the `Interrupt` instruction
    pub interrupts: [Option<Rc<InterruptFn>>; 256],
    /// function to call if a module is about to be loaded
    pub load_hook: Option<Rc<LoadHookFn>>,
    /// list of directories for module lookup
    pub load_paths: Vec<String>,

    /// call stack that contains local variables
    pub lstack: Vec<Frame>,
    /// value stack. this is shared across `CallProtocol` functionality
    pub vstack: Vec<RuValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            modules: vec![],
            globals: HashMap::new(),
            scope: HashMap::new(),
            interrupts: [None; 256],
            load_hook: None,
            load_paths: vec![format!(
                "{}/.local/lib/lovm2/",
                dirs::home_dir().unwrap().to_str().unwrap()
            )],

            lstack: vec![],
            vstack: vec![],
        }
    }

    /// lookup a module name in `load_paths` and add it to the context
    pub fn load_and_import_by_name(&mut self, name: &str) -> Lovm2Result<()> {
        let mut module = None;
        if let Some(load_hook) = self.load_hook.clone() {
            module = load_hook(self)?;
        }
        if module.is_none() {
            let path = find_module(name, &self.load_paths)?;
            module = Some(Module::load_from_file(path)?);
        }
        self.load_and_import_all(module.unwrap())
    }

    /// add the module and all of its slots to `scope`
    pub fn load_and_import_all(&mut self, module: Box<dyn ModuleProtocol>) -> Lovm2Result<()> {
        for (key, co_object) in module.slots().iter() {
            if self.scope.insert(key.clone(), co_object.clone()).is_some() {
                return Err(format!("import conflict: `{}` is already defined", key).into());
            }
        }

        self.modules.push(module);

        Ok(())
    }

    pub fn lookup_code_object(&self, name: &Variable) -> Lovm2Result<CodeObjectRef> {
        match self.scope.get(name).cloned() {
            Some(co) => Ok(co),
            _ => Err(format!("code object `{}` not found", name).into()),
        }
    }

    pub fn set_load_hook<T>(&mut self, hook: T)
    where
        T: Fn(&mut Context) -> Lovm2Result<Option<Box<dyn ModuleProtocol>>> + Sized + 'static,
    {
        self.load_hook = Some(Rc::new(hook));
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

    pub fn frame_mut(&mut self) -> Lovm2Result<&mut Frame> {
        match self.lstack.last_mut() {
            Some(frame) => Ok(frame),
            _ => Err("no frame on stack".into()),
        }
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

    pub fn pop_value(&mut self) -> Lovm2Result<RuValue> {
        match self.vstack.pop() {
            Some(val) => Ok(val),
            _ => Err("no value on stack".into()),
        }
    }
}
