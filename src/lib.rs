//! `lovm2` is a library for building your own programming language in the blink of an eye. It offers you easy to use constructs to generate bytecode for its virtual machine.

//! ## Features

//! - Dynamic typing
//! - Generate bytecode using a High-Level Intermediate Representation
//! - Define own instructions as `Interrupt`s
//! - Extend your programs with Rust: [lovm2 extend](README-extend.md)
//! - Standard library included: [lovm2_std](src/lovm2_std/README.md)
//! - Python bindings: [pylovm2](pylovm2/README.md)

//! ## Examples

//! Add this line to your `Cargo.toml`:

//! ``` toml
//! lovm2 = "0.4.9"
//! ```

//! ### Resources

//! - [The lovm2 Guide](https://lausek.eu/lovm2/guide/book/)

//! ### Projects

//! - [lol - a lisp language](https://github.com/lausek/lol)
//! - [quasicode - the best language around](https://github.com/witling/quasicode)

//! ### Generating Bytecode

//! ``` rust
//! use lovm2::prelude::*;

//! let mut module = ModuleBuilder::new();

//! // declare the variables our code will use
//! let n = &lv2_var!(n);
//! // a module needs a code object called `main`
//! // if you want to make it runnable
//! let main_hir = module.entry();
//! // set the local variable n to 10
//! main_hir.assign(n, 10);
//! // `print` is a builtin function
//! main_hir.step(Call::new("print").arg(n).arg("Hello World"));

//! // creates a `Module` from the `ModuleBuilder`
//! let module = module.build().unwrap();
//! println!("{}", module);

//! // load the module and run it
//! let mut vm = lovm2::create_vm_with_std();
//! vm.add_main_module(module).expect("load error");
//! vm.run().expect("run error");
//! ```

pub use lovm2_core::*;

/// Create a new instance with standard functions already imported
#[cfg(feature = "stdlib")]
pub fn create_vm_with_std() -> lovm2_core::vm::Vm {
    let module = lovm2_std::create_std_module();
    let mut vm = lovm2_core::vm::Vm::new();
    vm.add_module(module, false).unwrap();
    vm
}
