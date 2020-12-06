use lovm2::gen::prelude::*;
use lovm2::vm::Vm;

fn loading() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let mut hir = Hir::new();
    hir.push(Assign::local(lv2_var!(n), 0));

    let mut repeat = Repeat::until(Expr::eq(lv2_var!(n), 10));
    repeat.push(lv2_call!(print, n));
    repeat.push(Assign::local(lv2_var!(n), Expr::add(lv2_var!(n), 1)));
    hir.push(repeat);

    builder.add("main").hir(hir);

    builder
}

fn main() {
    let builder = loading();

    match builder.build() {
        Ok(result) => {
            println!("{}", result);

            let mut vm = Vm::with_std();
            vm.load_and_import_all(result).unwrap();

            if let Err(err) = vm.run() {
                println!("{}", err);
            }
        }
        Err(msg) => println!("{}", msg),
    }
}
