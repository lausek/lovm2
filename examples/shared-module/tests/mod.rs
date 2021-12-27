#![cfg(test)]

use lovm2_core::extend::prelude::*;

fn create_caller(modder: fn(&mut LV2Function)) -> LV2Vm {
    let mut builder = LV2ModuleBuilder::new();

    let f = builder.entry();
    f.import_from("libshared_module");
    modder(f);

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = lv2_create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_add() {
    let mut vm = create_caller(|hir| {
        let n = &lv2_var!(n);
        hir.global(n).assign(n, lv2_call!(native_add, 1, 2));
    });

    let n = vm.context_mut().value_of("n").unwrap();
    assert_eq!(LV2Value::Int(3), *n);
}

#[test]
fn native_negate() {
    let mut vm = create_caller(|hir| {
        let (b,  n) = &lv2_var!(b, n);
        hir.assign(b, false);
        hir.global(n).assign(n, lv2_call!(negate, b));
    });

    let n = vm.context_mut().value_of("n").unwrap();
    assert_eq!(LV2Value::Bool(true), *n);
}

#[test]
fn native_to_string() {
    let mut vm = create_caller(|hir| {
        let (f, ext, n) = &lv2_var!(f, ext, n);
        hir.assign(f, 5.);
        hir.assign(ext, "so");
        hir.global(n).assign(n, lv2_call!(to_string, f, ext));
    });

    let n = vm.context_mut().value_of("n").unwrap();
    assert_eq!(LV2Value::from("5.so"), *n);
}

#[test]
fn native_only_create() {
    let mut vm = create_caller(|hir| {
        let n = &lv2_var!(n);
        hir.global(n).assign(n, lv2_call!(enden_der_wurst));
    });

    let n = vm.context_mut().value_of("n").unwrap();
    assert_eq!(LV2Value::from(2), *n);
}

#[test]
fn native_assert_this() {
    create_caller(|hir| {
        let b = &lv2_var!(b);
        hir.global(b).assign(b, true);
        hir.step(lv2_call!(assert_this, b));
    });
}

#[test]
fn native_use_context() {
    let mut vm = create_caller(|hir| {
        let n = &lv2_var!(n);
        hir.global(n).assign(n, lv2_call!(use_context));
    });

    let n = vm.context_mut().value_of("n").unwrap();
    assert_eq!(LV2Value::from(2), *n);
}

#[test]
fn run_module() {
    let mut vm = lv2_create_test_vm();
    let s = &lv2_var!(s);

    let mut builder = LV2ModuleBuilder::new();
    builder
        .entry()
        .import_from("libshared_module")
        .global(s).assign(s, lv2_call!(new));
    builder
        .add("name")
        .return_value(lv2_call!(get_name, s));
    builder
        .add("update")
        .step(lv2_call!(set_name, s, "yolo"));
    let module = builder.build().unwrap();

    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    assert_eq!(LV2Value::Nil, vm.call("name", &[]).unwrap());
    vm.call("update", &[]).unwrap();
    assert_eq!(LV2Value::from("yolo"), vm.call("name", &[]).unwrap());
}
