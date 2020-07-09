#![allow(unused_parens)]

use lovm2::expr::Expr;
use lovm2::hir::prelude::*;
use lovm2::value::RuValue;
use lovm2::vm::Vm;
use lovm2::Context;
use lovm2::{Module, ModuleBuilder};

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
            assert_eq!(RuValue::Int(4), *frame.locals.get(&var!(n)).unwrap());
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
            assert_eq!(RuValue::Int(4), *frame.locals.get(&var!(n)).unwrap());
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
            assert_eq!(RuValue::Int(1), *frame.locals.get(&var!(rest)).unwrap());
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
            assert_eq!(RuValue::Int(10), *frame.locals.get(&var!(n)).unwrap());
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
            assert_eq!(RuValue::Int(1), *frame.locals.get(&var!(n)).unwrap());
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
            assert_eq!(RuValue::Int(7), *frame.locals.get(&var!(dat0)).unwrap());
            assert_eq!(RuValue::Int(10), *frame.locals.get(&var!(lat0)).unwrap());
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
            assert_eq!(RuValue::Int(2), *frame.locals.get(&var!(lendict)).unwrap());
            assert_eq!(RuValue::Int(3), *frame.locals.get(&var!(lenls)).unwrap());
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
            assert_eq!(RuValue::Int(5), *frame.locals.get(&var!(n)).unwrap());
        })
    }
}

#[test]
fn true_branching() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local("n".into(), CoValue::Int(0)));

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::not(CoValue::Bool(false)))
        .push(Assign::local("n".into(), CoValue::Int(2)));
    branch
        .default_condition()
        .push(Assign::local("n".into(), CoValue::Int(1)));
    hir.push(branch);

    hir.push(Interrupt::new(10));

    builder.add("main").hir(hir);

    run_module_test(builder.build().unwrap(), |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(RuValue::Int(2), *frame.locals.get(&var!(n)).unwrap());
    });
}

#[test]
fn multiple_branches() {
    let mut builder = ModuleBuilder::new();
    let mut hir = HIR::new();

    hir.push(Assign::local("n".into(), CoValue::Int(5)));

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
            *frame.locals.get(&var!(result)).unwrap()
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
        assert_eq!(RuValue::Int(2), *frame.locals.get(&var!(a)).unwrap());
        assert_eq!(RuValue::Int(7), *frame.locals.get(&var!(b)).unwrap());
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
        assert_eq!(RuValue::Int(10), *frame.locals.get(&var!(n)).unwrap());
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
            *frame.locals.get(&var!(a)).unwrap()
        );
        assert_eq!(
            RuValue::Str("10.1".to_string()),
            *frame.locals.get(&var!(b)).unwrap()
        );
        assert_eq!(
            RuValue::Str("10".to_string()),
            *frame.locals.get(&var!(c)).unwrap()
        );
        assert_eq!(
            RuValue::Str("true".to_string()),
            *frame.locals.get(&var!(d)).unwrap()
        );
    });
}
