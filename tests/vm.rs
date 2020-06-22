use lovm2::{CodeObjectBuilder, CoValue, define_code, Instruction, RuValue, Variable, vm::Vm};

#[test]
fn pushing_constant() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { CoValue::Int(2) }

        {
            Pushc 0, 0;
        }
    };

    vm.run_object(&co);

    assert_eq!(1, vm.context_mut().stack_mut().len());
    assert_eq!(RuValue::Int(2), vm.context_mut().pop_value().unwrap());
}

#[test]
fn store_global() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { CoValue::Int(42) }
        globals { globaln }

        {
            Pushc 0, 0;
            Moveg 0, 0;
        }
    };

    vm.run_object(&co);

    assert_eq!(RuValue::Int(42), vm.context_mut().globals.get("globaln").cloned().unwrap());
}
