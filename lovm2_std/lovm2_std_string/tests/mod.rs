#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder
        .entry()
        .step(Include::import_global("liblovm2_std_string"));
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_join() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::from("a & b"),
        vm.call("join", &[vec!["a", "b"].into(), " & ".into()])
            .unwrap()
    );
    assert_eq!(
        Value::from("a"),
        vm.call("join", &[vec!["a"].into(), " & ".into()]).unwrap()
    );
}

#[test]
fn native_split() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::List(vec!["a".into(), "b".into(), "c".into()]),
        vm.call("split", &["a;b;c".into(), ";".into()]).unwrap()
    );
    assert_eq!(
        Value::List(vec!["a".into()]),
        vm.call("split", &["a".into(), ";".into()]).unwrap()
    );
}

#[test]
fn native_index_of() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::Int(0),
        vm.call("index_of", &["abc".into(), "a".into()]).unwrap()
    );
    assert_eq!(
        Value::Int(2),
        vm.call("index_of", &["abc".into(), "c".into()]).unwrap()
    );
    assert_eq!(
        Value::Nil,
        vm.call("index_of", &["abc".into(), "d".into()]).unwrap()
    );
}

#[test]
fn native_char_conversion() {
    let mut vm = run_module_test(|_| {});

    let x_as_str = Value::from("x");
    let x_as_int = Value::from(120);

    assert_eq!(x_as_int, vm.call("ord", &[x_as_str.clone()]).unwrap(),);
    assert_eq!(x_as_str, vm.call("chr", &[x_as_int.clone()]).unwrap(),);

    let sigma_as_str = Value::from("∑");
    let sigma_as_int = Value::from(i64::from_be_bytes([
        0, 0, 0, 0, 0, 0b11100010, 0b10001000, 0b10010001,
    ]));

    assert_eq!(
        sigma_as_int,
        vm.call("ord", &[sigma_as_str.clone()]).unwrap(),
    );
    assert_eq!(
        sigma_as_str,
        vm.call("chr", &[sigma_as_int.clone()]).unwrap(),
    );
}

#[test]
fn native_replace() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::from("abb"),
        vm.call("replace", &["abc".into(), "c".into(), "b".into()])
            .unwrap()
    );
    assert_eq!(
        Value::from("abc"),
        vm.call("replace", &["abc".into(), "d".into(), "b".into()])
            .unwrap()
    );
    assert_eq!(
        Value::from("bbb"),
        vm.call("replace", &["aba".into(), "a".into(), "b".into()])
            .unwrap()
    );
}

#[test]
fn native_to_upper() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::from("AA"),
        vm.call("to_upper", &["aA".into()]).unwrap()
    );
}

#[test]
fn native_to_lower() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::from("aa"),
        vm.call("to_lower", &["aA".into()]).unwrap()
    );
}

#[test]
fn native_trim() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(Value::from("a"), vm.call("trim", &[" a".into()]).unwrap());
    assert_eq!(Value::from("a"), vm.call("trim", &["a ".into()]).unwrap());

    assert_eq!(
        Value::from("a"),
        vm.call("trim", &["   a   ".into()]).unwrap()
    );
}
