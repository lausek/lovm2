# Branching

While working on your functions hir, you can call the `.branch()` method to create a point of conditional execution.

``` rust,no_run
let main_hir = builder.entry();

// ...

let equal_check = main_hir.branch();

equal_check
    .add_condition(expr)
    .step(...);

equal_check
    .default_condition()
    .step(...);
```
