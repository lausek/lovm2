//! Runs modules and maintains program state

use std::collections::HashMap;
use std::ops::*;
use std::rc::Rc;

use crate::bytecode::Instruction;
use crate::code::{LV2CallProtocol, LV2CallableRef, LV2CodeObject};
use crate::error::*;
use crate::module::LV2Module;
use crate::value::{box_value, LV2Value, LV2ValueType};
use crate::var::LV2Variable;

mod context;
mod frame;

pub use self::context::Context;
pub use self::frame::Frame;

pub const LOVM2_RESERVED_INTERRUPTS: u16 = 63;
pub const LOVM2_INT_DEBUG: u16 = 10;

/// Virtual machine for executing [modules](crate::module::Module)
///
/// Call convention is pascal style. If you have a function like `f(a, b, c)` it will be translated
/// to
///
/// ``` ignore
/// push a
/// push b
/// push c
/// call f, 3
/// ```
///
/// ... and the function has to do the popping in reverse
///
/// ``` ignore
/// pop c
/// pop b
/// pop a
/// ```
///

/// Function signature for module loading callback.
pub type LoadHookFn = dyn Fn(&LoadRequest) -> LV2Result<Option<LV2Module>>;
/// Function signature of interrupts.
pub type InterruptFn = dyn Fn(&mut Vm) -> LV2Result<()>;
/// Function signature for `Callable` importing.
pub type ImportHookFn = dyn Fn(Option<&str>, &str) -> LV2Result<Option<String>>;

/// Structure containing relevant information for module resolvement.
pub struct LoadRequest {
    pub module: String,
    pub relative_to: Option<String>,
}

pub fn get_lovm2_user_dir() -> String {
    format!(
        "{}/.local/lib/lovm2/",
        dirs::home_dir().unwrap().to_str().unwrap()
    )
}

macro_rules! value_operation {
    ($vm:expr, $fn:ident) => {{
        let second = $vm.ctx.pop_value()?;
        let second = second.clone_inner()?;
        let first = $vm.ctx.last_value_mut()?;
        first.unref_inplace()?;
        first.$fn(second)?;
    }};
}

macro_rules! value_compare {
    ($vm:expr, $fn:ident) => {{
        let second = $vm.ctx.pop_value()?;
        let first = $vm.ctx.pop_value()?;
        $vm.ctx.push_value(LV2Value::Bool(first.$fn(&second)));
    }};
}

/// VM structure containing hooks and loaded modules
pub struct Vm {
    ctx: Context,
    /// List of loaded modules: `Module` or `SharedObjectModule`.
    modules: HashMap<String, Rc<LV2Module>>,

    import_hook: Rc<ImportHookFn>,
    // TODO: make this an array once const_in_array_repeat_expressions was stabilized
    /// Interrupt table. These functions can be triggered using the `Interrupt` instruction.
    pub(crate) interrupts: Vec<Option<Rc<InterruptFn>>>,
    /// Function to call if a module is about to be loaded.
    load_hook: Option<Rc<LoadHookFn>>,
    /// List of directories for module lookup.
    pub load_paths: Vec<String>,
}

impl Vm {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
            modules: HashMap::new(),

