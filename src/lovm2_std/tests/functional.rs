#![cfg(test)]

use test_utils::*;

use lovm2_core::extend::prelude::*;

// TODO: use new api
#[test]
fn dynamic_varargs() {
    let (arg, args, result) = &lv2_var!(arg, args, result);
    let mut vm = run_module_test(|builder| {
        let hir = builder
            .add("sum")
            .step(Assign::var(args, lv2_call!(argn)))
            .step(Assign::var(result, 0));

        hir.repeat_until(LV2Expr::eq(args, 0))
            .step(Assign::var(arg, lv2_call!(pop_vstack)))
            .step(Assign::var(result, LV2Expr::add(result, arg)))
            .step(Assign::decrement(args));

        hir.step(Return::value(result));
    });

    assert_eq!(
        LV2Value::from(6),
        vm.call("sum", &[1.into(), 2.into(), 3.into()]).unwrap()
    );
    assert_eq!(
        LV2Value::from(3),
        vm.call("sum", &[1.into(), 2.into()]).unwrap()
    );
    assert_eq!(LV2Value::from(1), vm.call("sum", &[1.into()]).unwrap());
    assert_eq!(LV2Value::from(0), vm.call("sum", &[]).unwrap());
}

#[test]
fn dynamic_call() {
    let (n, i) = &lv2_var!(n, i);

    let mut vm = run_module_test(|builder| {
        let hir = builder
            .add("return_argn")
            .step(Assign::var(n, lv2_call!(argn)));

        hir.step(Assign::var(i, n))
            .repeat_until(LV2Expr::eq(i, 0))
            .step(lv2_call!(pop_vstack))
            .step(Assign::decrement(i));

        hir.step(Return::value(n));
    });

    let mut args = vec![];
    for i in 0..5 {
        assert_eq!(
            LV2Value::from(i),
            vm.call("call", &["return_argn".into(), args.clone().into()])
                .unwrap(),
        );
        args.push(LV2Value::from(i));
    }
}
