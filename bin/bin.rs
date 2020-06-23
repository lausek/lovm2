use lovm2::hir::*;
use lovm2::hir::prelude::*;
use lovm2::vm::Vm;

fn fill(hir: &mut HIR) {
    hir.push(Assign::local("n".into(), CoValue::Int(2).into()));

    hir.push(Call::new("len").arg(CoValue::List(vec![])));
}

fn main() {
    let mut hir = HIR::new();

    fill(&mut hir);

    match hir.build() {
        Ok(result) => {
            println!("{:#?}", result);

            let mut vm = Vm::new();
            vm.run_object(&result);
        }
        Err(msg) => println!("{}", msg),
    }
}
