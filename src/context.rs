//! vm state

use std::collections::HashMap;
use std::rc::Rc;

use lovm2_error::*;

use crate::code::CodeObjectRef;
use crate::frame::Frame;
use crate::module::{GenericModule, Module};
use crate::value::Value;
use crate::var::Variable;

pub type LoadHookFn = dyn Fn(&LoadRequest) -> Lovm2Result<Option<GenericModule>>;
pub type InterruptFn = dyn Fn(&mut Context) -> Lovm2Result<()>;

fn filter_entry_reimport(name: &Variable) -> bool {
    name.as_ref() != crate::prelude::ENTRY_POINT
}

fn find_module(name: &str, load_paths: &[String]) -> Lovm2Result<String> {
    use std::fs::read_dir;
    for path in load_paths.iter() {
        if let Ok(dir) = read_dir(path) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let fname = entry.path();
                    if fname.file_stem().unwrap() == name {
                        let abspath = std::fs::canonicalize(fname).unwrap();
                        let abspath = abspath.to_string_lossy();
                        return Ok(abspath.into_owned());
                    }
                }
            }
        }
    }
    Err((Lovm2ErrorTy::ModuleNotFound, name).into())
}

pub struct LoadRequest {
    pub module: String,
    pub relative_to: Option<String>,
}

/// the state of the virtual machine
///
/// this contains all necessary runtime data and gets shared with objects that
/// implement `CallProtocol` as well as interrupts.
pub struct Context {
    /// list of loaded modules: `Module` or `SharedObjectModule`
    pub modules: HashMap<String, GenericModule>,
    /// global variables that can be altered from every object
    pub globals: HashMap<Variable, Value>,
    /// entries in this map can directly be called from lovm2 bytecode
    pub scope: HashMap<Variable, CodeObjectRef>,
    /// interrupt table. these functions can be triggered using the `Interrupt` instruction
    pub interrupts: Vec<Option<Rc<InterruptFn>>>,
    /// function to call if a module is about to be loaded
    pub load_hook: Option<Rc<LoadHookFn>>,
    /// list of directories for module lookup
    pub load_paths: Vec<String>,

    /// call stack that contains local variables
    pub lstack: Vec<Frame>,
    /// value stack. this is shared across `CallProtocol` functionality
    pub vstack: Vec<Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            globals: HashMap::new(),
            scope: HashMap::new(),
            interrupts: vec![None; 256],
            load_hook: None,
            load_paths: vec![format!(
                "{}/.local/lib/lovm2/",
                dirs::home_dir().unwrap().to_str().unwrap()
            )],

            lstack: vec![],
            vstack: vec![],
        }
    }

    pub fn add_load_path(&mut self, path: String) {
        self.load_paths.push(path);
    }

    fn load_and_import_filter(
        &mut self,
        module: GenericModule,
        filter: impl Fn(&Variable) -> bool,
    ) -> Lovm2Result<()> {
        if !self.modules.get(module.name()).is_some() {
            // load static dependencies of module
            for used_module in module.uses() {
                self.load_and_import_by_name(used_module, module.location().cloned())?;
            }

            for (key, co) in module.slots().iter() {
                // if the key should be ignored by the filter function
                if !filter(key) {
                    continue;
                }

                if self.scope.insert(key.clone(), co.clone()).is_some() {
                    return Err((Lovm2ErrorTy::ImportConflict, key).into());
                }
            }

            self.modules.insert(module.name().to_string(), module);
        }
        Ok(())
    }

    /// lookup a module name in `load_paths` and add it to the context.
    /// `relative_to` is expected to be an absolute path to importing module
    pub fn load_and_import_by_name(
        &mut self,
        name: &str,
        relative_to: Option<String>,
    ) -> Lovm2Result<()> {
        if self.modules.get(name).is_some() {
            return Ok(());
        }

        let mut module = None;

        if let Some(load_hook) = self.load_hook.clone() {
            let load_request = LoadRequest {
                module: name.to_string(),
                relative_to: relative_to.clone(),
            };
            module = load_hook(&load_request)?;
        }

        if module.is_none() {
            if let Some(relative_to) = relative_to {
                // take directory path from module location and search for requested
                // module name
                let path = std::path::Path::new(&relative_to);
                let relative_to = path.parent().unwrap().to_str().unwrap();
                if let Ok(path) = find_module(name, &[relative_to.to_string()]) {
                    module = Some(Module::load_from_file(path)?);
                }
            }
        }

        if module.is_none() {
            let path = find_module(name, &self.load_paths)?;
            module = Some(Module::load_from_file(path)?);
        }

        self.load_and_import_filter(module.unwrap(), filter_entry_reimport)
    }

    /// add the module and all of its slots to `scope`
    pub fn load_and_import_all(&mut self, module: GenericModule) -> Lovm2Result<()> {
        self.load_and_import_filter(module, |_| true)
    }

    pub fn lookup_code_object(&self, name: &Variable) -> Lovm2Result<CodeObjectRef> {
        match self.scope.get(name).cloned() {
            Some(co) => Ok(co),
            _ => Err((Lovm2ErrorTy::LookupFailed, name).into()),
        }
    }

    /// register a new callback function that is used for resolving dependencies at runtime
    pub fn set_load_hook<T>(&mut self, hook: T)
    where
        T: Fn(&LoadRequest) -> Lovm2Result<Option<GenericModule>> + Sized + 'static,
    {
        self.load_hook = Some(Rc::new(hook));
    }

    /// register a new callback function on interrupt `n`
    pub fn set_interrupt<T>(&mut self, n: u16, func: T)
    where
        T: Fn(&mut Context) -> Lovm2Result<()> + Sized + 'static,
    {
        self.interrupts[n as usize] = Some(Rc::new(func));
    }

    pub fn stack_mut(&mut self) -> &mut Vec<Value> {
        &mut self.vstack
    }

    /// get last stack frame or raise error
    pub fn frame_mut(&mut self) -> Lovm2Result<&mut Frame> {
        match self.lstack.last_mut() {
            Some(frame) => Ok(frame),
            _ => Err(Lovm2ErrorTy::FrameStackEmpty.into()),
        }
    }

    pub fn push_frame(&mut self, argn: u8) {
        self.lstack.push(Frame::new(argn));
    }

    pub fn pop_frame(&mut self) {
        self.lstack.pop();
    }

    pub fn push_value(&mut self, value: Value) {
        self.vstack.push(value);
    }

    pub fn pop_value(&mut self) -> Lovm2Result<Value> {
        match self.vstack.pop() {
            Some(val) => Ok(val),
            _ => Err(Lovm2ErrorTy::ValueStackEmpty.into()),
        }
    }

    pub fn value_of(&self, var: &Variable) -> Option<Value> {
        self.globals.get(var).cloned()
    }
}
