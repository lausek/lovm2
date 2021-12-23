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
            .assign(args, lv2_call!(argn))
            .assign(result, 0);

        hir.repeat_until(LV2Expr::from(args).eq(0))
            .assign(arg, lv2_call!(pop_vstack))
            .assign(result, LV2Expr::from(result).add(arg))
            .decrement(args);

        hir.return_value(result);
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
        let hir = builder.add("return_argn").assign(n, lv2_call!(argn));

        hir.assign(i, n)
            .repeat_until(LV2Expr::from(i).eq(0))
            .step(lv2_call!(pop_vstack))
            .decrement(i);

        hir.return_value(n);
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
