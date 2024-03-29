use criterion::Criterion;

use lovm2_sexp::create_module;

use super::*;

pub fn ackermann(c: &mut Criterion) {
    /*
    function ack(n, m)
        if n = 0
            return m + 1
        else if m = 0
            return ack(n - 1, 1)
        else
            return ack(n - 1, ack(n, m - 1))

    function ack(n, m)
        while n ≠ 0
            if m = 0
                m:= 1
            else
                m:= ack(n, m - 1)
            n:= n - 1
        return m + 1
    */

    let mut vm = create_vm();
    let module = create_module(
        "main",
        "
        (def main ())

        (def ackr (n m)
            (if (eq n 0)
                (ret (+ m 1))
                (if (eq m 0)
                    (ret (ackr (- n 1) 1))
                    (ret (ackr (- n 1) (ack n (- m 1)))))))
        
        (def ack (n m)
            (loop
                (if (eq n 0)
                    (break))
                (if (eq m 0)
                    (let m 1)
                    (let m (ack n (- m 1))))
                (let n (- n 1)))
            (ret (+ m 1)))
        ",
    )
    .unwrap();

    // hack to get around dev-dependency limitation:
    // https://github.com/rust-lang/rust/issues/79381
    const PERSISTANCE_HACK: &str = "/tmp/ack.lovm2c";
    module.store_to_file(PERSISTANCE_HACK).unwrap();
    if let Ok(module) = LV2Module::load_from_file(PERSISTANCE_HACK) {
        vm.add_main_module(module).unwrap();

        c.bench_function("ack", |b| {
            b.iter(|| {
                // ack(3, 2) = 29
                assert_eq!(
                    29,
                    vm.call("ack", &[3.into(), 2.into()])
                        .unwrap()
                        .as_integer_round_inner()
                        .unwrap()
                );
            })
        });
    } else {
        println!("lol and lovm2 versions are incompatible");
    }
}
