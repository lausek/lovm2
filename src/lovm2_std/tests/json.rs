#![cfg(test)]

use test_utils::*;

use lovm2_core::extend::prelude::*;

#[test]
fn native_decode() {
    let (a, b) = &lv2_var!(a, b);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(a).step(Assign::var(a, lv2_dict!("a" => 10, "b" => Value::Nil)))
            .global(b).step(Assign::var(b, lv2_list!(lv2_dict!(), lv2_dict!(), 1.5)));
    });

    vm.call("init", &[]).unwrap();

    let a = vm.context_mut().value_of(a).unwrap().clone();
    let b = vm.context_mut().value_of(b).unwrap().clone();

    assert_eq!(
        a,
        vm.call("decode", &["{\"a\":10, \"b\":null}".into()])
            .unwrap()
    );
    assert_eq!(b, vm.call("decode", &["[{},{},1.5]".into()]).unwrap());
}

#[test]
fn native_encode() {
    let (ls, d, dd, n) = &lv2_var!(ls, d, dd, n);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(d).step(Assign::var(d, lv2_dict!(true => 0.5)))
            .global(dd).step(Assign::var(dd, lv2_dict!("a" => lv2_dict!("b" => "c"))))
            .global(ls).step(Assign::var(ls, lv2_list!(1, "abc", d, Value::Nil)))
            .global(n).step(Assign::var(n, 2));
    });

    vm.call("init", &[]).unwrap();

    let ls = vm.context_mut().value_of(ls).unwrap().clone();
    let d = vm.context_mut().value_of(d).unwrap().clone();
    let dd = vm.context_mut().value_of(dd).unwrap().clone();
    let n = vm.context_mut().value_of(n).unwrap().clone();

    assert_eq!(
        Value::from("[1,\"abc\",{\"true\":0.5},null]"),
        vm.call("encode", &[ls]).unwrap()
    );
    assert_eq!(
        Value::from("{\"a\":{\"b\":\"c\"}}"),
        vm.call("encode", &[dd]).unwrap()
    );
    assert_eq!(
        Value::from("{\"true\":0.5}"),
        vm.call("encode", &[d]).unwrap()
    );
    assert_eq!(Value::from("2"), vm.call("encode", &[n]).unwrap());
}
