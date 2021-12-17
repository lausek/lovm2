use lovm2::create_vm_with_std;
use lovm2::gen::prelude::*;

fn loading() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let hir = builder.entry();
    let n = &lv2_var!(n);

    hir.assign(n, 0);

    hir.repeat_until(Expr::eq(n, 10))
        .step(lv2_call!(print, n))
        .assign(n, Expr::add(n, 1));

    builder
}

fn main() {
    let builder = loading();

    match builder.build() {
        Ok(result) => {
            println!("{}", result);

            let mut vm = create_vm_with_std();
            vm.add_main_module(result).unwrap();

            if let Err(err) = vm.run() {
                println!("{}", err);
            }
        }
        Err(msg) => println!("{:?}", msg),
    }
}
