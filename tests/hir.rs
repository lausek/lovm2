#![allow(unused_parens)]

use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::value::RuValue;
use lovm2::vm::Vm;

fn run_module_test(module: Module, testfn: impl Fn(&mut Context) + 'static) {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let mut vm = Vm::new();
    let called_ref = called.clone();
    vm.context_mut().set_interrupt(10, move |ctx| {
        called_ref.set(true);
        testfn(ctx);
        Ok(())
    });

    println!("{:?}", module);
    vm.load_and_import_all(module).unwrap();
    vm.run().unwrap();

    assert!(called.get());
}

#[macro_export]
macro_rules! define_test {
    {
        $( $fname:ident { $( $inx:expr ; )* } )*
            #ensure $ensure:tt
    } => {{
        let mut builder = ModuleBuilder::new();

        $(
            let hir = {
                let mut hir = HIR::new();
                $(
                    hir.code.push($inx);
                )*
                    hir.code.push(Interrupt::new(10));
                hir
            };
            builder.add(stringify!($fname)).hir(hir);
        )*

        run_module_test(builder.build().unwrap(), $ensure);
    }};
}

#[test]
fn assign_local() {
    define_test! {
        main {
            Assign::local(var!(n), 4);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(4), frame.value_of(&var!(n)).unwrap());
        })
    }
}

#[test]
fn assign_local_add() {
    define_test! {
        main {
            Assign::local(var!(n), 2);
            Assign::local(var!(n), Expr::add(var!(n), 2));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(4), frame.value_of(&var!(n)).unwrap());
        })
    }
}

#[test]
fn rem_lowering() {
    define_test! {
        main {
            Assign::local(var!(rest), Expr::rem(1, 2));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(1), frame.value_of(&var!(rest)).unwrap());
        })
    }
}

#[test]
fn easy_loop() {
    define_test! {
        main {
            Assign::local(var!(n), 0);
            Repeat::until(Expr::eq(var!(n), 10))
                .push(call!(print, n))
                .push(Assign::local(var!(n), Expr::add(var!(n), 1)));
            }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(10), frame.value_of(&var!(n)).unwrap());
        })
    }
}

#[test]
fn explicit_break() {
    define_test! {
        main {
            Assign::local(var!(n), 0);
            Repeat::endless()
                .push(Assign::local(var!(n), Expr::add(var!(n), 1)))
                .push(Break::new());
            }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(1), frame.value_of(&var!(n)).unwrap());
        })
    }
}

#[test]
fn try_getting() {
    define_test! {
        main {
            Assign::local(var!(dict), co_dict!(0 => 6, 1 => 7));
            Assign::local(var!(dat0), access!(dict, 1));
            Assign::local(var!(list), co_list!("a", 10, 20., true));
            Assign::local(var!(lat0), access!(list, 1));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(7), frame.value_of(&var!(dat0)).unwrap());
            assert_eq!(RuValue::Int(10), frame.value_of(&var!(lat0)).unwrap());
        })
    }
}

#[test]
fn try_setting() {
    define_test! {
        main {
            Assign::local(var!(list), co_list!("a", 10, 20., true));
            Assign::set(access!(list, 1), 7);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let list = &frame.value_of(&var!(list)).unwrap();
            assert_eq!(RuValue::Int(7), list.get(RuValue::Int(1)).unwrap().deref().unwrap());
        })
    }
}

#[test]
fn try_retrieving_len() {
    define_test! {
        main {
            Assign::local(var!(dict), co_dict!(0 => 6, 1 => 7));
            Assign::local(var!(ls), co_list!(1, 2, 3));
            Assign::local(var!(lendict), call!(len, dict));
            Assign::local(var!(lenls), call!(len, ls));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(2), frame.value_of(&var!(lendict)).unwrap());
            assert_eq!(RuValue::Int(3), frame.value_of(&var!(lenls)).unwrap());
        })
    }
}

#[test]
fn try_casting() {
    define_test! {
        main {
            Assign::local(var!(n), Cast::to_integer(5.));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(5), frame.value_of(&var!(n)).unwrap());
        })
    }
}

#[test]
fn true_branching() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local(var!(n), CoValue::Int(0)));

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::not(CoValue::Bool(false)))
        .push(Assign::local(var!(n), CoValue::Int(2)));
    branch
        .default_condition()
        .push(Assign::local(var!(n), CoValue::Int(1)));
    hir.push(branch);

    hir.push(Interrupt::new(10));

    builder.add(ENTRY_POINT).hir(hir);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(RuValue::Int(2), frame.value_of(&var!(n)).unwrap());
    });
}

