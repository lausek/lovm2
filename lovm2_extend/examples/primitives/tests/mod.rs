#![cfg(test)]

use lovm2::prelude::*;
use lovm2::vm::Vm;

fn create_vm() -> Vm {
    let cargo_root = std::env::var("CARGO_MANIFEST_DIR").expect("no cargo manifest");
    let build_dir = format!("{}/target/debug", cargo_root);
    assert!(std::path::Path::new(&build_dir).exists());

    let mut vm = Vm::with_std();
    vm.context_mut().add_load_path(build_dir);

    vm
}

#[test]
fn native_add() {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("primitives".to_string());

    let mut hir = Hir::new();
    hir.code
        .push(Assign::global(lv2_var!(n), lv2_call!(native_add, 1, 2)));

    builder.add("main").hir(hir);

    let module = builder.build().unwrap();
    println!("{:?}", module);

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::Int(3), *n);
}

#[test]
fn native_negate() {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("primitives".to_string());

    let mut hir = Hir::new();
    hir.code.push(Assign::local(lv2_var!(b), false));
    hir.code
        .push(Assign::global(lv2_var!(n), lv2_call!(negate, b)));

    builder.add("main").hir(hir);

    let module = builder.build().unwrap();
    println!("{:?}", module);

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::Bool(true), *n);
}

#[test]
fn native_to_string() {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("primitives".to_string());

    let mut hir = Hir::new();
    hir.code.push(Assign::local(lv2_var!(f), 5.));
    hir.code.push(Assign::local(lv2_var!(ext), "so"));
    hir.code
        .push(Assign::global(lv2_var!(n), lv2_call!(to_string, f, ext)));

    builder.add("main").hir(hir);

    let module = builder.build().unwrap();
    println!("{:?}", module);

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::from("5.so"), *n);
}

#[test]
fn native_only_create() {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("primitives".to_string());

    let mut hir = Hir::new();
    hir.code
        .push(Assign::global(lv2_var!(n), lv2_call!(enden_der_wurst)));

    builder.add("main").hir(hir);

    let module = builder.build().unwrap();
    println!("{:?}", module);

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::from(2), *n);
}

#[test]
fn native_assert_this() {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("primitives".to_string());

    let mut hir = Hir::new();
    hir.code.push(Assign::global(lv2_var!(b), true));
    hir.code.push(lv2_call!(assert_this, b));

    builder.add("main").hir(hir);

    let module = builder.build().unwrap();
    println!("{:?}", module);

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();
}

#[test]
fn native_use_context() {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("primitives".to_string());

    let mut hir = Hir::new();
    hir.code
        .push(Assign::global(lv2_var!(n), lv2_call!(use_context)));

    builder.add("main").hir(hir);

    let module = builder.build().unwrap();
    println!("{:?}", module);

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    let n = vm.context_mut().globals.get(&lv2_var!(n)).unwrap();
    assert_eq!(Value::from(2), *n);
}
