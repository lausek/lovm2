#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder.add_dependency("liblovm2_std_string");
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_join() {
    let (joined, ls) = &lv2_var!(joined, ls);

    let mut vm = run_module_test(|builder| {
        builder
            .entry()
            .step(Assign::local(ls, lv2_list!("a", "b")))
            .step(Assign::global(
                joined,
                Call::new("liblovm2_std_string.join").arg(ls).arg(" & "),
            ));
    });

    let result = vm.context_mut().value_of(joined).unwrap();
    assert_eq!(Value::from("a & b"), *result);
}

#[test]
fn native_split() {
    let (s, splitted, first, last) = &lv2_var!(s, splitted, first, last);

    let mut vm = run_module_test(|builder| {
        builder
            .entry()
            .step(Assign::local(s, "a;b;c"))
            .step(Assign::global(
                splitted,
                Call::new("liblovm2_std_string.split").arg(s).arg(";"),
            ))
            .step(Assign::global(first, lv2_access!(splitted, 0)))
            .step(Assign::global(last, lv2_access!(splitted, 2)));
    });

    let result = vm.context_mut().value_of(first).unwrap();
    assert_eq!(Value::from("a"), *result);
    let result = vm.context_mut().value_of(last).unwrap();
    assert_eq!(Value::from("c"), *result);
}
