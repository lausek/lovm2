#![allow(unused_parens)]

use lovm2::context::Context;
use lovm2::hir::prelude::*;
use lovm2::module::Module;
use lovm2::value::RuValue;
use lovm2::vm::Vm;

fn run_module_test(module: Module, testfn: impl Fn(&mut Context) + 'static) {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let mut vm = Vm::new();
    let called_ref = called.clone();
    vm.context_mut().set_interrupt(10, move |ctx| {
        called_ref.set(true);
        testfn(ctx);
    });

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
            assert_eq!(RuValue::Int(4), *frame.locals.get(&var!(n)).unwrap().borrow());
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
            assert_eq!(RuValue::Int(4), *frame.locals.get(&var!(n)).unwrap().borrow());
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
            assert_eq!(RuValue::Int(1), *frame.locals.get(&var!(rest)).unwrap().borrow());
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
            assert_eq!(RuValue::Int(10), *frame.locals.get(&var!(n)).unwrap().borrow());
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
            assert_eq!(RuValue::Int(1), *frame.locals.get(&var!(n)).unwrap().borrow());
        })
    }
}

#[test]
fn try_getting() {
    define_test! {
        main {
            Assign::local(var!(dict), co_dict!(0 => 6, 1 => 7));
            Assign::local(var!(dat0), call!(get, dict, 1));
            Assign::local(var!(list), co_list!("a", 10, 20., true));
            Assign::local(var!(lat0), call!(get, list, 1));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(7), *frame.locals.get(&var!(dat0)).unwrap().borrow());
            assert_eq!(RuValue::Int(10), *frame.locals.get(&var!(lat0)).unwrap().borrow());
        })
    }
}

#[test]
fn try_setting() {
    define_test! {
        main {
            Assign::local(var!(list), co_list!("a", 10, 20., true));
            call!(set, list, 1, 7);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            let list = &*frame.locals.get(&var!(list)).unwrap().borrow();
            assert_eq!(RuValue::Int(7), list.get(RuValue::Int(1)).unwrap());
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
            assert_eq!(RuValue::Int(2), *frame.locals.get(&var!(lendict)).unwrap().borrow());
            assert_eq!(RuValue::Int(3), *frame.locals.get(&var!(lenls)).unwrap().borrow());
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
            assert_eq!(RuValue::Int(5), *frame.locals.get(&var!(n)).unwrap().borrow());
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

    builder.add("main").hir(hir);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Int(2),
            *frame.locals.get(&var!(n)).unwrap().borrow()
        );
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

    builder.add("main").hir(hir);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Str("buzz".to_string()),
            *frame.locals.get(&var!(result)).unwrap().borrow()
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
    builder.add("main").hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Int(2),
            *frame.locals.get(&var!(a)).unwrap().borrow()
        );
        assert_eq!(
            RuValue::Int(7),
            *frame.locals.get(&var!(b)).unwrap().borrow()
        );
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
    builder.add("main").hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Int(10),
            *frame.locals.get(&var!(n)).unwrap().borrow()
        );
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
    builder.add("main").hir(main);

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

    builder.add("main").hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(
            RuValue::Str("10".to_string()),
            *frame.locals.get(&var!(a)).unwrap().borrow()
        );
        assert_eq!(
            RuValue::Str("10.1".to_string()),
            *frame.locals.get(&var!(b)).unwrap().borrow()
        );
        assert_eq!(
            RuValue::Str("10".to_string()),
            *frame.locals.get(&var!(c)).unwrap().borrow()
        );
        assert_eq!(
            RuValue::Str("true".to_string()),
            *frame.locals.get(&var!(d)).unwrap().borrow()
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

    builder.add("main").hir(main);

    run_module_test(builder.build().unwrap(), |ctx| {
        let a = ctx.globals.get(&var!(a)).unwrap();
        let n = ctx.globals.get(&var!(n)).unwrap();
        assert_eq!(RuValue::Int(2), *a.borrow());
        assert_eq!(RuValue::Int(1), *n.borrow());
    });
}

#[test]
fn get_field_from_dict() {
    define_test! {
        main {
            Assign::local(var!(d1), co_dict!("x" => 37));
            Assign::local(var!(d2), co_dict!("x" => co_dict!("y" => 42)));
            Assign::global(var!(g), co_dict!("x" => 67));
            Assign::local(var!(x), access!(d1, x));
            Assign::local(var!(y), access!(d2, x, y));
            Assign::local(var!(z), access!(g, x));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(37), *frame.locals.get(&var!(x)).unwrap().borrow());
            assert_eq!(RuValue::Int(42), *frame.locals.get(&var!(y)).unwrap().borrow());
            assert_eq!(RuValue::Int(67), *frame.locals.get(&var!(z)).unwrap().borrow());
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
            Assign::local(access!(d1, x), 37);
            Assign::local(access!(d2, x, y), 42);
            Assign::global(access!(g, x), 67);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(
                RuValue::Int(37),
                frame.locals.get(&var!(d1)).unwrap().borrow()
                    .get(RuValue::Str("x".to_string())).unwrap()
            );
            assert_eq!(
                RuValue::Int(42),
                frame.locals.get(&var!(d2)).unwrap().borrow()
                    .get(RuValue::Str("x".to_string())).unwrap()
                    .get(RuValue::Str("y".to_string())).unwrap()
            );
            assert_eq!(
                RuValue::Int(67),
                ctx.globals.get(&var!(g)).unwrap().borrow()
                    .get(RuValue::Str("x".to_string())).unwrap()
            );
        })
    }
}
