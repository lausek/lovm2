use lovm2::block::Block;
use lovm2::expr::Expr;
use lovm2::hir::prelude::*;
use lovm2::value::RuValue;
use lovm2::vm::Vm;
use lovm2::Context;
use lovm2::ModuleBuilder;

#[macro_export]
macro_rules! define_test {
    {
        $( $fname:ident { $( $inx:expr ; )* } )*
            #ensure $ensure:tt
    } => {{
        let mut builder = ModuleBuilder::new();
        let called = std::rc::Rc::new(std::cell::Cell::new(false));

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

        let mut vm = Vm::new();
        let called_ref = called.clone();
        vm.context_mut().set_interrupt(10, move |ctx| {
            called_ref.set(true);
            $ensure(ctx);
        });

        let module = builder.build().unwrap();
        vm.load_and_import_all(module).unwrap();
        vm.run().unwrap();

        assert!(called.get());
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
            Assign::local(var!(at0), call!(get, dict, 1));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(7), *frame.locals.get(&var!(at0)).unwrap());
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

/*
 * TODO: fix these tests
#[test]
fn true_branching() {
    define_test! {
        main {
            Assign::local("n".into(), CoValue::Int(0).into());
            Branch::new()
                    .add_condition(
                        Expr::not(CoValue::Bool(false).into()),
                        Block::new()
                                .push(Assign::local("n".into(), CoValue::Int(2).into()))
                    )
                    .default_condition(
                        Block::new()
                                .push(Assign::local("n".into(), CoValue::Int(1).into()))
                    );
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(2), *frame.locals.get("n").unwrap());
        })
    }
}

#[test]
fn multiple_branches() {
    define_test! {
        main {
            Assign::local("n".into(), CoValue::Int(5).into());
            Branch::new()
                    .add_condition(
                        Expr::eq(Expr::rem(Variable::from("n").into(), CoValue::Int(3).into()), CoValue::Int(0).into()),
                        Block::new()
                                .push(Assign::local("result".into(), CoValue::Str("fizz".to_string()).into()))
                    )
                    .add_condition(
                        Expr::eq(Expr::rem(Variable::from("n").into(), CoValue::Int(5).into()), CoValue::Int(0).into()),
                        Block::new()
                                .push(Assign::local("result".into(), CoValue::Str("buzz".to_string()).into()))
                    )
                    .default_condition(
                        Block::new()
                                .push(Assign::local("result".into(), CoValue::Str("none".to_string()).into()))
                    );
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Str("buzz".to_string()), *frame.locals.get("result").unwrap());
        })
    }
}
*/
