#![cfg(test)]

use lovm2::prelude::*;
use lovm2_extend::prelude::*;

fn run_module_test(func: impl Fn(&mut ModuleBuilder)) -> Vm {
    let mut builder = ModuleBuilder::new();
    builder
        .entry()
        .step(Include::import_global("liblovm2_std_json"));
    func(&mut builder);
    let module = builder.build().unwrap();

    let mut vm = create_test_vm();
    vm.add_main_module(module).unwrap();
    vm.run().unwrap();

    vm
}

#[test]
fn native_decode() {
    let (a, b) = &lv2_var!(a, b);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .step(Assign::global(a, lv2_dict!("a" => 10, "b" => Value::Nil)))
            .step(Assign::global(b, lv2_list!(lv2_dict!(), lv2_dict!(), 1.5)));
    });

    vm.call("init", &[]).unwrap();

    let a = vm.context_mut().value_of(a).unwrap().clone();
    let b = vm.context_mut().value_of(b).unwrap().clone();

    assert_eq!(
        a,
        vm.call("decode", &["{\"a\":10, \"b\":null}".into()])
            .unwrap()
    );
    assert_eq!(b, vm.call("decode", &["[{},{},1.5]".into()]).unwrap());
}

#[test]
fn native_encode() {
    let (ls, d, dd, n) = &lv2_var!(ls, d, dd, n);

    let mut vm = run_module_test(|builder| {
        builder
            .add("init")
            .step(Assign::global(d, lv2_dict!(true => 0.5)))
            .step(Assign::global(dd, lv2_dict!("a" => lv2_dict!("b" => "c"))))
            .step(Assign::global(ls, lv2_list!(1, "abc", d, Value::Nil)))
            .step(Assign::global(n, 2));
    });

    vm.call("init", &[]).unwrap();

    let ls = vm.context_mut().value_of(ls).unwrap().clone();
    let d = vm.context_mut().value_of(d).unwrap().clone();
    let dd = vm.context_mut().value_of(dd).unwrap().clone();
    let n = vm.context_mut().value_of(n).unwrap().clone();

    assert_eq!(
        Value::from("[1,\"abc\",{\"true\":0.5},null]"),
        vm.call("encode", &[ls]).unwrap()
    );
    assert_eq!(
        Value::from("{\"a\":{\"b\":\"c\"}}"),
        vm.call("encode", &[dd]).unwrap()
    );
    assert_eq!(
        Value::from("{\"true\":0.5}"),
        vm.call("encode", &[d]).unwrap()
    );
    assert_eq!(Value::from("2"), vm.call("encode", &[n]).unwrap());
}
