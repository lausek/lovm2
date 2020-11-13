use lovm2::bytecode::Instruction;
use lovm2::define_code;
use lovm2::hir::prelude::*;
use lovm2::value::Value;
use lovm2::var::Variable;
use lovm2::vm::Vm;

#[test]
fn pushing_constant() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { 2 }

        {
            Pushc 0;
        }
    };

    vm.run_object(&co).unwrap();

    assert_eq!(1, vm.context_mut().stack_mut().len());
    assert_eq!(Value::Int(2), vm.context_mut().pop_value().unwrap());
}

#[test]
fn store_global() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { 42 }
        idents { globaln }

        {
            Pushc 0;
            Moveg 0;
        }
    };

    vm.run_object(&co).unwrap();

    assert_eq!(
        Value::Int(42),
        vm.context_mut().value_of(&lv2_var!(globaln)).unwrap()
    );
}

#[test]
fn calculation() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { 2, 3 }
        idents { result_add, result_sub, result_mul, result_div }

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

    vm.run_object(&co).unwrap();

    assert_eq!(
        Value::Int(5),
        vm.context_mut().value_of(&lv2_var!(result_add)).unwrap()
    );
    assert_eq!(
        Value::Int(1),
        vm.context_mut().value_of(&lv2_var!(result_sub)).unwrap()
    );
    assert_eq!(
        Value::Int(6),
        vm.context_mut().value_of(&lv2_var!(result_mul)).unwrap()
    );
    assert_eq!(
        Value::Int(1),
        vm.context_mut().value_of(&lv2_var!(result_div)).unwrap()
    );
}

#[test]
fn jumping() {
    let mut vm = Vm::new();
    let co = define_code! {
        consts { 0, 1, 10, "a" }
        idents { i, output }

        {
            Pushc 1;
            Movel 0;

            Pushc 3;
            Moveg 1;

            Pushl 0;
            Pushc 1;
            Add;

            Pushg 1;
            Pushc 3;
            Add;
            Moveg 1;

            Dup;
            Movel 0;

            Pushc 2;
            Eq;
            Jt 17;

            Jmp 4;
        }
    };

    vm.run_object(&co).unwrap();

    assert_eq!(
        Value::Str("aaaaaaaaaa".to_string()),
        vm.context_mut().value_of(&lv2_var!(output)).unwrap()
    );
}
