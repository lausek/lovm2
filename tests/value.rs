use lovm2::value::{box_value, LV2Value};

#[test]
fn integer_casting() {
    let a = LV2Value::Int(1);
    let e = LV2Value::dict();
    let f = LV2Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("1", a.as_str_inner().unwrap());

    assert!(e.as_integer().is_err());
    assert!(f.as_integer().is_err());
}

#[test]
fn float_casting() {
    let a = LV2Value::Float(1.);
    let e = LV2Value::dict();
    let f = LV2Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    //assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("1", a.as_str_inner().unwrap());

    assert!(e.as_float().is_err());
    assert!(f.as_float().is_err());
}

#[test]
fn bool_casting() {
    let a = LV2Value::Bool(true);
    let e = LV2Value::dict();
    let f = LV2Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("true", a.as_str_inner().unwrap());

    assert!(!e.as_bool_inner().unwrap());
    assert!(f.as_bool_inner().unwrap());
}

#[test]
fn str_casting() {
    let a = LV2Value::from("1");
    let e = LV2Value::dict();
    let f = LV2Value::from(vec![1]);

    assert_eq!(1, a.as_integer_inner().unwrap());
    assert_eq!(1., a.as_float_inner().unwrap());
    assert_eq!(true, a.as_bool_inner().unwrap());
    assert_eq!("1", a.as_str_inner().unwrap());

    assert!(e.as_str().is_ok());
    assert!(f.as_str().is_ok());
}

#[test]
fn implicit_float_conversion_sub() {
    let a = LV2Value::Int(1) - LV2Value::Float(2.);
    let b = LV2Value::Float(1.) - LV2Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(-1., b.as_float_inner().unwrap());
    assert_eq!(a, b);

    let mut c = LV2Value::Int(1);
    c.sub_inplace(LV2Value::Float(2.)).unwrap();
    assert_eq!(LV2Value::Float(-1.), c);
}

#[test]
fn implicit_float_conversion_div() {
    let a = LV2Value::Int(1) / LV2Value::Float(2.);
    let b = LV2Value::Float(1.) / LV2Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(0.5, b.as_float_inner().unwrap());
    assert_eq!(a, b);

    let mut c = LV2Value::Int(1);
    c.div_inplace(LV2Value::Float(2.)).unwrap();
    assert_eq!(LV2Value::Float(0.5), c);
}

#[test]
fn implicit_float_conversion_rem() {
    let a = LV2Value::Int(1) % LV2Value::Float(2.);
    let b = LV2Value::Float(1.) % LV2Value::Int(2);
    let (a, b) = (a.unwrap(), b.unwrap());

    assert_eq!(1., b.as_float_inner().unwrap());
    assert_eq!(a, b);

    let mut c = LV2Value::Int(1);
    c.rem_inplace(LV2Value::Float(2.)).unwrap();
    assert_eq!(LV2Value::Float(1.), c);
}

#[test]
fn reference_equality() {
    let a = LV2Value::Int(0);
    let b = LV2Value::Ref(LV2Value::Int(0).into());

    let a2 = LV2Value::Ref(LV2Value::Float(1.).into());
    let b2 = LV2Value::Bool(true);

    let ls = box_value(LV2Value::List(vec![1.into(), 2.into(), 3.into()]));

    let item_a = ls.get(&a).unwrap();
    let item_b = ls.get(&b).unwrap();

    assert!(a == b);
    assert!(item_a == item_b);
    assert!(a2 != b2);
}

#[test]
fn xor_operation() {
    assert_eq!(
        LV2Value::Int(0),
        (LV2Value::from(0) ^ LV2Value::from(0)).unwrap()
    );
    assert_eq!(
        LV2Value::Int(1),
        (LV2Value::from(0) ^ LV2Value::from(1)).unwrap()
    );
    assert_eq!(
        LV2Value::Int(1),
        (LV2Value::from(1) ^ LV2Value::from(0)).unwrap()
    );
    assert_eq!(
        LV2Value::Int(0),
        (LV2Value::from(1) ^ LV2Value::from(1)).unwrap()
    );

    assert_eq!(
        LV2Value::Bool(false),
        (LV2Value::from(false) ^ LV2Value::from(false)).unwrap()
    );
    assert_eq!(
        LV2Value::Bool(true),
        (LV2Value::from(false) ^ LV2Value::from(true)).unwrap()
    );
    assert_eq!(
        LV2Value::Bool(true),
        (LV2Value::from(true) ^ LV2Value::from(false)).unwrap()
    );
    assert_eq!(
        LV2Value::Bool(false),
        (LV2Value::from(true) ^ LV2Value::from(true)).unwrap()
    );

    let mut a = LV2Value::Int(1);
    let b = a.clone();

    a.xor_inplace(b).unwrap();
    assert_eq!(LV2Value::Int(0), a);
}

