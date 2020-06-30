use std::path::Path;

use lovm2::{hir::prelude::*, value::RuValue, vm::Vm, Module, ModuleBuilder};

const SERIALIZE_PATH: &str = "/tmp/hello-world.lovm2c";
const DESERIALIZE_PATH: &str = "/tmp/assign-global.lovm2c";

#[test]
fn serialize_module() {
    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Assign::local(var!(msg), "hello world"));
    main_hir.push(call!(print, msg));

    builder.add("main").hir(main_hir);

    let module = builder.build().unwrap();

    module.store_to_file(SERIALIZE_PATH).unwrap();

    assert!(Path::new(SERIALIZE_PATH).exists());
}

#[test]
fn deserialize_module() {
    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Assign::global(var!(n), 10));

    builder.add("main").hir(main_hir);
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

    let n = vm.context_mut().globals.get(&var!(n)).unwrap().borrow();
    assert_eq!(RuValue::Int(10), *n);
}
