#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("liblovm2_std_collection");
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_contains() {
    let (ls, s, d, n, ls_has_n, s_has_n, d_has_n) =
        &lv2_var!(ls, s, d, n, ls_has_n, s_has_n, d_has_n);

    let mut vm = run_module_test(|builder| {
        builder
            .entry()
            .step(Assign::local(n, 10))
            .step(Assign::local(ls, lv2_list!("a", true, n)))
            .step(Assign::local(s, "abc10d"))
            .step(Assign::local(d, lv2_dict!(10 => 1, "b" => 2)))
            .step(Assign::global(
                ls_has_n,
                Call::new("liblovm2_std_collection.contains").arg(ls).arg(n),
            ))
            .step(Assign::global(
                s_has_n,
                Call::new("liblovm2_std_collection.contains").arg(s).arg(n),
            ))
            .step(Assign::global(
                d_has_n,
                Call::new("liblovm2_std_collection.contains").arg(d).arg(n),
            ));
    });

    let result = vm.context_mut().value_of(ls_has_n).unwrap();
    assert_eq!(Value::from(true), *result);
    let result = vm.context_mut().value_of(s_has_n).unwrap();
    assert_eq!(Value::from(true), *result);
    let result = vm.context_mut().value_of(d_has_n).unwrap();
    assert_eq!(Value::from(true), *result);
}
