# Repeating

Loops can be created inside blocks using the `.repeat()` and `.repeat_until(..)` methods.

``` rust,no_run
let main_hir = builder.entry();

// ...

let repeat_endless = main_hir.repeat();

let repeat_until = main_hir.repeat_until(expr);
```

## Control Flow

Inside loop blocks you are free to use `Break` and `Continue` to precisely control the flow. As in every programming language, `Break` terminates the loop while `Continue` jumps to its start again.
