use lovm2::value::Value;

#[test]
fn integer_casting() {
    let a = Value::Int(1);
    let e = Value::Dict(std::collections::HashMap::new());
    let f = Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("1", a.as_str_inner().unwrap());

    assert!(e.as_integer().is_err());
    assert!(f.as_integer().is_err());
}

#[test]
fn float_casting() {
    let a = Value::Float(1.);
    let e = Value::Dict(std::collections::HashMap::new());
    let f = Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    //assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("1", a.as_str_inner().unwrap());

    assert!(e.as_float().is_err());
    assert!(f.as_float().is_err());
}

#[test]
fn bool_casting() {
    let a = Value::Bool(true);
    let e = Value::Dict(std::collections::HashMap::new());
    let f = Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("true", a.as_str_inner().unwrap());

    assert!(!e.as_bool_inner().unwrap());
    assert!(f.as_bool_inner().unwrap());
}

#[test]
fn str_casting() {
    let a = Value::from("1");
    let e = Value::Dict(std::collections::HashMap::new());
    let f = Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("1", a.as_str_inner().unwrap());

    assert!(e.as_str().is_ok());
    assert!(f.as_str().is_ok());
}

#[test]
fn implicit_float_conversion_sub() {
    let a = Value::Int(1) - Value::Float(2.);
    let b = Value::Float(1.) - Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(-1., b.as_float_inner().unwrap());
    assert_eq!(a, b);
}

#[test]
fn implicit_float_conversion_div() {
    let a = Value::Int(1) / Value::Float(2.);
    let b = Value::Float(1.) / Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(0.5, b.as_float_inner().unwrap());
    assert_eq!(a, b);
}

#[test]
fn implicit_float_conversion_rem() {
    let a = Value::Int(1) % Value::Float(2.);
    let b = Value::Float(1.) % Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(1., b.as_float_inner().unwrap());
    assert_eq!(a, b);
}
