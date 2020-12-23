use lovm2::value::box_value;
use lovm2_extend::prelude::*;

#[lovm2_function]
fn index_of(base: String, pat: String) -> Option<i64> {
    base.find(&pat).map(|i| i as i64)
}

#[lovm2_function]
fn join(base: Value, sep: String) -> Lovm2Result<String> {
    let base = base.deref().unwrap();
    if let Value::List(ls) = base {
        let mut items = vec![];

        for item in ls.iter() {
            if item.is_ref() {
                items.push(item.deref().unwrap().as_str_inner()?);
            } else {
                items.push(item.as_str_inner()?);
            }
        }

        Ok(items.join(sep.as_ref()))
    } else {
        Err("argument is not a list".into())
    }
}

#[lovm2_function]
fn replace(base: String, pat: String, repl: String) -> String {
    base.replace(&pat, &repl)
}

#[lovm2_function]
fn split(base: String, sep: String) -> Lovm2Result<Value> {
    let ls = base.split(&sep).map(Value::from).collect::<Vec<_>>();
    let val = Value::List(ls);
    Ok(box_value(val))
}

#[lovm2_function]
fn to_lower(base: String) -> String {
    base.to_lowercase()
}

#[lovm2_function]
fn to_upper(base: String) -> String {
    base.to_uppercase()
}

#[lovm2_function]
fn trim(base: String) -> String {
    base.trim().to_string()
}

lovm2_module_init!(string);
