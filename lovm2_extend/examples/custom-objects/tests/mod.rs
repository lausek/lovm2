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

#[test]
fn run_module() {
    let mut vm = create_vm();
    let s = &lv2_var!(s);
    const NAME: &str = "yolo";

    let mut builder = ModuleBuilder::new();
    builder
        .entry()
        .step(Include::import_global("libcustom_objects"))
        .step(Assign::global(s, lv2_call!(new)));
    builder
        .add("name")
        .step(Return::value(lv2_call!(get_name, s)));
    builder
        .add("update")
        .step(Call::new("set_name").arg(s).arg(NAME));
    let module = builder.build().unwrap();

    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    assert_eq!(Value::Nil, vm.call("name", &[]).unwrap());
    vm.call("update", &[]).unwrap();
    assert_eq!(Value::from(NAME), vm.call("name", &[]).unwrap());
}