#[test]
fn multiple_branches() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local(var!(n), CoValue::Int(5)));

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::eq(
            Expr::rem(var!(n), CoValue::Int(3)),
            CoValue::Int(0),
        ))
        .push(Assign::local(
            var!(result),
            CoValue::Str("fizz".to_string()),
        ));
    branch
        .add_condition(Expr::eq(
            Expr::rem(var!(n), CoValue::Int(5)),
            CoValue::Int(0),
        ))
        .push(Assign::local(
            var!(result),
            CoValue::Str("buzz".to_string()),
        ));
    branch.default_condition().push(Assign::local(
        var!(result),
        CoValue::Str("none".to_string()),
    ));
    hir.push(branch);

    hir.push(Interrupt::new(10));

    builder.add(ENTRY_POINT).hir(hir);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Str("buzz".to_string()),
            frame.value_of(&var!(result)).unwrap()
        );
    });
}

#[test]
fn taking_parameters() {
    let mut builder = ModuleBuilder::new();

    let mut called = HIR::with_args(vec![var!(a), var!(b)]);
    called.push(Interrupt::new(10));

    let mut main = HIR::new();
    main.push(Call::new("called").arg(2).arg(7));

    builder.add("called").hir(called);
    builder.add(ENTRY_POINT).hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(RuValue::Int(2), frame.value_of(&var!(a)).unwrap());
        assert_eq!(RuValue::Int(7), frame.value_of(&var!(b)).unwrap());
    });
}

#[test]
fn return_values() {
    let mut builder = ModuleBuilder::new();

    let mut returner = HIR::new();
    returner.push(Return::value(10));

    let mut main = HIR::new();
    main.push(Assign::local(var!(n), Call::new("returner")));
    main.push(Interrupt::new(10));

    builder.add("returner").hir(returner);
    builder.add(ENTRY_POINT).hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(RuValue::Int(10), frame.value_of(&var!(n)).unwrap());
    });
}

#[test]
fn drop_call_values() {
    let mut builder = ModuleBuilder::new();

    let returner = HIR::new();

    let mut main = HIR::new();
    main.push(Call::new("returner"));
    main.push(Interrupt::new(10));

    builder.add("returner").hir(returner);
    builder.add(ENTRY_POINT).hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        assert!(ctx.vstack.is_empty());
    });
}

#[test]
fn cast_to_string() {
    let mut builder = ModuleBuilder::new();

    let mut main = HIR::new();
    main.push(Assign::local(var!(a), Cast::to_str(10)));
    main.push(Assign::local(var!(b), Cast::to_str(10.1)));
    main.push(Assign::local(var!(c), Cast::to_str("10")));
    main.push(Assign::local(var!(d), Cast::to_str(true)));
    main.push(Interrupt::new(10));

    builder.add(ENTRY_POINT).hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Str("10".to_string()),
            frame.value_of(&var!(a)).unwrap()
        );
        assert_eq!(
            RuValue::Str("10.1".to_string()),
            frame.value_of(&var!(b)).unwrap()
        );
        assert_eq!(
            RuValue::Str("10".to_string()),
            frame.value_of(&var!(c)).unwrap()
        );
        assert_eq!(
            RuValue::Str("true".to_string()),
            frame.value_of(&var!(d)).unwrap()
        );
    });
}

#[test]
fn folding_expr() {
    let mut builder = ModuleBuilder::new();

    let mut main = HIR::new();

    main.push(Assign::global(
        var!(a),
        Expr::from_opn(Operator2::Div, vec![8.into(), 4.into()]),
    ));
    main.push(Assign::global(
        var!(n),
        Expr::from_opn(Operator2::Div, vec![8.into(), 4.into(), 2.into()]),
    ));
    main.push(Interrupt::new(10));

    builder.add(ENTRY_POINT).hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let a = ctx.value_of(&var!(a)).unwrap();
        let n = ctx.value_of(&var!(n)).unwrap();
        assert_eq!(RuValue::Int(2), a);
        assert_eq!(RuValue::Int(1), n);
    });
}

#[test]
fn get_field_from_dict() {
    define_test! {
        main {
            Assign::local(var!(d1), co_dict!("x" => 37));
            Assign::local(var!(d2), co_dict!("x" => co_dict!("y" => 42)));
            Assign::global(var!(g), co_dict!("x" => 67));
            Assign::local(var!(x), access!(d1, "x"));
            Assign::local(var!(y), access!(d2, "x", "y"));
            Assign::local(var!(z), access!(g, "x"));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(37), frame.value_of(&var!(x)).unwrap());
            assert_eq!(RuValue::Int(42), frame.value_of(&var!(y)).unwrap());
            assert_eq!(RuValue::Int(67), frame.value_of(&var!(z)).unwrap());
        })
    }
}

#[test]
fn set_field_on_dict() {
    define_test! {
        main {
            Assign::local(var!(d1), co_dict!());
            Assign::local(var!(d2), co_dict!("x" => co_dict!()));
            Assign::global(var!(g), co_dict!());
            Assign::set(access!(d1, "x"), 37);
            Assign::set(access!(d2, "x", "y"), 42);
            Assign::set(access!(g, "x"), 67);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(
                RuValue::Int(37),
                frame.value_of(&var!(d1)).unwrap()
                    .get(RuValue::Str("x".to_string())).unwrap()
                    .deref().unwrap()
            );
            assert!(
                !frame.value_of(&var!(d2)).unwrap()
                    .get(RuValue::Str("x".to_string())).unwrap()
                    .deref().unwrap()
                    .is_ref()
            );
            assert_eq!(
                RuValue::Int(42),
                frame.value_of(&var!(d2)).unwrap()
                    .get(RuValue::Str("x".to_string())).unwrap()
                    .get(RuValue::Str("y".to_string())).unwrap()
                    .deref().unwrap()
            );
            assert_eq!(
                RuValue::Int(67),
                ctx.value_of(&var!(g)).unwrap()
                    .get(RuValue::Str("x".to_string())).unwrap()
                    .deref().unwrap()
            );
        })
    }
}

