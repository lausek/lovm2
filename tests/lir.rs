use lovm2::create_vm_with_std;
use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::Instruction;

#[test]
fn merge_not_jump_false() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    let n = &lv2_var!(n);

    hir.step(Assign::local(n, Value::Int(0)));

    let branch = hir.branch();
    branch
        .add_condition(Expr::not(Expr::eq(n, Value::Int(2))))
        .step(Return::value(Value::Int(1)));
    branch
        .default_condition()
        .step(Return::value(Value::Int(2)));

    let module = builder.build().unwrap();
    println!("{}", module);

    assert!(!module
        .code_object
        .code
        .iter()
        .any(|c| matches!(c, Instruction::Not)));

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(Value::Int(1), result.clone());
}

#[test]
fn merge_constant_jump() {
    let mut builder = ModuleBuilder::named("main");
    let hir = builder.entry();

    let branch = hir.branch();
    branch
        .add_condition(Expr::not(Value::Bool(false)))
        .step(Return::value(Value::Int(1)));
    branch
        .default_condition()
        .step(Return::value(Value::Int(2)));

    let module = builder.build().unwrap();
    println!("{}", module);

    assert!(!module
        .code_object
        .code
        .iter()
        .any(|c| matches!(c, Instruction::Jt(_) | Instruction::Jf(_))));

    // `false` is constant and should be eliminated
    assert!(!module.code_object.consts.contains(&Value::Bool(false)));
    // check if dead code elimination is working
    assert_eq!(2, module.code_object.code.len());

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(Value::Int(1), result.clone());
}

#[test]
fn short_circuit_and() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    let n = &lv2_var!(n);

    hir.step(Assign::local(n, Value::Int(0)));
    hir.step(Return::value(Expr::and(
        Expr::eq(n, Value::Int(1)),
        Expr::div(Value::Int(1), n),
    )));

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(Value::Bool(false), result.clone());
}

#[test]
fn short_circuit_or() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    let n = &lv2_var!(n);

    hir.step(Assign::local(n, Value::Int(0)));
    hir.step(Return::value(Expr::or(
        Expr::eq(n, Value::Int(0)),
        Expr::div(Value::Int(1), n),
    )));

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(Value::Bool(true), result.clone());
}

#[test]
fn compute_constants() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();

    hir.step(Return::value(Expr::rem(
        Expr::mul(Expr::add(Expr::sub(6, 1), 2), Expr::div(4, 2)),
        5,
    )));

    let module = builder.build().unwrap();
    println!("{}", module);

    assert!(!module
        .code_object
        .code
        .iter()
        .any(|c| matches!(c, Instruction::Jt(_) | Instruction::Jf(_))));

    let expected = Value::Int(4);
    assert_eq!(1, module.code_object.consts.len());
    assert!(module.code_object.consts.contains(&expected));

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(expected, result.clone());
}

#[test]
fn dead_code_elimination_else_branche() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    let (n, y) = &lv2_var!(n, y);

    hir.step(Assign::local(n, 3));

    let branch = hir.branch();
    branch
        .add_condition(Expr::eq(Expr::rem(n, 2), 0))
        .step(Assign::local(y, 0));

    // this condition will always be met
    branch
        .add_condition(Expr::not(false))
        .step(Assign::local(y, 1));

    // this code will never be reached
    branch.default_condition().step(Assign::local(y, 7));

    hir.step(Return::value(y));

    let module = builder.build().unwrap();
    println!("{}", module);

    assert_eq!(16, module.code_object.code.len());
    assert_eq!(4, module.code_object.consts.len());
    assert!(!module.code_object.consts.contains(&Value::Int(7)));

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(Value::Int(1), result.clone());
}

#[test]
fn compile_options() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();

    hir.step(Return::value(Expr::add(Expr::mul(3, 2), 2)));

    let module = builder.clone().build().unwrap();
    let module_noop = builder
        .build_with_options(CompileOptions { optimize: false })
        .unwrap();

    assert!(module.code_object.code.len() < module_noop.code_object.code.len());
}
