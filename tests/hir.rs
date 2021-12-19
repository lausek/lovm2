#![allow(unused_parens)]

use lovm2::create_vm_with_std;
use lovm2::prelude::*;
use lovm2::vm::{LV2Context, LV2Vm};

use test_utils::*;

#[test]
fn assign_local() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();

    builder.entry().assign(n, 4).trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(4), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn assign_local_add() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(n, 2)
        .assign(n, LV2Expr::from(n).add(2))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(4), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn assign_incremet_decrement() {
    let (a, b) = &lv2_var!(a, b);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(a, 0)
        .global(b)
        .assign(b, 1)
        .increment(a)
        .decrement(b)
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        assert_eq!(LV2Value::Int(0), *ctx.value_of("b").unwrap());

        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(1), *frame.value_of("a").unwrap());
    })
    .unwrap();
}

#[test]
fn rem_lowering() {
    let rest = &lv2_var!(rest);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(rest, LV2Expr::from(1).rem(2))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(1), *frame.value_of("rest").unwrap());
    })
    .unwrap();
}

#[test]
fn easy_loop() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();

    let main_hir = builder.entry();
    main_hir.assign(n, 0);
    main_hir
        .repeat_until(LV2Expr::from(n).eq(10))
        .step(lv2_call!(print, n))
        .increment(n);
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(10), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn explicit_break() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();

    let main_hir = builder.entry();
    main_hir.assign(n, 0);
    main_hir.repeat().increment(n).break_repeat();
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(1), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn try_getting() {
    let (dict, dat0, list, lat0) = &lv2_var!(dict, dat0, list, lat0);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(dict, lv2_dict!(0 => 6, 1 => 7))
        .assign(dat0, lv2_access!(dict, 1))
        .assign(list, lv2_list!("a", 10, 20., true))
        .assign(lat0, lv2_access!(list, 1))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(7), *frame.value_of("dat0").unwrap());
        assert_eq!(LV2Value::Int(10), *frame.value_of("lat0").unwrap());
    })
    .unwrap();
}

#[test]
fn try_setting() {
    let list = &lv2_var!(list);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(list, lv2_list!("a", 10, 20., true))
        .set(lv2_access!(list, 1), 7)
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        let list = &frame.value_of("list").unwrap();
        assert_eq!(LV2Value::Int(7), list.get(&LV2Value::Int(1)).unwrap());
    })
    .unwrap();
}

#[test]
fn try_retrieving_len() {
    let (dict, ls, lendict, lenls) = &lv2_var!(dict, ls, lendict, lenls);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(dict, lv2_dict!(0 => 6, 1 => 7))
        .assign(ls, lv2_list!(1, 2, 3))
        .assign(lendict, lv2_call!(len, dict))
        .assign(lenls, lv2_call!(len, ls))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(2), *frame.value_of("lendict").unwrap());
        assert_eq!(LV2Value::Int(3), *frame.value_of("lenls").unwrap());
    })
    .unwrap();
}

#[test]
fn try_casting() {
    let n = &lv2_var!(n);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(n, LV2Expr::from(5.).to_integer())
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(5), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn true_branching() {
    let mut builder = LV2ModuleBuilder::new();
    let hir = builder.entry();
    let n = lv2_var!(n);

    hir.assign(&n, 0);

    let branch = hir.branch();
    branch
        .add_condition(LV2Expr::from(false).not())
        .assign(&n, 2);
    branch.default_condition().assign(&n, 1);

    hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(2), *frame.value_of(&n).unwrap());
    })
    .unwrap();
}

#[test]
fn multiple_branches() {
    let mut builder = LV2ModuleBuilder::new();
    let hir = builder.entry();
    let (result, n) = &lv2_var!(result, n);

    hir.assign(n, 5);

    let branch = hir.branch();
    branch
        .add_condition(LV2Expr::from(n).rem(3).eq(0))
        .assign(result, "fizz");
    branch
        .add_condition(LV2Expr::from(n).rem(5).eq(0))
        .assign(result, "buzz");
    branch.default_condition().assign(result, "none");

    hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::from("buzz"), *frame.value_of("result").unwrap());
    })
    .unwrap();
}

