# Building Programs

This chapter will show you how to utilize the `gen` module in order to compile your own `lovm2` programs. It is also possible to persist compiled modules onto your disk and load them later.

``` rust,no_run
use lovm2::prelude::*;

#fn main() {
let mut builder = ModuleBuilder::new();

// creates the entry point `HIR` and returns a mutable reference.
// this is actually a shortcut for builder.add(ENTRY_POINT)
let main_hir = builder.entry();

// modify `main_hir` with statements
// if in doubt, just call the `step` method and pass it the hir element
main_hir.step(Interrupt::new(10));

let module = builder.build().except("compile error");
println!("{}", module);
#}
```

The main generation functionality is exposed via `Block` and every structure that contains it like `Branch`, `Repeat` and functions. You can use these methods on all of them:

- `step(..)` append a new statement to the block.
- `branch()` create a new branch at the current position. This returns a `BranchBuilder`.
- `repeat()` and `repeat_until(..)` which return a mutable reference to a new block. The first variant is an endless loop, while the latter supports breaking once a condition is met.

## Functions

The whole `ModuleBuilder` is centered around the creation of `HIR`. As we already found out in the [Concepts](../concepts/bytecode.md) chapter, a `HIR` is conceptually equal to a function. The resulting bytecode is able to process a given amount of parameters and leave a return value in place.

As you can see in this example listing, you should not need to create such data manually as there is functionality for adding it to the builder directly.

``` rust,no_run
use lovm2::prelude::*;

#fn main() {
// creates a hir with no arguments
let fn_no_args = builder.add("fn1");

// creates a hir that expects parameter n
let fn_with_args = builder.add_with_args("fn2", &[lv2_var!(n)]);
#}
```

To return from function, add a `Return::value(expr)` to the hir specifying the returned value or `Return::nil()` if no value is produced.

Due to the convention that every function has to return a value, an implicit `Return::nil()` is appended if the last instruction is not a return already.

## Helper Macros

There are a bunch of macros inside the prelude that trivialize creating more complicated `lovm2` constructs for developers.

- `lv2_var!(ident, ...)` turns all the identifiers given into the special type `Variable` which is needed basically everywhere. If more than one ident is declared, this returns a tuple.
- `lv2_dict!(ident => expr, ...)` creates an `Expr` that will dynamically initialize a dictionary with the key-values pairs specified.
- `lv2_list!(item, ...)` creates an `Expr` that initializes a list dynamically.
- `lv2_call!(ident, ... args)` syntactic sugar for the `Call` element.
- `lv2_access!(ident, ... keys)` syntactic sugar for the `Access` element.
