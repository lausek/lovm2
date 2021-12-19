#![cfg(test)]

use test_utils::*;

use lovm2_core::extend::prelude::*;

// TODO: use new api
#[test]
fn native_set_predicates() {
    let (a, b, c, d, e, f) = &lv2_var!(a, b, c, d, e, f);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(a).step(Assign::var(a, lv2_list!(true)))
            .global(b).step(Assign::var(b, lv2_list!(true, "abc", 1)))
            .global(c).step(Assign::var(c, lv2_list!(true, "", 1)))
            .global(d).step(Assign::var(d, lv2_list!()))
            .global(e).step(Assign::var(e, lv2_list!(false, true)))
            .global(f).step(Assign::var(f, lv2_list!(false)));
    });

    vm.call("init", &[]).unwrap();

    let a = vm.context_mut().value_of(a).unwrap().clone();
    let b = vm.context_mut().value_of(b).unwrap().clone();
    let c = vm.context_mut().value_of(c).unwrap().clone();
    let d = vm.context_mut().value_of(d).unwrap().clone();
    let e = vm.context_mut().value_of(e).unwrap().clone();
    let f = vm.context_mut().value_of(f).unwrap().clone();

    assert_eq!(LV2Value::from(true), vm.call("all", &[a.clone()]).unwrap());
    assert_eq!(LV2Value::from(true), vm.call("all", &[b.clone()]).unwrap());
    assert_eq!(LV2Value::from(false), vm.call("all", &[c.clone()]).unwrap());
    assert_eq!(LV2Value::from(true), vm.call("all", &[d.clone()]).unwrap());
    assert_eq!(LV2Value::from(false), vm.call("all", &[e.clone()]).unwrap());
    assert_eq!(LV2Value::from(false), vm.call("all", &[f.clone()]).unwrap());

    assert_eq!(LV2Value::from(true), vm.call("any", &[a.clone()]).unwrap());
    assert_eq!(LV2Value::from(true), vm.call("any", &[b.clone()]).unwrap());
    assert_eq!(LV2Value::from(true), vm.call("any", &[c.clone()]).unwrap());
    assert_eq!(LV2Value::from(false), vm.call("any", &[d.clone()]).unwrap());
    assert_eq!(LV2Value::from(true), vm.call("any", &[e.clone()]).unwrap());
    assert_eq!(LV2Value::from(false), vm.call("any", &[f.clone()]).unwrap());
}

#[test]
fn native_contains() {
    let (n, s, d, ls) = &lv2_var!(n, s, d, ls);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(n).step(Assign::var(n, 10))
            .global(s).step(Assign::var(s, "abc10d"))
            .global(d).step(Assign::var(d, lv2_dict!(10 => 1, "b" => 2)))
            .global(ls).step(Assign::var(ls, lv2_list!("a", true, n)));
    });

    vm.call("init", &[]).unwrap();

    let n = vm.context_mut().value_of(n).unwrap().clone();
    let s = vm.context_mut().value_of(s).unwrap().clone();
    let d = vm.context_mut().value_of(d).unwrap().clone();
    let ls = vm.context_mut().value_of(ls).unwrap().clone();

    assert_eq!(
        LV2Value::from(true),
        vm.call("contains", &[s.clone(), n.clone()]).unwrap()
    );
    assert_eq!(
        LV2Value::from(true),
        vm.call("contains", &[d.clone(), n.clone()]).unwrap()
    );
    assert_eq!(
        LV2Value::from(true),
        vm.call("contains", &[ls.clone(), n.clone()]).unwrap()
    );
    assert_eq!(
        LV2Value::from(false),
        vm.call("contains", &[d.clone(), s.clone()]).unwrap()
    );
}

#[test]
fn native_len() {
    let (n, s, d, ls) = &lv2_var!(n, s, d, ls);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(n).step(Assign::var(n, 10))
            .global(s).step(Assign::var(s, "abc10d"))
            .global(d).step(Assign::var(d, lv2_dict!(10 => 1, "b" => 2)))
            .global(ls).step(Assign::var(ls, lv2_list!("a", true, n)));
    });

    vm.call("init", &[]).unwrap();

    let n = vm.context_mut().value_of(n).unwrap().clone();
    let s = vm.context_mut().value_of(s).unwrap().clone();
    let d = vm.context_mut().value_of(d).unwrap().clone();
    let ls = vm.context_mut().value_of(ls).unwrap().clone();

    assert_eq!(LV2Value::from(6), vm.call("len", &[s.clone()]).unwrap());
    assert_eq!(LV2Value::from(2), vm.call("len", &[d.clone()]).unwrap());
    assert_eq!(LV2Value::from(3), vm.call("len", &[ls.clone()]).unwrap());
    assert!(vm.call("len", &[n.clone()]).is_err());
}

#[test]
fn native_deep_clone() {
    let ls = box_value(LV2Value::List(vec![1.into(), 2.into()]));

    let mut d = LV2Value::dict();
    d.set(&1.into(), 2.into()).unwrap();
    let d = box_value(d);

    let mut vm = run_module_test(|_| {});

    let mut dc = vm.call("deep_clone", &[d.clone()]).unwrap();
    dc.set(&1.into(), 3.into()).unwrap();
    assert_eq!(LV2Value::from(2), d.get(&1.into()).unwrap());

    let mut lsc = vm.call("deep_clone", &[d.clone()]).unwrap();
    lsc.delete(&0.into()).unwrap();
    assert_eq!(2, ls.len().unwrap());
}

