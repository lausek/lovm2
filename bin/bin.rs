use lovm2::expr::Expr;
use lovm2::hir::prelude::*;
use lovm2::module::ModuleBuilder;
use lovm2::vm::Vm;

fn looping() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let mut hir = HIR::new();
    let repeat = Repeat::until(Expr::eq(
        Variable::from("n").into(),
        CoValue::Int(10).into(),
    ))
    .push(Call::new("print").arg(Variable::from("n")))
    .push(Assign::local(
        "n".into(),
        Expr::add(Variable::from("n").into(), CoValue::Int(1).into()),
    ));
    hir.push(Assign::local("n".into(), CoValue::Int(0).into()));
    hir.push(repeat);

    builder.add("main").hir(hir);

    builder
}

fn simple(hir: &mut HIR) {
    hir.push(Assign::local("n".into(), CoValue::Int(2).into()));

    hir.push(Call::new("len").arg(CoValue::List(vec![])));
}

fn create_call_example() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let mut nop_hir = HIR::new();
    nop_hir.push(Assign::local("a".into(), CoValue::Bool(true).into()));

    let mut main_hir = HIR::new();
    main_hir.push(Assign::local("n".into(), CoValue::Int(2).into()));
    main_hir.push(Call::new("print").arg(CoValue::Str("hej".to_string())));

    builder.add("main").hir(main_hir);
    builder.add("nop").hir(nop_hir);

    builder
}

fn create_greet() -> ModuleBuilder {
    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Call::new("print").arg(Call::new("input")));

    builder.add("main").hir(main_hir);

    builder
}

fn main() {
    let builder = looping();

    match builder.build() {
        Ok(result) => {
            println!("{:#?}", result);

            let mut vm = Vm::new();
            vm.load_and_import_all(result).unwrap();
            vm.run().unwrap();
        }
        Err(msg) => println!("{}", msg),
    }
}
