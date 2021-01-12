use lovm2_extend::prelude::*;

#[lovm2_function]
fn all(collection: &Value) -> Lovm2Result<bool> {
    match collection {
        Value::List(ls) => {
            for item in ls.iter() {
                if !item.as_bool_inner()? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => err_not_supported(),
    }
}

#[lovm2_function]
fn any(collection: &Value) -> Lovm2Result<bool> {
    match collection {
        Value::List(ls) => {
            for item in ls.iter() {
                if item.as_bool_inner()? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        _ => err_not_supported(),
    }
}

#[lovm2_function]
fn append(collection: &mut Value, value: Value) -> Lovm2Result<()> {
    match collection {
        Value::List(ls) => {
            ls.push(value);
            Ok(())
        }
        _ => err_not_supported(),
    }
}

#[lovm2_function]
fn contains(haystack: &Value, needle: Value) -> Lovm2Result<bool> {
    match haystack {
        Value::Dict(_) => Ok(haystack.get(&needle).is_ok()),
        Value::List(ls) => {
            let mut found = false;
            for item in ls.iter() {
                if *item == needle {
                    found = true;
                    break;
                }
            }
            Ok(found)
        }
        Value::Str(s) => {
            let needle = needle.as_str_inner()?;
            Ok(s.contains(&needle))
        }
        _ => err_method_not_supported("contains"),
    }
}

#[lovm2_function]
fn deep_clone(val: Value) -> Value {
    val.deep_clone()
}

#[lovm2_function]
fn delete(collection: &mut Value, key: Value) -> Lovm2Result<bool> {
    collection.delete(&key).map(|_| true)
}

#[lovm2_function]
fn filter(vm: &mut Vm, collection: &Value, func_name: String) -> Lovm2Result<Value> {
    let mut it = collection.iter()?;
    let mut ls = vec![];

    while it.has_next() {
        let item = it.next()?;
        if vm
            .call(func_name.as_ref(), &[item.clone()])?
            .as_bool_inner()?
        {
            ls.push(item);
        }
    }

    Ok(box_value(Value::List(ls)))
}

#[lovm2_function]
fn get(collection: &Value, key: Value) -> Lovm2Result<Value> {
    collection.get(&key)
}

#[lovm2_function]
fn map(vm: &mut Vm, collection: &Value, func_name: String) -> Lovm2Result<Value> {
    let mut it = collection.iter()?;
    let mut ls = vec![];

    while it.has_next() {
        let item = it.next()?;
        let result = vm.call(func_name.as_ref(), &[item])?;
        ls.push(result);
    }

    Ok(box_value(Value::List(ls)))
}

#[lovm2_function]
fn set(collection: &mut Value, key: Value, val: Value) -> Lovm2Result<bool> {
    collection.set(&key, val).map(|_| true)
}

#[lovm2_function]
fn sort(collection: &Value) -> Lovm2Result<Value> {
    let sorted = match collection {
        Value::Str(s) => {
            let mut cs: Vec<char> = s.chars().collect();
            cs.sort_unstable();
            let sorted: String = cs.into_iter().collect();
            Value::from(sorted)
        }
        Value::List(ls) => {
            let mut ls: Vec<Value> = ls.iter().map(Value::deep_clone).collect();
            ls.sort();
            box_value(Value::from(ls))
        }
        Value::Dict(_) => {
            let mut d = collection.deep_clone();
            d.unref_inplace()?;
            if let Value::Dict(mut d) = d {
                d.sort_keys();
                box_value(Value::Dict(d))
            } else {
                unreachable!()
            }
        }
        _ => err_method_not_supported("sort")?,
    };

    Ok(sorted)
}