#[test]
fn native_delete() {
    let ls = box_value(LV2Value::List(vec![1.into(), 2.into()]));

    let mut vm = run_module_test(|_| {});

    assert_eq!(2, ls.len().unwrap());
    vm.call("delete", &[ls.clone(), 1.into()]).unwrap();
    assert_eq!(1, ls.len().unwrap());
    assert!(ls.get(&0.into()).is_ok());
    assert!(ls.get(&1.into()).is_err())
}

#[test]
fn native_get() {
    let s = LV2Value::from("abcd");
    let ls = box_value(LV2Value::List(vec![1.into()]));

    let mut d = LV2Value::dict();
    d.set(&1.into(), 2.into()).unwrap();
    let d = box_value(d);

    let mut vm = run_module_test(|_| {});

    assert_eq!(
        LV2Value::from("c"),
        vm.call("get", &[s.clone(), 2.into()]).unwrap()
    );
    assert_eq!(
        LV2Value::from(1),
        vm.call("get", &[ls.clone(), 0.into()]).unwrap()
    );
    assert_eq!(
        LV2Value::from(2),
        vm.call("get", &[d.clone(), 1.into()]).unwrap()
    );
}

#[test]
fn native_set() {
    let ls = box_value(LV2Value::List(vec![1.into()]));

    let mut d = LV2Value::dict();
    d.set(&1.into(), 2.into()).unwrap();
    let d = box_value(d);

    let mut vm = run_module_test(|_| {});

    assert_eq!(LV2Value::from(1), ls.get(&0.into()).unwrap());
    vm.call("set", &[ls.clone(), 0.into(), 2.into()]).unwrap();
    assert_eq!(LV2Value::from(2), ls.get(&0.into()).unwrap());
    assert_eq!(1, ls.len().unwrap());

    assert!(d.get(&"ab".into()).is_err());
    vm.call("set", &[d.clone(), "ab".into(), 3.into()]).unwrap();
    assert_eq!(LV2Value::from(3), d.get(&"ab".into()).unwrap());
}

#[test]
fn native_sort() {
    let (d, ds, ls, lss) = &lv2_var!(d, ds, ls, lss);
    let s = LV2Value::from("bcda");

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(d).step(Assign::var(d, lv2_dict!("b" => 1, "a" => 1)))
            .global(ds).step(Assign::var(ds, lv2_dict!("a" => 1, "b" => 1)))
            .global(ls).step(Assign::var(ls, lv2_list!(3, 1, 2)))
            .global(lss).step(Assign::var(lss, lv2_list!(1, 2, 3)));
    });

    vm.call("init", &[]).unwrap();

    let d = vm.context_mut().value_of(d).unwrap().clone();
    let ds = vm.context_mut().value_of(ds).unwrap().clone();
    let ls = vm.context_mut().value_of(ls).unwrap().clone();
    let lss = vm.context_mut().value_of(lss).unwrap().clone();

    assert_eq!(LV2Value::from("abcd"), vm.call("sort", &[s.clone()]).unwrap());

    assert_eq!(lss, vm.call("sort", &[ls.clone()]).unwrap());

    assert_eq!(ds, vm.call("sort", &[d.clone()]).unwrap());

    assert!(vm.call("sort", &[LV2Value::from(1)]).is_err());
}

#[test]
fn native_map() {
    let mut vm = run_module_test(|builder| {
        builder
            .add_with_args("inc", vec![lv2_var!(x)])
            .step(Return::value(LV2Expr::add(lv2_var!(x), 1)));

        builder
            .add_with_args("toe", vec![lv2_var!(x)])
            .step(Return::value(LV2Value::from("e")));
    });

    assert_eq!(
        LV2Value::from(vec!["e", "e", "e"]),
        vm.call("map", &["abc".into(), "toe".into()]).unwrap(),
    );
    assert_eq!(
        LV2Value::from(vec![101, 102, 103]),
        vm.call("map", &[vec![100, 101, 102].into(), "inc".into()])
            .unwrap(),
    );
}

#[test]
fn native_filter() {
    let mut vm = run_module_test(|builder| {
        builder
            .add_with_args("even", vec![lv2_var!(x)])
            .step(Return::value(LV2Expr::eq(LV2Expr::rem(lv2_var!(x), 2), 0)));
    });

    assert_eq!(
        LV2Value::from(vec![100, 102]),
        vm.call("filter", &[vec![100, 101, 102].into(), "even".into()])
            .unwrap(),
    );
}

#[test]
fn native_append() {
    let mut vm = run_module_test(|_| {});

    let ls = box_value(LV2Value::List(vec![]));
    assert_eq!(0, ls.len().unwrap());
    assert!(vm.call("append", &[ls.clone(), 2.into()]).is_ok());
    assert_eq!(1, ls.len().unwrap());
    assert_eq!(LV2Value::from(2), ls.get(&0.into()).unwrap());

    assert!(vm.call("append", &[1.into(), 2.into()]).is_err());
}
