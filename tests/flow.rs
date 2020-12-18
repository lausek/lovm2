use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::vm::Vm;
use lovm2::{define_code, Instruction};

#[test]
fn pushing_constant() {
    let mut vm = Vm::with_std();
    let co = define_code! {
        consts { 2 }

        {
            CPush 0;
        }
    };

    let result = vm.run_object(&co).unwrap();

    assert!(vm.context_mut().stack_mut().is_empty());
    assert_eq!(Value::Int(2), result);
}

#[test]
fn store_global() {
    let mut vm = Vm::with_std();
    let co = define_code! {
        consts { 42 }
        idents { globaln }

        {
            CPush 0;
            GMove 0;
            CPush 0;
        }
    };

    vm.run_object(&co).unwrap();

    assert_eq!(
        Value::Int(42),
        *vm.context_mut().value_of("globaln").unwrap()
    );
}

#[test]
fn calculation() {
    let mut vm = Vm::with_std();
    let co = define_code! {
        consts { 2, 3 }
        idents { result_add, result_sub, result_mul, result_div }

        {
            CPush 1;
            CPush 0;
            Add;
            GMove 0;

            CPush 1;
            CPush 0;
            Sub;
            GMove 1;

            CPush 1;
            CPush 0;
            Mul;
            GMove 2;

            CPush 1;
            CPush 0;
            Div;

            GMove 3;
            CPush 0;
        }
    };

    vm.run_object(&co).unwrap();

    assert_eq!(
        Value::Int(5),
        *vm.context_mut().value_of("result_add").unwrap()
    );
    assert_eq!(
        Value::Int(1),
        *vm.context_mut().value_of("result_sub").unwrap()
    );
    assert_eq!(
        Value::Int(6),
        *vm.context_mut().value_of("result_mul").unwrap()
    );
    assert_eq!(
        Value::Int(1),
        *vm.context_mut().value_of("result_div").unwrap()
    );
}

#[test]
fn jumping() {
    let mut vm = Vm::with_std();
    let co = define_code! {
        consts { 0, 1, 10, "a" }
        idents { i, output }

        {
            CPush 1;
            LMove 0;

            CPush 3;
            GMove 1;

            LPush 0;
            CPush 1;
            Add;

            GPush 1;
            CPush 3;
            Add;
            GMove 1;

            Dup;
            LMove 0;

            CPush 2;
            Eq;
            Jt 17;

            Jmp 4;

            CPush 0;
        }
    };

    vm.run_object(&co).unwrap();

    assert_eq!(
        Value::Str("aaaaaaaaaa".to_string()),
        *vm.context_mut().value_of("output").unwrap()
    );
}
