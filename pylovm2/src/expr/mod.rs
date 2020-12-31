mod conv;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

use lovm2::prelude::{Conv, Iter, Operator2};
use lovm2::Variable;

pub use self::conv::*;
use crate::lv2::*;

macro_rules! auto_wrapper {
    ($method_name:ident, $($arg:expr),*) => {
        Ok(Self {
            inner: Lovm2Expr::$method_name(
                $( any_to_expr($arg)? ),*
            ),
        })
    };
    ($op:path, $args:expr) => {{
        let mut args = vec![];
        for arg in $args.iter() {
            args.push(any_to_expr(arg)?)
        }
        Ok(Self {
            inner: Lovm2Expr::from_opn(
                $op,
                args
            ),
        })
    }};
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Expr {
    pub inner: lovm2::gen::Expr,
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Expr {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

#[pymethods]
impl Expr {
    pub fn ty(&self) -> PyResult<String> {
        let name = match &self.inner {
            Lovm2Expr::Access(_) => "access",
            _ => "none",
        };
        Ok(name.to_string())
    }
}

#[pymethods]
impl Expr {
    #[classmethod]
    #[args(args = "*")]
    pub fn access(_this: &PyAny, name: &PyAny, args: &PyTuple) -> PyResult<Self> {
        use lovm2::prelude::*;
        let name = any_to_ident(name)?;
        let args = pyargs_to_exprs(args)?;
        Ok(Self {
            inner: Lovm2Expr::Access(Access::new(name, args)),
        })
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn call(_this: &PyAny, name: &PyAny, args: &PyTuple) -> PyResult<Self> {
        use lovm2::prelude::*;
        let name = Variable::from(name.str()?.to_string());
        let args = pyargs_to_exprs(args)?;
        Ok(Self {
            inner: Lovm2Expr::Call(Call::with_args(name, args)),
        })
    }

    #[classmethod]
    pub fn slice(_this: &PyAny, target: &PyAny, start: &PyAny, end: &PyAny) -> PyResult<Self> {
        use lovm2::prelude::*;
        let target = any_to_expr(target)?;
        let mut slice = Slice::new(target);
        if !start.is_none() {
            slice = slice.start(any_to_expr(start)?);
        }
        if !end.is_none() {
            slice = slice.end(any_to_expr(end)?);
        }
        Ok(Self {
            inner: slice.into(),
        })
    }

    #[classmethod]
    pub fn val(_this: &PyAny, arg: &PyAny) -> PyResult<Self> {
        Ok(Self {
            inner: any_to_expr(arg)?,
        })
    }

    #[classmethod]
    pub fn var(_this: &PyAny, arg: &PyAny) -> PyResult<Self> {
        let name = arg.to_string();
        Ok(Self {
            inner: Lovm2Expr::Variable(Variable::from(name)),
        })
    }
}

#[pymethods]
impl Expr {
    #[classmethod]
    #[args(args = "*")]
    pub fn add(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::Add, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn sub(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::Sub, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn mul(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::Mul, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn div(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::Div, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn rem(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::Rem, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn land(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::And, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn lor(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::Or, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn lnot(_this: &PyAny, arg1: &PyAny) -> PyResult<Self> {
        auto_wrapper!(not, arg1)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn eq(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(eq, arg1, arg2)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn ne(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(ne, arg1, arg2)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn ge(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(ge, arg1, arg2)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn gt(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(gt, arg1, arg2)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn le(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(le, arg1, arg2)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn lt(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(lt, arg1, arg2)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn pow(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(Operator2::Pow, args)
    }
}

// conversion methods
#[pymethods]
impl Expr {
    pub fn to_bool(&self) -> PyResult<Self> {
        Ok(Self {
            inner: Conv::to_bool(self.inner.clone()).into(),
        })
    }

    pub fn to_int(&self) -> PyResult<Self> {
        Ok(Self {
            inner: Conv::to_integer(self.inner.clone()).into(),
        })
    }

    pub fn to_float(&self) -> PyResult<Self> {
        Ok(Self {
            inner: Conv::to_float(self.inner.clone()).into(),
        })
    }

    pub fn to_str(&self) -> PyResult<Self> {
        Ok(Self {
            inner: Conv::to_str(self.inner.clone()).into(),
        })
    }
}

// iterator methods
#[pymethods]
impl Expr {
    #[classmethod]
    pub fn iter(_this: &PyAny, from: &PyAny) -> PyResult<Self> {
        let from = any_to_expr(from)?;
        Ok(Expr {
            inner: Iter::create(from).into(),
        })
    }

    #[classmethod]
    pub fn range(_this: &PyAny, from: &PyAny, to: Option<&PyAny>) -> PyResult<Self> {
        if let Some(to) = to {
            let (from, to) = (any_to_expr(from)?, any_to_expr(to)?);
            Ok(Expr {
                inner: Iter::create_ranged(from, to).into(),
            })
        } else {
            let to = any_to_expr(from)?;
            Ok(Expr {
                inner: Iter::create_ranged(Lovm2ValueRaw::Nil, to).into(),
            })
        }
    }

    pub fn reverse(&mut self) -> PyResult<Self> {
        match &self.inner {
            Lovm2Expr::Iter(it) => Ok(Expr {
                inner: Iter::reverse(it.clone()).into(),
            }),
            _ => todo!(),
        }
    }
}
