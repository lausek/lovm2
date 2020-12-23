#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder
        .entry()
        .step(Include::import_global("liblovm2_std_regex"));
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_is_match() {
    let re = &lv2_var!(re);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .step(Assign::global(re, lv2_call!(new_regex, "\\d{2}")));
    });

    vm.call("init", &[]).unwrap();

    let re = vm.context_mut().value_of(re).unwrap().clone();

    assert_eq!(
        Value::Bool(true),
        vm.call("is_match", &[re.clone(), "ab10cd".into()]).unwrap()
    );
    assert_eq!(
        Value::Bool(false),
        vm.call("is_match", &[re.clone(), "ab1cd".into()]).unwrap()
    );
    assert_eq!(
        Value::Bool(false),
        vm.call("is_match", &[re, "abcd".into()]).unwrap()
    );
}

#[test]
fn native_captures() {
    let re = &lv2_var!(re);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .step(Assign::global(re, lv2_call!(new_regex, "(\\d)?(\\S+)")));
    });

    vm.call("init", &[]).unwrap();

    let re = vm.context_mut().value_of(re).unwrap().clone();

    let result = vm.call("captures", &[re.clone(), "1abc".into()]).unwrap();
    assert_eq!(Value::from("1abc"), result.get(&Value::Int(0)).unwrap());
    assert_eq!(Value::from("1"), result.get(&Value::Int(1)).unwrap());
    assert_eq!(Value::from("abc"), result.get(&Value::Int(2)).unwrap());

    let result = vm.call("captures", &[re.clone(), "abc".into()]).unwrap();
    assert_eq!(Value::from("abc"), result.get(&Value::Int(0)).unwrap());
    assert_eq!(Value::Nil, result.get(&Value::Int(1)).unwrap());
    assert_eq!(Value::from("abc"), result.get(&Value::Int(2)).unwrap());
}