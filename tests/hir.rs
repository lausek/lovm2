#![allow(unused_parens)]

use lovm2::create_vm_with_std;
use lovm2::prelude::*;
use lovm2::vm::{Context, Vm};

use test_utils::*;

#[macro_export]
macro_rules! define_test {
    {
        $( $fname:ident { $( $inx:expr ; )* } )*
            #ensure $ensure:tt
    } => {{
        let mut builder = ModuleBuilder::new();

        $(
            let hir = builder.add(stringify!($fname));
            $(
                hir.step($inx);
            )*
            hir.trigger(10);
        )*

        run_module_test(create_vm_with_std(), builder.build().unwrap(), $ensure).unwrap();
    }};
}

#[test]
fn assign_local() {
    let n = lv2_var!(n);
    define_test! {
        main {
            Assign::var(&n, 4);
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(4), *frame.value_of(&n).unwrap());
        })
    }
}

#[test]
fn assign_local_add() {
    let n = &lv2_var!(n);
    define_test! {
        main {
            Assign::var(n, 2);
            Assign::var(n, Expr::add(n, 2));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(4), *frame.value_of("n").unwrap());
        })
    }
}

#[test]
fn assign_incremet_decrement() {
    let (a, b) = &lv2_var!(a, b);
    define_test! {
        main {
            Assign::var(a, 0);
            Assign::global(b, 1);
            Assign::increment(a);
            Assign::decrement(b);
        }

        #ensure (|ctx: &mut Context| {
            assert_eq!(Value::Int(0), *ctx.value_of("b").unwrap());

            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(1), *frame.value_of("a").unwrap());
        })
    }
}

#[test]
fn rem_lowering() {
    let rest = &lv2_var!(rest);
    define_test! {
        main {
            Assign::var(rest, Expr::rem(1, 2));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(1), *frame.value_of("rest").unwrap());
        })
    }
}

#[test]
fn easy_loop() {
    let n = lv2_var!(n);
    define_test! {
        main {
            Assign::var(&n, 0);
            Repeat::until(Expr::eq(&n, 10))
                .step(lv2_call!(print, n))
                .step(Assign::var(&n, Expr::add(&n, 1)));
            }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(10), *frame.value_of(&n).unwrap());
        })
    }
}

#[test]
fn explicit_break() {
    let n = lv2_var!(n);
    define_test! {
        main {
            Assign::var(&n, 0);
            Repeat::endless()
                .step(Assign::var(&n, Expr::add(&n, 1)))
                .step(Break::new());
            }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(1), *frame.value_of(&n).unwrap());
        })
    }
}

#[test]
fn try_getting() {
    let (dict, dat0, list, lat0) = lv2_var!(dict, dat0, list, lat0);
    define_test! {
        main {
            Assign::var(&dict, lv2_dict!(0 => 6, 1 => 7));
            Assign::var(&dat0, lv2_access!(dict, 1));
            Assign::var(&list, lv2_list!("a", 10, 20., true));
            Assign::var(&lat0, lv2_access!(list, 1));
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(7), *frame.value_of(&dat0).unwrap());
            assert_eq!(Value::Int(10), *frame.value_of(&lat0).unwrap());
        })
    }
}

#[test]
fn try_setting() {
    let list = lv2_var!(list);
    define_test! {
        main {
            Assign::var(&list, lv2_list!("a", 10, 20., true));
            Assign::set(&lv2_access!(list, 1), 7);
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let list = &frame.value_of(&list).unwrap();
            assert_eq!(Value::Int(7), list.get(&Value::Int(1)).unwrap());
        })
    }
}

#[test]
fn try_retrieving_len() {
    let (dict, ls, lendict, lenls) = lv2_var!(dict, ls, lendict, lenls);
    define_test! {
        main {
            Assign::var(&dict, lv2_dict!(0 => 6, 1 => 7));
            Assign::var(&ls, lv2_list!(1, 2, 3));
            Assign::var(&lendict, lv2_call!(len, dict));
            Assign::var(&lenls, lv2_call!(len, ls));
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(2), *frame.value_of(&lendict).unwrap());
            assert_eq!(Value::Int(3), *frame.value_of(&lenls).unwrap());
        })
    }
}

