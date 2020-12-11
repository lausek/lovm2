use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::vm::Vm;

fn run_module_test(
    mut vm: Vm,
    module: Module,
    testfn: impl Fn(&mut Context) + 'static,
) -> Lovm2Result<()> {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let called_ref = called.clone();
    vm.set_interrupt(10, move |vm| {
        called_ref.set(true);
        testfn(&mut vm.ctx);
        Ok(())
    });

    vm.load_and_import_all(module).unwrap();
    vm.run()?;

    assert!(called.get());

    Ok(())
}

#[test]
fn load_avoid_sigabrt() {
    use std::path::Path;

    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    hir.step(Include::load("io"));
    hir.step(Interrupt::new(10));

    let module = builder.build().unwrap();

    let this_dir = Path::new(file!()).parent().unwrap().canonicalize().unwrap();
    let this_dir = this_dir.to_str().unwrap();
    let mut vm = Vm::with_std();
    vm.load_paths.clear();
    vm.load_paths.push(this_dir.to_string());

    assert!(run_module_test(vm, module, |_ctx| ()).is_err());
}

#[test]
fn avoid_double_import() {
    let mut builder = ModuleBuilder::named("main");

    let main_hir = builder.entry();
    main_hir.step(Include::load("abc"));
    main_hir.step(Include::load("abc"));
    main_hir.step(Interrupt::new(10));

    let module = builder.build().unwrap();

    let mut vm = Vm::with_std();
    vm.set_load_hook(|_name| {
        let mut builder = ModuleBuilder::named("abc");
        builder.add("add");
        let module = builder.build().unwrap();
        Ok(Some(module.into()))
    });
    assert!(run_module_test(vm, module, |_ctx| ()).is_ok());
}
