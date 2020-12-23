use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::vm::{Context, LOVM2_DEBUG_INTERRUPT, Vm};

pub fn run_module_test(
    mut vm: Vm,
    module: Module,
    testfn: impl Fn(&mut Context) + 'static,
) -> Lovm2Result<()> {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let called_ref = called.clone();
    vm.set_interrupt(LOVM2_DEBUG_INTERRUPT, move |vm| {
        called_ref.set(true);
        testfn(vm.context_mut());
        Ok(())
    }).unwrap();

    println!("{}", module);
    vm.add_main_module(module)?;
    vm.run()?;

    assert!(called.get());
    
    Ok(())
}
