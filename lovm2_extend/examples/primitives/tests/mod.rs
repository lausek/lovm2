#![cfg(test)]

use lovm2::prelude::*;
use lovm2::vm::Vm;

fn create_vm() -> Vm {
    let cargo_root = std::env::var("CARGO_MANIFEST_DIR").expect("no cargo manifest");
    let build_dir = format!("{}/target/debug", cargo_root);
    assert!(std::path::Path::new(&build_dir).exists());

    let mut vm = Vm::with_std();
    vm.add_load_path(build_dir);

    vm
}

fn create_caller(modder: fn(&mut Hir)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("primitives".to_string());

    let hir = builder.entry();
    modder(hir);

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = create_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_add() {
    let mut vm = create_caller(|hir| {
        hir.step(Assign::global(&lv2_var!(n), lv2_call!(native_add, 1, 2)));
    });

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::Int(3), *n);
}

#[test]
fn native_negate() {
    let mut vm = create_caller(|hir| {
        hir.step(Assign::local(&lv2_var!(b), false));
        hir.step(Assign::global(&lv2_var!(n), lv2_call!(negate, b)));
    });

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::Bool(true), *n);
}

#[test]
fn native_to_string() {
    let mut vm = create_caller(|hir| {
        hir.step(Assign::local(&lv2_var!(f), 5.));
        hir.step(Assign::local(&lv2_var!(ext), "so"));
        hir.step(Assign::global(&lv2_var!(n), lv2_call!(to_string, f, ext)));
    });

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::from("5.so"), *n);
}

#[test]
fn native_only_create() {
    let mut vm = create_caller(|hir| {
        hir.step(Assign::global(&lv2_var!(n), lv2_call!(enden_der_wurst)));
    });

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::from(2), *n);
}

#[test]
fn native_assert_this() {
    create_caller(|hir| {
        hir.step(Assign::global(&lv2_var!(b), true));
        hir.step(lv2_call!(assert_this, b));
    });
}

#[test]
fn native_use_context() {
    let mut vm = create_caller(|hir| {
        hir.step(Assign::global(&lv2_var!(n), lv2_call!(use_context)));
    });

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::from(2), *n);
}
