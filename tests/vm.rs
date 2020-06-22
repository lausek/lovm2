use lovm2::{CodeObjectBuilder, CoValue, Instruction, RuValue, Variable, vm::Vm};

#[test]
fn pushing_constant() {
    let mut vm = Vm::new();
    let co = CodeObjectBuilder::new()
        .consts(
            vec![
                CoValue::Int(2),
            ]
        )
        .code(
            vec![
                Instruction::Pushc as u8,
                0, 0,
            ]
        )
        .build()
        .unwrap();

    vm.run_object(&co);

    assert_eq!(1, vm.context_mut().stack_mut().len());
    assert_eq!(RuValue::Int(2), vm.context_mut().pop_value().unwrap());
}

#[test]
fn store_global() {
    let mut vm = Vm::new();
    let co = CodeObjectBuilder::new()
        .consts(
            vec![
                CoValue::Int(42),
            ]
        )
        .globals(
            vec![
                Variable::from("globaln"),
            ]
        )
        .code(
            vec![
                Instruction::Pushc as u8,
                0, 0,
                Instruction::Moveg as u8,
                0, 0,
            ]
        )
        .build()
        .unwrap();

    vm.run_object(&co);

    assert_eq!(RuValue::Int(42), vm.context_mut().globals.get("globaln").cloned().unwrap());
}
