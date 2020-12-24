use lovm2::value::{box_value, Value};

#[test]
fn integer_casting() {
    let a = Value::Int(1);
    let e = Value::dict();
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
    let e = Value::dict();
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
    let e = Value::dict();
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
    let e = Value::dict();
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

#[test]
fn xor_operation() {
    assert_eq!(Value::Int(0), (Value::from(0) ^ Value::from(0)).unwrap());
    assert_eq!(Value::Int(1), (Value::from(0) ^ Value::from(1)).unwrap());
    assert_eq!(Value::Int(1), (Value::from(1) ^ Value::from(0)).unwrap());
    assert_eq!(Value::Int(0), (Value::from(1) ^ Value::from(1)).unwrap());

    assert_eq!(
        Value::Bool(false),
        (Value::from(false) ^ Value::from(false)).unwrap()
    );
    assert_eq!(
        Value::Bool(true),
        (Value::from(false) ^ Value::from(true)).unwrap()
    );
    assert_eq!(
        Value::Bool(true),
        (Value::from(true) ^ Value::from(false)).unwrap()
    );
    assert_eq!(
        Value::Bool(false),
        (Value::from(true) ^ Value::from(true)).unwrap()
    );

    let mut a = Value::Int(1);
    let b = a.clone();

    a.xor_inplace(b).unwrap();
    assert_eq!(Value::Int(0), a);
}

#[test]
fn test_iterator_list() {
    let ls = Value::List(vec![1.into(), 2.into(), 3.into(), 4.into()]);
    let mut ls_iter = ls.iter().unwrap();

    for n in vec![1, 2, 3, 4].into_iter().map(Value::from) {
        assert!(ls_iter.has_next());
        let item = ls_iter.next().unwrap();
        assert_eq!(n, item);
    }

    assert!(!ls_iter.has_next());
}

#[test]
fn test_iterator_dict() {
    let mut d = Value::dict();
    d.set(&Value::from("method"), Value::from("get")).unwrap();
    d.set(&Value::from("host"), Value::from("localhost"))
        .unwrap();
    let mut d_iter = d.iter().unwrap();

    assert!(d_iter.has_next());
    let item = d_iter.next().unwrap();
    assert_eq!(Value::from("method"), item.get(&0.into()).unwrap());
    assert_eq!(Value::from("get"), item.get(&1.into()).unwrap());

    assert!(d_iter.has_next());
    let item = d_iter.next().unwrap();
    assert_eq!(Value::from("host"), item.get(&0.into()).unwrap());
    assert_eq!(Value::from("localhost"), item.get(&1.into()).unwrap());

    assert!(!d_iter.has_next());
}

#[test]
fn test_iterator_string() {
    let mut s_iter = Value::from("abcd").iter().unwrap();

    for c in "abcd".chars().map(String::from).map(Value::from) {
        assert!(s_iter.has_next());
        let item = s_iter.next().unwrap();
        assert_eq!(c, item);
    }

    assert!(!s_iter.has_next());
}
