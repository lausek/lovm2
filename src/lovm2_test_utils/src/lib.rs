//use lovm2_core::module::LV2Module;
use lovm2_core::prelude::*;
use lovm2_core::vm::{LV2Context, LV2Vm, LOVM2_INT_DEBUG};

pub fn run_module_test(
    mut vm: LV2Vm,
    module: LV2Module,
    testfn: impl Fn(&mut LV2Context) + 'static,
) -> LV2Result<()> {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let called_ref = called.clone();
    vm.set_interrupt(LOVM2_INT_DEBUG, move |vm| {
        called_ref.set(true);
        testfn(vm.context_mut());
        Ok(())
    })
    .unwrap();

    println!("{}", module);
    vm.add_main_module(module)?;
    vm.run()?;

    assert!(called.get());

    Ok(())
}

pub fn run_module_test_builder(func: impl Fn(&mut LV2ModuleBuilder)) -> LV2Vm {
    let mut builder = LV2ModuleBuilder::new();
    builder.entry();
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = LV2Vm::new();
    vm.add_module(lovm2_std::create_std_module(), false)
        .unwrap();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}
