use std::collections::HashMap;
use std::rc::Rc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use lovm2::hir;
use lovm2::module;
use lovm2::var;

use crate::code::CodeObject;
use crate::expr::any_to_expr;

type Lovm2Block = lovm2::block::Block;
type Lovm2Branch = lovm2::branch::Branch;

#[pyclass]
pub struct Module {
    pub inner: Option<module::Module>,
}

impl Module {
    pub fn from(inner: module::Module) -> Self {
        Self { inner: Some(inner) }
    }
}

#[pymethods]
impl Module {
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.as_ref().unwrap())
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

    pub fn build(&mut self, py: Python) -> PyResult<Module> {
        let mut module = module::Module::new();

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
}

#[pyclass]
pub struct ModuleBuilderSlot {
    inner: Option<hir::HIR>,
}

#[pymethods]
impl ModuleBuilderSlot {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Some(hir::HIR::new()),
        }
    }

    pub fn code(&mut self) -> PyResult<BlockBuilder> {
        let hir = self.inner.as_mut().unwrap();
        let inner = &mut hir.code as *mut Lovm2Block;
        Ok(BlockBuilder { inner })
    }

    pub fn complete(&mut self) -> PyResult<CodeObject> {
        if let Some(hir) = self.inner.take() {
            return match hir.build() {
                Ok(co) => Ok(CodeObject::from(co)),
                Err(msg) => TypeError::into(msg),
            };
        }
        TypeError::into("hir was already built")
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
    pub fn assign(&mut self, n: String, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Assign::local(var::Variable::from(n), any_to_expr(expr)?));
        }
        Ok(())
    }

    pub fn assign_global(&mut self, n: String, expr: &PyAny) -> PyResult<()> {
        // TODO: allow usage of Expr::Variable here
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Assign::global(var::Variable::from(n), any_to_expr(expr)?));
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

    pub fn interrupt(&mut self, id: u16) -> PyResult<()> {
        use lovm2::prelude::*;
        unsafe {
            (*self.inner).push(Interrupt::new(id));
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
}
