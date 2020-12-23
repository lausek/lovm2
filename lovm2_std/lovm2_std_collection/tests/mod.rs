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
fn native_set_predicates() {
    let (a, b, c, d, e, f) = &lv2_var!(a, b, c, d, e, f);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .step(Assign::global(a, lv2_list!(true)))
            .step(Assign::global(b, lv2_list!(true, "abc", 1)))
            .step(Assign::global(c, lv2_list!(true, "", 1)))
            .step(Assign::global(d, lv2_list!()))
            .step(Assign::global(e, lv2_list!(false, true)))
            .step(Assign::global(f, lv2_list!(false)));
    });

    vm.call("init", &[]).unwrap();

    let a = vm.context_mut().value_of(a).unwrap().clone();
    let b = vm.context_mut().value_of(b).unwrap().clone();
    let c = vm.context_mut().value_of(c).unwrap().clone();
    let d = vm.context_mut().value_of(d).unwrap().clone();
    let e = vm.context_mut().value_of(e).unwrap().clone();
    let f = vm.context_mut().value_of(f).unwrap().clone();

    assert_eq!(Value::from(true), vm.call("all", &[a.clone()]).unwrap());
    assert_eq!(Value::from(true), vm.call("all", &[b.clone()]).unwrap());
    assert_eq!(Value::from(false), vm.call("all", &[c.clone()]).unwrap());
    assert_eq!(Value::from(true), vm.call("all", &[d.clone()]).unwrap());
    assert_eq!(Value::from(false), vm.call("all", &[e.clone()]).unwrap());
    assert_eq!(Value::from(false), vm.call("all", &[f.clone()]).unwrap());

    assert_eq!(Value::from(true), vm.call("any", &[a.clone()]).unwrap());
    assert_eq!(Value::from(true), vm.call("any", &[b.clone()]).unwrap());
    assert_eq!(Value::from(true), vm.call("any", &[c.clone()]).unwrap());
    assert_eq!(Value::from(false), vm.call("any", &[d.clone()]).unwrap());
    assert_eq!(Value::from(true), vm.call("any", &[e.clone()]).unwrap());
    assert_eq!(Value::from(false), vm.call("any", &[f.clone()]).unwrap());
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

    assert_eq!(Value::from(6), vm.call("count", &[s.clone()]).unwrap());
    assert_eq!(Value::from(2), vm.call("count", &[d.clone()]).unwrap());
    assert_eq!(Value::from(3), vm.call("count", &[ls.clone()]).unwrap());
    assert!(vm.call("count", &[n.clone()]).is_err());
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
