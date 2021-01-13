//! lovm2 is a lightweight virtual machine with a focus on simplicity and extendability.
//!
//! ## Features
//!
//! - dynamic typing
//! - generate bytecode using highlevel intermediate representation
//! - call into shared objects: [lovm2_extend](lovm2_extend/README.md)
//! - python bindings: [pylovm2](pylovm2/README.md)
//! - define own callbacks for interrupts
//!
//! ## Example
//!
//! ``` rust
//! use lovm2::create_vm_with_std;
//! use lovm2::prelude::*;
//!
//! let mut module = ModuleBuilder::new();
//!
//! // a module needs a code object called `main`
//! // if you want to make it runnable
//! let main_hir = module.entry();
//!
//! // set the local variable n to 10
//! main_hir.step(Assign::local(&lv2_var!(n), 10));
//!
//! // `print` is a builtin function. the `lv2_var!` macro
//! // ensures that the given identifier is not confused
//! // with a string.
//! main_hir.step(Call::new("print").arg(lv2_var!(n)).arg("Hello World"));
//! // ... this is equivalent to the developer-friendly version:
//! main_hir.step(lv2_call!(print, n, "Hello World"));
//!
//! // consumes the `ModuleBuilder` and transforms
//! // it into a `Module`
//! let module = module.build().unwrap();
//! println!("{}", module);
//!
//! // load the module and run it
//! let mut vm = create_vm_with_std();
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
