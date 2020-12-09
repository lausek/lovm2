//! runs modules and maintains program state

use std::ops::*;
use std::rc::Rc;

use lovm2_error::*;

use crate::bytecode::Instruction;
use crate::code::{CallProtocol, CodeObject};
use crate::context::Context;
use crate::gen::Expr;
use crate::module::{create_standard_module, Module};
use crate::value::{box_value, Value};
use crate::var::Variable;

/// virtual machine for executing [modules](crate::module::Module)
///
/// call convention is pascal style. if you have a function like `f(a, b, c)` it will be translated
/// to
///
/// ``` ignore
/// push a
/// push b
/// push c
/// call f, 3
/// ```
///
/// and the function has to do the popping in reverse
///
/// ``` ignore
/// pop c
/// pop b
/// pop a
/// ```
///

pub type InterruptFn = dyn Fn(&mut Vm) -> Lovm2Result<()>;
pub type ImportHookFn = dyn Fn(&str, &str) -> String;
pub type LoadHookFn = dyn Fn(&LoadRequest) -> Lovm2Result<Option<Module>>;

macro_rules! value_operation {
    ($vm:expr, $fn:ident) => {{
        let second = $vm.ctx.pop_value()?;
        let first = $vm.ctx.last_value_mut()?;
        first.$fn(second)?;
    }};
}

macro_rules! value_compare {
    ($vm:expr, $fn:ident) => {{
        let second = $vm.ctx.pop_value()?;
        let first = $vm.ctx.pop_value()?;
        $vm.ctx.push_value(Value::Bool(first.$fn(&second)));
    }};
}

pub struct Vm {
    pub ctx: Context,

