use lovm2::create_vm_with_std;
use lovm2::prelude::*;

use lovm2_test_utils::*;

#[test]
fn load_avoid_sigabrt() {
    use std::path::Path;

    let mut builder = LV2ModuleBuilder::new();
    builder.entry().import("io").trigger(10);

    let module = builder.build().unwrap();

    let this_dir = Path::new(file!()).parent().unwrap().canonicalize().unwrap();
    let this_dir = this_dir.to_str().unwrap();
    let mut vm = create_vm_with_std();
    vm.load_paths.clear();
    vm.load_paths.push(this_dir.to_string());

    assert!(run_module_test(vm, module, |_ctx| ()).is_err());
}

#[test]
fn avoid_double_import() {
    let mut builder = LV2ModuleBuilder::named("main");

    builder.entry().import("abc").import("abc").trigger(10);

    let module = builder.build().unwrap();

    let mut vm = create_vm_with_std();
    vm.set_load_hook(|_name| {
        let mut builder = LV2ModuleBuilder::named("abc");
        builder.add("add");
        let module = builder.build().unwrap();
        Ok(Some(module.into()))
    });

    run_module_test(vm, module, |_ctx| ()).unwrap();
}