#[test]
fn taking_parameters() {
    let mut builder = LV2ModuleBuilder::new();
    let (a, b) = lv2_var!(a, b);

    builder
        .add_with_args("called", vec![a.clone(), b.clone()])
        .trigger(10);

    builder.entry().step(lv2_call!(called, 2, 7));

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(2), *frame.value_of(&a).unwrap());
        assert_eq!(LV2Value::Int(7), *frame.value_of(&b).unwrap());
    })
    .unwrap();
}

#[test]
fn automatic_return() {
    let mut vm = LV2Vm::new();
    let mut builder = LV2ModuleBuilder::new();

    builder.entry();
    builder.add("no-return");

    let module = builder.build().unwrap();

    vm.add_main_module(module).unwrap();

    assert_eq!(LV2Value::Nil, vm.call("no-return", &[]).unwrap());
}

#[test]
fn return_values() {
    let mut builder = LV2ModuleBuilder::new();
    let n = lv2_var!(n);

    builder.add("returner").return_value(10);

    builder
        .entry()
        .assign(&n, LV2Call::new("returner"))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(10), *frame.value_of(&n).unwrap());
    })
    .unwrap();
}

#[test]
fn drop_call_values() {
    let mut builder = LV2ModuleBuilder::new();

    let _ = builder.add("returner");

    builder.entry().step(LV2Call::new("returner")).trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        assert!(ctx.last_value_mut().is_err());
    })
    .unwrap();
}

#[test]
fn cast_to_string() {
    let (a, b, c, d) = &lv2_var!(a, b, c, d);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(a, LV2Expr::from(10).to_str())
        .assign(b, LV2Expr::from(10.1).to_str())
        .assign(c, LV2Expr::from("10").to_str())
        .assign(d, LV2Expr::from(true).to_str())
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::from("10"), *frame.value_of("a").unwrap());
        assert_eq!(LV2Value::from("10.1"), *frame.value_of("b").unwrap());
        assert_eq!(LV2Value::from("10"), *frame.value_of("c").unwrap());
        assert_eq!(LV2Value::from("true"), *frame.value_of("d").unwrap());
    })
    .unwrap();
}

#[test]
fn folding_expr() {
    let mut builder = LV2ModuleBuilder::new();

    let main = builder.entry();
    let (a, n) = lv2_var!(a, n);

    main.global(&a)
        .assign(
            &a,
            LV2Expr::from_opn(LV2Operator2::Div, vec![8.into(), 4.into()]),
        )
        .global(&n)
        .assign(
            &n,
            LV2Expr::from_opn(LV2Operator2::Div, vec![8.into(), 4.into(), 2.into()]),
        )
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let a = ctx.value_of(&a).unwrap();
        let n = ctx.value_of(&n).unwrap();
        assert_eq!(LV2Value::Int(2), *a);
        assert_eq!(LV2Value::Int(1), *n);
    })
    .unwrap();
}

#[test]
fn get_field_from_dict() {
    let (x, y, z, d1, d2, g) = &lv2_var!(x, y, z, d1, d2, g);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(d1, lv2_dict!("x" => 37))
        .assign(d2, lv2_dict!("x" => lv2_dict!("y" => 42)))
        .global(g)
        .assign(g, lv2_dict!("x" => 67))
        .assign(x, lv2_access!(d1, "x"))
        .assign(y, lv2_access!(d2, "x", "y"))
        .assign(z, lv2_access!(g, "x"))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(37), *frame.value_of("x").unwrap());
        assert_eq!(LV2Value::Int(42), *frame.value_of("y").unwrap());
        assert_eq!(LV2Value::Int(67), *frame.value_of("z").unwrap());
    })
    .unwrap();
}

