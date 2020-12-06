use lovm2::bytecode::Instruction;
use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::vm::Vm;

#[test]
fn merge_not_jump_false() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local(lv2_var!(n), Value::Int(0)));

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::not(Expr::eq(lv2_var!(n), Value::Int(2))))
        .push(Return::value(Value::Int(1)));
    branch
        .default_condition()
        .push(Return::value(Value::Int(2)));
    hir.push(branch);

    builder.add(ENTRY_POINT).hir(hir);

    let module = builder.build().unwrap();
    println!("{}", module);

    assert!(!module
        .code_object
        .code
        .iter()
        .any(|c| matches!(c, Instruction::Not)));

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();
    let result = vm.call(ENTRY_POINT, &[]).unwrap();

    assert_eq!(Value::Int(1), result.clone());
}

#[test]
fn merge_constant_jump() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::not(Value::Bool(false)))
        .push(Return::value(Value::Int(1)));
    branch
        .default_condition()
        .push(Return::value(Value::Int(2)));
    hir.push(branch);

    builder.add(ENTRY_POINT).hir(hir);

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

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();
    let result = vm.call(ENTRY_POINT, &[]).unwrap();

    assert_eq!(Value::Int(1), result.clone());
}

#[test]
fn short_circuit_and() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local(lv2_var!(n), Value::Int(0)));
    hir.push(Return::value(Expr::and(
        Expr::eq(lv2_var!(n), Value::Int(1)),
        Expr::div(Value::Int(1), lv2_var!(n)),
    )));

    builder.add(ENTRY_POINT).hir(hir);

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();
    let result = vm.call(ENTRY_POINT, &[]).unwrap();

    assert_eq!(Value::Bool(false), result.clone());
}

#[test]
fn short_circuit_or() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local(lv2_var!(n), Value::Int(0)));
    hir.push(Return::value(Expr::or(
        Expr::eq(lv2_var!(n), Value::Int(0)),
        Expr::div(Value::Int(1), lv2_var!(n)),
    )));

    builder.add(ENTRY_POINT).hir(hir);

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();
    let result = vm.call(ENTRY_POINT, &[]).unwrap();

    assert_eq!(Value::Bool(true), result.clone());
}

#[test]
fn compute_constants() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Return::value(Expr::rem(
        Expr::mul(Expr::add(Expr::sub(6, 1), 2), Expr::div(4, 2)),
        5,
    )));

    builder.add(ENTRY_POINT).hir(hir);

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

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();
    let result = vm.call(ENTRY_POINT, &[]).unwrap();

    assert_eq!(expected, result.clone());
}

#[test]
fn dead_code_elimination_else_branche() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local(lv2_var!(n), 3));

    let mut branch = Branch::new();

    branch
        .add_condition(Expr::eq(Expr::rem(lv2_var!(n), 2), 0))
        .push(Assign::local(lv2_var!(y), 0));

    // this condition will always be met
    branch
        .add_condition(Expr::not(false))
        .push(Assign::local(lv2_var!(y), 1));

    // this code will never be reached
    branch
        .default_condition()
        .push(Assign::local(lv2_var!(y), 7));

    hir.push(branch);

    hir.push(Return::value(lv2_var!(y)));

    builder.add(ENTRY_POINT).hir(hir);

    let module = builder.build().unwrap();
    println!("{}", module);

    assert_eq!(16, module.code_object.code.len());
    assert_eq!(4, module.code_object.consts.len());
    assert!(!module.code_object.consts.contains(&Value::Int(7)));

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();
    let result = vm.call(ENTRY_POINT, &[]).unwrap();

    assert_eq!(Value::Int(1), result.clone());
}

#[test]
fn compile_options() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Return::value(Expr::add(Expr::mul(3, 2), 2)));

    builder.add(ENTRY_POINT).hir(hir);

    let module = builder.clone().build().unwrap();
    let module_noop = builder
        .build_with_options(CompileOptions { optimize: false })
        .unwrap();

    assert!(module.code_object.code.len() < module_noop.code_object.code.len());
}
