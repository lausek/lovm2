use super::*;

#[lovm2_function]
fn format(vm: &mut LV2Vm) -> LV2Result<String> {
    let argn = vm.context_mut().frame_mut()?.argn - 1;
    let mut args = vec![];

    for _ in 0..argn {
        args.push(vm.context_mut().pop_value()?.as_str_inner()?);
    }

    let mut format_base = vm.context_mut().pop_value()?.as_str_inner()?;

    for arg in args.iter().rev() {
        format_base = format_base.replacen("{}", arg, 1);
    }

    Ok(format_base)
}

#[lovm2_function]
fn index_of(base: String, pat: String) -> Option<i64> {
    base.find(&pat).map(|i| i as i64)
}

#[lovm2_function]
fn join(base: &LV2Value, sep: String) -> LV2Result<String> {
    if let LV2Value::List(ls) = base {
        let mut items = vec![];

        for item in ls.iter() {
            items.push(item.as_str_inner()?);
        }

        Ok(items.join(sep.as_ref()))
    } else {
        err_method_not_supported("join")
    }
}

#[lovm2_function]
fn chr(n: i64) -> LV2Result<String> {
    let bytes: Vec<u8> = n.to_be_bytes()[4..]
        .iter()
        .filter(|n| **n != 0)
        .map(u8::clone)
        .collect();

    String::from_utf8(bytes).or_else(err_from_string)
}

#[lovm2_function]
fn ord(c: String) -> LV2Result<i64> {
    if 1 != c.chars().count() {
        return err_from_string("ord requires string of length one");
    }

    let mut n: i64 = 0;

    for codepoint in c.as_bytes().iter() {
        n <<= 8;
        n |= *codepoint as i64;
    }

    Ok(n)
}

#[lovm2_function]
fn replace(base: String, pat: String, repl: String) -> String {
    base.replace(&pat, &repl)
}

#[lovm2_function]
fn split(base: String, sep: String) -> LV2Result<LV2Value> {
    let ls = base.split(&sep).map(LV2Value::from).collect::<Vec<_>>();
    let val = LV2Value::List(ls);

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