#[test]
fn try_casting() {
    let n = lv2_var!(n);
    define_test! {
        main {
            Assign::var(&n, Conv::to_integer(5.));
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(5), *frame.value_of(&n).unwrap());
        })
    }
}

#[test]
fn true_branching() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    let n = lv2_var!(n);

    hir.step(Assign::var(&n, Value::Int(0)));

    let branch = hir.branch();
    branch
        .add_condition(Expr::not(Value::Bool(false)))
        .step(Assign::var(&n, Value::Int(2)));
    branch
        .default_condition()
        .step(Assign::var(&n, Value::Int(1)));

    hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(2), *frame.value_of(&n).unwrap());
    })
    .unwrap();
}

#[test]
fn multiple_branches() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();
    let (result, n) = &lv2_var!(result, n);

    hir.step(Assign::var(n, Value::Int(5)));

    let branch = hir.branch();
    branch
        .add_condition(Expr::eq(Expr::rem(n, Value::Int(3)), Value::Int(0)))
        .step(Assign::var(result, Value::Str("fizz".to_string())));
    branch
        .add_condition(Expr::eq(Expr::rem(n, Value::Int(5)), Value::Int(0)))
        .step(Assign::var(result, Value::Str("buzz".to_string())));
    branch
        .default_condition()
        .step(Assign::var(result, Value::Str("none".to_string())));

    hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::from("buzz"), *frame.value_of("result").unwrap());
    })
    .unwrap();
}

#[test]
fn taking_parameters() {
    let mut builder = ModuleBuilder::new();
    let (a, b) = lv2_var!(a, b);

    builder
        .add_with_args("called", vec![a.clone(), b.clone()])
        .trigger(10);

    builder.entry().step(lv2_call!(called, 2, 7));

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(2), *frame.value_of(&a).unwrap());
        assert_eq!(Value::Int(7), *frame.value_of(&b).unwrap());
    })
    .unwrap();
}

#[test]
fn automatic_return() {
    let mut vm = Vm::new();
    let mut builder = ModuleBuilder::new();

    builder.entry();
    builder.add("no-return");

    let module = builder.build().unwrap();

    vm.add_main_module(module).unwrap();

    assert_eq!(Value::Nil, vm.call("no-return", &[]).unwrap());
}

#[test]
fn return_values() {
    let mut builder = ModuleBuilder::new();
    let n = lv2_var!(n);

    builder.add("returner").step(Return::value(10));

    builder
        .entry()
        .step(Assign::var(&n, Call::new("returner")))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(10), *frame.value_of(&n).unwrap());
    })
    .unwrap();
}

#[test]
fn drop_call_values() {
    let mut builder = ModuleBuilder::new();

    let _ = builder.add("returner");

    builder.entry().step(Call::new("returner")).trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), |ctx| {
        assert!(ctx.last_value_mut().is_err());
    })
    .unwrap();
}

#[test]
fn cast_to_string() {
    let mut builder = ModuleBuilder::new();

    let main = builder.entry();
    let (a, b, c, d) = lv2_var!(a, b, c, d);
    main.step(Assign::var(&a, Conv::to_str(10)));
    main.step(Assign::var(&b, Conv::to_str(10.1)));
    main.step(Assign::var(&c, Conv::to_str("10")));
    main.step(Assign::var(&d, Conv::to_str(true)));
    main.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::from("10"), *frame.value_of(&a).unwrap());
        assert_eq!(Value::from("10.1"), *frame.value_of(&b).unwrap());
        assert_eq!(Value::from("10"), *frame.value_of(&c).unwrap());
        assert_eq!(Value::from("true"), *frame.value_of(&d).unwrap());
    })
    .unwrap();
}

