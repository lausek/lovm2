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
fn load_hook_none() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    hir.step(Include::load("notfound"));
    hir.step(Interrupt::new(10));

    let module = builder.build().unwrap();

    let mut vm = Vm::with_std();
    vm.set_load_hook(|_name| Ok(None));

    assert!(run_module_test(vm, module, |_| ()).is_err());
}

#[test]
fn load_custom_module() {
    let mut builder = ModuleBuilder::named("main");
    let hir = builder.entry();
    let n = &lv2_var!(n);
    hir.step(Include::load("extern"));
    hir.step(Assign::local(n, Call::new("calc")));
    hir.step(Interrupt::new(10));

    let module = builder.build().unwrap();

    let mut vm = Vm::with_std();
    vm.set_load_hook(|req| {
        assert_eq!("extern", req.module);
        let mut builder = ModuleBuilder::named("extern");

        let hir = builder.add("calc");
        hir.step(Return::value(Expr::add(1, 1)));

        Ok(Some(builder.build().unwrap().into()))
    });

    run_module_test(vm, module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(2), *frame.value_of(&lv2_var!(n)).unwrap());
    })
    .unwrap();
}

#[test]
fn import_vice_versa() {
    const PASSED_VALUE: Value = Value::Int(72);
    let (n, result) = &lv2_var!(n, result);

    let mut builder = ModuleBuilder::named("main");
    builder
        .entry()
        .step(Include::import("extern"))
        .step(Call::new("extern.callextern"))
        .step(Interrupt::new(10));

    builder
        .add_with_args("callmain", vec![n.clone()])
        .step(Assign::global(result, n))
        .step(Return::value(2));

    let module = builder.build().unwrap();

    let mut vm = Vm::with_std();
    vm.set_load_hook(|req| {
        assert_eq!("extern", req.module);
        let mut builder = ModuleBuilder::named("extern");

        builder
            .add("callextern")
            .step(Include::import("main"))
            .step(Call::new("main.callmain").arg(PASSED_VALUE));

        Ok(Some(builder.build().unwrap().into()))
    });

    run_module_test(vm, module, |ctx| {
        assert_eq!(PASSED_VALUE, *ctx.value_of(&lv2_var!(result)).unwrap());
    })
    .unwrap();
}
