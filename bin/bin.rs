use lovm2::hir::prelude::*;
use lovm2::vm::Vm;

fn loading() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let mut hir = HIR::new();
    hir.push(Assign::local(var!(n), 0));

    let mut repeat = Repeat::until(Expr::eq(var!(n), 10));
    repeat.push(call!(print, n));
    repeat.push(Assign::local(var!(n), Expr::add(var!(n), 1)));
    hir.push(repeat);

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
            Expr::rem(Variable::from("n"), Value::Int(3)),
            Value::Int(0),
        ))
        .from(Block::new().with(Assign::local(
            "result".into(),
            Value::Str("fizz".to_string()),
        )));

    branch
        .add_condition(Expr::eq(
            Expr::rem(Variable::from("n"), Value::Int(5)),
            Value::Int(0),
        ))
        .from(Block::new().with(Assign::local(
            "result".into(),
            Value::Str("buzz".to_string()),
        )));

    branch
        .default_condition()
        .from(Block::new().with(Assign::local(
            "result".into(),
            Value::Str("none".to_string()),
        )));

    hir.push(Assign::local("n".into(), Value::Int(5)));
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
            vm.load_and_import_all(result).unwrap();

            if let Err(err) = vm.run() {
                println!("{}", err);
            }
        }
        Err(msg) => println!("{}", msg),
    }
}
