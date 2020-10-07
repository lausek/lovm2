use criterion::{criterion_group, criterion_main, Criterion};

use lovm2::prelude::*;
use lovm2::value::RuValue;
use lovm2::vm::Vm;

mod bisect;

use bisect::bisect;

fn fibonacci(c: &mut Criterion) {
    let mut trivial_return = Branch::new();
    trivial_return
        .add_condition(Expr::or(Expr::eq(var!(n), 0), Expr::eq(var!(n), 1)))
        .push(Return::value(var!(n)));

    let mut computation_loop = Repeat::until(Expr::eq(var!(n), 0));
    computation_loop.push(Assign::local(var!(h), var!(r)));
    computation_loop.push(Assign::local(var!(r), Expr::add(var!(l), var!(r))));
    computation_loop.push(Assign::local(var!(l), var!(h)));
    computation_loop.push(Assign::local(var!(n), Expr::sub(var!(n), 1)));

    let mut fib_hir = HIR::with_args(vec![var!(n)]);
    fib_hir.push(trivial_return);
    fib_hir.push(Assign::local(var!(l), 0));
    fib_hir.push(Assign::local(var!(r), 1));
    fib_hir.push(Assign::local(var!(n), Expr::sub(var!(n), 1)));
    fib_hir.push(computation_loop);
    fib_hir.push(Return::value(var!(r)));

    let mut module = ModuleBuilder::new();
    module.add("fib").hir(fib_hir);
    let module = module.build().unwrap();

    let mut vm = Vm::new();
    vm.load_and_import_all(module).unwrap();

    c.bench_function("fib 0", |b| {
        b.iter(|| {
            assert_eq!(RuValue::from(0), vm.call("fib", &[0.into()]).unwrap());
        })
    });

    c.bench_function("fib 1", |b| {
        b.iter(|| {
            assert_eq!(RuValue::from(1), vm.call("fib", &[1.into()]).unwrap());
        })
    });

    c.bench_function("fib 90", |b| {
        b.iter(|| {
            assert_eq!(
                RuValue::from(2880067194370816120),
                vm.call("fib", &[90.into()]).unwrap()
            );
        })
    });
}

criterion_group!(benches, bisect, fibonacci);
criterion_main!(benches);
