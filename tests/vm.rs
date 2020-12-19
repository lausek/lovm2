use test_utils::*;

use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::vm::Vm;

#[test]
fn load_hook_none() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    hir.step(Include::import("notfound"));
    hir.step(Interrupt::new(10));

    let module = builder.build().unwrap();

    let mut vm = Vm::with_std();
    vm.set_load_hook(|_name| Ok(None));

    assert!(run_module_test(vm, module, |_| ()).is_err());
}

#[test]
fn load_custom_module() {
    let mut builder = ModuleBuilder::named("main");
    let n = &lv2_var!(n);

    builder
        .entry()
        .step(Include::import("extern"))
        .step(Assign::local(n, Call::new("extern.calc")))
        .step(Interrupt::new(10));

    let module = builder.build().unwrap();

    let mut vm = Vm::with_std();
    vm.set_load_hook(|req| {
        assert_eq!("extern", req.module);
        let mut builder = ModuleBuilder::named("extern");

        builder.add("calc").step(Return::value(Expr::add(1, 1)));

        Ok(Some(builder.build().unwrap().into()))
    });

    run_module_test(vm, module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(2), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn import_global_scope() {
    let mut builder = ModuleBuilder::named("main");
    let n = &lv2_var!(n);

    builder
        .entry()
        .step(Include::import_global("extern"))
        .step(Assign::local(n, Call::new("calc")))
        .step(Interrupt::new(10));

    let module = builder.build().unwrap();

    let mut vm = Vm::with_std();
    vm.set_load_hook(|req| {
        assert_eq!("extern", req.module);
        let mut builder = ModuleBuilder::named("extern");

        builder.add("calc").step(Return::value(Expr::add(1, 1)));

        Ok(Some(builder.build().unwrap().into()))
    });

    run_module_test(vm, module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(2), *frame.value_of("n").unwrap());
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
        assert_eq!(PASSED_VALUE, *ctx.value_of("result").unwrap());
    })
    .unwrap();
}

#[test]
fn custom_naming_scheme() {
    let mut module = ModuleBuilder::named("main");
    module.add("call_main");
    let module = module.build().unwrap();

    let mut ex = ModuleBuilder::named("extern");
    ex.add("call_extern");
    ex.add("call");
    let ex = ex.build().unwrap();

    let mut vm = Vm::with_std();

    vm.set_import_hook(|module, name| {
        let key = to_lower_camel_case(name);
        let key = match module {
            // use dash as module name separator
            Some(module) => format!("{}-{}", module, key),
            _ => key,
        };
        println!("{}", key);
        key
    });

    vm.add_module(module, false).unwrap();
    vm.add_module(ex, true).unwrap();

    vm.call("callMain", &[]).unwrap();
    vm.call("extern-call", &[]).unwrap();
    vm.call("extern-callExtern", &[]).unwrap();
}

#[test]
fn main_has_no_entry_point() {
    let module = ModuleBuilder::named("main").build().unwrap();
    let mut vm = Vm::new();

    let e = vm.add_main_module(module).err().unwrap();
    assert!(matches!(e, Lovm2Error { ty: Lovm2ErrorTy::NoEntryPoint, .. }));
}

#[test]
fn main_import_as_global() {
    let mut module = ModuleBuilder::named("main");
    module.add("main");
    module.add("myfunc");
    let module = module.build().unwrap();

    let mut vm = Vm::new();
    vm.add_main_module(module).unwrap();

    assert!(vm.call("myfunc", &[]).is_ok());
    assert!(vm.call("main.myfunc", &[]).is_ok());
}

#[test]
fn namespaced_imports() {
    let mut a = ModuleBuilder::named("a");
    a.add("ina");
    let a = a.build().unwrap();

    let mut b = ModuleBuilder::named("b");
    b.add("inb");
    let b = b.build().unwrap();

    let mut vm = Vm::new();

    vm.add_module(a, true).unwrap();
    vm.add_module(b, false).unwrap();

    assert!(vm.call("a.ina", &[]).is_ok());
    assert!(vm.call("ina", &[]).is_err());
    assert!(vm.call("b.inb", &[]).is_ok());
    assert!(vm.call("inb", &[]).is_ok());
}
