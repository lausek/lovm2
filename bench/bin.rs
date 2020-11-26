use criterion::{criterion_group, criterion_main, Criterion};

use lovm2::prelude::*;
use lovm2::vm::Vm;

//mod ack;
mod bisect;

//use ack::ackermann;
use bisect::bisect;

fn fibonacci(c: &mut Criterion) {
    let mut trivial_return = Branch::new();
    trivial_return
        .add_condition(Expr::or(Expr::eq(lv2_var!(n), 0), Expr::eq(lv2_var!(n), 1)))
        .push(Return::value(lv2_var!(n)));

    let mut computation_loop = Repeat::until(Expr::eq(lv2_var!(n), 0));
    computation_loop.push(Assign::local(lv2_var!(h), lv2_var!(r)));
    computation_loop.push(Assign::local(
        lv2_var!(r),
        Expr::add(lv2_var!(l), lv2_var!(r)),
    ));
    computation_loop.push(Assign::local(lv2_var!(l), lv2_var!(h)));
    computation_loop.push(Assign::local(lv2_var!(n), Expr::sub(lv2_var!(n), 1)));

    let mut fib_hir = HIR::with_args(vec![lv2_var!(n)]);
    fib_hir.push(trivial_return);
    fib_hir.push(Assign::local(lv2_var!(l), 0));
    fib_hir.push(Assign::local(lv2_var!(r), 1));
    fib_hir.push(Assign::local(lv2_var!(n), Expr::sub(lv2_var!(n), 1)));
    fib_hir.push(computation_loop);
    fib_hir.push(Return::value(lv2_var!(r)));

    let mut module = ModuleBuilder::new();
    module.add("fib").hir(fib_hir);
    let module = module.build().unwrap();

    // check filesize of module
    assert_eq!(94, module.to_bytes().unwrap().len());

    let mut vm = Vm::with_std();
    vm.load_and_import_all(module).unwrap();

    c.bench_function("fib 0", |b| {
        b.iter(|| {
            assert_eq!(Value::from(0), vm.call("fib", &[0.into()]).unwrap());
        })
    });

    c.bench_function("fib 1", |b| {
        b.iter(|| {
            assert_eq!(Value::from(1), vm.call("fib", &[1.into()]).unwrap());
        })
    });

    c.bench_function("fib 90", |b| {
        b.iter(|| {
            assert_eq!(
                Value::from(2880067194370816120),
                vm.call("fib", &[90.into()]).unwrap()
            );
        })
    });
}

criterion_group!(benches, bisect, fibonacci /*, ackermann */);
criterion_main!(benches);
