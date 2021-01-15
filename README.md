[![crates.io badge](https://img.shields.io/crates/v/lovm2.svg)](https://crates.io/crates/lovm2)
[![docs.rs badge](https://docs.rs/lovm2/badge.svg?version=latest)](https://docs.rs/lovm2/)

# lovm2

`lovm2` is a library for building your own programming language in the blink of an eye. It offers you easy to use constructs to generate bytecode for its virtual machine.

## Features

- [X] Dynamic typing
- [X] Generate bytecode using a High-Level Intermediate Representation
- [X] Define own instructions as `Interrupt`s
- [X] Standard library included: [lovm2_std](src/lovm2_std/README.md)
- [X] Extend your programs with Rust: [lovm2_extend](src/lovm2_extend/README.md)
- [X] Python bindings: [pylovm2](pylovm2/README.md)

## Examples

Add this line to your `Cargo.toml`:

```
lovm2 = "0.4.8"
```

### Projects

- [lol - a lisp language](https://github.com/lausek/lol)
- [quasicode - the best language around](https://github.com/witling/quasicode)

### Generating Bytecode

``` rust
use lovm2::prelude::*;

let mut module = ModuleBuilder::new();

// a module needs a code object called `main`
// if you want to make it runnable
let main_hir = module.entry();

// set the local variable n to 10
main_hir.step(Assign::local(&lv2_var!(n), 10));

// `print` is a builtin function. the `lv2_var!` macro
// ensures that the given identifier is not confused
// with a string.
main_hir.step(Call::new("print").arg(lv2_var!(n)).arg("Hello World"));
// ... this is equivalent to the developer-friendly version:
main_hir.step(lv2_call!(print, n, "Hello World"));

// creates a `Module` from the `ModuleBuilder`
let module = module.build().unwrap();
println!("{}", module);

// load the module and run it
let mut vm = create_vm_with_std();
vm.add_main_module(module).expect("load error");
vm.run().expect("run error");
```

### Internal Source Code References

- [Bytecode](https://github.com/lausek/lovm2/blob/master/src/lovm2_core/src/bytecode.rs)
- [Context](https://github.com/lausek/lovm2/blob/master/src/lovm2_core/src/vm/context.rs)
- [Vm](https://github.com/lausek/lovm2/blob/master/src/lovm2_core/src/vm/mod.rs)

#### Customer Reviews

> *This Thing Fast* - Sonic

> *And I thought I was simple...* - Pythagorean Theorem
