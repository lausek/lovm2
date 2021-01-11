use std::path::Path;

use lovm2::create_vm_with_std;
use lovm2::code::CodeObject;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::value::Value;

const SERIALIZE_PATH: &str = "/tmp/hello-world.lovm2c";
const DESERIALIZE_PATH: &str = "/tmp/assign-global.lovm2c";

#[test]
fn serialize_module() {
    let mut builder = ModuleBuilder::new();

    let main_hir = builder.entry();
    main_hir.step(Assign::local(&lv2_var!(msg), "hello world"));
    main_hir.step(lv2_call!(print, msg));

    let module = builder.build().unwrap();

    module.store_to_file(SERIALIZE_PATH).unwrap();

    assert!(Path::new(SERIALIZE_PATH).exists());
}

#[test]
fn deserialize_module() {
    let mut builder = ModuleBuilder::new();
    let n = &lv2_var!(n);

    let main_hir = builder.entry();
    main_hir.step(Assign::global(n, 10));

    builder
        .build()
        .unwrap()
        .store_to_file(DESERIALIZE_PATH)
        .unwrap();

    assert!(Path::new(DESERIALIZE_PATH).exists());

    let module = Module::load_from_file(DESERIALIZE_PATH).unwrap();

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    let n = vm.context_mut().value_of(n).unwrap();
    assert_eq!(Value::Int(10), *n);
}

#[test]
fn global_uses() {
    use std::rc::Rc;

    const PRELOADED: &str = "preloaded";

    let mut builder = ModuleBuilder::new();
    builder.add_dependency(PRELOADED);

    let main_hir = builder.entry();
    main_hir.step(Assign::global(&lv2_var!(n), 10));

    let module = builder.build().unwrap();

    assert!(!module.uses().is_empty());

    let mut vm = create_vm_with_std();

    let called = Rc::new(std::cell::Cell::new(false));
    let called_ref = called.clone();
    vm.set_load_hook(move |req| {
        assert_eq!(req.module, PRELOADED);
        called_ref.set(true);
        Ok(Some(CodeObject::new().into()))
    });

    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    assert!(called.get());
}
