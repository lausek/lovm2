use lovm2::{CodeObjectBuilder, CoValue, define_code, Instruction, RuValue, Variable, vm::Vm};

#[test]
fn pushing_constant() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { CoValue::Int(2) }

        {
            Pushc 0;
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
            Pushc 0;
            Moveg 0;
        }
    };

    vm.run_object(&co);

    assert_eq!(RuValue::Int(42), vm.context_mut().globals.get("globaln").cloned().unwrap());
}

#[test]
fn calculation() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { CoValue::Int(2), CoValue::Int(3) }
        globals { result_add, result_sub, result_mul, result_div }

        {
            Pushc 1;
            Pushc 0;
            Add;
            Moveg 0;

            Pushc 1;
            Pushc 0;
            Sub;
            Moveg 1;
            
            Pushc 1;
            Pushc 0;
            Mul;
            Moveg 2;

            Pushc 1;
            Pushc 0;
            Div;
            Moveg 3;
        }
    };

    vm.run_object(&co);

    assert_eq!(RuValue::Int(5), vm.context_mut().globals.get("result_add").cloned().unwrap());
    assert_eq!(RuValue::Int(-1), vm.context_mut().globals.get("result_sub").cloned().unwrap());
    assert_eq!(RuValue::Int(6), vm.context_mut().globals.get("result_mul").cloned().unwrap());
    assert_eq!(RuValue::Int(0), vm.context_mut().globals.get("result_div").cloned().unwrap());
}

#[test]
fn jumping() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { CoValue::Int(0), CoValue::Int(1), CoValue::Int(10), CoValue::Str("a".to_string()) }
        globals { output }
        locals { i }

        {
            Pushc 1;
            Movel 0;

            Pushc 3;
            Moveg 0;

            Pushl 0;
            Pushc 1;
            Add;

            Pushg 0;
            Pushc 3;
            Add;
            Moveg 0;

            Dup;
            Movel 0;
            
            Pushc 2;
            Cmp;
            Jeq 17;

            Jmp 4;
        }
    };

    vm.run_object(&co);

    assert_eq!(RuValue::Str("aaaaaaaaaa".to_string()), vm.context_mut().globals.get("output").cloned().unwrap());
}