    import_hook: Option<Rc<ImportHookFn>>,
    /// interrupt table. these functions can be triggered using the `Interrupt` instruction
    pub interrupts: Vec<Option<Rc<InterruptFn>>>,
    /// function to call if a module is about to be loaded
    pub load_hook: Option<Rc<LoadHookFn>>,
    /// list of directories for module lookup
    pub load_paths: Vec<String>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),

            import_hook: None,
            interrupts: vec![None; 256],
            load_hook: None,
            load_paths: vec![format!(
                "{}/.local/lib/lovm2/",
                dirs::home_dir().unwrap().to_str().unwrap()
            )],
        }
    }

    pub fn with_std() -> Self {
        let mut vm = Self::new();
        vm.load_and_import_all(create_standard_module()).unwrap();
        vm
    }

    pub fn add_load_path(&mut self, path: String) {
        self.load_paths.push(path);
    }

    pub fn call(&mut self, name: &str, args: &[Value]) -> Lovm2Result<Value> {
        let name = Variable::from(name);
        let co = self.ctx.lookup_code_object(&name)?;

        let mut argn: u8 = 0;
        for arg in args.iter() {
            argn += 1;
            let arg = arg.clone();
            let arg = match arg {
                Value::Dict(_) | Value::List(_) => box_value(arg),
                _ => arg,
            };
            self.ctx.push_value(arg);
        }

        self.ctx.push_frame(argn);
        co.run(self)?;
        self.ctx.pop_frame();

        let val = self.context_mut().pop_value()?;
        Ok(val)
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    pub fn evaluate_expr(&mut self, expr: &Expr) -> Lovm2Result<Value> {
        match expr {
            Expr::Access(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::DynamicValue(_) => todo!(),
            Expr::Operation1(_, _) => todo!(),
            Expr::Operation2(_, _, _) => todo!(),

            Expr::Slice(_) => todo!(),
            Expr::Value { val, .. } => Ok(val.clone()),
            Expr::Variable(var) => match self.ctx.globals.get(&var) {
                Some(val) => Ok(val.clone()),
                _ => Err((Lovm2ErrorTy::LookupFailed, var).into()),
            },
        }
    }

    fn load_and_import_filter(
        &mut self,
        module: Module,
        filter: impl Fn(&Variable) -> bool,
        importer: Option<Rc<dyn Fn(&str, &str) -> String>>,
    ) -> Lovm2Result<()> {
        if self.ctx.modules.get(module.name()).is_none() {
            // load static dependencies of module
            for used_module in module.uses() {
                self.load_and_import_by_name(used_module, module.location().cloned())?;
            }

            let module = Rc::new(module);
            for (key, co) in module.slots().iter() {
                if filter(key) {
                    continue;
                }

                if self.ctx.scope.insert(key.clone(), co.clone()).is_some() {
                    return Err((Lovm2ErrorTy::ImportConflict, key).into());
                } else if key.as_ref() == crate::prelude::ENTRY_POINT {
                    self.ctx.entry = Some(co.clone());
                }
            }

            self.ctx.modules.insert(module.name().to_string(), module);
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
        if self.ctx.modules.get(name).is_some() {
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

        let import_hook = self.import_hook.as_ref().cloned();

        self.load_and_import_filter(module.unwrap(), filter_entry_reimport, import_hook)
    }

    /// add the module and all of its slots to `scope`
    pub fn load_and_import_all<T>(&mut self, module: T) -> Lovm2Result<()>
    where
        T: Into<Module>,
    {
        self.load_and_import_filter(module.into(), |_| false, None)
    }

    /// a wrapper for `run_bytecode` that handles pushing and popping stack frames
    pub fn run_object(&mut self, co: &dyn CallProtocol) -> Lovm2Result<()> {
        self.ctx.push_frame(0);
        co.run(self)?;
        self.ctx.pop_frame();

        Ok(())
    }

    /// start the execution at `ENTRY_POINT`
    pub fn run(&mut self) -> Lovm2Result<()> {
        if let Some(callable) = self.ctx.entry.take() {
            self.run_object(callable.as_ref())
        } else {
            Err(Lovm2ErrorTy::NoEntryPoint.into())
        }
    }

    /// implementation of lovm2 bytecode behavior
    ///
    /// *Note:* this function does not push a stack frame and could therefore mess up local variables
    /// if not handled correctly. see `Vm.run_object`
    pub fn run_bytecode(&mut self, co: &CodeObject, offset: usize) -> Lovm2Result<()> {
        let mut ip = offset;
        while let Some(inx) = co.code.get(ip) {
            match inx {
                Instruction::Pushl(lidx) => {
                    let variable = &co.idents[*lidx as usize];
                    match self.ctx.frame_mut()?.value_of(variable) {
                        Some(local) => self.ctx.push_value(local),
                        _ => return Err((Lovm2ErrorTy::LookupFailed, variable).into()),
                    }
                }
                Instruction::Pushg(gidx) => {
                    let variable = &co.idents[*gidx as usize];
                    match self.ctx.value_of(variable) {
                        Some(global) => self.ctx.push_value(global),
                        _ => return Err((Lovm2ErrorTy::LookupFailed, variable).into()),
                    }
                }
                Instruction::Pushc(cidx) => {
                    let value = &co.consts[*cidx as usize];
                    self.ctx.push_value(value.clone());
                }
                Instruction::Movel(lidx) => {
                    let variable = &co.idents[*lidx as usize];
                    let value = self.ctx.pop_value()?;
                    self.ctx.frame_mut()?.locals.insert(variable.clone(), value);
                }
                Instruction::Moveg(gidx) => {
                    let variable = &co.idents[*gidx as usize];
                    let value = self.ctx.pop_value()?;
                    self.ctx.globals.insert(variable.clone(), value);
                }
                Instruction::Discard => {
                    self.ctx.pop_value()?;
                }
                Instruction::Dup => match self.ctx.stack_mut().last().cloned() {
                    Some(last) => self.ctx.push_value(last),
                    _ => return Err(Lovm2ErrorTy::ValueStackEmpty.into()),
                },
                Instruction::Get => {
                    let key = self.ctx.pop_value()?;
                    let obj = self.ctx.pop_value()?;
                    let val = obj.get(key)?;
                    self.ctx.push_value(val.deref().unwrap());
                }
                Instruction::Getr => {
                    let key = self.ctx.pop_value()?;
                    let mut obj = self.ctx.pop_value()?;

                    if let Err(e) = obj.get(key.clone()) {
                        if Lovm2ErrorTy::KeyNotFound != e.ty {
                            return Err(e);
                        }
                        obj.set(key.clone(), box_value(Value::Nil))?;
                    }

                    let val = obj.get(key)?;
                    self.ctx.push_value(val);
                }
                Instruction::Set => {
                    let mut val = self.ctx.pop_value()?;
                    let target = self.ctx.pop_value()?;

                    deref_total(&mut val);

                    match target {
                        Value::Ref(Some(r)) => *r.borrow_mut() = val,
                        _ => return Err(format!("cannot use {:?} as set target", target).into()),
                    }
                }
                Instruction::Add => value_operation!(self, add_inplace),
                Instruction::Sub => value_operation!(self, sub_inplace),
                Instruction::Mul => value_operation!(self, mul_inplace),
                Instruction::Div => value_operation!(self, div_inplace),
                Instruction::Pow => value_operation!(self, pow_inplace),
                Instruction::Rem => value_operation!(self, rem_inplace),
                Instruction::And => value_operation!(self, and_inplace),
                Instruction::Or => value_operation!(self, or_inplace),
                Instruction::Not => {
                    let first = self.ctx.pop_value()?;
                    self.ctx.push_value(first.not()?);
                }
                Instruction::Eq => value_compare!(self, eq),
                Instruction::Ne => value_compare!(self, ne),
                Instruction::Ge => value_compare!(self, ge),
                Instruction::Gt => value_compare!(self, gt),
                Instruction::Le => value_compare!(self, le),
                Instruction::Lt => value_compare!(self, lt),
                Instruction::Jmp(addr) => {
                    ip = *addr as usize;
                    continue;
                }
                Instruction::Jt(addr) => {
                    let first = self.ctx.pop_value()?;
                    if first.as_bool_inner()? {
                        ip = *addr as usize;
                        continue;
                    }
                }
                Instruction::Jf(addr) => {
                    let first = self.ctx.pop_value()?;
                    if !first.as_bool_inner()? {
                        ip = *addr as usize;
                        continue;
                    }
                }
                Instruction::Call(argn, gidx) => {
                    let func = &co.idents[*gidx as usize];
                    let other_co = self.ctx.lookup_code_object(func)?;
                    self.ctx.push_frame(*argn);
                    other_co.run(self)?;
                    self.ctx.pop_frame();
                }
                Instruction::Ret => break,
                Instruction::Interrupt(n) => {
                    if let Some(func) = &self.interrupts[*n as usize] {
                        func.clone()(self)?;
                    }
                }
                Instruction::Cast(tid) => {
                    self.ctx.last_value_mut()?.cast_inplace(*tid)?;
                }
                Instruction::Load => {
                    let name = self.ctx.pop_value()?;
                    // TODO: use to_string() here
                    let name = format!("{}", name);
                    // path to the modules source code
                    let relative_to = if let Some(mname) = co.module() {
                        self.ctx
                            .modules
                            .get(&mname)
                            .and_then(|module| module.location())
                            .map(String::to_string)
                    } else {
                        None
                    };
                    self.load_and_import_by_name(name.as_ref(), relative_to)?;
                }
                Instruction::Box => {
                    let value = self.ctx.pop_value()?;
                    self.ctx.push_value(box_value(value));
                }
                Instruction::Slice => {
                    let end = self.ctx.pop_value()?;
                    let start = self.ctx.pop_value()?;
                    let target = self.ctx.pop_value()?;
                    let slice = create_slice(target, start, end)?;
                    self.ctx.push_value(slice);
                }
            }

            ip += 1;
        }

        Ok(())
    }

    pub fn set_import_hook<T>(&mut self, hook: T)
    where
        T: Fn(&str, &str) -> String + 'static,
    {
        self.import_hook = Some(Rc::new(hook));
    }

    /// register a new callback function that is used for resolving dependencies at runtime
    pub fn set_load_hook<T>(&mut self, hook: T)
    where
        T: Fn(&LoadRequest) -> Lovm2Result<Option<Module>> + Sized + 'static,
    {
        self.load_hook = Some(Rc::new(hook));
    }

    /// register a new callback function on interrupt `n`
    pub fn set_interrupt<T>(&mut self, n: u16, func: T)
    where
        T: Fn(&mut Vm) -> Lovm2Result<()> + Sized + 'static,
    {
        self.interrupts[n as usize] = Some(Rc::new(func));
    }
}

fn deref_total(val: &mut Value) {
    while let Value::Ref(Some(r)) = val {
        let r = r.borrow().clone();
        *val = r;
    }
}

fn create_slice(target: Value, start: Value, end: Value) -> Lovm2Result<Value> {
    let start_idx = match start {
        Value::Nil => 0,
        _ => start.as_integer_inner()?,
    };
    let end_idx = match end {
        Value::Nil => target.len()? as i64,
        _ => end.as_integer_inner()?,
    };
    let mut slice = vec![];

    for idx in start_idx..end_idx {
        slice.push(target.get(Value::from(idx))?);
    }

    Ok(box_value(Value::List(slice)))
}

fn filter_entry_reimport(name: &Variable) -> bool {
    name.as_ref() == crate::prelude::ENTRY_POINT
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

pub fn find_candidate(req: &LoadRequest) -> Lovm2Result<String> {
    if let Some(relative_to) = &req.relative_to {
        let paths = &[std::path::Path::new(relative_to)
            .parent()
            .unwrap()
            .display()
            .to_string()];
        find_module(&req.module, paths)
    } else {
        Err((Lovm2ErrorTy::ModuleNotFound, &req.module).into())
    }
}

pub struct LoadRequest {
    pub module: String,
    pub relative_to: Option<String>,
}
