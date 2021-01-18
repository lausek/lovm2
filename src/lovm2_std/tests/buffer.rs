#![cfg(test)]

use test_utils::*;

use lovm2_core::extend::prelude::*;

#[test]
fn test_reading() {
    let mut vm = run_module_test(|_| {});

    let buf = vm.call("new_buffer", &[]).unwrap();

    assert_eq!(
        Value::from(""),
        vm.call("readn", &[buf.clone(), 1.into()]).unwrap(),
    );

    assert_eq!(
        Value::from(true),
        vm.call("writes", &[buf.clone(), "abc".into()]).unwrap(),
    );

    assert_eq!(
        Value::from("abc"),
        vm.call("readn", &[buf.clone(), 4.into()]).unwrap(),
    );
    assert_eq!(
        Value::from(""),
        vm.call("readn", &[buf.clone(), 4.into()]).unwrap(),
    );
}

#[test]
fn test_readline() {
    let mut vm = run_module_test(|_| {});

    let buf = vm.call("new_buffer", &[]).unwrap();

    assert_eq!(
        Value::from(true),
        vm.call("writes", &[buf.clone(), "abc\ndef\n".into()])
            .unwrap(),
    );

    assert_eq!(
        Value::from(true),
        vm.call("has_data", &[buf.clone()]).unwrap(),
    );
    assert_eq!(
        Value::from("abc\n"),
        vm.call("read_line", &[buf.clone()]).unwrap(),
    );
    assert_eq!(
        Value::from(true),
        vm.call("has_data", &[buf.clone()]).unwrap(),
    );
    assert_eq!(
        Value::from("def\n"),
        vm.call("read_line", &[buf.clone()]).unwrap(),
    );
    assert_eq!(
        Value::from(""),
        vm.call("read_line", &[buf.clone()]).unwrap(),
    );
    assert_eq!(
        Value::from(false),
        vm.call("has_data", &[buf.clone()]).unwrap(),
    );
}
