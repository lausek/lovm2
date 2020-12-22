use lovm2::value::{box_value, Value};

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

    let mut c = Value::Int(1);
    c.sub_inplace(Value::Float(2.)).unwrap();
    assert_eq!(Value::Float(-1.), c);
}

#[test]
fn implicit_float_conversion_div() {
    let a = Value::Int(1) / Value::Float(2.);
    let b = Value::Float(1.) / Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(0.5, b.as_float_inner().unwrap());
    assert_eq!(a, b);

    let mut c = Value::Int(1);
    c.div_inplace(Value::Float(2.)).unwrap();
    assert_eq!(Value::Float(0.5), c);
}

#[test]
fn implicit_float_conversion_rem() {
    let a = Value::Int(1) % Value::Float(2.);
    let b = Value::Float(1.) % Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(1., b.as_float_inner().unwrap());
    assert_eq!(a, b);

    let mut c = Value::Int(1);
    c.rem_inplace(Value::Float(2.)).unwrap();
    assert_eq!(Value::Float(1.), c);
}

#[test]
fn reference_equality() {
    let a = Value::Int(0);
    let b = Value::Ref(Value::Int(0).into());

    let a2 = Value::Ref(Value::Float(1.).into());
    let b2 = Value::Bool(true);

    let ls = box_value(Value::List(vec![1.into(), 2.into(), 3.into()]));

    let item_a = ls.get(&a).unwrap();
    let item_b = ls.get(&b).unwrap();

    assert!(a == b);
    assert!(item_a == item_b);
    assert!(a2 != b2);
}
