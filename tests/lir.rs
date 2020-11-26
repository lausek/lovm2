use lovm2::bytecode::Instruction;
use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::vm::Vm;

fn run_module_test(module: Module, testfn: impl Fn(&mut Context) + 'static) {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let mut vm = Vm::with_std();
    let called_ref = called.clone();
    vm.context_mut().set_interrupt(10, move |ctx| {
        called_ref.set(true);
        testfn(ctx);
        Ok(())
    });

    println!("{:?}", module);
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    assert!(called.get());
}

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

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();
    let result = vm.call(ENTRY_POINT, &[]).unwrap();

    assert_eq!(Value::Int(1), result.clone());
}
