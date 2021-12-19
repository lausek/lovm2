use test_utils::*;

use lovm2::create_vm_with_std;
use lovm2::prelude::*;
use lovm2::value::LV2Value;
use lovm2::vm::Vm;

#[test]
fn load_hook_none() {
    let mut builder = LV2ModuleBuilder::new();

    builder.entry().import("notfound").trigger(10);

    let module = builder.build().unwrap();

    let mut vm = create_vm_with_std();
    vm.set_load_hook(|_name| Ok(None));

    assert!(run_module_test(vm, module, |_| ()).is_err());
}

#[test]
fn load_custom_module() {
    let mut builder = LV2ModuleBuilder::named("main");
    let n = &lv2_var!(n);

    builder
        .entry()
        .import("extern")
        .assign(n, LV2Call::new("extern.calc"))
        .trigger(10);

    let module = builder.build().unwrap();

    let mut vm = create_vm_with_std();
    vm.set_load_hook(|req| {
        assert_eq!("extern", req.module);
        let mut builder = LV2ModuleBuilder::named("extern");

        builder.add("calc").return_value(LV2Expr::from(1).add(1));

        Ok(Some(builder.build().unwrap().into()))
    });

    run_module_test(vm, module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(2), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn import_from_scope() {
    let mut builder = LV2ModuleBuilder::named("main");
    let n = &lv2_var!(n);

    builder
        .entry()
        .import_from("extern")
        .assign(n, LV2Call::new("calc"))
        .trigger(10);

    let module = builder.build().unwrap();

    let mut vm = create_vm_with_std();
    vm.set_load_hook(|req| {
        assert_eq!("extern", req.module);
        let mut builder = LV2ModuleBuilder::named("extern");

        builder.add("calc").return_value(LV2Expr::from(1).add(1));

        Ok(Some(builder.build().unwrap().into()))
    });

    run_module_test(vm, module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(2), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn import_vice_versa() {
    const PASSED_VALUE: LV2Value = LV2Value::Int(72);
    let (n, result) = &lv2_var!(n, result);

    let mut builder = LV2ModuleBuilder::named("main");
    builder
        .entry()
        .import("extern")
        .step(LV2Call::new("extern.callextern"))
        .trigger(10);

    builder
        .add_with_args("callmain", vec![n.clone()])
        .global(result)
        .assign(result, n)
        .return_value(2);

    let module = builder.build().unwrap();

    let mut vm = create_vm_with_std();
    vm.set_load_hook(|req| {
        assert_eq!("extern", req.module);
        let mut builder = LV2ModuleBuilder::named("extern");

        builder
            .add("callextern")
            .import("main")
            .step(LV2Call::new("main.callmain").arg(PASSED_VALUE));

        Ok(Some(builder.build().unwrap().into()))
    });

    run_module_test(vm, module, |ctx| {
        assert_eq!(PASSED_VALUE, *ctx.value_of("result").unwrap());
    })
    .unwrap();
}

#[test]
fn custom_naming_scheme() {
    let mut module = LV2ModuleBuilder::named("main");
    module.add("call_main");
    let module = module.build().unwrap();

    let mut ex = LV2ModuleBuilder::named("extern");
    ex.add("call_extern");
    ex.add("call");
    let ex = ex.build().unwrap();

    let mut vm = create_vm_with_std();

    vm.set_import_hook(|module, name| {
        let key = to_lower_camel_case(name);
        let key = match module {
            // use dash as module name separator
            Some(module) => format!("{}-{}", module, key),
            _ => key,
        };
        println!("{}", key);
        Ok(Some(key))
    });

    vm.add_module(module, false).unwrap();
    vm.add_module(ex, true).unwrap();

    vm.call("callMain", &[]).unwrap();
    vm.call("extern-call", &[]).unwrap();
    vm.call("extern-callExtern", &[]).unwrap();
}

#[test]
fn main_has_no_entry_point() {
    let module = LV2ModuleBuilder::named("main").build().unwrap();
    let mut vm = Vm::new();

    let e = vm.add_main_module(module).err().unwrap();
    assert!(matches!(
        e,
        LV2Error {
            ty: LV2ErrorTy::NoEntryPoint,
            ..
        }
    ));
}

#[test]
fn main_import_as_global() {
    let mut module = LV2ModuleBuilder::named("main");
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
    let mut a = LV2ModuleBuilder::named("a");
    a.add("ina");
    let a = a.build().unwrap();

    let mut b = LV2ModuleBuilder::named("b");
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

#[test]
fn setting_interrupts() {
    let mut vm = Vm::new();

    assert!(vm.set_interrupt(1, |_| { unreachable!() }).is_err());
    assert!(vm.set_interrupt(10, |_| { unreachable!() }).is_ok());
    assert!(vm.set_interrupt(11, |_| { unreachable!() }).is_err());
    assert!(vm.set_interrupt(64, |_| { unreachable!() }).is_ok());
}

#[test]
fn call_stdlib_functions() {
    let mut vm = create_vm_with_std();

    assert_eq!(
        LV2Value::from("a"),
        vm.call("trim", &["    a ".into()]).unwrap(),
    );
    assert!(vm.call("new_request", &["".into()]).is_ok());
}
