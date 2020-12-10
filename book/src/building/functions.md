# Functions

As we already found out in the [Concepts](../concepts/bytecode.md) chapter, a `Hir` is conceptually equal to a function. The resulting bytecode can process a given amount of parameters and leave a return value in place.

``` rust,no_run
use lovm2::prelude::*;

#fn main() {
// creates a hir with no arguments
let mut fn_no_args = Hir::new();

// creates a hir that expects parameter n
let mut fn_with_args = Hir::with_args(&[lv2_var!(n)]);
#}
```

To return from function, add a `Return::value(expr)` to the hir specifying the returned value or `Return::nil()` if no value is produced.

Due to the convention that every function has to return a value an implicit `Return::nil()` is appended if the last instruction is not a return already.
