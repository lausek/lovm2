# Branching

While working on your functions hir, you can call the `.branch()` method to create a point of conditional execution. A branch can have several conditions with associated blocks and at most one default condition that is always met. Branches with just a `default_condition` are not allowed.

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

## LIR Layout

``` lir
is_2:
	Store(Local, n)
branch_0_start:
cond_0_start:
	Push(Local, n)
	CPush(2)
	Operator2(Equal)
	Jump(Some(false), cond_0_end)
	CPush(1)
	Ret
cond_0_end:
	CPush(0)
	Ret
```