#[test]
fn folding_expr() {
    let mut builder = ModuleBuilder::new();

    let main = builder.entry();
    let (a, n) = lv2_var!(a, n);

    main.global(&a)
        .step(Assign::var(
            &a,
            Expr::from_opn(Operator2::Div, vec![8.into(), 4.into()]),
        ))
        .global(&n)
        .step(Assign::var(
            &n,
            Expr::from_opn(Operator2::Div, vec![8.into(), 4.into(), 2.into()]),
        ))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), move |ctx| {
        let a = ctx.value_of(&a).unwrap();
        let n = ctx.value_of(&n).unwrap();
        assert_eq!(Value::Int(2), *a);
        assert_eq!(Value::Int(1), *n);
    })
    .unwrap();
}

#[test]
fn get_field_from_dict() {
    let (x, y, z, d1, d2, g) = lv2_var!(x, y, z, d1, d2, g);
    define_test! {
        main {
            Assign::var(&d1, lv2_dict!("x" => 37));
            Assign::var(&d2, lv2_dict!("x" => lv2_dict!("y" => 42)));
            Assign::global(&g, lv2_dict!("x" => 67));
            Assign::var(&x, lv2_access!(d1, "x"));
            Assign::var(&y, lv2_access!(d2, "x", "y"));
            Assign::var(&z, lv2_access!(g, "x"));
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(37), *frame.value_of(&x).unwrap());
            assert_eq!(Value::Int(42), *frame.value_of(&y).unwrap());
            assert_eq!(Value::Int(67), *frame.value_of(&z).unwrap());
        })
    }
}

#[test]
fn set_field_on_dict() {
    let (d1, d2, g) = lv2_var!(d1, d2, g);
    define_test! {
        main {
            Assign::var(&d1, lv2_dict!());
            Assign::var(&d2, lv2_dict!("x" => lv2_dict!()));
            Assign::global(&g, lv2_dict!());
            Assign::set(&lv2_access!(d1, "x"), 37);
            Assign::set(&lv2_access!(d2, "x", "y"), 42);
            Assign::set(&lv2_access!(g, "x"), 67);
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(
                Value::Int(37),
                frame.value_of(&d1).unwrap()
                    .get(&Value::from("x")).unwrap()
            );
            assert!(
                frame.value_of(&d2).unwrap()
                    .get(&Value::from("x")).unwrap()
                    .is_ref()
            );
            assert_eq!(
                Value::Int(42),
                frame.value_of(&d2).unwrap()
                    .get(&Value::from("x")).unwrap()
                    .get(&Value::from("y")).unwrap()
            );
            assert_eq!(
                Value::Int(67),
                ctx.value_of(&g).unwrap()
                    .get(&Value::from("x")).unwrap()
            );
        })
    }
}

#[test]
fn is_constant() {
    assert!(!Expr::from(lv2_var!(n)).is_const());
    assert!(Expr::add(1, 2).is_const());
    assert!(Expr::from("abc").is_const());
    assert!(Expr::from(10).is_const());
}

#[test]
fn call_into_vm() {
    let mut builder = ModuleBuilder::named("main");
    builder.entry().step(lv2_call!(call_me, 10));

    builder
        .add_with_args("call_me", vec![lv2_var!(n)])
        .trigger(10);

    let module = builder.build().unwrap();

    // ensure that the interrupt has been called
    run_module_test(create_vm_with_std(), module, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(10), *frame.value_of("n").unwrap());
    })
    .unwrap();
}

#[test]
fn comparison() {
    let (lt, le1, le2, gt, ge1, ge2) = lv2_var!(lt, le1, le2, gt, ge1, ge2);
    define_test! {
        main {
            Assign::var(&lt, Expr::lt(2, 3));
            Assign::var(&le1, Expr::le(2, 3));
            Assign::var(&le2, Expr::le(2, 2));
            Assign::var(&gt, Expr::gt(3, 2));
            Assign::var(&ge1, Expr::ge(3, 2));
            Assign::var(&ge2, Expr::ge(3, 3));
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Bool(true), *frame.value_of(&lt).unwrap());
            assert_eq!(Value::Bool(true), *frame.value_of(&le1).unwrap());
            assert_eq!(Value::Bool(true), *frame.value_of(&le2).unwrap());
            assert_eq!(Value::Bool(true), *frame.value_of(&gt).unwrap());
            assert_eq!(Value::Bool(true), *frame.value_of(&ge1).unwrap());
            assert_eq!(Value::Bool(true), *frame.value_of(&ge2).unwrap());
        })
    }
}

