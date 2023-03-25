[![crates.io badge](https://img.shields.io/crates/v/lovm2.svg)](https://crates.io/crates/lovm2)
[![docs.rs badge](https://docs.rs/lovm2/badge.svg?version=latest)](https://docs.rs/lovm2/)

# lovm2

`lovm2` is a library for building your own programming language in the blink of an eye. It offers you easy to use constructs to generate bytecode for its virtual machine.

> :warning **lovm2** is not actively maintained and might be subject to vulnerabilities.

## Features

- Dynamic typing
- Generate bytecode using a High-Level Intermediate Representation
- Define own instructions as `Interrupt`s
- Extend your programs with Rust: [lovm2 extend](README-extend.md)
- Standard library included: [lovm2_std](crates/lovm2_std/README.md)
- Python bindings: [pylovm2](pylovm2/README.md)

## Examples

Add this line to your `Cargo.toml`:

``` toml
lovm2 = { git = "https://github.com/lausek/lovm2", rev = "d454b21" }
```

### Resources

- [The lovm2 Guide](https://lausek.eu/lovm2/guide/book/)

### Projects

- [lol - a lisp language](https://github.com/lausek/lol)
- [quasicode - the best language around](https://github.com/witling/quasicode)

### Generating Bytecode

``` rust
use lovm2::prelude::*;

let mut module = LV2ModuleBuilder::new();

// declare the variables our code will use
let n = &lv2_var!(n);
// a module needs a code object called `main`
// if you want to make it runnable
let main_hir = module.entry();
// set the local variable n to 10
main_hir.assign(n, 10);
// `print` is a builtin function
main_hir.step(LV2Call::new("print").arg(n).arg("Hello World"));

// creates a `Module` from the `ModuleBuilder`
let module = module.build().unwrap();
println!("{}", module);

// load the module and run it
let mut vm = lovm2::create_vm_with_std();
vm.add_main_module(module).expect("load error");
vm.run().expect("run error");
```

### Internal Source Code References

- [Bytecode](https://github.com/lausek/lovm2/blob/master/crates/lovm2_core/src/bytecode.rs)
- [Context](https://github.com/lausek/lovm2/blob/master/crates/lovm2_core/src/vm/context.rs)
- [Vm](https://github.com/lausek/lovm2/blob/master/crates/lovm2_core/src/vm/mod.rs)

#### Customer Reviews

> *This Thing Fast* - Sonic

> *And I thought I was simple...* - Pythagorean Theorem
