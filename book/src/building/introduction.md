# Building Programs

This chapter will show you how to utilize the `gen` module in order to compile your own `lovm2` programs. It is also possible to persist compiled modules onto your disk and load them later.

``` rust,no_run
use lovm2::prelude::*;

#fn main() {
let mut builder = ModuleBuilder::new();
let mut main_hir = Hir::new();

// ...

builder.entry().hir(main_hir);

let module = builder.build().except("compile error");
println!("{}", module);
#}
```
