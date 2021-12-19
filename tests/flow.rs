use lovm2::create_vm_with_std;
use lovm2::prelude::*;
use lovm2::value::LV2Value;
use lovm2::Instruction;

/// Define `CodeObject` on a low-level basis
#[macro_export]
macro_rules! define_code {
    {
        $(consts {$($cval:expr),*})?
        $(idents {$($name:ident),*})?
        {
            $( $inx:ident $($args:expr),* ; )*
        }
    } => {{
        let mut co = lovm2::code::CodeObject::new();
        $( co.idents = vec![$( LV2Variable::from(stringify!($name)) ),*]; )?
        $( co.consts = vec![$( LV2Value::from($cval) ),*]; )?

        let c = vec![
            $(
                define_code! { compile_inx $inx $(, $args)* },
            )*
        ];

        co.code = c;
        co
    }};

    { compile_inx $inx:ident } => { Instruction::$inx };
    { compile_inx $inx:ident $(, $args:expr)+ } => { Instruction::$inx($($args),*) };
}

#[test]
fn pushing_constant() {
    let mut vm = create_vm_with_std();
    let co = define_code! {
        consts { 2 }

        {
            CPush 0;
        }
    };

    let result = vm.run_object(&co).unwrap();

    assert!(vm.context_mut().stack_mut().is_empty());
    assert_eq!(LV2Value::Int(2), result);
}

#[test]
fn store_global() {
    let mut vm = create_vm_with_std();
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
        LV2Value::Int(42),
        *vm.context_mut().value_of("globaln").unwrap()
    );
}

#[test]
fn calculation() {
    let mut vm = create_vm_with_std();
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
        LV2Value::Int(5),
        *vm.context_mut().value_of("result_add").unwrap()
    );
    assert_eq!(
        LV2Value::Int(1),
        *vm.context_mut().value_of("result_sub").unwrap()
    );
    assert_eq!(
        LV2Value::Int(6),
        *vm.context_mut().value_of("result_mul").unwrap()
    );
    assert_eq!(
        LV2Value::Int(1),
        *vm.context_mut().value_of("result_div").unwrap()
    );
}

#[test]
fn jumping() {
    let mut vm = create_vm_with_std();
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
        LV2Value::Str("aaaaaaaaaa".to_string()),
        *vm.context_mut().value_of("output").unwrap()
    );
}

#[test]
#[ignore]
fn assign_segfault() {
    use lovm2::value::box_value;
    use lovm2::value::LV2Value::*;

    // this code creates a reference counted dict and tries setting itself as an
    // attribute resulting in an endless deref cycle and eventually a stackoverflow.

    let mut vm = create_vm_with_std();
    let co = define_code! {
        consts { Nil, box_value(LV2Value::dict()), "b", 1, "sub", "a", 2 }
        idents { main, ret, obj1 }

        {
        CPush(1);
        LMove(2);
        LPush(2);
        CPush(2);
        RGet;
        CPush(3);
        Set;

        LPush(2);
        CPush(4);
        RGet;
        CPush(1);
            Set;

        LPush(2);
        CPush(4);
        RGet;
        CPush(5);
        RGet;
        CPush(6);
            Set;

        LPush(2);
        Ret;
        }
    };

    let result = vm.run_object(&co).unwrap();

    assert!(vm.context_mut().stack_mut().is_empty());
    assert_eq!(LV2Value::Int(2), result);
}
