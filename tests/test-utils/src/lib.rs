use lovm2::module::LV2Module;
use lovm2::prelude::*;
use lovm2::vm::{LV2Context, LOVM2_INT_DEBUG, LV2Vm};

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
    }).unwrap();

    println!("{}", module);
    vm.add_main_module(module)?;
    vm.run()?;

    assert!(called.get());
    
    Ok(())
}
