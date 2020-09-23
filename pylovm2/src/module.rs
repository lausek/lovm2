use std::collections::HashMap;
use std::rc::Rc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use lovm2::hir;
use lovm2::module;
use lovm2::var;

use crate::code::CodeObject;
use crate::expr::{any_to_expr, any_to_wpos, Expr};

type Lovm2Branch = lovm2::hir::branch::Branch;
type Lovm2Block = lovm2::hir::block::Block;
type Lovm2Module = lovm2::module::Module;

#[pyclass]
pub struct Module {
    pub inner: Option<Box<dyn module::ModuleProtocol>>,
}

impl Module {
    pub fn from(inner: Lovm2Module) -> Self {
        Self {
            inner: Some(inner.into()),
        }
    }
}

#[pymethods]
impl Module {
    #[classmethod]
    pub fn load(_this: &PyAny, path: &PyAny) -> PyResult<Self> {
        let path = path.str()?.to_string()?;
        match Lovm2Module::load_from_file(path.as_ref()) {
            Ok(inner) => Ok(Self { inner: Some(inner) }),
            Err(err) => TypeError::into(err),
        }
    }

    pub fn save(&self, path: String) -> PyResult<()> {
        if let Some(inner) = self.inner.as_ref() {
            return match inner.store_to_file(&path) {
                Ok(_) => Ok(()),
                Err(err) => TypeError::into(err),
            };
        }
        TypeError::into("inner module not loaded")
    }

    pub fn uses(&self) -> PyResult<Vec<String>> {
        // TODO: implement this
        Ok(vec![])
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Module {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner.as_ref().unwrap()))
    }
}

#[pyclass]
pub struct ModuleBuilder {
    slots: HashMap<String, Py<ModuleBuilderSlot>>,
}

#[pymethods]
impl ModuleBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn add(&mut self, py: Python, name: String) -> Py<ModuleBuilderSlot> {
        let inst = Py::new(py, ModuleBuilderSlot::new()).unwrap();
        self.slots.insert(name.clone(), inst);
        self.slots.get(&name).unwrap().clone_ref(py)
    }

    // TODO: can we avoid duplicating the code here?
    pub fn build(&mut self, py: Python) -> PyResult<Module> {
        let mut module = Lovm2Module::new();

        for (key, co_builder) in self.slots.drain() {
            let mut co_builder: PyRefMut<ModuleBuilderSlot> = co_builder.as_ref(py).borrow_mut();
            match co_builder.complete() {
                Ok(co) => {
                    module.slots.insert(var::Variable::from(key), Rc::new(co));
                }
                Err(msg) => return Err(msg),
            }
        }

        Ok(Module::from(module))
    }

    pub fn entry(&mut self, py: Python) -> Py<ModuleBuilderSlot> {
        let name = lovm2::module::ENTRY_POINT.to_string();
        if !self.slots.contains_key(&name) {
            let inst = Py::new(py, ModuleBuilderSlot::new()).unwrap();
            self.slots.insert(name.clone(), inst);
        }
        self.slots.get(&name).unwrap().clone_ref(py)
    }
}

enum ModuleBuilderSlotInner {
    Lovm2Hir(Option<hir::HIR>),
    PyFn(Option<PyObject>),
}

#[pyclass]
pub struct ModuleBuilderSlot {
    inner: ModuleBuilderSlotInner,
}

