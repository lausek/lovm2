use lovm2::create_vm_with_std;
use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::Instruction;

fn check_instruction_elimination(expr: Expr) {
    let mut builder = ModuleBuilder::new();

    builder.entry().return_value(expr);

    let module = builder.clone().build().unwrap();
    let module_noop = builder
        .build_with_options(CompileOptions { optimize: false })
        .unwrap();

    assert!(module.code_object.code.len() < module_noop.code_object.code.len());
}

#[test]
fn merge_not_jump_false() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    let n = &lv2_var!(n);

    hir.assign(n, 0);

    let branch = hir.branch();
    branch
        .add_condition(Expr::not(Expr::eq(n, 2)))
        .return_value(1);
    branch.default_condition().return_value(2);

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
    branch.add_condition(Expr::not(false)).return_value(1);
    branch.default_condition().return_value(2);

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

    hir.assign(n, 0);
    hir.return_value(Expr::and(Expr::eq(n, 1), Expr::div(1, n)));

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

    hir.assign(n, 0);
    hir.return_value(Expr::or(Expr::eq(n, 0), Expr::div(1, n)));

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

    hir.return_value(Expr::rem(
        Expr::mul(Expr::add(Expr::sub(6, 1), 2), Expr::div(4, 2)),
        5,
    ));

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

    hir.step(Assign::var(n, 3));

    let branch = hir.branch();
    branch
        .add_condition(Expr::eq(Expr::rem(n, 2), 0))
        .step(Assign::var(y, 0));

    // this condition will always be met
    branch
        .add_condition(Expr::not(false))
        .step(Assign::var(y, 1));

    // this code will never be reached
    branch.default_condition().step(Assign::var(y, 7));

    hir.return_value(y);

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
    check_instruction_elimination(Expr::add(Expr::mul(3, 2), 2));
}

#[test]
fn flip_comparison_operator() {
    let x = &lv2_var!(x);

    check_instruction_elimination(Expr::not(Expr::eq(x, 5)));
    check_instruction_elimination(Expr::not(Expr::ne(x, 5)));
    check_instruction_elimination(Expr::not(Expr::gt(x, 5)));
    check_instruction_elimination(Expr::not(Expr::ge(x, 5)));
    check_instruction_elimination(Expr::not(Expr::lt(x, 5)));
    check_instruction_elimination(Expr::not(Expr::le(x, 5)));
}

#[test]
fn eliminate_double_not() {
    let x = &lv2_var!(x);

    check_instruction_elimination(Expr::not(Expr::not(x)));
}