#[test]
fn test_iterator_list() {
    let ls = LV2Value::List(vec![1.into(), 2.into(), 3.into(), 4.into()]);
    let mut ls_iter = ls.iter().unwrap();

    for n in vec![1, 2, 3, 4].into_iter().map(LV2Value::from) {
        assert!(ls_iter.has_next());
        let item = ls_iter.next().unwrap();
        assert_eq!(n, item);
    }

    assert!(!ls_iter.has_next());
}

#[test]
fn test_iterator_dict() {
    let mut d = LV2Value::dict();
    d.set(&LV2Value::from("method"), LV2Value::from("get"))
        .unwrap();
    d.set(&LV2Value::from("host"), LV2Value::from("localhost"))
        .unwrap();
    let mut d_iter = d.iter().unwrap();

    assert!(d_iter.has_next());
    let item = d_iter.next().unwrap();
    assert_eq!(LV2Value::from("method"), item.get(&0.into()).unwrap());
    assert_eq!(LV2Value::from("get"), item.get(&1.into()).unwrap());

    assert!(d_iter.has_next());
    let item = d_iter.next().unwrap();
    assert_eq!(LV2Value::from("host"), item.get(&0.into()).unwrap());
    assert_eq!(LV2Value::from("localhost"), item.get(&1.into()).unwrap());

    assert!(!d_iter.has_next());
}

#[test]
fn test_iterator_string() {
    let mut s_iter = LV2Value::from("abcd").iter().unwrap();

    for c in "abcd".chars().map(String::from).map(LV2Value::from) {
        assert!(s_iter.has_next());
        let item = s_iter.next().unwrap();
        assert_eq!(c, item);
    }

    assert!(!s_iter.has_next());
}

#[test]
fn test_iterator_ranged() {
    use lovm2::value::LV2Iter;
    use std::convert::TryFrom;

    let expected: Vec<LV2Value> = (0..10).map(LV2Value::from).collect();
    let r_iter = LV2Iter::ranged(0, 10);
    let r_to_iter = LV2Iter::ranged_to(10);
    assert_eq!(expected, r_iter.collect());
    assert_eq!(expected, r_to_iter.collect());

    let mut r_from_iter = LV2Iter::ranged_from(5);
    assert_eq!(LV2Value::from(5), r_from_iter.next().unwrap());

    let expected: Vec<LV2Value> = (0..10).rev().map(LV2Value::from).collect();
    assert_eq!(expected, LV2Iter::ranged(0, 10).reverse().collect());
    assert_eq!(expected, LV2Iter::ranged(10, 0).collect());
    assert_eq!(
        expected,
        LV2Iter::ranged(10, 0).reverse().reverse().collect()
    );

    let expected: Vec<LV2Value> = (-5..5).rev().map(LV2Value::from).collect();
    assert_eq!(expected, LV2Iter::ranged(-5, 5).reverse().collect());

    let base: Vec<LV2Value> = vec![
        LV2Value::from(true),
        LV2Value::from(10),
        LV2Value::from("abc"),
        LV2Value::from(1.5),
    ];
    let rev_expected: Vec<LV2Value> = base.clone().into_iter().rev().collect();
    let lv2_base = box_value(LV2Value::List(base.clone()));
    assert_eq!(
        rev_expected,
        LV2Iter::try_from(lv2_base.clone())
            .unwrap()
            .reverse()
            .collect()
    );
    assert_eq!(
        base,
        LV2Iter::try_from(lv2_base)
            .unwrap()
            .reverse()
            .reverse()
            .collect()
    );
}

#[test]
fn test_iterator_try_from() {
    let iter1: Vec<LV2Value> = vec![1, 2, 3, 4].into_iter().map(LV2Value::from).collect();

    let iter2 = LV2Value::from(box_value(LV2Value::List(iter1.clone())).iter().unwrap());
    let iter2 = iter2.iter().unwrap();

    assert_eq!(iter1, iter2.collect());
}
