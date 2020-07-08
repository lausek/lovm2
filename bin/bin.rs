use lovm2::block::Block;
use lovm2::expr::Expr;
use lovm2::hir::prelude::*;
use lovm2::module::ModuleBuilder;
use lovm2::vm::Vm;

fn loading() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let mut hir = HIR::new();
    hir.push(Include::load("mfunky"));
    hir.push(Call::new("gofunky"));

    builder.add("main").hir(hir);

    builder
}

/*
fn true_branching() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let mut hir = HIR::new();

    let mut branch = Branch::new();
    branch
        .add_condition(Expr::eq(
            Expr::rem(Variable::from("n"), CoValue::Int(3)),
            CoValue::Int(0),
        ))
        .from(Block::new().with(Assign::local(
            "result".into(),
            CoValue::Str("fizz".to_string()),
        )));

    branch
        .add_condition(Expr::eq(
            Expr::rem(Variable::from("n"), CoValue::Int(5)),
            CoValue::Int(0),
        ))
        .from(Block::new().with(Assign::local(
            "result".into(),
            CoValue::Str("buzz".to_string()),
        )));

    branch
        .default_condition()
        .from(Block::new().with(Assign::local(
            "result".into(),
            CoValue::Str("none".to_string()),
        )));

    hir.push(Assign::local("n".into(), CoValue::Int(5)));
    hir.push(branch);

    builder.add("main").hir(hir);

    builder
}
*/

fn main() {
    let builder = loading();

    match builder.build() {
        Ok(result) => {
            println!("{:#?}", result);

            let mut vm = Vm::new();
            vm.load_and_import_all(result.boxed()).unwrap();

            if let Err(err) = vm.run() {
                println!("{}", err);
            }
        }
        Err(msg) => println!("{}", msg),
    }
}
