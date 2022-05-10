use ::json::{object::Object, JsonValue};

use super::*;

#[lv2_function]
fn decode(json: String) -> LV2Result<LV2Value> {
    ::json::parse(&json)
        .or_else(err_from_string)
        .and_then(|val| from_json_value(&val))
}

#[lv2_function]
fn encode(val: LV2Value) -> LV2Result<String> {
    let val = to_json_value(val)?;

    Ok(::json::stringify(val))
}

fn from_json_value(val: &JsonValue) -> LV2Result<LV2Value> {
    let val = match val {
        JsonValue::Null => LV2Value::Nil,
        JsonValue::Short(s) => LV2Value::from(s.as_str().to_string()),
        JsonValue::String(s) => LV2Value::from(s.to_string()),
        JsonValue::Number(n) => {
            let iparse: Result<i64, ::json::number::NumberOutOfScope> = (*n).try_into();

            if let Ok(n) = iparse {
                LV2Value::from(n)
            } else {
                let n: f64 = (*n).into();
                LV2Value::from(n)
            }
        }
        JsonValue::Boolean(b) => LV2Value::from(*b),
        JsonValue::Object(obj) => {
            let mut dict = box_value(LV2Value::dict());

            for (key, val) in obj.iter() {
                let key = LV2Value::from(key);
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

            box_value(LV2Value::List(list))
        }
    };

    Ok(val)
}

// TODO: can this be changed to accept a &Value?
fn to_json_value(val: LV2Value) -> LV2Result<JsonValue> {
    let json = match val {
        LV2Value::Nil => JsonValue::Null,
        LV2Value::Bool(b) => JsonValue::Boolean(b),
        LV2Value::Int(n) => JsonValue::Number(n.into()),
        LV2Value::Float(n) => JsonValue::Number(n.into()),
        LV2Value::Str(s) => JsonValue::String(s),
        LV2Value::Dict(d) => {
            let mut obj = Object::new();

            for (key, val) in d.into_iter() {
                let key = key.as_str_inner()?;
                let val = to_json_value(val)?;

                obj.insert(&key, val);
            }

            obj.into()
        }
        LV2Value::List(ls) => {
            let mut json_ls = vec![];

            for val in ls.into_iter() {
                json_ls.push(to_json_value(val)?);
            }

            json_ls.into()
        }
        LV2Value::Ref(r) => to_json_value(r.unref_to_value()?.borrow().clone())?,
        _ => return err_from_string(format!("{:?} not supported for json", val)),
    };

    Ok(json)
}
