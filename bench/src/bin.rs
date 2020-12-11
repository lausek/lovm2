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
    let mut module = ModuleBuilder::new();

    let (h, l, n, r) = &lv2_var!(h, l, n, r);
    let fib_hir = module.add_with_args("fib", vec![n.clone()]);

    fib_hir
        .branch()
        .add_condition(Expr::or(Expr::eq(n, 0), Expr::eq(n, 1)))
        .step(Return::value(n));

    fib_hir
        .step(Assign::local(l, 0))
        .step(Assign::local(r, 1))
        .step(Assign::local(n, Expr::sub(n, 1)));

    fib_hir
        .repeat_until(Expr::eq(n, 0))
        .step(Assign::local(h, r))
        .step(Assign::local(r, Expr::add(l, r)))
        .step(Assign::local(l, h))
        .step(Assign::local(n, Expr::sub(n, 1)));

    fib_hir.step(Return::value(r));

    c.bench_function("fib compile", |b| {
        b.iter(|| {
            let module = module.clone();
            module.build().unwrap()
        })
    });

    let module = module.build().unwrap();

    // check filesize of module
    //assert_eq!(94, module.to_bytes().unwrap().len());

    let mut vm = create_vm();
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

criterion_group!(benches, bisect, fibonacci, ackermann);
criterion_main!(benches);
