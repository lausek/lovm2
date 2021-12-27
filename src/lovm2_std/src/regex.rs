use super::*;

#[lv2_function]
fn new_regex(pat: String) -> LV2Result<Regex> {
    let inner = ::regex::Regex::new(&pat).or_else(err_from_string)?;

    Ok(Regex { inner })
}

#[lv2_function]
fn captures(regex: &Regex, text: String) -> Option<LV2Value> {
    regex.inner.captures(&text).map(|c| {
        let mut vals = vec![];

        for i in 0..c.len() {
            if let Some(m) = c.get(i) {
                vals.push(LV2Value::from(m.as_str()));
            } else {
                vals.push(LV2Value::Nil);
            }
        }

        box_value(LV2Value::List(vals))
    })
}

#[lv2_function]
fn is_match(regex: &Regex, text: String) -> bool {
    regex.inner.is_match(&text)
}
