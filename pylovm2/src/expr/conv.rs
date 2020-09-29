use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

use lovm2::prelude::*;

use super::{Lovm2Expr, Lovm2Value};
use crate::value::RuValue;
use crate::Expr;

pub fn any_to_expr(any: &PyAny) -> PyResult<Lovm2Expr> {
    match any.get_type().name().as_ref() {
        "str" | "bool" | "int" | "float" | "NoneType" => match any_to_value(any) {
            Ok(val) => Ok(val.into()),
            Err(e) => Err(e),
        },
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
    let ty = any.get_type().name();
    match ty.as_ref() {
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
                _ => RuntimeError::into(format!(
                    "value {} of type {} cannot be converted to value",
                    any, ty
                )),
            }
        }
        _ => RuntimeError::into(format!(
            "value {} of type {} cannot be converted to value",
            any, ty
        )),
    }
}

pub fn any_to_ruvalue(any: &PyAny) -> PyResult<RuValue> {
    use lovm2::value::instantiate;
    let ty = any.get_type().name();
    match ty.as_ref() {
        "RuValue" => any.extract::<RuValue>(),
        _ => {
            let val = any_to_value(any)?;
            Ok(RuValue::from_struct(instantiate(&val)))
        }
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

pub fn pyargs_to_exprs(args: &PyTuple) -> PyResult<Vec<Lovm2Expr>> {
    let mut v = vec![];
    for arg in args.iter() {
        v.push(any_to_expr(arg)?);
    }
    Ok(v)
}
