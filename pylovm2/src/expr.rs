use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use lovm2::var;

type Lovm2Expr = lovm2::hir::expr::Expr;
type Lovm2Value = lovm2::value::CoValue;

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
    match any.get_type().name().as_ref() {
        "str" | "bool" | "int" | "float" | "list" | "dict" | "NoneType" => {
            match any_to_value(any) {
                Ok(val) => Ok(val.into()),
                Err(e) => Err(e),
            }
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;
            Ok(data.inner)
        }
        _ => RuntimeError::into(format!("value {} cannot be converted to expression", any)),
    }
}

pub fn any_to_ident(any: &PyAny) -> PyResult<var::Variable> {
    match any.get_type().name().as_ref() {
        "str" => {
            let name = any.str().unwrap().to_string()?;
            Ok(var::Variable::from(name.to_string()).into())
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;
            match &data.inner {
                Lovm2Expr::Variable(var) => Ok(var.clone()),
                _ => RuntimeError::into("expression is not an identifier".to_string()),
            }
        }
        _ => RuntimeError::into(format!("value {} cannot be converted to identifier", any)),
    }
}

pub fn any_to_value(any: &PyAny) -> PyResult<Lovm2Value> {
    match any.get_type().name().as_ref() {
        "str" => {
            let data = any.str().unwrap().to_string()?;
            Ok(Lovm2Value::Str(data.to_string()))
        }
        "bool" => {
            let data = any.extract::<bool>()?;
            Ok(Lovm2Value::Bool(data))
        }
        "int" => {
            let data = any.extract::<i64>()?;
            Ok(Lovm2Value::Int(data))
        }
        "float" => {
            let data = any.extract::<f64>()?;
            Ok(Lovm2Value::Float(data))
        }
        "list" => {
            let mut ls = vec![];
            for item in any.iter()? {
                let item = item?;
                ls.push(any_to_value(item)?);
            }
            Ok(Lovm2Value::List(ls).into())
        }
        "dict" => {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            let dict = any.downcast::<PyDict>()?;
            for (key, value) in dict.iter() {
                let (key, value) = (any_to_value(key)?, any_to_value(value)?);
                map.insert(key, Box::new(value));
            }
            Ok(Lovm2Value::Dict(map).into())
        }
        "NoneType" => Ok(Lovm2Value::Nil),
        "Expr" => {
            let data = any.extract::<Expr>()?;
            match data.inner {
                Lovm2Expr::Value(val) => Ok(val),
                _ => RuntimeError::into(format!("value {} cannot be converted to value", any)),
            }
        }
        _ => RuntimeError::into(format!("value {} cannot be converted to value", any)),
    }
}

pub fn any_to_wpos(any: &PyAny) -> PyResult<Lovm2Expr> {
    match any.get_type().name().as_ref() {
        "Expr" => {
            let data = any.extract::<Expr>()?;
            if let Lovm2Expr::Access(var) = &data.inner {
                Ok(var.clone().into())
            } else {
                any_to_ident(any).map(|i| Lovm2Expr::from(i))
            }
        }
        _ => any_to_ident(any).map(|i| Lovm2Expr::from(i)),
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Expr {
    pub inner: lovm2::hir::expr::Expr,
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Expr {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

#[pymethods]
impl Expr {
    #[classmethod]
    #[args(args = "*")]
    pub fn access(_this: &PyAny, name: &PyAny, args: &PyTuple) -> PyResult<Self> {
        use lovm2::prelude::*;
        let name = any_to_ident(name)?;
        let args = args.iter().map(|arg| any_to_expr(arg).unwrap()).collect();
        Ok(Self {
            inner: Lovm2Expr::Access(Access::new(name, args)),
        })
    }

    #[classmethod]
    #[args(args = "*")]
    pub fn call(_this: &PyAny, name: &PyAny, args: &PyTuple) -> PyResult<Self> {
        use lovm2::prelude::*;
        let name = Variable::from(name.str().unwrap().to_string().unwrap().to_string());
        let args = args
            .into_iter()
            .map(|arg| any_to_expr(arg).unwrap())
            .collect();
        Ok(Self {
            inner: Lovm2Expr::Call(Call::with_args(name, args)),
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
            inner: Lovm2Expr::Variable(var::Variable::from(name)),
        })
    }
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
