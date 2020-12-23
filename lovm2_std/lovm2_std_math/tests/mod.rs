#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder
        .entry()
        .step(Include::import_global("liblovm2_std_math"));
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_consts() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        std::f64::consts::E,
        vm.call("e", &[]).unwrap().as_float_inner().unwrap()
    );
    assert_eq!(
        std::f64::consts::PI,
        vm.call("pi", &[]).unwrap().as_float_inner().unwrap()
    );
}

#[test]
fn native_trigonometry() {
    let mut vm = run_module_test(|_| {});
    let pi2 = 2. * std::f64::consts::PI;
    let pih = std::f64::consts::PI * 0.5;
    let n = 0f64;

    assert_eq!(Value::from(n.sin()), vm.call("sin", &[0.into()]).unwrap());
    assert_eq!(
        Value::from(pi2.sin()),
        vm.call("sin", &[pi2.into()]).unwrap()
    );
    assert_eq!(
        Value::from(pih.sin()),
        vm.call("sin", &[pih.into()]).unwrap()
    );

    assert_eq!(Value::from(n.cos()), vm.call("cos", &[0.into()]).unwrap());
    assert_eq!(
        Value::from(pi2.cos()),
        vm.call("cos", &[pi2.into()]).unwrap()
    );
    assert_eq!(
        Value::from(pih.cos()),
        vm.call("cos", &[pih.into()]).unwrap()
    );

    assert_eq!(Value::from(n.tan()), vm.call("tan", &[0.into()]).unwrap());
    assert_eq!(
        Value::from(pi2.tan()),
        vm.call("tan", &[pi2.into()]).unwrap()
    );
    assert_eq!(
        Value::from(pih.tan()),
        vm.call("tan", &[pih.into()]).unwrap()
    );

    assert_eq!(Value::from(n.asin()), vm.call("asin", &[n.into()]).unwrap());
    assert_eq!(Value::from(n.acos()), vm.call("acos", &[n.into()]).unwrap());
    assert_eq!(Value::from(n.atan()), vm.call("atan", &[n.into()]).unwrap());
}

#[test]
fn native_clamp() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::from(1.5),
        vm.call("clamp", &[1.5.into(), 1.into(), 3.into()]).unwrap()
    );
    assert_eq!(
        Value::from(1.),
        vm.call("clamp", &[0.5.into(), 1.into(), 3.into()]).unwrap()
    );
    assert_eq!(
        Value::from(3.),
        vm.call("clamp", &[4.into(), 1.into(), 3.into()]).unwrap()
    );
}

#[test]
fn native_ceil() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(Value::from(2.), vm.call("ceil", &[1.5.into()]).unwrap());
    assert_eq!(Value::from(2.), vm.call("ceil", &[1.2.into()]).unwrap());
    assert_eq!(Value::from(1.), vm.call("ceil", &[1.into()]).unwrap());
}

#[test]
fn native_floor() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(Value::from(1.), vm.call("floor", &[1.5.into()]).unwrap());
    assert_eq!(Value::from(1.), vm.call("floor", &[1.2.into()]).unwrap());
    assert_eq!(Value::from(1.), vm.call("floor", &[1.into()]).unwrap());
}

#[test]
fn native_round() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(Value::from(2.), vm.call("round", &[1.6.into()]).unwrap());
    assert_eq!(Value::from(2.), vm.call("round", &[1.5.into()]).unwrap());
    assert_eq!(Value::from(1.), vm.call("round", &[1.2.into()]).unwrap());
    assert_eq!(Value::from(1.), vm.call("round", &[1.into()]).unwrap());
}

#[test]
fn native_log() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(
        Value::from(2.),
        vm.call("log", &[4.into(), 2.into()]).unwrap()
    );
    assert_eq!(
        Value::from(2.),
        vm.call("log", &[9.into(), 3.into()]).unwrap()
    );
}

#[test]
fn native_sqrt() {
    let mut vm = run_module_test(|_| {});

    assert_eq!(Value::from(2.), vm.call("sqrt", &[4.into()]).unwrap());
    assert_eq!(Value::from(3.), vm.call("sqrt", &[9.into()]).unwrap());
}
