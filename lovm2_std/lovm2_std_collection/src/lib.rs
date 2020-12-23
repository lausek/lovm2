use lovm2_extend::prelude::*;

#[lovm2_function]
fn contains(mut haystack: Value, needle: Value) -> Lovm2Result<bool> {
    if haystack.is_ref() {
        haystack = haystack.deref().unwrap();
    }

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
        _ => Err(Lovm2ErrorTy::OperationNotSupported.into()),
    }
}

#[lovm2_function]
fn count(mut val: Value) -> Lovm2Result<i64> {
    val.len().map(|n| n as i64)
}

#[lovm2_function]
fn deep_clone(val: Value) -> Value {
    todo!()
}

#[lovm2_function]
fn delete(collection: Value, key: Value) -> Lovm2Result<bool> {
    todo!()
}

#[lovm2_function]
fn get(collection: Value, key: Value) -> Lovm2Result<Value> {
    todo!()
}

#[lovm2_function]
fn insert(collection: Value, key: Value, val: Value) -> bool {
    todo!()
}

#[lovm2_function]
fn sort(collection: Value) -> Lovm2Result<bool> {
    todo!()
}

lovm2_module_init!(collection);