#[pymethods]
impl ModuleBuilderSlot {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: ModuleBuilderSlotInner::Lovm2Hir(Some(hir::HIR::new())),
        }
    }

    pub fn args(&mut self, args: &PyList) {
        if let ModuleBuilderSlotInner::Lovm2Hir(ref mut hir) = self.inner {
            use lovm2::var::Variable;
            let args = args
                .iter()
                .map(|name| {
                    let name = name.str().unwrap().to_string().unwrap().to_string();
                    Variable::from(name)
                })
                .collect();
            hir.replace(hir::HIR::with_args(args));
        } else {
            unimplemented!()
        }
    }

    pub fn code(&mut self) -> PyResult<BlockBuilder> {
        if let ModuleBuilderSlotInner::Lovm2Hir(ref mut hir) = self.inner {
            let hir = hir.as_mut().unwrap();
            let inner = &mut hir.code as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        } else {
            unimplemented!()
        }
    }

    // TODO: can we use consuming self here?
    pub fn complete(&mut self) -> PyResult<CodeObject> {
        match &mut self.inner {
            ModuleBuilderSlotInner::Lovm2Hir(ref mut hir) => {
                if let Some(hir) = hir.take() {
                    return match hir.build() {
                        Ok(co) => Ok(CodeObject::from(co)),
                        Err(msg) => TypeError::into(msg),
                    };
                }
                TypeError::into("hir was already built")
            }
            ModuleBuilderSlotInner::PyFn(ref mut pyfn) => {
                Ok(CodeObject::from(pyfn.take().unwrap()))
            }
        }
    }

    pub fn pyfn(&mut self, pyfn: PyObject) -> PyResult<()> {
        self.inner = ModuleBuilderSlotInner::PyFn(Some(pyfn));
        Ok(())
    }
}

#[pyclass]
pub struct BranchBuilder {
    inner: *mut Lovm2Branch,
}

#[pymethods]
impl BranchBuilder {
    pub fn add_condition(&mut self, condition: &PyAny) -> PyResult<BlockBuilder> {
        let condition = any_to_expr(condition)?;
        unsafe {
            let inner = (*self.inner).add_condition(condition) as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn default_condition(&mut self) -> PyResult<BlockBuilder> {
        unsafe {
            let inner = (*self.inner).default_condition() as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }
}

#[pyclass]
pub struct BlockBuilder {
    inner: *mut Lovm2Block,
}

#[pymethods]
impl BlockBuilder {
    pub fn assign(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Assign::local(any_to_wpos(n)?, any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn assign_global(&mut self, n: &PyAny, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Assign::global(any_to_wpos(n)?, any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn branch(&mut self) -> PyResult<BranchBuilder> {
        unsafe {
            let inner = (*self.inner).branch() as *mut Lovm2Branch;
            Ok(BranchBuilder { inner })
        }
    }

    #[args(args = "*")]
    pub fn call(&mut self, name: String, args: &PyTuple) -> PyResult<()> {
        use lovm2::prelude::*;
        let mut call = Call::new(name);
        for arg in args.into_iter() {
            call = call.arg(any_to_expr(arg)?);
        }
        unsafe {
            (*self.inner).push(call);
        }
        Ok(())
    }

    pub fn expr(&mut self, expr: &Expr) -> PyResult<()> {
        match &expr.inner {
            hir::expr::Expr::Call(call) => unsafe {
                (*self.inner).push(call.clone());
                Ok(())
            },
            _ => RuntimeError::into("expression cannot be placed here.".to_string()),
        }
    }

    pub fn interrupt(&mut self, id: u16) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Interrupt::new(id));
        }
        Ok(())
    }

    pub fn repeat(&mut self) -> PyResult<BlockBuilder> {
        unsafe {
            let repeat = (*self.inner).repeat(None);
            let inner = &mut repeat.block as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn repeat_break(&mut self) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Break::new());
        }
        Ok(())
    }

    pub fn repeat_continue(&mut self) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Continue::new());
        }
        Ok(())
    }

    pub fn repeat_until(&mut self, condition: &PyAny) -> PyResult<BlockBuilder> {
        let condition = any_to_expr(condition)?;
        unsafe {
            let repeat = (*self.inner).repeat(Some(condition));
            let inner = &mut repeat.block as *mut Lovm2Block;
            Ok(BlockBuilder { inner })
        }
    }

    pub fn ret(&mut self, val: &PyAny) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            let val = any_to_expr(val)?;
            (*self.inner).push(Return::value(val));
        }
        Ok(())
    }
}