            import_hook: Rc::new(default_import_hook),
            interrupts: vec![None; 256],
            load_hook: None,
            load_paths: vec![get_lovm2_user_dir()],
        }
    }

    /// Add a directory for module lookup.
    pub fn add_load_path(&mut self, path: String) {
        self.load_paths.push(path);
    }

    /// Call a function by name with given arguments.
    pub fn call(&mut self, name: &str, args: &[LV2Value]) -> LV2Result<LV2Value> {
        let name = LV2Variable::from(name);
        let co = self.ctx.lookup_code_object(&name)?;

        let mut argn: u8 = 0;

        // move given arguments onto stack and remember amount
        for arg in args.iter() {
            argn += 1;

            let arg = arg.clone();
            let arg = match arg {
                LV2Value::Dict(_) | LV2Value::List(_) => box_value(arg),
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

    /// Get a mutable reference to the current `Context`
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    /// Add a new function by name to the virtual machine.
    pub fn add_function<T: Into<LV2Variable>>(
        &mut self,
        key: T,
        co: LV2CallableRef,
    ) -> LV2Result<()> {
        let key = key.into();

        // this overwrites the slot with the new function. maybe not so good
        if self.ctx.scope.insert(key.clone(), co).is_some() {
            return Err((LV2ErrorTy::ImportConflict, key).into());
        }

        Ok(())
    }

    /// Add the module and all of its slots to `scope`.
    pub fn add_module<T>(&mut self, module: T, namespaced: bool) -> LV2Result<()>
    where
        T: Into<LV2Module>,
    {
        let module = module.into();

        if self.modules.get(module.name()).is_none() {
            // load static dependencies of module
            for used_module in module.uses() {
                // static dependencies are imported
                self.add_module_by_name(used_module, module.location().cloned(), true)?;
            }

            let module = Rc::new(module);

            for (key, co) in module.slots().iter() {
                // if `import` was set, all function names should be patched with the import_hook
                if let Some(nfunc) = (self.import_hook)(Some(module.name().as_ref()), key.as_ref())?
                {
                    self.add_function(nfunc, co.clone())?;

                    // add unnamespaced function as well
                    if !namespaced {
                        if let Some(func) = (self.import_hook)(None, key.as_ref())? {
                            self.add_function(func, co.clone())?;
                        }
                    }
                }
            }

            self.modules.insert(module.name().to_string(), module);
        }

        Ok(())
    }

    /// Lookup a module name in `load_paths` and add it to the context.
    /// `relative_to` is expected to be an absolute path to the imported module.
    pub fn add_module_by_name(
        &mut self,
        name: &str,
        relative_to: Option<String>,
        namespaced: bool,
    ) -> LV2Result<()> {
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
                    module = Some(LV2Module::load_from_file(path)?);
                }
            }
        }

        if module.is_none() {
            let path = find_module(name, &self.load_paths)?;
            module = Some(LV2Module::load_from_file(path)?);
        }

        self.add_module(module.unwrap(), namespaced)
    }

    /// Add the module and all of its slots to `scope`.
    pub fn add_main_module<T>(&mut self, module: T) -> LV2Result<()>
    where
        T: Into<LV2Module>,
    {
        let module = module.into();

        if let Some(co) = module.slots.get(&crate::module::LV2_ENTRY_POINT.into()) {
            self.ctx.entry = Some(co.clone());
            self.add_module(module, false)
        } else {
            Err(LV2ErrorTy::NoEntryPoint.into())
        }
    }

    /// A wrapper for `run_bytecode` that handles pushing and popping stack frames.
    pub fn run_object(&mut self, co: &dyn LV2CallProtocol) -> LV2Result<LV2Value> {
        self.ctx.push_frame(0);
        co.run(self)?;
        self.ctx.pop_frame();

        self.ctx.pop_value()
    }

    /// Start the execution at `ENTRY_POINT`.
    pub fn run(&mut self) -> LV2Result<LV2Value> {
        if let Some(callable) = self.ctx.entry.take() {
            self.run_object(callable.as_ref())
        } else {
            Err(LV2ErrorTy::NoEntryPoint.into())
        }
    }

    #[inline]
    fn run_bytecode_inner(&mut self, co: &LV2CodeObject, ip: &mut usize) -> LV2Result<()> {
        use crate::value::iter::*;

        while let Some(inx) = co.code.get(*ip) {
            match inx {
                Instruction::LPush(lidx) => {
                    let variable = &co.idents[*lidx as usize];
                    let local = self
                        .ctx
                        .frame_mut()?
                        .value_of(variable)
                        .map(LV2Value::clone)?;

                    self.ctx.push_value(local);
                }
                Instruction::GPush(gidx) => {
                    let variable = &co.idents[*gidx as usize];
                    let global = self.ctx.value_of(variable).map(LV2Value::clone)?;

                    self.ctx.push_value(global);
                }
                Instruction::CPush(cidx) => {
                    let value = &co.consts[*cidx as usize];

                    self.ctx.push_value(value.clone());
                }
                Instruction::LMove(lidx) => {
                    let variable = &co.idents[*lidx as usize];
                    let value = self.ctx.pop_value()?;

                    self.ctx.frame_mut()?.set_local(variable, value);
                }
                Instruction::GMove(gidx) => {
                    let variable = &co.idents[*gidx as usize];
                    let value = self.ctx.pop_value()?;

                    self.ctx.set_global(variable, value);
                }
                Instruction::Drop => {
                    self.ctx.pop_value()?;
                }
                Instruction::Dup => {
                    let last = self.ctx.last_value_mut().map(|v| v.clone())?;

                    self.ctx.push_value(last);
                }
                Instruction::Get => {
                    let key = self.ctx.pop_value()?;
                    let obj = self.ctx.pop_value()?;
                    let val = obj.get(&key)?;

                    self.ctx.push_value(val.clone_inner()?);
                }
                Instruction::RGet => {
                    let key = self.ctx.pop_value()?;
                    let mut obj = self.ctx.pop_value()?;

                    if let Err(e) = obj.get(&key) {
                        if LV2ErrorTy::KeyNotFound != e.ty {
                            return Err(e);
                        }

                        obj.set(&key, box_value(LV2Value::Nil))?;
                    }

                    let val = obj.get(&key)?;

                    self.ctx.push_value(val);
                }
                Instruction::Set => {
                    let val = self.ctx.pop_value()?;
                    let target = self.ctx.pop_value()?;

                    match target {
                        LV2Value::Ref(r) => *r.borrow_mut()? = val.clone_inner()?,
                        _ => return err_invalid_set_target(target),
                    }
                }
                Instruction::Append => {
                    let val = self.ctx.pop_value()?;
                    let mut target = self.ctx.pop_value()?;
                    target.append(val)?;
                }
                Instruction::Add => value_operation!(self, add_inplace),
                Instruction::Sub => value_operation!(self, sub_inplace),
                Instruction::Mul => value_operation!(self, mul_inplace),
                Instruction::Div => value_operation!(self, div_inplace),
                Instruction::Pow => value_operation!(self, pow_inplace),
                Instruction::Rem => value_operation!(self, rem_inplace),
                Instruction::Shl => value_operation!(self, shl_inplace),
                Instruction::Shr => value_operation!(self, shr_inplace),
                Instruction::And => value_operation!(self, and_inplace),
                Instruction::Or => value_operation!(self, or_inplace),
                Instruction::XOr => value_operation!(self, xor_inplace),
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
                    *ip = *addr as usize;
                    continue;
                }
                Instruction::Jt(addr) => {
                    let first = self.ctx.pop_value()?;

                    if first.as_bool_inner()? {
                        *ip = *addr as usize;
                        continue;
                    }
                }
                Instruction::Jf(addr) => {
                    let first = self.ctx.pop_value()?;

                    if !first.as_bool_inner()? {
                        *ip = *addr as usize;
                        continue;
                    }
                }
                Instruction::Call(gidx, argn) => {
                    let func = &co.idents[*gidx as usize];
                    let other_co = self.ctx.lookup_code_object(func)?;

                    self.ctx.push_frame(*argn);
                    other_co.run(self)?;
                    self.ctx.pop_frame();
                }
                Instruction::LCall(gidx, argn) => {
                    let (_, off) = co
                        .entries
                        .iter()
                        .find(|(iidx, _)| *iidx == *gidx as usize)
                        .unwrap_or_else(|| todo!());

                    self.ctx.push_frame(*argn);
                    self.run_bytecode(co, *off)?;
                    self.ctx.pop_frame();
                }
                Instruction::Ret => break,
                Instruction::Interrupt(n) => {
                    if let Some(func) = &self.interrupts[*n as usize] {
                        func.clone()(self)?;
                    }
                }
                Instruction::Conv(tid) => {
                    let ty = LV2ValueType::from_raw(*tid)?;

                    self.ctx.last_value_mut()?.cast_inplace(ty)?;
                }
                Instruction::Import | Instruction::NImport => {
                    let name = self.ctx.pop_value()?;
                    let name = name.as_str_inner()?;
                    let namespaced = *inx == Instruction::NImport;
                    // path to the modules source code
                    let relative_to = if let Some(mname) = co.module() {
                        self.modules
                            .get(&mname)
                            .and_then(|module| module.location())
                            .map(String::to_string)
                    } else {
                        None
                    };

                    self.add_module_by_name(name.as_ref(), relative_to, namespaced)?;
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
                Instruction::IterCreate => vm_iter_create(self)?,
                Instruction::IterCreateRanged => vm_iter_create_ranged(self)?,
                Instruction::IterHasNext => vm_iter_has_next(self)?,
                Instruction::IterNext => vm_iter_next(self)?,
                Instruction::IterReverse => vm_iter_reverse(self)?,
            }

            *ip += 1;
        }

        Ok(())
    }

    /// implementation of lovm2 bytecode behavior
    ///
    /// **Note:** This function does not push a stack frame and could therefore mess up local variables
    /// if not handled correctly. See [Vm::run_object].
    pub fn run_bytecode(&mut self, co: &LV2CodeObject, offset: usize) -> LV2Result<()> {
        let mut ip = offset;

        if let Err(mut e) = self.run_bytecode_inner(co, &mut ip) {
            e.inx_offsets.push(ip);
            return Err(e);
        }

        Ok(())
    }

    /// Register a new callback function that is used for patching function names.
    pub fn set_import_hook<T>(&mut self, hook: T)
    where
        T: Fn(Option<&str>, &str) -> LV2Result<Option<String>> + 'static,
    {
        self.import_hook = Rc::new(hook);
    }

    /// Register a new callback function that is used for resolving dependencies at runtime.
    pub fn set_load_hook<T>(&mut self, hook: T)
    where
        T: Fn(&LoadRequest) -> LV2Result<Option<LV2Module>> + Sized + 'static,
    {
        self.load_hook = Some(Rc::new(hook));
    }

    /// Gegister a new callback function on interrupt `n`.
    pub fn set_interrupt<T>(&mut self, n: u16, func: T) -> LV2Result<()>
    where
        T: Fn(&mut Vm) -> LV2Result<()> + Sized + 'static,
    {
        if n != LOVM2_INT_DEBUG && n <= LOVM2_RESERVED_INTERRUPTS {
            return err_reserved_interrupt(n);
        }

        self.interrupts[n as usize] = Some(Rc::new(func));

        Ok(())
    }
}

