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

fn calc_hir(module: &mut ModuleBuilder) {
    let (coeffs, x, factor, i, sigma) = &lv2_var!(coeffs, x, factor, i, sigma);
    let hir = module.add_with_args("calc", vec![coeffs.clone(), x.clone()]);

    hir.step(Assign::local(sigma, 0))
        .step(Assign::local(i, 0))
        .step(Assign::local(factor, Expr::sub(lv2_call!(len, coeffs), 1)));

    let delta = Expr::mul(lv2_access!(coeffs, i), Expr::pow(x, factor));
    hir.repeat_until(Expr::not(Expr::le(0, factor)))
        .step(Assign::local(sigma, Expr::add(sigma, delta)))
        .step(Assign::local(i, Expr::add(i, 1)))
        .step(Assign::local(factor, Expr::sub(factor, 1)));

    hir.step(Return::value(sigma));
}

fn derive_hir(module: &mut ModuleBuilder) {
    let (coeffs, i, factor, d) = &lv2_var!(coeffs, i, factor, d);
    let hir = module.add_with_args("derive", vec![coeffs.clone()]);

    hir.step(Assign::local(d, lv2_list!()))
        .step(Assign::local(i, 0))
        .step(Assign::local(factor, Expr::sub(lv2_call!(len, coeffs), 1)));

    hir.repeat_until(Expr::not(Expr::lt(0, factor)))
        .step(Assign::set(
            &lv2_access!(d, i),
            Expr::mul(lv2_access!(coeffs, i), factor),
        ))
        .step(Assign::local(i, Expr::add(i, 1)))
        .step(Assign::local(factor, Expr::sub(factor, 1)));

    hir.step(Return::value(d));
}

pub fn bisect_hir(module: &mut ModuleBuilder) {
    let (prev, x, d1, d2, coeffs, dcoeffs, startx) =
        &lv2_var!(prev, x, d1, d2, coeffs, dcoeffs, startx);
    let hir = module.add_with_args("bisect", vec![coeffs.clone(), startx.clone()]);

    // function setup
    hir.step(Assign::local(x, Cast::to_float(startx)))
        .step(Assign::local(dcoeffs, lv2_call!(derive, coeffs)))
        .step(Assign::local(prev, Value::Nil));

    let computation_loop = hir.repeat();
    computation_loop.step(Assign::local(d2, lv2_call!(calc, dcoeffs, x)));
    computation_loop
        .branch()
        .add_condition(Expr::eq(d2, 0.))
        .step(Assign::local(d2, 0.001));
    computation_loop.step(Assign::local(d1, lv2_call!(calc, coeffs, x)));
    computation_loop.step(Assign::local(x, Expr::sub(x, Expr::div(d1, d2))));
    computation_loop
        .branch()
        .add_condition(Expr::eq(prev, x))
        .step(Break::new());
    computation_loop.step(Assign::local(prev, x));

    hir.step(Return::value(x));
}

pub fn bisect(c: &mut Criterion) {
    let mut module = ModuleBuilder::new();

    calc_hir(&mut module);
    derive_hir(&mut module);
    bisect_hir(&mut module);

    c.bench_function("bisect compile", |b| b.iter(|| module.build().unwrap()));

    let module = module.build().unwrap();

    // check filesize of module
    //assert_eq!(317, module.to_bytes().unwrap().len());

    let mut vm = create_vm();
    vm.load_and_import_all(module).unwrap();

    // f(x)=2x^3 + 2x^2 - x
    c.bench_function("bisect f", |b| {
        b.iter(|| {
            assert_eq!(
                0,
                vm.call("bisect", &[vec![2, 2, -1, 0].into(), 1.into()])
                    .unwrap()
                    .as_integer_round_inner()
                    .unwrap()
            );
        })
    });

    // g(x)=x^2 - 4x + 4
    c.bench_function("bisect g", |b| {
        b.iter(|| {
            assert_eq!(
                2,
                vm.call("bisect", &[vec![1, -4, 4].into(), 1.into()])
                    .unwrap()
                    .as_integer_round_inner()
                    .unwrap()
            );
        })
    });

    // h(x)=x^2 - 2x - 3
    c.bench_function("bisect h", |b| {
        b.iter(|| {
            assert_eq!(
                3,
                vm.call("bisect", &[vec![1, -2, -3].into(), 1.into()])
                    .unwrap()
                    .as_integer_round_inner()
                    .unwrap()
            );
        })
    });
}