#[test]
fn set_field_on_dict() {
    let (d1, d2, g) = &lv2_var!(d1, d2, g);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(d1, lv2_dict!())
        .assign(d2, lv2_dict!("x" => lv2_dict!()))
        .global(g)
        .assign(g, lv2_dict!())
        .set(lv2_access!(d1, "x"), 37)
        .set(lv2_access!(d2, "x", "y"), 42)
        .set(lv2_access!(g, "x"), 67)
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            LV2Value::Int(37),
            frame
                .value_of("d1")
                .unwrap()
                .get(&LV2Value::from("x"))
                .unwrap()
        );
        assert!(frame
            .value_of("d2")
            .unwrap()
            .get(&LV2Value::from("x"))
            .unwrap()
            .is_ref());
        assert_eq!(
            LV2Value::Int(42),
            frame
                .value_of("d2")
                .unwrap()
                .get(&LV2Value::from("x"))
                .unwrap()
                .get(&LV2Value::from("y"))
                .unwrap()
        );
        assert_eq!(
            LV2Value::Int(67),
            ctx.value_of("g")
                .unwrap()
                .get(&LV2Value::from("x"))
                .unwrap()
        );
    })
    .unwrap();
}

#[test]
fn is_constant() {
    assert!(!LV2Expr::from(lv2_var!(n)).is_const());
    assert!(LV2Expr::from(1).add(2).is_const());
    assert!(LV2Expr::from("abc").is_const());
    assert!(LV2Expr::from(10).is_const());
}

#[test]
fn call_into_vm() {
    let mut builder = LV2ModuleBuilder::named("main");
    builder.entry().step(lv2_call!(call_me, 10));

    builder
        .add_with_args("call_me", vec![lv2_var!(n)])
        .trigger(10);

    let module = builder.build().unwrap();

    // ensure that the interrupt has been called
    run_module_test(create_vm_with_std(), module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(10), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn comparison() {
    let (lt, le1, le2, gt, ge1, ge2) = &lv2_var!(lt, le1, le2, gt, ge1, ge2);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(lt, LV2Expr::from(2).lt(3))
        .assign(le1, LV2Expr::from(2).le(3))
        .assign(le2, LV2Expr::from(2).le(2))
        .assign(gt, LV2Expr::from(3).gt(2))
        .assign(ge1, LV2Expr::from(3).ge(2))
        .assign(ge2, LV2Expr::from(3).ge(3))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Bool(true), *frame.value_of("lt").unwrap());
        assert_eq!(LV2Value::Bool(true), *frame.value_of("le1").unwrap());
        assert_eq!(LV2Value::Bool(true), *frame.value_of("le2").unwrap());
        assert_eq!(LV2Value::Bool(true), *frame.value_of("gt").unwrap());
        assert_eq!(LV2Value::Bool(true), *frame.value_of("ge1").unwrap());
        assert_eq!(LV2Value::Bool(true), *frame.value_of("ge2").unwrap());
    })
    .unwrap();
}

#[test]
fn raise_to_power() {
    let (a, b) = &lv2_var!(a, b);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(a, LV2Expr::from(2).pow(3))
        .assign(b, LV2Expr::from(3.).pow(3.))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(8), *frame.value_of("a").unwrap());
        assert_eq!(LV2Value::Float(27.), *frame.value_of("b").unwrap());
    })
    .unwrap();
}

#[test]
fn initialize_objects() {
    let (n, ae, ag, be, bg) = &lv2_var!(n, ae, ag, be, bg);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(n, 2)
        .assign(ae, lv2_list!(1, 2, 3))
        .assign(ag, lv2_list!(1, n, 3))
        .assign(be, lv2_dict!(1 => 2, 2 => 2, 4 => 4))
        .assign(bg, lv2_dict!(1 => 2, n => n, 4 => 4))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        let ae = frame.value_of("ae").unwrap();
        let ag = frame.value_of("ag").unwrap();
        let be = frame.value_of("be").unwrap();
        let bg = frame.value_of("bg").unwrap();
        assert_eq!(ae, ag);
        assert_eq!(be, bg);
    })
    .unwrap();
}