fn create_slice(target: LV2Value, start: LV2Value, end: LV2Value) -> LV2Result<LV2Value> {
    let start_idx = match start {
        LV2Value::Nil => 0,
        _ => start.as_integer_inner()?,
    };

    let end_idx = match end {
        LV2Value::Nil => target.len()? as i64,
        _ => end.as_integer_inner()?,
    };

    let mut slice = vec![];

    for idx in start_idx..end_idx {
        slice.push(target.get(&LV2Value::from(idx))?);
    }

    Ok(box_value(LV2Value::List(slice)))
}

/// Search for a name inside the given directories. This strips the file extension
/// and compares the file stem.
pub fn find_module(name: &str, load_paths: &[String]) -> LV2Result<String> {
    use std::fs::read_dir;

    for path in load_paths.iter() {
        if let Ok(dir) = read_dir(path) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let fname = entry.path();

                    if fname.file_stem().unwrap() == name && LV2Module::is_loadable(&fname)? {
                        let abspath = std::fs::canonicalize(fname).unwrap();
                        let abspath = abspath.to_string_lossy();

                        return Ok(abspath.into_owned());
                    }
                }
            }
        }
    }

    Err((LV2ErrorTy::ModuleNotFound, name).into())
}

/// Takes a [LoadRequest] and calls [find_module] to search for possible candidates
/// relative to the current module.
pub fn find_candidate(req: &LoadRequest) -> LV2Result<String> {
    if let Some(relative_to) = &req.relative_to {
        let path = std::path::Path::new(relative_to)
            .parent()
            .unwrap()
            .display()
            .to_string();

        find_module(&req.module, &[path])
    } else {
        Err((LV2ErrorTy::ModuleNotFound, &req.module).into())
    }
}

#[inline]
fn default_import_hook(module: Option<&str>, name: &str) -> LV2Result<Option<String>> {
    let name = match module {
        Some(module) => format!("{}.{}", module, name),
        _ => name.to_string(),
    };

    Ok(Some(name))
}
