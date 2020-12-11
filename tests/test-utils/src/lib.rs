use lovm2::context::Context;
use lovm2::module::Module;
use lovm2::prelude::*;
use lovm2::vm::Vm;

pub fn run_module_test(
    mut vm: Vm,
    module: Module,
    testfn: impl Fn(&mut Context) + 'static,
) -> Lovm2Result<()> {
    let called = std::rc::Rc::new(std::cell::Cell::new(false));

    let called_ref = called.clone();
    vm.context_mut().set_interrupt(10, move |ctx| {
        called_ref.set(true);
        testfn(ctx);
        Ok(())
    });

    println!("{}", module);
    vm.load_and_import_all(module)?;
    vm.run()?;

    assert!(called.get());
    
    Ok(())
}
