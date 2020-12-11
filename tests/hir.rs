#![allow(unused_parens)]

use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::value::Value;
use lovm2::vm::Vm;

fn run_module_test(module: Module, testfn: impl Fn(&mut Context) + 'static) {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let mut vm = Vm::with_std();
    let called_ref = called.clone();
    vm.context_mut().set_interrupt(10, move |ctx| {
        called_ref.set(true);
        testfn(ctx);
        Ok(())
    });

    println!("{}", module);
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
            let hir = builder.add(stringify!($fname));
            $(
                hir.push($inx);
            )*
            hir.push(Interrupt::new(10));
        )*

        run_module_test(builder.build().unwrap(), $ensure);
    }};
}

#[test]
fn assign_local() {
    define_test! {
        main {
            Assign::local(lv2_var!(n), 4);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(4), frame.value_of(&lv2_var!(n)).unwrap());
        })
    }
}

#[test]
fn assign_local_add() {
    define_test! {
        main {
            Assign::local(lv2_var!(n), 2);
            Assign::local(lv2_var!(n), Expr::add(lv2_var!(n), 2));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(4), frame.value_of(&lv2_var!(n)).unwrap());
        })
    }
}

#[test]
fn rem_lowering() {
    define_test! {
        main {
            Assign::local(lv2_var!(rest), Expr::rem(1, 2));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(1), frame.value_of(&lv2_var!(rest)).unwrap());
        })
    }
}

#[test]
fn easy_loop() {
    define_test! {
        main {
            Assign::local(lv2_var!(n), 0);
            Repeat::until(Expr::eq(lv2_var!(n), 10))
                .push(lv2_call!(print, n))
                .push(Assign::local(lv2_var!(n), Expr::add(lv2_var!(n), 1)));
            }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(10), frame.value_of(&lv2_var!(n)).unwrap());
        })
    }
}

#[test]
fn explicit_break() {
    define_test! {
        main {
            Assign::local(lv2_var!(n), 0);
            Repeat::endless()
                .push(Assign::local(lv2_var!(n), Expr::add(lv2_var!(n), 1)))
                .push(Break::new());
            }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(1), frame.value_of(&lv2_var!(n)).unwrap());
        })
    }
}

#[test]
fn try_getting() {
    define_test! {
        main {
            Assign::local(lv2_var!(dict), lv2_dict!(0 => 6, 1 => 7));
            Assign::local(lv2_var!(dat0), lv2_access!(dict, 1));
            Assign::local(lv2_var!(list), lv2_list!("a", 10, 20., true));
            Assign::local(lv2_var!(lat0), lv2_access!(list, 1));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(7), frame.value_of(&lv2_var!(dat0)).unwrap());
            assert_eq!(Value::Int(10), frame.value_of(&lv2_var!(lat0)).unwrap());
        })
    }
}

#[test]
fn try_setting() {
    define_test! {
        main {
            Assign::local(lv2_var!(list), lv2_list!("a", 10, 20., true));
            Assign::set(lv2_access!(list, 1), 7);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let list = &frame.value_of(&lv2_var!(list)).unwrap();
            assert_eq!(Value::Int(7), list.get(Value::Int(1)).unwrap().deref().unwrap());
        })
    }
}

#[test]
fn try_retrieving_len() {
    define_test! {
        main {
            Assign::local(lv2_var!(dict), lv2_dict!(0 => 6, 1 => 7));
            Assign::local(lv2_var!(ls), lv2_list!(1, 2, 3));
            Assign::local(lv2_var!(lendict), lv2_call!(len, dict));
            Assign::local(lv2_var!(lenls), lv2_call!(len, ls));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(2), frame.value_of(&lv2_var!(lendict)).unwrap());
            assert_eq!(Value::Int(3), frame.value_of(&lv2_var!(lenls)).unwrap());
        })
    }
}

#[test]
fn try_casting() {
    define_test! {
        main {
            Assign::local(lv2_var!(n), Cast::to_integer(5.));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(5), frame.value_of(&lv2_var!(n)).unwrap());
        })
    }
}

#[test]
fn true_branching() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();

    hir.push(Assign::local(lv2_var!(n), Value::Int(0)));

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::not(Value::Bool(false)))
        .push(Assign::local(lv2_var!(n), Value::Int(2)));
    branch
        .default_condition()
        .push(Assign::local(lv2_var!(n), Value::Int(1)));
    hir.push(branch);

    hir.push(Interrupt::new(10));

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(2), frame.value_of(&lv2_var!(n)).unwrap());
    });
}

#[test]
fn multiple_branches() {
    let mut builder = ModuleBuilder::new();
    let hir = builder.entry();

    hir.push(Assign::local(lv2_var!(n), Value::Int(5)));

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::eq(
            Expr::rem(lv2_var!(n), Value::Int(3)),
            Value::Int(0),
        ))
        .push(Assign::local(
            lv2_var!(result),
            Value::Str("fizz".to_string()),
        ));
    branch
        .add_condition(Expr::eq(
            Expr::rem(lv2_var!(n), Value::Int(5)),
            Value::Int(0),
        ))
        .push(Assign::local(
            lv2_var!(result),
            Value::Str("buzz".to_string()),
        ));
    branch.default_condition().push(Assign::local(
        lv2_var!(result),
        Value::Str("none".to_string()),
    ));
    hir.push(branch);

    hir.push(Interrupt::new(10));

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            Value::Str("buzz".to_string()),
            frame.value_of(&lv2_var!(result)).unwrap()
        );
    });
}

