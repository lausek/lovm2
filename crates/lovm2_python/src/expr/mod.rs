mod conv;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

pub use self::conv::*;

macro_rules! auto_implement {
    ($operator:ident, $method:ident) => {
        #[pymethods]
        impl LV2Expr {
            #[args(args = "*")]
            pub fn $method(&self, args: &PyTuple) -> PyResult<Self> {
                auto_wrapper!(self.clone(), lovm2::prelude::LV2Operator2::$operator, args)
            }
        }
    };
}

macro_rules! auto_wrapper {
    ($self:expr, $op:path, $args:expr) => {{
        let mut args = vec![];

        for arg in $args.iter() {
            args.push(any_to_expr(arg)?)
        }

        Ok(Self {
            inner: $self.inner.expand_op($op, args),
        })
    }};
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LV2Expr {
    pub inner: lovm2::gen::LV2Expr,
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for LV2Expr {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

#[pymethods]
impl LV2Expr {
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

auto_implement!(Add, add);
auto_implement!(Sub, sub);
auto_implement!(Mul, mul);
auto_implement!(Div, div);
auto_implement!(Shl, shl);
auto_implement!(Shr, shr);
auto_implement!(Rem, rem);
auto_implement!(And, and_);
auto_implement!(Or, or_);
auto_implement!(XOr, xor);
auto_implement!(Eq, eq);
auto_implement!(Ne, ne);
auto_implement!(Ge, ge);
auto_implement!(Gt, gt);
auto_implement!(Le, le);
auto_implement!(Lt, lt);
auto_implement!(Pow, pow);

#[pymethods]
impl LV2Expr {
    #[new]
    pub fn new(from: &PyAny) -> PyResult<Self> {
        any_to_expr(from).map(|inner| Self { inner })
    }

    pub fn not_(&self) -> Self {
        Self {
            inner: self.inner.clone().not(),
        }
    }
}

// conversion methods
#[pymethods]
impl LV2Expr {
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
impl LV2Expr {
    #[classmethod]
    pub fn range(_this: &PyAny, from: &PyAny, to: Option<&PyAny>) -> PyResult<Self> {
        if let Some(to) = to {
            let (from, to) = (any_to_expr(from)?, any_to_expr(to)?);

            Ok(LV2Expr {
                inner: lovm2::prelude::LV2Expr::iter_ranged(from, to).into(),
            })
        } else {
            let to = any_to_expr(from)?;

            Ok(LV2Expr {
                inner: lovm2::prelude::LV2Expr::iter_ranged(lovm2::value::LV2Value::Nil, to).into(),
            })
        }
    }

    pub fn has_next(&mut self) -> PyResult<Self> {
        Ok(LV2Expr {
            inner: self.inner.clone().has_next(),
        })
    }

    pub fn next(&mut self) -> PyResult<Self> {
        Ok(LV2Expr {
            inner: self.inner.clone().next(),
        })
    }

    pub fn reverse(&mut self) -> PyResult<Self> {
        Ok(LV2Expr {
            inner: self.inner.clone().reverse(),
        })
    }
}
