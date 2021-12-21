mod conv;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

//use lovm2::prelude::Operator2;
//use lovm2::Variable;

pub use self::conv::*;

macro_rules! auto_wrapper {
    ($method_name:ident, $($arg:expr),*) => {
        Ok(Self {
            inner: lovm2::prelude::LV2Expr::$method_name(
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
            inner: lovm2::prelude::LV2Expr::from_opn(
                $op,
                args
            ),
        })
    }};
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Expr {
    pub inner: lovm2::gen::LV2Expr,
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Expr {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

/*
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
*/

#[pymethods]
impl Expr {
    #[args(args = "*")]
    pub fn get(&mut self, _this: &PyAny, key: &PyAny) -> PyResult<Self> {
        let key = any_to_expr(key)?;

        Ok(Self {
            inner: self.inner.clone().get(key),
        })
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn call(_this: &PyAny, name: &PyAny, args: &PyTuple) -> PyResult<Self> {
        let name = lovm2::prelude::LV2Variable::from(name.str()?.to_string());
        let args = pyargs_to_exprs(args)?;

        Ok(Self {
            inner: lovm2::prelude::LV2Expr::Call(lovm2::prelude::LV2Call::with_args(name, args)),
        })
    }

    #[classmethod]
    pub fn slice(_this: &PyAny, target: &PyAny, start: &PyAny, end: &PyAny) -> PyResult<Self> {
        let target = any_to_expr(target)?;
        //let (mut start, mut end): (lovm2::prelude::LV2Expr, lovm2::prelude::LV2Expr) = (lovm2::value::LV2Value::Nil.into(), lovm2::value::LV2Value::Nil.into());
        let (start, end) = (any_to_expr(start)?, any_to_expr(end)?);

        let slice = target.slice(start, end);

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
            inner: lovm2::prelude::LV2Expr::from(lovm2::prelude::LV2Variable::from(name)),
        })
    }
}

#[pymethods]
impl Expr {
    #[classmethod]
    #[args(args = "*")]
    pub fn add(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Add, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn sub(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Sub, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn mul(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Mul, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn div(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Div, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn shl(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Shl, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn shr(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Shr, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn rem(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Rem, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn land(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::And, args)
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn lor(_this: &PyAny, args: &PyTuple) -> PyResult<Self> {
        auto_wrapper!(lovm2::prelude::LV2Operator2::Or, args)
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
        auto_wrapper!(lovm2::prelude::LV2Operator2::Pow, args)
    }
}

// conversion methods
#[pymethods]
impl Expr {
    pub fn to_bool(&mut self) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone().to_bool(),
        })
    }

    pub fn to_int(&mut self) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone().to_integer(),
        })
    }

    pub fn to_float(&mut self) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone().to_float(),
        })
    }

    pub fn to_iter(&mut self) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone().to_iter(),
        })
    }

    pub fn to_str(&mut self) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone().to_str(),
        })
    }
}

// iterator methods
#[pymethods]
impl Expr {
    #[classmethod]
    pub fn range(_this: &PyAny, from: &PyAny, to: Option<&PyAny>) -> PyResult<Self> {
        if let Some(to) = to {
            let (from, to) = (any_to_expr(from)?, any_to_expr(to)?);

            Ok(Expr {
                inner: lovm2::prelude::LV2Expr::iter_ranged(from, to).into(),
            })
        } else {
            let to = any_to_expr(from)?;

            Ok(Expr {
                inner: lovm2::prelude::LV2Expr::iter_ranged(lovm2::value::LV2Value::Nil, to).into(),
            })
        }
    }

    pub fn reverse(&mut self) -> PyResult<Self> {
        Ok(Expr {
            inner: self.inner.clone().reverse(),
        })
    }
}
