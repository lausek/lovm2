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

## Expression Branches

Regular branches are considered statements and do not leave values on stack as such. However, there is a special expression variant `LV2Expr::branch()` that lets you create the adhoc version of a ternary operator.

``` rust,no_run
// f(n) = n == 2 ? "a" : "b"

let n = lv2_var!(n);
let f_body = LV2Expr::branch().add_condition(LV2Expr::from(n).eq(2), "a").default_value("b");

builder
    .add_with_args("f", vec![n])
    .return_value(f_body);
```

> **Note:** `LV2ExprBranch` must always return a value meaning that you have to call the `default_value` method at least once. Otherwise the compiler will complain that a structure of type `LV2ExprBranchIncomplete` cannot be converted into an expression. This is a compile time check to ensure that those branches always evaluate to a value.

## Example

Let's implement a function that returns *1* if the given value is equal to 2 and *0* otherwise. From a Rust perspective, we could generate the function like this:

``` rust,no_run
let main = builder.add_with_args("is_2", vec![n.clone()]);
let branch = main.branch();

branch
    .add_condition(LV2Expr::from(n).eq(2))
    .return_value(1);

branch
    .default_condition()
    .return_value(0);
```

The representation will be translated temporarily to the following optimized LIR. As you can see, a lot of labels (prefixed with a `.`) got involved right now. Everything between `.cond_0`'s start and end label is derived from our first conditions predicate and body. The `JumpIfFalse` instruction separates them by making sure that the body will be skipped if the expression evaluates to false. As usual, whenever we hit a return instruction, the function will terminate assuring that we will not fall through into our default branch.

``` lir
is_2:
	Store(n)
.branch_start:
.cond_start:
	Push(n)
	CPush(2)
	Operator2(Equal)
	JumpIfFalse(.cond_end)
	CPush(1)
	Ret
.cond_end:
	CPush(0)
	Ret
```

In the last lowering step, there are only two things left to do: resolving label offsets and patching them into the jump instructions.
