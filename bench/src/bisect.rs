use criterion::Criterion;

use super::*;

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

fn calc_hir() -> Hir {
    let mut computation_loop = Repeat::until(Expr::not(Expr::le(0, lv2_var!(factor))));
    let delta = Expr::mul(
        lv2_access!(coeffs, lv2_var!(i)),
        Expr::pow(lv2_var!(x), lv2_var!(factor)),
    );
    computation_loop.push(Assign::local(
        lv2_var!(sigma),
        Expr::add(lv2_var!(sigma), delta),
    ));
    computation_loop.push(Assign::local(lv2_var!(i), Expr::add(lv2_var!(i), 1)));
    computation_loop.push(Assign::local(
        lv2_var!(factor),
        Expr::sub(lv2_var!(factor), 1),
    ));

    let mut hir = Hir::with_args(vec![lv2_var!(coeffs), lv2_var!(x)]);
    hir.push(Assign::local(lv2_var!(sigma), 0));
    hir.push(Assign::local(lv2_var!(i), 0));
    hir.push(Assign::local(
        lv2_var!(factor),
        Expr::sub(Call::new("len").arg(lv2_var!(coeffs)), 1),
    ));
    hir.push(computation_loop);
    hir.push(Return::value(lv2_var!(sigma)));
    hir
}

fn derive_hir() -> Hir {
    let mut computation_loop = Repeat::until(Expr::not(Expr::lt(0, lv2_var!(factor))));
    let val = Expr::mul(lv2_access!(coeffs, lv2_var!(i)), lv2_var!(factor));
    computation_loop.push(Assign::set(lv2_access!(d, lv2_var!(i)), val));
    computation_loop.push(Assign::local(lv2_var!(i), Expr::add(lv2_var!(i), 1)));
    computation_loop.push(Assign::local(
        lv2_var!(factor),
        Expr::sub(lv2_var!(factor), 1),
    ));

    let mut hir = Hir::with_args(vec![lv2_var!(coeffs)]);
    hir.push(Assign::local(lv2_var!(d), lv2_list!()));
    hir.push(Assign::local(lv2_var!(i), 0));
    hir.push(Assign::local(
        lv2_var!(factor),
        Expr::sub(Call::new("len").arg(lv2_var!(coeffs)), 1),
    ));
    hir.push(computation_loop);
    hir.push(Return::value(lv2_var!(d)));
    hir
}

pub fn bisect(c: &mut Criterion) {
    let mut exit_condition = Branch::new();
    exit_condition
        .add_condition(Expr::eq(lv2_var!(prev), lv2_var!(x)))
        .push(Break::new());

    let mut patch_divisor = Branch::new();
    patch_divisor
        .add_condition(Expr::eq(lv2_var!(d2), 0.))
        .push(Assign::local(lv2_var!(d2), 0.001));

    let mut computation_loop = Repeat::endless();
    computation_loop.push(Assign::local(
        lv2_var!(d2),
        Call::new("calc").arg(lv2_var!(dcoeffs)).arg(lv2_var!(x)),
    ));
    computation_loop.push(patch_divisor);
    computation_loop.push(Assign::local(
        lv2_var!(d1),
        Call::new("calc").arg(lv2_var!(coeffs)).arg(lv2_var!(x)),
    ));
    computation_loop.push(Assign::local(
        lv2_var!(x),
        Expr::sub(lv2_var!(x), Expr::div(lv2_var!(d1), lv2_var!(d2))),
    ));
    computation_loop.push(exit_condition);
    computation_loop.push(Assign::local(lv2_var!(prev), lv2_var!(x)));

    let mut bisect_hir = Hir::with_args(vec![lv2_var!(coeffs), lv2_var!(startx)]);
    bisect_hir.push(Assign::local(lv2_var!(x), Cast::to_float(lv2_var!(startx))));
    bisect_hir.push(Assign::local(
        lv2_var!(dcoeffs),
        Call::new("derive").arg(lv2_var!(coeffs)),
    ));
    bisect_hir.push(Assign::local(lv2_var!(prev), Value::Nil));
    bisect_hir.push(computation_loop);
    bisect_hir.push(Return::value(lv2_var!(x)));

    let mut module = ModuleBuilder::new();
    module.add("calc").hir(calc_hir());
    module.add("derive").hir(derive_hir());
    module.add("bisect").hir(bisect_hir);

    c.bench_function("bisect compile", |b| {
        b.iter(|| {
            let module = module.clone();
            module.build().unwrap()
        })
    });

    let module = module.build().unwrap();

    // check filesize of module
    //assert_eq!(317, module.to_bytes().unwrap().len());

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();

    // f(x)=2x^3 + 2x^2 - x
    c.bench_function("bisect f", |b| {
        b.iter(|| {
            assert_eq!(
                Value::from(0),
                vm.call("bisect", &[vec![2, 2, -1, 0].into(), 1.into()])
                    .unwrap()
                    .into_integer_round()
                    .unwrap()
            );
        })
    });

    // g(x)=x^2 - 4x + 4
    c.bench_function("bisect g", |b| {
        b.iter(|| {
            assert_eq!(
                Value::from(2),
                vm.call("bisect", &[vec![1, -4, 4].into(), 1.into()])
                    .unwrap()
                    .into_integer_round()
                    .unwrap()
            );
        })
    });

    // h(x)=x^2 - 2x - 3
    c.bench_function("bisect h", |b| {
        b.iter(|| {
            assert_eq!(
                Value::from(3),
                vm.call("bisect", &[vec![1, -2, -3].into(), 1.into()])
                    .unwrap()
                    .into_integer_round()
                    .unwrap()
            );
        })
    });
}
