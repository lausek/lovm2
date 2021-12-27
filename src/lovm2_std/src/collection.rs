use super::*;

#[lv2_function]
fn all(collection: &LV2Value) -> LV2Result<bool> {
    match collection {
        LV2Value::List(ls) => {
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

#[lv2_function]
fn any(collection: &LV2Value) -> LV2Result<bool> {
    match collection {
        LV2Value::List(ls) => {
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

#[lv2_function]
fn append(collection: &mut LV2Value, value: LV2Value) -> LV2Result<()> {
    match collection {
        LV2Value::List(ls) => {
            ls.push(value);
            Ok(())
        }
        _ => err_not_supported(),
    }
}

#[lv2_function]
fn contains(haystack: &LV2Value, needle: LV2Value) -> LV2Result<bool> {
    match haystack {
        LV2Value::Dict(_) => Ok(haystack.get(&needle).is_ok()),
        LV2Value::List(ls) => {
            let mut found = false;

            for item in ls.iter() {
                if *item == needle {
                    found = true;
                    break;
                }
            }

            Ok(found)
        }
        LV2Value::Str(s) => {
            let needle = needle.as_str_inner()?;

            Ok(s.contains(&needle))
        }
        _ => err_method_not_supported("contains"),
    }
}

#[lv2_function]
fn deep_clone(val: LV2Value) -> LV2Value {
    val.deep_clone()
}

#[lv2_function]
fn delete(collection: &mut LV2Value, key: LV2Value) -> LV2Result<bool> {
    collection.delete(&key).map(|_| true)
}

#[lv2_function]
fn filter(vm: &mut LV2Vm, collection: &LV2Value, func_name: String) -> LV2Result<LV2Value> {
    let mut it = collection.iter()?;
    let mut ls = vec![];

    while it.has_next() {
        let item = it.next()?;

        if vm
            .call(func_name.clone(), &[item.clone()])?
            .as_bool_inner()?
        {
            ls.push(item);
        }
    }

    Ok(box_value(LV2Value::List(ls)))
}

#[lv2_function]
fn get(collection: &LV2Value, key: LV2Value) -> LV2Result<LV2Value> {
    collection.get(&key)
}

#[lv2_function]
fn map(vm: &mut LV2Vm, collection: &LV2Value, func_name: String) -> LV2Result<LV2Value> {
    let mut it = collection.iter()?;
    let mut ls = vec![];

    while it.has_next() {
        let item = it.next()?;
        let result = vm.call(func_name.clone(), &[item])?;

        ls.push(result);
    }

    Ok(box_value(LV2Value::List(ls)))
}

#[lv2_function]
fn set(collection: &mut LV2Value, key: LV2Value, val: LV2Value) -> LV2Result<bool> {
    collection.set(&key, val).map(|_| true)
}

#[lv2_function]
fn sort(collection: &LV2Value) -> LV2Result<LV2Value> {
    let sorted = match collection {
        LV2Value::Str(s) => {
            let mut cs: Vec<char> = s.chars().collect();

            cs.sort_unstable();

            let sorted: String = cs.into_iter().collect();

            LV2Value::from(sorted)
        }
        LV2Value::List(ls) => {
            let mut ls: Vec<LV2Value> = ls.iter().map(LV2Value::deep_clone).collect();

            ls.sort();

            box_value(LV2Value::from(ls))
        }
        LV2Value::Dict(_) => {
            let mut d = collection.deep_clone();

            d.unref_inplace()?;

            if let LV2Value::Dict(mut d) = d {
                d.sort_keys();
                box_value(LV2Value::Dict(d))
            } else {
                unreachable!()
            }
        }
        _ => err_method_not_supported("sort")?,
    };

    Ok(sorted)
}
