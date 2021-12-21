use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

//use lovm2::prelude::*;
//use lovm2::Variable;

use crate::value::Value;
use crate::Expr;

pub fn any_to_expr(any: &PyAny) -> PyResult<lovm2::prelude::LV2Expr> {
    match any.get_type().name().as_ref() {
        "dict" => {
            let dict = any.downcast::<PyDict>()?;
            let mut obj = lovm2::prelude::LV2Expr::dict();

            for (key, val) in dict.iter() {
                let (key, val) = (any_to_expr(key)?, any_to_expr(val)?);

                obj = obj.set(key, val);
            }

            Ok(obj.into())
        }
        "list" => {
            let list = any.downcast::<PyList>()?;
            let mut obj = lovm2::prelude::LV2Expr::list();

            for val in list.iter() {
                let val = any_to_expr(val)?;

                obj = obj.append(val);
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

pub fn any_to_ident(any: &PyAny) -> PyResult<lovm2::prelude::LV2Variable> {
    match any.get_type().name().as_ref() {
        "str" => {
            let name = any.str()?.to_string();

            Ok(lovm2::prelude::LV2Variable::from(name).into())
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;

            match &data.inner {
                lovm2::prelude::LV2Expr::Variable(var) => Ok(var.clone()),
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

pub fn any_to_value(any: &PyAny) -> PyResult<lovm2::value::LV2Value> {
    let ty = any.get_type().name();
    match ty.as_ref() {
        "str" => {
            let data = any.str()?.to_string();

            Ok(lovm2::value::LV2Value::Str(data))
        }
        "bool" => {
            let data = any.extract::<bool>()?;

            Ok(lovm2::value::LV2Value::Bool(data))
        }
        "int" => {
            let data = any.extract::<i64>()?;

            Ok(lovm2::value::LV2Value::Int(data))
        }
        "float" => {
            let data = any.extract::<f64>()?;

            Ok(lovm2::value::LV2Value::Float(data))
        }
        "list" => {
            let mut ls = vec![];

            for item in any.iter()? {
                ls.push(any_to_value(item?)?);
            }

            Ok(lovm2::value::LV2Value::List(ls))
        }
        "dict" => {
            let dict = any.downcast::<PyDict>()?;
            let mut map = lovm2::value::LV2Value::dict();

            for (key, value) in dict.iter() {
                let (key, value) = (any_to_value(key)?, any_to_value(value)?);

                map.set(&key, value).unwrap();
            }

            Ok(map)
        }
        "NoneType" => Ok(lovm2::value::LV2Value::Nil),
        "Value" => {
            let data = any.extract::<Value>()?;

            Ok(lovm2::value::LV2Value::Ref(data.inner))
        }
        "Expr" => {
            let data = any.extract::<Expr>()?;

            match data.inner {
                lovm2::prelude::LV2Expr::Value { val, .. } => Ok(val),
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
        stringify!(Value) => any.extract::<Value>(),
        _ => any_to_value(any).map(Value::from_struct),
    }
}

/*
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
*/

pub fn pyargs_to_exprs(args: &PyTuple) -> PyResult<Vec<lovm2::prelude::LV2Expr>> {
    args.iter().map(any_to_expr).collect()
}
