use lovm2::create_vm_with_std;
use lovm2::prelude::*;
use lovm2::value::LV2Value;
use lovm2::Instruction;

fn check_instruction_elimination(expr: LV2Expr) {
    let mut builder = LV2ModuleBuilder::new();

    builder.entry().return_value(expr);

    let module = builder.clone().build().unwrap();
    let module_noop = builder
        .build_with_options(CompileOptions { optimize: false })
        .unwrap();

    assert!(module.code_object.code.len() < module_noop.code_object.code.len());
}

#[test]
fn merge_not_jump_false() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();
    let branch = builder.entry().assign(n, 0).branch();

    branch
        .add_condition(LV2Expr::not(LV2Expr::from(n).eq(2)))
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

    assert_eq!(LV2Value::Int(1), result.clone());
}

#[test]
fn merge_constant_jump() {
    let mut builder = LV2ModuleBuilder::named("main");
    let branch = builder.entry().branch();

    branch
        .add_condition(LV2Expr::from(false).not())
        .return_value(1);
    branch.default_condition().return_value(2);

    let module = builder.build().unwrap();
    println!("{}", module);

    assert!(!module
        .code_object
        .code
        .iter()
        .any(|c| matches!(c, Instruction::Jt(_) | Instruction::Jf(_))));

    // `false` is constant and should be eliminated
    assert!(!module.code_object.consts.contains(&LV2Value::Bool(false)));
    // check if dead code elimination is working
    assert_eq!(2, module.code_object.code.len());

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(LV2Value::Int(1), result.clone());
}

#[test]
fn short_circuit_and() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();
    let div_by_null = LV2Expr::from(1).div(n);

    builder
        .entry()
        .assign(n, 0)
        .return_value(LV2Expr::from(n).eq(1).and(div_by_null));

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(LV2Value::Bool(false), result.clone());
}

#[test]
fn short_circuit_or() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();
    let div_by_null = LV2Expr::from(1).div(n);

    builder
        .entry()
        .assign(n, 0)
        .return_value(LV2Expr::from(n).eq(0).or(div_by_null));

    let module = builder.build().unwrap();
    println!("{}", module);

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(LV2Value::Bool(true), result.clone());
}

#[test]
fn compute_constants() {
    let mut builder = LV2ModuleBuilder::new();

    builder.entry().return_value(LV2Expr::rem(
        LV2Expr::from(6).sub(1).add(2).mul(LV2Expr::from(4).div(2)),
        5,
    ));

    let module = builder.build().unwrap();
    println!("{}", module);

    assert!(!module
        .code_object
        .code
        .iter()
        .any(|c| matches!(c, Instruction::Jt(_) | Instruction::Jf(_))));

    let expected = LV2Value::Int(4);
    assert_eq!(1, module.code_object.consts.len());
    assert!(module.code_object.consts.contains(&expected));

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(expected, result.clone());
}

#[test]
fn dead_code_elimination_else_branche() {
    let (n, y) = &lv2_var!(n, y);
    let mut builder = LV2ModuleBuilder::new();

    let hir = builder.entry().assign(n, 3);

    let branch = hir.branch();
    branch
        .add_condition(LV2Expr::from(n).rem(2).eq(0))
        .assign(y, 0);

    // this condition will always be met
    branch
        .add_condition(LV2Expr::from(false).not())
        .assign(y, 1);

    // this code will never be reached
    branch.default_condition().assign(y, 7);

    hir.return_value(y);

    let module = builder.build().unwrap();
    println!("{}", module);

    assert_eq!(16, module.code_object.code.len());
    assert_eq!(4, module.code_object.consts.len());
    assert!(!module.code_object.consts.contains(&LV2Value::Int(7)));

    let mut vm = create_vm_with_std();
    vm.add_main_module(module).unwrap();
    let result = vm.run().unwrap();

    assert_eq!(LV2Value::Int(1), result.clone());
}

#[test]
fn compile_options() {
    check_instruction_elimination(LV2Expr::from(3).mul(2).add(2));
}

#[test]
fn flip_comparison_operator() {
    let x = &lv2_var!(x);

    check_instruction_elimination(LV2Expr::from(x).eq(5).not());
    check_instruction_elimination(LV2Expr::from(x).ne(5).not());
    check_instruction_elimination(LV2Expr::from(x).gt(5).not());
    check_instruction_elimination(LV2Expr::from(x).ge(5).not());
    check_instruction_elimination(LV2Expr::from(x).lt(5).not());
    check_instruction_elimination(LV2Expr::from(x).le(5).not());
}

#[test]
fn eliminate_double_not() {
    let x = &lv2_var!(x);

    check_instruction_elimination(LV2Expr::from(x).not().not());
}
