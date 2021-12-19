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

fn calc_hir(module: &mut LV2ModuleBuilder) {
    let (coeffs, x, factor, i, sigma) = &lv2_var!(coeffs, x, factor, i, sigma);
    let hir = module.add_with_args("calc", vec![coeffs.clone(), x.clone()]);

    hir.assign(sigma, 0)
        .assign(i, 0)
        .assign(factor, lv2_call!(len, coeffs).sub(1));

    let delta = lv2_access!(coeffs, i).mul(LV2Expr::from(x).pow(factor));
    hir.repeat_until(LV2Expr::from(0).le(factor).not())
        .assign(sigma, LV2Expr::from(sigma).add(delta))
        .increment(i)
        .decrement(factor);

    hir.return_value(sigma);
}

fn derive_hir(module: &mut LV2ModuleBuilder) {
    let (coeffs, i, factor, d) = &lv2_var!(coeffs, i, factor, d);
    let hir = module.add_with_args("derive", vec![coeffs.clone()]);

    hir.assign(d, lv2_list!())
        .assign(i, 0)
        .assign(factor, lv2_call!(len, coeffs).sub(1));

    hir.repeat_until(LV2Expr::from(0).lt(factor).not())
        .set(
            lv2_access!(d, i),
            LV2Expr::mul(lv2_access!(coeffs, i), factor),
        )
        .increment(i)
        .decrement(factor);

    hir.return_value(d);
}

pub fn bisect_hir(module: &mut LV2ModuleBuilder) {
    let (prev, x, d1, d2, coeffs, dcoeffs, startx) =
        &lv2_var!(prev, x, d1, d2, coeffs, dcoeffs, startx);
    let hir = module.add_with_args("bisect", vec![coeffs.clone(), startx.clone()]);

    // function setup
    hir.assign(x, LV2Expr::from(startx).to_float())
        .assign(dcoeffs, lv2_call!(derive, coeffs))
        .assign(prev, LV2Value::Nil);

    let computation_loop = hir.repeat();
    computation_loop.assign(d2, lv2_call!(calc, dcoeffs, x));
    computation_loop
        .branch()
        .add_condition(LV2Expr::from(d2).eq(0.))
        .assign(d2, 0.001);
    computation_loop.assign(d1, lv2_call!(calc, coeffs, x));
    computation_loop.assign(x, LV2Expr::from(d1).div(d2).sub(x));
    computation_loop
        .branch()
        .add_condition(LV2Expr::from(prev).eq(x))
        .break_repeat();
    computation_loop.assign(prev, x);

    hir.return_value(x);
}

pub fn bisect(c: &mut Criterion) {
    let mut module = LV2ModuleBuilder::new();
    module.entry();

    calc_hir(&mut module);
    derive_hir(&mut module);
    bisect_hir(&mut module);

    c.bench_function("bisect compile", |b| b.iter(|| module.build().unwrap()));

    let module = module.build().unwrap();

    // check filesize of module
    //assert_eq!(317, module.to_bytes().unwrap().len());

    let mut vm = create_vm();
    vm.add_main_module(module).unwrap();

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
