use std::path::Path;

use lovm2::module::{Module, ModuleProtocol};
use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::vm::Vm;

const SERIALIZE_PATH: &str = "/tmp/hello-world.lovm2c";
const DESERIALIZE_PATH: &str = "/tmp/assign-global.lovm2c";

#[test]
fn serialize_module() {
    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Assign::local(var!(msg), "hello world"));
    main_hir.push(call!(print, msg));

    builder.add(ENTRY_POINT).hir(main_hir);

    let module = builder.build().unwrap();

    module.store_to_file(SERIALIZE_PATH).unwrap();

    assert!(Path::new(SERIALIZE_PATH).exists());
}

#[test]
fn deserialize_module() {
    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Assign::global(var!(n), 10));

    builder.add(ENTRY_POINT).hir(main_hir);
    builder
        .build()
        .unwrap()
        .store_to_file(DESERIALIZE_PATH)
        .unwrap();

    assert!(Path::new(DESERIALIZE_PATH).exists());

    let module = Module::load_from_file(DESERIALIZE_PATH).unwrap();

    let mut vm = Vm::new();
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    let n = vm.context_mut().value_of(&var!(n)).unwrap();
    assert_eq!(Value::Int(10), n);
}

#[test]
fn global_uses() {
    use std::rc::Rc;

    const PRELOADED: &str = "preloaded";

    let mut builder = ModuleBuilder::new();
    builder.add_dependency(PRELOADED.into());

    let mut main_hir = HIR::new();
    main_hir.push(Assign::global(var!(n), 10));
    builder.add(ENTRY_POINT).hir(main_hir);

    let module = builder.build().unwrap();

    assert!(!module.uses().is_empty());

    let mut vm = Vm::new();

    let called = Rc::new(std::cell::Cell::new(false));
    let called_ref = called.clone();
    vm.context_mut().set_load_hook(move |req| {
        assert_eq!(req.module, PRELOADED);
        called_ref.set(true);
        Ok(Some(Rc::new(Module::new())))
    });

    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    assert!(called.get());
}
