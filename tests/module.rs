use std::path::Path;

use lovm2::{hir::prelude::*, ModuleBuilder};

const SERIALIZE_PATH: &str = "/tmp/hello-world.lovm2c";

#[test]
fn serialize_module() {
    let mut builder = ModuleBuilder::new();

    let mut main_hir = HIR::new();
    main_hir.push(Assign::local(var!(msg), "hello world"));
    main_hir.push(call!(print, msg));

    builder.add("main").hir(main_hir);

    let module = builder.build().unwrap();

    module.store_to_file(SERIALIZE_PATH).unwrap();

    assert!(Path::new(SERIALIZE_PATH).exists());
}
