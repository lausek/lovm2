use criterion::Criterion;

use lovm2::prelude::*;
use lovm2::value::RuValue;
use lovm2::vm::Vm;

/*
 * Bisect algorithm in Python
 *
def derive(coeffs):
    d, i, factor = [], 0, len(coeffs) - 1
    while 0 < factor:
        d.append(coeffs[i] * factor)
        i, factor = i + 1, factor - 1
    return d

def calc(coeffs, x):
    sigma, i, factor = 0, 0, len(coeffs) - 1
    while 0 <= factor:
        sigma += coeffs[i] * (x ** factor)
        i, factor = i + 1, factor - 1
    return sigma

def bisect(coeffs, startx):
    dcoeffs = derive(coeffs)

    prev, current = None, startx
    while True:
        divisor = calc(dcoeffs, current)
        if divisor == 0:
            divisor = 0.001

        current -= calc(coeffs, current) / divisor
        if prev == current:
            return current
        prev = current
*/

fn calc_hir() -> HIR {
    let mut computation_loop = Repeat::until(Expr::not(Expr::le(0, var!(factor))));
    let delta = Expr::mul(
        Call::new("get").arg(var!(coeffs)).arg(var!(i)),
        Expr::pow(var!(x), var!(factor)),
    );
    computation_loop.push(Assign::local(var!(sigma), Expr::sub(var!(sigma), delta)));
    computation_loop.push(Assign::local(var!(i), Expr::add(var!(i), 1)));
    computation_loop.push(Assign::local(var!(factor), Expr::sub(var!(factor), 1)));

    let mut hir = HIR::with_args(vec![var!(coeffs), var!(x)]);
    hir.push(Assign::local(var!(sigma), 0));
    hir.push(Assign::local(var!(i), 0));
    hir.push(Assign::local(
        var!(factor),
        Expr::sub(Call::new("len").arg(var!(coeffs)), 1),
    ));
    hir.push(computation_loop);
    hir.push(Return::value(var!(sigma)));
    hir
}

fn derive_hir() -> HIR {
    let mut computation_loop = Repeat::until(Expr::not(Expr::lt(0, var!(factor))));
    let val = Expr::mul(
        Call::new("get").arg(var!(coeffs)).arg(var!(i)),
        var!(factor),
    );
    computation_loop.push(Call::new("set").arg(var!(d)).arg(var!(i)).arg(val));
    computation_loop.push(Assign::local(var!(i), Expr::add(var!(i), 1)));
    computation_loop.push(Assign::local(var!(factor), Expr::sub(var!(factor), 1)));

    let mut hir = HIR::with_args(vec![var!(coeffs)]);
    hir.push(Assign::local(var!(d), co_list!()));
    hir.push(Assign::local(var!(i), 0));
    hir.push(Assign::local(
        var!(factor),
        Expr::sub(Call::new("len").arg(var!(coeffs)), 1),
    ));
    hir.push(computation_loop);
    hir.push(Return::value(var!(d)));
    hir
}

pub fn bisect(c: &mut Criterion) {
    let mut exit_condition = Branch::new();
    exit_condition
        .add_condition(Expr::eq(var!(prev), var!(x)))
        .push(Break::new());

    let mut patch_divisor = Branch::new();
    patch_divisor
        .add_condition(Expr::eq(var!(d2), 0))
        .push(Assign::local(var!(d2), 0.001));

    let mut computation_loop = Repeat::endless();
    computation_loop.push(Assign::local(
        var!(d2),
        Call::new("calc").arg(var!(dcoeffs)).arg(var!(x)),
    ));
    computation_loop.push(patch_divisor);
    computation_loop.push(Assign::local(
        var!(d1),
        Call::new("calc").arg(var!(coeffs)).arg(var!(x)),
    ));
    computation_loop.push(Assign::local(
        var!(x),
        Expr::sub(var!(x), Expr::div(var!(d1), var!(d2))),
    ));
    computation_loop.push(exit_condition);
    computation_loop.push(Assign::local(var!(prev), var!(x)));

    let mut bisect_hir = HIR::with_args(vec![var!(coeffs), var!(startx)]);
    bisect_hir.push(Assign::local(var!(x), Cast::to_float(var!(startx))));
    bisect_hir.push(Assign::local(
        var!(dcoeffs),
        Call::new("derive").arg(var!(coeffs)),
    ));
    bisect_hir.push(Assign::local(var!(prev), CoValue::Nil));
    bisect_hir.push(computation_loop);
    bisect_hir.push(Return::value(var!(x)));

    let mut module = ModuleBuilder::new();
    module.add("calc").hir(calc_hir());
    module.add("derive").hir(derive_hir());
    module.add("bisect").hir(bisect_hir);
    let module = module.build().unwrap();

    let mut vm = Vm::new();
    vm.load_and_import_all(module).unwrap();

    c.bench_function("bisect f(x)=2x^3 + 2x^2 - x", |b| {
        b.iter(|| {
            assert_eq!(
                RuValue::from(0),
                vm.call("bisect", &[vec![2, 2, -1, 0].into(), 0.into()])
                    .unwrap()
            );
        })
    });

    c.bench_function("bisect g(x)=x^2 - 4x + 4", |b| {
        b.iter(|| {
            assert_eq!(
                RuValue::from(2),
                vm.call("bisect", &[vec![1, -4, 4].into(), 1.into()])
                    .unwrap()
            );
        })
    });

    c.bench_function("bisect h(x)=x^2 - 2x - 3", |b| {
        b.iter(|| {
            assert_eq!(
                RuValue::from(3),
                vm.call("bisect", &[vec![1, -2, -3].into(), 1.into()])
                    .unwrap()
            );
        })
    });
}
