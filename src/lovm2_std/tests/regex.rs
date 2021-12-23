#![cfg(test)]

use test_utils::*;

use lovm2_core::extend::prelude::*;

#[test]
fn native_is_match() {
    let re = &lv2_var!(re);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(re)
            .assign(re, lv2_call!(new_regex, "\\d{2}"));
    });

    vm.call("init", &[]).unwrap();

    let re = vm.context_mut().value_of(re).unwrap().clone();

    assert_eq!(
        LV2Value::Bool(true),
        vm.call("is_match", &[re.clone(), "ab10cd".into()]).unwrap()
    );
    assert_eq!(
        LV2Value::Bool(false),
        vm.call("is_match", &[re.clone(), "ab1cd".into()]).unwrap()
    );
    assert_eq!(
        LV2Value::Bool(false),
        vm.call("is_match", &[re, "abcd".into()]).unwrap()
    );
}

#[test]
fn native_captures() {
    let re = &lv2_var!(re);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .global(re)
            .assign(re, lv2_call!(new_regex, "(\\d)?(\\S+)"));
    });

    vm.call("init", &[]).unwrap();

    let re = vm.context_mut().value_of(re).unwrap().clone();

    let result = vm.call("captures", &[re.clone(), "1abc".into()]).unwrap();
    assert_eq!(
        LV2Value::from("1abc"),
        result.get(&LV2Value::Int(0)).unwrap()
    );
    assert_eq!(LV2Value::from("1"), result.get(&LV2Value::Int(1)).unwrap());
    assert_eq!(
        LV2Value::from("abc"),
        result.get(&LV2Value::Int(2)).unwrap()
    );

    let result = vm.call("captures", &[re.clone(), "abc".into()]).unwrap();
    assert_eq!(
        LV2Value::from("abc"),
        result.get(&LV2Value::Int(0)).unwrap()
    );
    assert_eq!(LV2Value::Nil, result.get(&LV2Value::Int(1)).unwrap());
    assert_eq!(
        LV2Value::from("abc"),
        result.get(&LV2Value::Int(2)).unwrap()
    );
}
