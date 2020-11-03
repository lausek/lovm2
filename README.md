[![crates.io badge](https://img.shields.io/crates/v/lovm2.svg)](https://crates.io/crates/lovm2)
[![docs.rs badge](https://docs.rs/lovm2/badge.svg?version=latest)](https://docs.rs/lovm2/)

# lovm2

Is a lightweight virtual machine with a focus on simplicity and extendability.

```
lovm2 = "0.3.8"
```

## Features

- [X] dynamic typing
- [X] generate bytecode using highlevel intermediate representation
- [X] call into shared objects: [lovm2_extend](lovm2_extend/README.md)
- [X] python bindings: [pylovm2](pylovm2/README.md)
- [X] define own callbacks for interrupts

## Examples

### Projects

- [lol - a lisp language](https://github.com/lausek/lol)

### Source Code References

- [Bytecode](https://github.com/lausek/lovm2/blob/master/src/bytecode.rs)
- [Context](https://github.com/lausek/lovm2/blob/master/src/context.rs)
- [Vm](https://github.com/lausek/lovm2/blob/master/src/vm.rs)

### Generating Bytecode

``` rust
use lovm2::prelude::*;
use lovm2::vm::Vm;

fn main() {
    let mut main_hir = HIR::new();

    // set the local variable n to 10
    main_hir.push(Assign::local(lv2_var!(n), 10));

    // `print` is a builtin function. the `lv2_var!` macro
    // ensures that the given identifier is not confused
    // with a string.
    main_hir.push(Call::new("print").arg(lv2_var!(n)).arg("Hello World"));
    // ... this is equivalent to the developer-friendly version:
    main_hir.push(lv2_call!(print, n, "Hello World"));

    let mut module = ModuleBuilder::new();

    // a module needs a code object called `main`
    // if you want to make it runnable
    module.add("main").hir(main_hir);

    // consumes the `ModuleBuilder` and transforms
    // it into a `Module`
    let module = module.build().unwrap();
    println!("{:#?}", module);

    // load the module and run it
    let mut vm = Vm::new();
    vm.load_and_import_all(module).expect("load error");
    vm.run().expect("run error");
}
```

#### Customer Reviews

> *This Thing Fast* - Sonic

> *And I thought I was simple...* - Pythagorean Theorem
