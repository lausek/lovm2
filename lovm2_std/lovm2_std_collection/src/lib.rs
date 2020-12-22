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

lovm2_module_init!();
