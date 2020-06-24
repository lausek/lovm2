use lovm2::hir::prelude::*;
use lovm2::Context;
use lovm2::ModuleBuilder;
use lovm2::value::RuValue;
use lovm2::vm::Vm;

#[macro_export]
macro_rules! define_test {
    {
        $( $fname:ident { $( $inx:expr ; )* } )*
            #ensure $ensure:tt
    } => {{
        let mut builder = ModuleBuilder::new();
        let mut called = std::rc::Rc::new(std::cell::Cell::new(false));

        $(
            let hir = {
                let mut hir = HIR::new();
                $(
                    hir.push($inx);
                )*
                hir
            };
            builder.add(stringify!($fname)).hir(hir);
        )*

        let mut vm = Vm::new();
        let mut called_ref = called.clone();
        vm.context_mut().set_interrupt(10, move |ctx| {
            called_ref.set(true);
            $ensure(ctx);
        });

        let module = builder.build().unwrap();
        vm.load_and_import_all(module).unwrap();
        vm.run();

        assert!(called.get());
    }};
}

#[test]
fn assign_local() {
    define_test! {
        main {
            Assign::local("n".into(), CoValue::Int(4).into());
            Interrupt::new(10);
        }

        #ensure (|ctx: &mut Context| {
            let frame = ctx.frame_mut().unwrap();
            assert_eq!(RuValue::Int(4), *frame.locals.get("n").unwrap());
        })
    }
}