#[test]
fn store_without_reference() {
    let (n, x, y) = &lv2_var!(n, x, y);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(n, 2)
        .assign(x, LV2Expr::from(5).boxed())
        .assign(y, x)
        .set(y, 7)
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::Int(2), *frame.value_of("n").unwrap());
        assert_eq!(LV2Value::Int(7), *frame.value_of("y").unwrap());
    })
    .unwrap();
}

#[test]
fn create_slice() {
    let (ls, s) = &lv2_var!(ls, s);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .assign(ls, lv2_list!(1, 2, 3, 4, 5))
        .assign(s, LV2Expr::from(ls).slice(1, 4))
        .set(lv2_access!(s, 1), 9)
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        let ls = frame.value_of("ls").unwrap();
        let s = frame.value_of("s").unwrap();
        assert_eq!(LV2Value::Int(9), s.get(&LV2Value::Int(1)).unwrap());
        assert_eq!(LV2Value::Int(9), ls.get(&LV2Value::Int(2)).unwrap());
    })
    .unwrap();
}

#[test]
fn iterating_repeat() {
    fn check(ctx: &mut LV2Context) {
        assert_eq!(LV2Value::from(10), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
    }

    let mut builder = LV2ModuleBuilder::new();
    let (sum, i, iter) = &lv2_var!(sum, i, iter);

    let main_hir = builder.entry();

    main_hir.global(sum).assign(sum, 0);
    main_hir
        .assign(iter, lv2_list!(1, 2, 3, 4).to_iter())
        .repeat_iterating(iter, i)
        .global(sum)
        .assign(sum, LV2Expr::from(sum).add(i));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn iterating_repeat_inplace() {
    fn check(ctx: &mut LV2Context) {
        assert_eq!(LV2Value::from(10), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
        assert_eq!(ctx.value_of("orig").unwrap(), ctx.value_of("ls").unwrap());
    }

    let mut builder = LV2ModuleBuilder::new();
    let (sum, i, ls, orig) = &lv2_var!(sum, i, ls, orig);

    let main_hir = builder.entry();

    main_hir.global(sum).assign(sum, 0);
    main_hir.global(orig).assign(orig, lv2_list!(1, 2, 3, 4));
    main_hir.global(ls).assign(ls, lv2_list!(1, 2, 3, 4));
    main_hir
        .repeat_iterating(ls, i)
        .global(sum)
        .assign(sum, LV2Expr::from(sum).add(i));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn iterating_repeat_ranged() {
    fn check(ctx: &mut LV2Context) {
        assert_eq!(LV2Value::from(45), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
    }

    let mut builder = LV2ModuleBuilder::new();
    let (sum, i) = &lv2_var!(sum, i);

    let main_hir = builder.entry();

    main_hir.global(sum).assign(sum, 0);
    main_hir
        .repeat_iterating(LV2Expr::iter_ranged(LV2Value::Nil, 10), i)
        .global(sum)
        .assign(sum, LV2Expr::from(sum).add(i));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn iterating_repeat_nested() {
    fn check(ctx: &mut LV2Context) {
        assert_eq!(LV2Value::from(17199), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
    }

    let mut builder = LV2ModuleBuilder::new();
    let (sum, i, j) = &lv2_var!(sum, i, j);

    let main_hir = builder.entry();

    main_hir.global(sum).assign(sum, 0);
    main_hir
        .repeat_iterating(LV2Expr::iter_ranged(0, 5), i)
        .repeat_iterating(LV2Expr::iter_ranged(5, 10), j)
        .global(sum)
        .assign(sum, LV2Expr::from(sum).add(LV2Expr::from(j).pow(i)));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn shift_values() {
    fn check(ctx: &mut LV2Context) {
        assert_eq!(LV2Value::from(4), *ctx.value_of("a").unwrap());
        assert_eq!(LV2Value::from(8), *ctx.value_of("b").unwrap());
        assert_eq!(LV2Value::from(0b10100000), *ctx.value_of("c").unwrap());
        assert_eq!(LV2Value::from(0), *ctx.value_of("d").unwrap());
    }

    let (a, b, c, d) = &lv2_var!(a, b, c, d);
    let mut builder = LV2ModuleBuilder::new();

    builder
        .entry()
        .global(a)
        .assign(a, LV2Expr::from(2).shl(1))
        .global(b)
        .assign(b, LV2Expr::from(16).shr(1))
        .global(c)
        .assign(c, LV2Expr::from(0b00001010).shl(4))
        .global(d)
        .assign(d, LV2Expr::from(0b0001010).shr(4))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn conditional_expression() {
    fn check(ctx: &mut LV2Context) {
        assert_eq!(LV2Value::from(true), *ctx.value_of("x").unwrap());
        assert_eq!(LV2Value::from(false), *ctx.value_of("y").unwrap());
    }

    let mut builder = LV2ModuleBuilder::new();
    let (x, y, z) = &lv2_var!(x, y, z);

    builder
        .entry()
        .assign(z, 2)
        .global(x)
        .assign(
            x,
            LV2Expr::branch()
                .add_condition(LV2Expr::from(z).eq(1), false)
                .default_value(true),
        )
        .global(y)
        .assign(
            y,
            LV2Expr::branch()
                .add_condition(LV2Expr::from(z).eq(2), false)
                .default_value(true),
        )
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn variable_scoping() {
    fn check(ctx: &mut LV2Context) {
        assert_eq!(LV2Value::from(1), *ctx.value_of("x").unwrap());

        let frame = ctx.frame_mut().unwrap();
        assert_eq!(LV2Value::from(2), *frame.value_of("x").unwrap());
        assert_eq!(LV2Value::from(true), *frame.value_of("y").unwrap());
    }

    let mut builder = LV2ModuleBuilder::new();
    let (x, y) = &lv2_var!(x, y);

    builder
        .entry()
        .assign(y, true)
        .global(x)
        .assign(x, 1)
        .local(x)
        .assign(x, 2)
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn iterator_has_next() {
    let mut builder = LV2ModuleBuilder::new();
    let (y, n, it) = &lv2_var!(y, n, it);

    builder
        .entry()
        .assign(it, LV2Expr::iter_ranged(0, 1))
        .global(y)
        .assign(y, LV2Expr::from(it).has_next())
        .step(LV2Expr::from(it).next())
        .global(n)
        .assign(n, LV2Expr::from(it).has_next())
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        assert_eq!(LV2Value::from(true), *ctx.value_of("y").unwrap());
        assert_eq!(LV2Value::from(false), *ctx.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn iterator_reverse() {
    let mut builder = LV2ModuleBuilder::new();
    let (y, n, it) = &lv2_var!(y, n, it);

    builder
        .entry()
        .assign(it, LV2Expr::iter_ranged(0, 2).reverse())
        .global(y)
        .assign(y, LV2Expr::from(it).next())
        .global(n)
        .assign(n, LV2Expr::from(it).next())
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        assert_eq!(LV2Value::from(1), *ctx.value_of("y").unwrap());
        assert_eq!(LV2Value::from(0), *ctx.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn iterators() {
    let mut builder = LV2ModuleBuilder::new();
    let (x, y, it) = &lv2_var!(x, y, it);

    builder
        .entry()
        .assign(x, LV2Expr::list().append(1).append(2).append(3))
        .global(y)
        .assign(y, LV2Expr::list())
        .assign(it, LV2Expr::from(x).to_iter().reverse())
        .repeat_until(LV2Expr::not(LV2Expr::from(it).has_next()))
        .step(LV2Expr::from(y).append(LV2Expr::from(it).next()));

    builder.entry().trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let expected = vec![3, 2, 1];
        assert_eq!(LV2Value::from(expected), *ctx.value_of("y").unwrap());
    })
    .unwrap();
}
