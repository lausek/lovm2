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
                    hir.push($inx);
                )*
                    hir.push(Interrupt::new(10));
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
            Assign::local("n".into(), CoValue::Int(4).into());
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
            Assign::local("n".into(), CoValue::Int(2).into());
            Assign::local("n".into(), Expr::add(Variable::from("n").into(), CoValue::Int(2).into()));
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
            Assign::local("rest".into(), Expr::rem(CoValue::Int(1).into(), CoValue::Int(2).into()));
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
            Assign::local("n".into(), CoValue::Int(0).into());
            Repeat::until(Expr::eq(Variable::from("n").into(), CoValue::Int(10).into()))
                    .push(Call::new("print").arg(Variable::from("n")))
                    .push(Assign::local("n".into(), Expr::add(Variable::from("n").into(), CoValue::Int(1).into())));
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
            Assign::local("n".into(), CoValue::Int(0).into());
            Repeat::endless()
                    .push(Assign::local("n".into(), Expr::add(Variable::from("n").into(), CoValue::Int(1).into())))
                    .push(Break::new());
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(1), *frame.locals.get("n").unwrap());
        })
    }
}

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
