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
            Assign::local("n".into(), CoValue::Int(4));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(4), *frame.locals.get("n").unwrap());
        })
    }
}

#[test]
fn assign_local_add() {
    define_test! {
        main {
            Assign::local("n".into(), CoValue::Int(2));
            Assign::local("n".into(), Expr::add(Variable::from("n"), CoValue::Int(2)));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(4), *frame.locals.get("n").unwrap());
        })
    }
}

#[test]
fn rem_lowering() {
    define_test! {
        main {
            Assign::local("rest".into(), Expr::rem(CoValue::Int(1), CoValue::Int(2)));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(1), *frame.locals.get("rest").unwrap());
        })
    }
}

#[test]
fn easy_loop() {
    define_test! {
        main {
            Assign::local("n".into(), CoValue::Int(0));
            Repeat::until(Expr::eq(Variable::from("n"), CoValue::Int(10)))
                    .push(Call::new("print").arg(Variable::from("n")))
                    .push(Assign::local("n".into(), Expr::add(Variable::from("n"), CoValue::Int(1))));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(10), *frame.locals.get("n").unwrap());
        })
    }
}

#[test]
fn explicit_break() {
    define_test! {
        main {
            Assign::local("n".into(), CoValue::Int(0));
            Repeat::endless()
                    .push(Assign::local("n".into(), Expr::add(Variable::from("n"), CoValue::Int(1))))
                    .push(Break::new());
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(1), *frame.locals.get("n").unwrap());
        })
    }
}

#[test]
fn try_setting() {
    define_test! {
        main {
            Assign::local("ls".into(), CoValue::List(vec![Box::new(CoValue::Int(7))]));
            Assign::local("at0".into(), Call::new("get").arg(Variable::from("ls")).arg(0));
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(7), *frame.locals.get("at0").unwrap());
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