#[test]
fn raise_to_power() {
    let (a, b) = lv2_var!(a, b);
    define_test! {
        main {
            Assign::var(&a, Expr::pow(2, 3));
            Assign::var(&b, Expr::pow(3., 3.));
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(8), *frame.value_of(&a).unwrap());
            assert_eq!(Value::Float(27.), *frame.value_of(&b).unwrap());
        })
    }
}

#[test]
fn initialize_objects() {
    let (n, ae, ag, be, bg) = lv2_var!(n, ae, ag, be, bg);
    define_test! {
        main {
            Assign::var(&n, 2);
            Assign::var(&ae, lv2_list!(1, 2, 3));
            Assign::var(&ag, lv2_list!(1, &n, 3));
            Assign::var(&be, lv2_dict!(1 => 2, 2 => 2, 4 => 4));
            Assign::var(&bg, lv2_dict!(1 => 2, &n => &n, 4 => 4));
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let ae = frame.value_of(&ae).unwrap();
            let ag = frame.value_of(&ag).unwrap();
            let be = frame.value_of(&be).unwrap();
            let bg = frame.value_of(&bg).unwrap();
            assert_eq!(ae, ag);
            assert_eq!(be, bg);
        })
    }
}

#[test]
fn store_without_reference() {
    let (n, x, y) = lv2_var!(n, x, y);
    define_test! {
        main {
            Assign::var(&n, 2);
            Assign::var(&x, Expr::from(5).boxed());
            Assign::var(&y, &x);
            Assign::set(&y, 7);
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(2), *frame.value_of(&n).unwrap());
            assert_eq!(Value::Int(7), *frame.value_of(&y).unwrap());
        })
    }
}

#[test]
fn create_slice() {
    let (ls, s) = lv2_var!(ls, s);
    define_test! {
        main {
            Assign::var(&ls, lv2_list!(1, 2, 3, 4, 5));
            Assign::var(&s, Slice::new(&ls).start(1).end(4));
            Assign::set(&lv2_access!(s, 1), 9);
        }

        #ensure (move |ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let ls = frame.value_of(&ls).unwrap();
            let s = frame.value_of(&s).unwrap();
            assert_eq!(
                Value::Int(9),
                s.get(&Value::Int(1)).unwrap()
            );
            assert_eq!(
                Value::Int(9),
                ls.get(&Value::Int(2)).unwrap()
            );
        })
    }
}

