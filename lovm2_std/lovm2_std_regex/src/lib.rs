use lovm2::prelude::*;
use lovm2::value::box_value;
use lovm2_extend::prelude::*;

#[lovm2_object]
pub struct Regex {
    inner: regex::Regex,
}

#[lovm2_function]
fn new_regex(pat: String) -> Lovm2Result<Regex> {
    let inner = regex::Regex::new(&pat).map_err(|e| Lovm2Error::from(e.to_string()))?;
    Ok(Regex { inner })
}

#[lovm2_function]
fn captures(regex: &Regex, text: String) -> Option<Value> {
    regex.inner.captures(&text).map(|c| {
        let mut vals = vec![];
        for i in 0..c.len() {
            if let Some(m) = c.get(i) {
                vals.push(Value::from(m.as_str()));
            } else {
                vals.push(Value::Nil);
            }
        }
        box_value(Value::List(vals))
    })
}

#[lovm2_function]
fn is_match(regex: &Regex, text: String) -> bool {
    regex.inner.is_match(&text)
}

lovm2_module_init!(regex);
