use lovm2::hir::prelude::*;
use lovm2::ModuleBuilder;
use lovm2::value::RuValue;
use lovm2::vm::Vm;

#[test]
fn assign_local() {
    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Assign::local("n".into(), CoValue::Int(4).into()));
    main_hir.push(Interrupt::new(10));

    builder.add("main").hir(main_hir);

    let mut vm = Vm::new();
    vm.context_mut().set_interrupt(10, |ctx| {
        let frame = ctx.frame_mut().unwrap();
        assert_eq!(RuValue::Int(4), *frame.locals.get("n").unwrap());
    });

    let module = builder.build().unwrap();
    vm.load_and_import_all(module).unwrap();
    vm.run();
}