#[test]
fn iterating_repeat() {
    fn check(ctx: &mut Context) {
        assert_eq!(Value::from(10), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
    }

    let mut builder = ModuleBuilder::new();
    let (sum, i, iter) = &lv2_var!(sum, i, iter);

    let main_hir = builder.entry();

    main_hir.global(sum).step(Assign::var(sum, 0));
    main_hir
        .step(Assign::var(iter, Iter::create(lv2_list!(1, 2, 3, 4))))
        .repeat_iterating(iter, i)
        .global(sum)
        .step(Assign::var(sum, Expr::add(sum, i)));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn iterating_repeat_inplace() {
    fn check(ctx: &mut Context) {
        assert_eq!(Value::from(10), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
        assert_eq!(ctx.value_of("orig").unwrap(), ctx.value_of("ls").unwrap());
    }

    let mut builder = ModuleBuilder::new();
    let (sum, i, ls, orig) = &lv2_var!(sum, i, ls, orig);

    let main_hir = builder.entry();

    main_hir.global(sum).step(Assign::var(sum, 0));
    main_hir
        .global(orig)
        .step(Assign::var(orig, lv2_list!(1, 2, 3, 4)));
    main_hir
        .global(ls)
        .step(Assign::var(ls, lv2_list!(1, 2, 3, 4)));
    main_hir
        .repeat_iterating(ls, i)
        .global(sum)
        .step(Assign::var(sum, Expr::add(sum, i)));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn iterating_repeat_ranged() {
    fn check(ctx: &mut Context) {
        assert_eq!(Value::from(45), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
    }

    let mut builder = ModuleBuilder::new();
    let (sum, i) = &lv2_var!(sum, i);

    let main_hir = builder.entry();

    main_hir.global(sum).step(Assign::var(sum, 0));
    main_hir
        .repeat_iterating(Iter::create_ranged(Value::Nil, 10), i)
        .global(sum)
        .step(Assign::var(sum, Expr::add(sum, i)));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn iterating_repeat_nested() {
    fn check(ctx: &mut Context) {
        assert_eq!(Value::from(17199), ctx.value_of("sum").unwrap().clone());
        assert!(ctx.last_value_mut().is_err());
    }

    let mut builder = ModuleBuilder::new();
    let (sum, i, j) = &lv2_var!(sum, i, j);

    let main_hir = builder.entry();

    main_hir.global(sum).step(Assign::var(sum, 0));
    main_hir
        .repeat_iterating(Iter::create_ranged(0, 5), i)
        .repeat_iterating(Iter::create_ranged(5, 10), j)
        .global(sum)
        .step(Assign::var(sum, Expr::add(sum, Expr::pow(j, i))));
    main_hir.trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn shift_values() {
    fn check(ctx: &mut Context) {
        assert_eq!(Value::from(4), *ctx.value_of("a").unwrap());
        assert_eq!(Value::from(8), *ctx.value_of("b").unwrap());
        assert_eq!(Value::from(0b10100000), *ctx.value_of("c").unwrap());
        assert_eq!(Value::from(0), *ctx.value_of("d").unwrap());
    }

    let (a, b, c, d) = &lv2_var!(a, b, c, d);
    let mut builder = ModuleBuilder::new();

    builder
        .entry()
        .global(a)
        .step(Assign::var(a, Expr::shl(2, 1)))
        .global(b)
        .step(Assign::var(b, Expr::shr(16, 1)))
        .global(c)
        .step(Assign::var(c, Expr::shl(0b00001010, 4)))
        .global(d)
        .step(Assign::var(d, Expr::shr(0b0001010, 4)))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn conditional_expression() {
    fn check(ctx: &mut Context) {
        assert_eq!(Value::from(true), *ctx.value_of("x").unwrap());
        assert_eq!(Value::from(false), *ctx.value_of("y").unwrap());
    }

    let mut builder = ModuleBuilder::new();
    let (x, y, z) = &lv2_var!(x, y, z);

    builder
        .entry()
        .step(Assign::var(z, 2))
        .global(x)
        .step(Assign::var(
            x,
            Expr::branch()
                .add_condition(Expr::eq(z, 1), false)
                .default_value(true),
        ))
        .global(y)
        .step(Assign::var(
            y,
            Expr::branch()
                .add_condition(Expr::eq(z, 2), false)
                .default_value(true),
        ))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}

#[test]
fn variable_scoping() {
    fn check(ctx: &mut Context) {
        assert_eq!(Value::from(1), *ctx.value_of("x").unwrap());

        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::from(2), *frame.value_of("x").unwrap());
        assert_eq!(Value::from(true), *frame.value_of("y").unwrap());
    }

    let mut builder = ModuleBuilder::new();
    let (x, y) = &lv2_var!(x, y);

    builder
        .entry()
        .step(Assign::var(y, true))
        .global(x)
        .step(Assign::var(x, 1))
        .local(x)
        .step(Assign::var(x, 2))
        .trigger(10);

    run_module_test(create_vm_with_std(), builder.build().unwrap(), check).unwrap();
}
