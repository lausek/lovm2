use lovm2_extend::prelude::*;

#[lovm2_function]
fn index_of(base: String, pat: String) -> Option<i64> {
    base.find(&pat).map(|i| i as i64)
}

#[lovm2_function]
fn join(base: &Value, sep: String) -> Lovm2Result<String> {
    if let Value::List(ls) = base {
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
fn chr(n: i64) -> Lovm2Result<String> {
    let bytes: Vec<u8> = n.to_be_bytes()[4..]
        .iter()
        .filter(|n| **n != 0)
        .map(u8::clone)
        .collect();

    String::from_utf8(bytes).map_err(|e| Lovm2Error::from(e.to_string()))
}

#[lovm2_function]
fn ord(c: String) -> Lovm2Result<i64> {
    if 1 != c.chars().count() {
        return Err(Lovm2Error::from("ord requires string of length one"));
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