use lovm2_extend::prelude::*;
use lovm2_std::create_std_module;

pub fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder.entry();
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = Vm::new();
    vm.add_module(create_std_module(), false).unwrap();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}
