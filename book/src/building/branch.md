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

## Example

Let's implement a function that returns *1* if the given value is equal to 2 and *0* otherwise. From a Rust perspective, we could generate the function like this:

``` rust,no_run
let main = builder.add_with_args("is_2", vec![n.clone()]);
let branch = main.branch();

branch
    .add_condition(Expr::eq(n, 2))
    .step(Return::value(1));

branch
    .default_condition()
    .step(Return::value(0));
```

The representation will be translated temporarily to the following optimized LIR. As you can see, a lot of labels (prefixed with a `.`) got involved right now. Everything between `.cond_0`'s start and end label is derived from our first conditions predicate and body. The `JumpIfFalse` instruction separates them by making sure that the body will be skipped if the expression evaluates to false. As usual, whenever we hit a return instruction, the function will terminate assuring that we will not fall through into our default branch.

``` lir
is_2:
	StoreLocal(n)
.branch_0_start:
.cond_0_start:
	PushLocal(n)
	CPush(2)
	Operator2(Equal)
	JumpIfFalse(.cond_0_end)
	CPush(1)
	Ret
.cond_0_end:
	CPush(0)
	Ret
```

In the last lowering step, there are only two things left to do: resolving label offsets and patching them into the jump instructions.
