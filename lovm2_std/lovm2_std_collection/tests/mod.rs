#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder
        .entry()
        .step(Include::import_global("liblovm2_std_collection"));
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_contains() {
    let (n, s, d, ls) = &lv2_var!(n, s, d, ls);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .step(Assign::global(n, 10))
            .step(Assign::global(s, "abc10d"))
            .step(Assign::global(d, lv2_dict!(10 => 1, "b" => 2)))
            .step(Assign::global(ls, lv2_list!("a", true, n)));
    });

    vm.call("init", &[]).unwrap();

    let n = vm.context_mut().value_of(n).unwrap().clone();
    let s = vm.context_mut().value_of(s).unwrap().clone();
    let d = vm.context_mut().value_of(d).unwrap().clone();
    let ls = vm.context_mut().value_of(ls).unwrap().clone();

    assert_eq!(
        Value::from(true),
        vm.call("contains", &[s.clone(), n.clone()]).unwrap()
    );
    assert_eq!(
        Value::from(true),
        vm.call("contains", &[d.clone(), n.clone()]).unwrap()
    );
    assert_eq!(
        Value::from(true),
        vm.call("contains", &[ls.clone(), n.clone()]).unwrap()
    );
    assert_eq!(
        Value::from(false),
        vm.call("contains", &[d.clone(), s.clone()]).unwrap()
    );
}

#[test]
fn native_count() {
    assert!(false);
}

#[test]
fn native_deep_clone() {
    assert!(false);
}

#[test]
fn native_delete() {
    assert!(false);
}

#[test]
fn native_get() {
    assert!(false);
}

#[test]
fn native_insert() {
    assert!(false);
}

#[test]
fn native_sort() {
    assert!(false);
}
