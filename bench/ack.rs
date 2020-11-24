use criterion::Criterion;

use lol::create_lol_module;
use lovm2::prelude::*;
use lovm2::vm::Vm;

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

    let mut vm = Vm::new();
    let module = create_lol_module(
        "main",
        "
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
        "
    ).unwrap();

    vm.load_and_import_all(module).unwrap();

    c.bench_function("ack", |b| {
        b.iter(|| {
            // ack(3, 2) = 29
            assert_eq!(
                Value::from(29),
                vm.call("ack", &[3.into(), 2.into()])
                    .unwrap()
                    .into_integer_round()
                    .unwrap()
            );
        })
    });
}
