use json::{object::Object, JsonValue};

use lovm2::prelude::*;
use lovm2::value::box_value;
use lovm2_extend::prelude::*;

#[lovm2_function]
fn decode(json: String) -> Lovm2Result<Value> {
    json::parse(&json)
        .map_err(|e| Lovm2Error::from(e.to_string()))
        .and_then(|val| from_json_value(&val))
}

#[lovm2_function]
fn encode(val: Value) -> Lovm2Result<String> {
    let val = to_json_value(val)?;
    Ok(json::stringify(val))
}

fn from_json_value(val: &JsonValue) -> Lovm2Result<Value> {
    use std::convert::TryInto;
    let val = match val {
        JsonValue::Null => Value::Nil,
        JsonValue::Short(s) => Value::from(s.as_str().to_string()),
        JsonValue::String(s) => Value::from(s.to_string()),
        JsonValue::Number(n) => {
            let iparse: Result<i64, json::number::NumberOutOfScope> = (*n).try_into();
            if let Ok(n) = iparse {
                Value::from(n)
            } else {
                let n: f64 = (*n).into();
                Value::from(n)
            }
        }
        JsonValue::Boolean(b) => Value::from(*b),
        JsonValue::Object(obj) => {
            let mut dict = box_value(Value::dict());
            for (key, val) in obj.iter() {
                let key = Value::from(key);
                let val = from_json_value(val)?;
                dict.set(&key, val)?;
            }
            dict
        }
        JsonValue::Array(ls) => {
            let mut list = vec![];
            for val in ls.iter() {
                let val = from_json_value(val)?;
                list.push(val);
            }
            box_value(Value::List(list))
        }
    };
    Ok(val)
}

fn to_json_value(val: Value) -> Lovm2Result<JsonValue> {
    let json = match val {
        Value::Nil => JsonValue::Null,
        Value::Bool(b) => JsonValue::Boolean(b),
        Value::Int(n) => JsonValue::Number(n.into()),
        Value::Float(n) => JsonValue::Number(n.into()),
        Value::Str(s) => JsonValue::String(s),
        Value::Dict(d) => {
            let mut obj = Object::new();
            for (key, val) in d.into_iter() {
                let key = key.as_str_inner()?;
                let val = to_json_value(val)?;
                obj.insert(&key, val);
            }
            obj.into()
        }
        Value::List(ls) => {
            let mut json_ls = vec![];
            for val in ls.into_iter() {
                json_ls.push(to_json_value(val)?);
            }
            json_ls.into()
        }
        Value::Ref(r) => to_json_value(r.unref_total()?)?,
        _ => {
            return Err(Lovm2Error::from(format!(
                "{:?} not supported for json",
                val
            )))
        }
    };
    Ok(json)
}

lovm2_module_init!();