#[test]
fn taking_parameters() {
    let mut builder = ModuleBuilder::new();

    let called = builder.add_with_args("called", vec![lv2_var!(a), lv2_var!(b)]);
    called.push(Interrupt::new(10));

    let main = builder.entry();
    main.push(Call::new("called").arg(2).arg(7));

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(2), frame.value_of(&lv2_var!(a)).unwrap());
        assert_eq!(Value::Int(7), frame.value_of(&lv2_var!(b)).unwrap());
    });
}

#[test]
fn return_values() {
    let mut builder = ModuleBuilder::new();

    let returner = builder.add("returner");
    returner.push(Return::value(10));

    let main = builder.entry();
    main.push(Assign::local(lv2_var!(n), Call::new("returner")));
    main.push(Interrupt::new(10));

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(10), frame.value_of(&lv2_var!(n)).unwrap());
    });
}

#[test]
fn drop_call_values() {
    let mut builder = ModuleBuilder::new();

    let _ = builder.add("returner");

    let main = builder.entry();
    main.push(Call::new("returner"));
    main.push(Interrupt::new(10));

    run_module_test(builder.build().unwrap(), |ctx| {
        assert!(ctx.vstack.is_empty());
    });
}

#[test]
fn cast_to_string() {
    let mut builder = ModuleBuilder::new();

    let main = builder.entry();
    main.push(Assign::local(lv2_var!(a), Cast::to_str(10)));
    main.push(Assign::local(lv2_var!(b), Cast::to_str(10.1)));
    main.push(Assign::local(lv2_var!(c), Cast::to_str("10")));
    main.push(Assign::local(lv2_var!(d), Cast::to_str(true)));
    main.push(Interrupt::new(10));

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            Value::Str("10".to_string()),
            frame.value_of(&lv2_var!(a)).unwrap()
        );
        assert_eq!(
            Value::Str("10.1".to_string()),
            frame.value_of(&lv2_var!(b)).unwrap()
        );
        assert_eq!(
            Value::Str("10".to_string()),
            frame.value_of(&lv2_var!(c)).unwrap()
        );
        assert_eq!(
            Value::Str("true".to_string()),
            frame.value_of(&lv2_var!(d)).unwrap()
        );
    });
}

#[test]
fn folding_expr() {
    let mut builder = ModuleBuilder::new();

    let main = builder.entry();

    main.push(Assign::global(
        lv2_var!(a),
        Expr::from_opn(Operator2::Div, vec![8.into(), 4.into()]),
    ));
    main.push(Assign::global(
        lv2_var!(n),
        Expr::from_opn(Operator2::Div, vec![8.into(), 4.into(), 2.into()]),
    ));
    main.push(Interrupt::new(10));

    run_module_test(builder.build().unwrap(), |ctx| {
        let a = ctx.value_of(&lv2_var!(a)).unwrap();
        let n = ctx.value_of(&lv2_var!(n)).unwrap();
        assert_eq!(Value::Int(2), a);
        assert_eq!(Value::Int(1), n);
    });
}

#[test]
fn get_field_from_dict() {
    define_test! {
        main {
            Assign::local(lv2_var!(d1), lv2_dict!("x" => 37));
            Assign::local(lv2_var!(d2), lv2_dict!("x" => lv2_dict!("y" => 42)));
            Assign::global(lv2_var!(g), lv2_dict!("x" => 67));
            Assign::local(lv2_var!(x), lv2_access!(d1, "x"));
            Assign::local(lv2_var!(y), lv2_access!(d2, "x", "y"));
            Assign::local(lv2_var!(z), lv2_access!(g, "x"));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(37), frame.value_of(&lv2_var!(x)).unwrap());
            assert_eq!(Value::Int(42), frame.value_of(&lv2_var!(y)).unwrap());
            assert_eq!(Value::Int(67), frame.value_of(&lv2_var!(z)).unwrap());
        })
    }
}

