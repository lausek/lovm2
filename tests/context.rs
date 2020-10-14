use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::value::RuValue;
use lovm2::vm::Vm;

fn run_module_test(
    mut vm: Vm,
    module: Module,
    testfn: impl Fn(&mut Context) + 'static,
) -> Lovm2Result<()> {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let called_ref = called.clone();
    vm.context_mut().set_interrupt(10, move |ctx| {
        called_ref.set(true);
        testfn(ctx);
        Ok(())
    });

    vm.load_and_import_all(module).unwrap();
    vm.run()?;

    assert!(called.get());

    Ok(())
}

#[test]
fn load_hook_none() {
    let mut vm = Vm::new();
    vm.context_mut().set_load_hook(|_name| Ok(None));

    let mut hir = HIR::new();
    hir.push(Include::load("std"));
    hir.push(Interrupt::new(10));

    let mut builder = ModuleBuilder::new();
    builder.add(ENTRY_POINT).hir(hir);

    let module = builder.build().unwrap();

    assert!(run_module_test(vm, module, |_| ()).is_err());
}

#[test]
fn load_custom_module() {
    let mut vm = Vm::new();
    vm.context_mut().set_load_hook(|_name| {
        let mut hir = HIR::new();
        hir.push(Return::value(Expr::add(1, 1)));

        let mut builder = ModuleBuilder::new();
        builder.add("calc").hir(hir);
        Ok(Some(
            std::rc::Rc::new(builder.build().unwrap()) as GenericModule
        ))
    });

    let mut hir = HIR::new();
    hir.push(Include::load("std"));
    hir.push(Assign::local(var!(n), Call::new("calc")));
    hir.push(Interrupt::new(10));

    let mut builder = ModuleBuilder::named("main");
    builder.add(ENTRY_POINT).hir(hir);

    let module = builder.build().unwrap();

    run_module_test(vm, module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Int(2),
            *frame.locals.get(&var!(n)).unwrap().borrow()
        );
    })
    .unwrap();
}

#[test]
fn load_avoid_sigabrt() {
    use std::path::Path;

    let mut hir = HIR::new();
    hir.push(Include::load("io"));
    hir.push(Interrupt::new(10));

    let mut builder = ModuleBuilder::new();
    builder.add(ENTRY_POINT).hir(hir);
    let module = builder.build().unwrap();

    let this_dir = Path::new(file!()).parent().unwrap().canonicalize().unwrap();
    let this_dir = this_dir.to_str().unwrap();
    let mut vm = Vm::new();
    vm.context_mut().load_paths.clear();
    vm.context_mut().load_paths.push(this_dir.to_string());

    assert!(run_module_test(vm, module, |_ctx| ()).is_err());
}

#[test]
fn avoid_double_import() {
    use std::rc::Rc;

    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Include::load("std"));
    main_hir.push(Include::load("std"));
    main_hir.push(Interrupt::new(10));

    builder.add(ENTRY_POINT).hir(main_hir);

    let module = builder.build().unwrap();

    let mut vm = Vm::new();
    vm.context_mut().set_load_hook(|_name| {
        let mut builder = ModuleBuilder::new();
        builder.add("add").hir(HIR::new());
        let module = builder.build().unwrap();
        Ok(Some(Rc::new(module) as GenericModule))
    });

    assert!(run_module_test(vm, module, |_ctx| ()).is_ok());
}
