#[allow(unused_variables)]
use criterion::{criterion_group, criterion_main, Criterion};

use lovm2::prelude::*;

mod ack;
mod bisect;
#[macro_use]
mod legacy;

use self::ack::ackermann;
use self::bisect::bisect;
use self::legacy::*;

fn fibonacci(c: &mut Criterion) {
    let mut module = LV2ModuleBuilder::new();
    // TODO: not needed (?)
    module.entry();

    let (h, l, n, r) = &lv2_var!(h, l, n, r);
    let fib_hir = module.add_with_args("fib", vec![n.clone()]);

    fib_hir
        .branch()
        //.add_condition(LV2Expr::or(LV2Expr::eq(n, 0), LV2Expr::eq(n, 1)))
        .add_condition(LV2Expr::from(n).eq(0).or(LV2Expr::from(n).eq(1)))
        .return_value(n);

    fib_hir
        .assign(l, 0)
        .assign(r, 1)
        .decrement(n);

    fib_hir
        .repeat_until(LV2Expr::from(n).eq(0))
        .assign(h, r)
        .assign(r, LV2Expr::from(l).add(r))
        .assign(l, h)
        .decrement(n);

    fib_hir.return_value(r);

    c.bench_function("fib compile", |b| b.iter(|| module.build().unwrap()));

    let module = module.build().unwrap();

    // check filesize of module
    //assert_eq!(94, module.to_bytes().unwrap().len());

    let mut vm = create_vm();
    vm.add_main_module(module).unwrap();

    c.bench_function("fib 0", |b| {
        b.iter(|| {
            assert_eq!(LV2Value::from(0), vm.call("fib", &[0.into()]).unwrap());
        })
    });

    c.bench_function("fib 1", |b| {
        b.iter(|| {
            assert_eq!(LV2Value::from(1), vm.call("fib", &[1.into()]).unwrap());
        })
    });

    c.bench_function("fib 90", |b| {
        b.iter(|| {
            assert_eq!(
                LV2Value::from(2880067194370816120),
                vm.call("fib", &[90.into()]).unwrap()
            );
        })
    });
}

criterion_group!(benches, bisect, fibonacci, ackermann);
criterion_main!(benches);
