use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

use lovm2::prelude::*;
use lovm2::Variable;

use crate::lv2::*;
use crate::value::Value;
use crate::Expr;

pub fn any_to_expr(any: &PyAny) -> PyResult<Lovm2Expr> {
    match any.get_type().name().as_ref() {
        "dict" => {
            let dict = any.downcast::<PyDict>()?;
            let mut obj = Initialize::dict();

            for (key, val) in dict.iter() {
                let (key, val) = (any_to_expr(key)?, any_to_expr(val)?);

                obj.add_by_key(key, val);
            }

            Ok(obj.into())
        }
        "list" => {
            let list = any.downcast::<PyList>()?;
            let mut obj = Initialize::list();

            for val in list.iter() {
                let val = any_to_expr(val)?;

                obj.add(val);
            }

            Ok(obj.into())
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;

            Ok(data.inner)
        }
        _ => {
            if let Ok(val) = any_to_value(any) {
                return Ok(val.into());
            }

            Err(PyRuntimeError::new_err(format!(
                "value {} cannot be converted to expression",
                any
            )))
        }
    }
}

pub fn any_to_ident(any: &PyAny) -> PyResult<Variable> {
    match any.get_type().name().as_ref() {
        "str" => {
            let name = any.str()?.to_string();

            Ok(Variable::from(name).into())
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;

            match &data.inner {
                Lovm2Expr::Variable(var) => Ok(var.clone()),
                _ => Err(PyRuntimeError::new_err(
                    "expression is not an identifier".to_string(),
                )),
            }
        }
        _ => Err(PyRuntimeError::new_err(format!(
            "value {} cannot be converted to identifier",
            any
        ))),
    }
}

pub fn any_to_value(any: &PyAny) -> PyResult<Lovm2ValueRaw> {
    let ty = any.get_type().name();
    match ty.as_ref() {
        "str" => {
            let data = any.str()?.to_string();

            Ok(Lovm2ValueRaw::Str(data))
        }
        "bool" => {
            let data = any.extract::<bool>()?;

            Ok(Lovm2ValueRaw::Bool(data))
        }
        "int" => {
            let data = any.extract::<i64>()?;

            Ok(Lovm2ValueRaw::Int(data))
        }
        "float" => {
            let data = any.extract::<f64>()?;

            Ok(Lovm2ValueRaw::Float(data))
        }
        "list" => {
            let mut ls = vec![];

            for item in any.iter()? {
                ls.push(any_to_value(item?)?);
            }

            Ok(Lovm2ValueRaw::List(ls))
        }
        "dict" => {
            let dict = any.downcast::<PyDict>()?;
            let mut map = Lovm2ValueRaw::dict();

            for (key, value) in dict.iter() {
                let (key, value) = (any_to_value(key)?, any_to_value(value)?);

                map.set(&key, value).unwrap();
            }

            Ok(map)
        }
        "NoneType" => Ok(Lovm2ValueRaw::Nil),
        "Value" => {
            let data = any.extract::<Value>()?;

            Ok(lovm2::value::Value::Ref(data.inner))
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;

            match data.inner {
                Lovm2Expr::Value { val, .. } => Ok(val),
                _ => Err(PyRuntimeError::new_err(format!(
                    "value {} of type {} cannot be converted to value",
                    any, ty
                ))),
            }
        }
        _ => Err(PyRuntimeError::new_err(format!(
            "value {} of type {} cannot be converted to value",
            any, ty
        ))),
    }
}

pub fn any_to_pylovm2_value(any: &PyAny) -> PyResult<Value> {
    let ty = any.get_type().name();

    match ty.as_ref() {
        "Value" => any.extract::<Value>(),
        _ => any_to_value(any).map(Value::from_struct),
    }
}

pub fn any_to_access(any: &PyAny) -> PyResult<Lovm2Access> {
    match any.get_type().name().as_ref() {
        "Expr" => {
            let data = any.extract::<Expr>()?;

            if let Lovm2Expr::Access(var) = &data.inner {
                Ok(var.clone().into())
            } else {
                any_to_ident(any).map(Lovm2Access::from)
            }
        }
        _ => any_to_ident(any).map(Lovm2Access::from),
    }
}

pub fn pyargs_to_exprs(args: &PyTuple) -> PyResult<Vec<Lovm2Expr>> {
    args.iter().map(any_to_expr).collect()
}