#[test]
fn is_constant() {
    assert!(!Expr::from(var!(n)).is_const());
    assert!(Expr::add(1, 2).is_const());
    assert!(Expr::from("abc").is_const());
    assert!(Expr::from(10).is_const());
}

#[test]
fn call_into_vm() {
    let mut builder = ModuleBuilder::new();

    let mut main = HIR::with_args(vec![var!(n)]);
    main.push(Interrupt::new(10));
    builder.add("call_me").hir(main);

    let module = builder.build().unwrap();

    // ensure that the interrupt has been called
    let called = std::rc::Rc::new(std::cell::Cell::new(false));
    let called_ref = called.clone();

    let mut vm = Vm::new();
    vm.context_mut().set_interrupt(10, move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(RuValue::Int(10), frame.value_of(&var!(n)).unwrap());
        called_ref.set(true);
        Ok(())
    });
    vm.load_and_import_all(module).unwrap();
    vm.call("call_me", &[RuValue::Int(10)]).unwrap();

    assert!(called.get());
}

#[test]
fn comparison() {
    define_test! {
        main {
            Assign::local(var!(lt), Expr::lt(2, 3));
            Assign::local(var!(le1), Expr::le(2, 3));
            Assign::local(var!(le2), Expr::le(2, 2));
            Assign::local(var!(gt), Expr::gt(3, 2));
            Assign::local(var!(ge1), Expr::ge(3, 2));
            Assign::local(var!(ge2), Expr::ge(3, 3));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Bool(true), frame.value_of(&var!(lt)).unwrap());
            assert_eq!(RuValue::Bool(true), frame.value_of(&var!(le1)).unwrap());
            assert_eq!(RuValue::Bool(true), frame.value_of(&var!(le2)).unwrap());
            assert_eq!(RuValue::Bool(true), frame.value_of(&var!(gt)).unwrap());
            assert_eq!(RuValue::Bool(true), frame.value_of(&var!(ge1)).unwrap());
            assert_eq!(RuValue::Bool(true), frame.value_of(&var!(ge2)).unwrap());
        })
    }
}

#[test]
fn raise_to_power() {
    define_test! {
        main {
            Assign::local(var!(a), Expr::pow(2, 3));
            Assign::local(var!(b), Expr::pow(3., 3.));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(8), frame.value_of(&var!(a)).unwrap());
            assert_eq!(RuValue::Float(27.), frame.value_of(&var!(b)).unwrap());
        })
    }
}

#[test]
fn initialize_objects() {
    define_test! {
        main {
            Assign::local(var!(n), 2);
            Assign::local(var!(ae), co_list!(1, 2, 3));
            Assign::local(var!(ag), co_list!(1, var!(n), 3));
            Assign::local(var!(be), co_dict!(1 => 2, 2 => 2, 4 => 4));
            Assign::local(var!(bg), co_dict!(1 => 2, var!(n) => var!(n), 4 => 4));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let ae = frame.value_of(&var!(ae)).unwrap();
            let ag = frame.value_of(&var!(ag)).unwrap();
            let be = frame.value_of(&var!(be)).unwrap();
            let bg = frame.value_of(&var!(bg)).unwrap();
            assert_eq!(ae, ag);
            assert_eq!(be, bg);
        })
    }
}

#[test]
fn store_without_reference() {
    define_test! {
        main {
            Assign::local(var!(n), 2);
            Assign::local(var!(x), Expr::from(5).boxed());
            Assign::local(var!(y), var!(x));
            Assign::set(var!(y), 7);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(2), frame.value_of(&var!(n)).unwrap());
            assert_eq!(RuValue::Int(7), frame.value_of(&var!(y)).unwrap().deref().unwrap());
        })
    }
}

#[test]
fn create_slice() {
    define_test! {
        main {
            Assign::local(var!(ls), co_list!(1, 2, 3, 4, 5));
            Assign::local(var!(s), Slice::new(var!(ls)).start(1).end(4));
            Assign::set(access!(s, 1), 9);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let ls = frame.value_of(&var!(ls)).unwrap();
            let s = frame.value_of(&var!(s)).unwrap();
            assert_eq!(
                RuValue::Int(9),
                s.get(RuValue::Int(1)).unwrap().deref().unwrap()
            );
            assert_eq!(
                RuValue::Int(9),
                ls.get(RuValue::Int(2)).unwrap().deref().unwrap()
            );
        })
    }
}