#[test]
fn set_field_on_dict() {
    define_test! {
        main {
            Assign::local(lv2_var!(d1), lv2_dict!());
            Assign::local(lv2_var!(d2), lv2_dict!("x" => lv2_dict!()));
            Assign::global(lv2_var!(g), lv2_dict!());
            Assign::set(lv2_access!(d1, "x"), 37);
            Assign::set(lv2_access!(d2, "x", "y"), 42);
            Assign::set(lv2_access!(g, "x"), 67);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(
                Value::Int(37),
                frame.value_of(&lv2_var!(d1)).unwrap()
                    .get(Value::Str("x".to_string())).unwrap()
                    .deref().unwrap()
            );
            assert!(
                !frame.value_of(&lv2_var!(d2)).unwrap()
                    .get(Value::Str("x".to_string())).unwrap()
                    .deref().unwrap()
                    .is_ref()
            );
            assert_eq!(
                Value::Int(42),
                frame.value_of(&lv2_var!(d2)).unwrap()
                    .get(Value::Str("x".to_string())).unwrap()
                    .get(Value::Str("y".to_string())).unwrap()
                    .deref().unwrap()
            );
            assert_eq!(
                Value::Int(67),
                ctx.value_of(&lv2_var!(g)).unwrap()
                    .get(Value::Str("x".to_string())).unwrap()
                    .deref().unwrap()
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
    let mut builder = ModuleBuilder::new();

    let main = builder.add_with_args("call_me", vec![lv2_var!(n)]);
    main.push(Interrupt::new(10));

    let module = builder.build().unwrap();

    // ensure that the interrupt has been called
    let called = std::rc::Rc::new(std::cell::Cell::new(false));
    let called_ref = called.clone();

    let mut vm = Vm::with_std();
    vm.context_mut().set_interrupt(10, move |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(Value::Int(10), frame.value_of(&lv2_var!(n)).unwrap());
        called_ref.set(true);
        Ok(())
    });
    vm.load_and_import_all(module).unwrap();
    vm.call("call_me", &[Value::Int(10)]).unwrap();

    assert!(called.get());
}

#[test]
fn comparison() {
    define_test! {
        main {
            Assign::local(lv2_var!(lt), Expr::lt(2, 3));
            Assign::local(lv2_var!(le1), Expr::le(2, 3));
            Assign::local(lv2_var!(le2), Expr::le(2, 2));
            Assign::local(lv2_var!(gt), Expr::gt(3, 2));
            Assign::local(lv2_var!(ge1), Expr::ge(3, 2));
            Assign::local(lv2_var!(ge2), Expr::ge(3, 3));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Bool(true), frame.value_of(&lv2_var!(lt)).unwrap());
            assert_eq!(Value::Bool(true), frame.value_of(&lv2_var!(le1)).unwrap());
            assert_eq!(Value::Bool(true), frame.value_of(&lv2_var!(le2)).unwrap());
            assert_eq!(Value::Bool(true), frame.value_of(&lv2_var!(gt)).unwrap());
            assert_eq!(Value::Bool(true), frame.value_of(&lv2_var!(ge1)).unwrap());
            assert_eq!(Value::Bool(true), frame.value_of(&lv2_var!(ge2)).unwrap());
        })
    }
}

#[test]
fn raise_to_power() {
    define_test! {
        main {
            Assign::local(lv2_var!(a), Expr::pow(2, 3));
            Assign::local(lv2_var!(b), Expr::pow(3., 3.));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(8), frame.value_of(&lv2_var!(a)).unwrap());
            assert_eq!(Value::Float(27.), frame.value_of(&lv2_var!(b)).unwrap());
        })
    }
}

#[test]
fn initialize_objects() {
    define_test! {
        main {
            Assign::local(lv2_var!(n), 2);
            Assign::local(lv2_var!(ae), lv2_list!(1, 2, 3));
            Assign::local(lv2_var!(ag), lv2_list!(1, lv2_var!(n), 3));
            Assign::local(lv2_var!(be), lv2_dict!(1 => 2, 2 => 2, 4 => 4));
            Assign::local(lv2_var!(bg), lv2_dict!(1 => 2, lv2_var!(n) => lv2_var!(n), 4 => 4));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let ae = frame.value_of(&lv2_var!(ae)).unwrap();
            let ag = frame.value_of(&lv2_var!(ag)).unwrap();
            let be = frame.value_of(&lv2_var!(be)).unwrap();
            let bg = frame.value_of(&lv2_var!(bg)).unwrap();
            assert_eq!(ae, ag);
            assert_eq!(be, bg);
        })
    }
}

#[test]
fn store_without_reference() {
    define_test! {
        main {
            Assign::local(lv2_var!(n), 2);
            Assign::local(lv2_var!(x), Expr::from(5).boxed());
            Assign::local(lv2_var!(y), lv2_var!(x));
            Assign::set(lv2_var!(y), 7);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(Value::Int(2), frame.value_of(&lv2_var!(n)).unwrap());
            assert_eq!(Value::Int(7), frame.value_of(&lv2_var!(y)).unwrap().deref().unwrap());
        })
    }
}

#[test]
fn create_slice() {
    define_test! {
        main {
            Assign::local(lv2_var!(ls), lv2_list!(1, 2, 3, 4, 5));
            Assign::local(lv2_var!(s), Slice::new(lv2_var!(ls)).start(1).end(4));
            Assign::set(lv2_access!(s, 1), 9);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let ls = frame.value_of(&lv2_var!(ls)).unwrap();
            let s = frame.value_of(&lv2_var!(s)).unwrap();
            assert_eq!(
                Value::Int(9),
                s.get(Value::Int(1)).unwrap().deref().unwrap()
            );
            assert_eq!(
                Value::Int(9),
                ls.get(Value::Int(2)).unwrap().deref().unwrap()
            );
        })
    }
}
