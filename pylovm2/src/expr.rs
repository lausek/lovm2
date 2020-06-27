use pyo3::exceptions::*;
use pyo3::prelude::*;

use lovm2::expr;

type Lovm2Expr = lovm2::hir::expr::Expr;

macro_rules! auto_wrapper {
    ($method_name:ident, $($arg:expr),*) => {
        Ok(Self {
            inner: Lovm2Expr::$method_name(
                $( any_to_expr($arg)? ),*
            ),
        })
    };
}

pub fn any_to_expr(any: &PyAny) -> PyResult<Lovm2Expr> {
    use lovm2::value::CoValue;

    match any.get_type().name().as_ref() {
        "str" => {
            let data = any.str().unwrap().to_string()?;
            Ok(Lovm2Expr::Value(CoValue::Str(data.to_string())))
        }
        "bool" => {
            let data = any.extract::<bool>()?;
            Ok(Lovm2Expr::Value(CoValue::Bool(data)))
        }
        "int" => {
            let data = any.extract::<i64>()?;
            Ok(Lovm2Expr::Value(CoValue::Int(data)))
        }
        "float" => {
            let data = any.extract::<f64>()?;
            Ok(Lovm2Expr::Value(CoValue::Float(data)))
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;
            Ok(data.inner)
        }
        /*
        "list" => {}
        "dict" => {}
        */
        name => TypeError::into(format!(
            "value of type {} cannot be converted to expression",
            name
        )),
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Expr {
    pub inner: expr::Expr,
}

#[pymethods]
impl Expr {
    #[classmethod]
    pub fn add(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(add, arg1, arg2)
    }

    #[classmethod]
    pub fn sub(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(sub, arg1, arg2)
    }

    #[classmethod]
    pub fn mul(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(mul, arg1, arg2)
    }

    #[classmethod]
    pub fn div(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(div, arg1, arg2)
    }

    #[classmethod]
    pub fn rem(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(rem, arg1, arg2)
    }

    #[classmethod]
    pub fn land(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(and, arg1, arg2)
    }

    #[classmethod]
    pub fn lor(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(or, arg1, arg2)
    }

    #[classmethod]
    pub fn lnot(_this: &PyAny, arg1: &PyAny) -> PyResult<Self> {
        auto_wrapper!(not, arg1)
    }

    #[classmethod]
    pub fn eq(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(eq, arg1, arg2)
    }

    #[classmethod]
    pub fn ne(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(ne, arg1, arg2)
    }

    #[classmethod]
    pub fn ge(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(ge, arg1, arg2)
    }

    #[classmethod]
    pub fn gt(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(gt, arg1, arg2)
    }

    #[classmethod]
    pub fn le(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(le, arg1, arg2)
    }

    #[classmethod]
    pub fn lt(_this: &PyAny, arg1: &PyAny, arg2: &PyAny) -> PyResult<Self> {
        auto_wrapper!(lt, arg1, arg2)
    }
}