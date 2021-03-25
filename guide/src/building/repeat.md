# Repeating

Loops can be created inside blocks using the `.repeat()` and `.repeat_until(..)` methods.

``` rust,no_run
let main_hir = builder.entry();

let repeat_endless = main_hir.repeat();

// run until the condition is true
let repeat_until = main_hir.repeat_until(expr);
```

To avoid namespace collissions in Rust, there is no `while` construct but you can simply create it via `.repeat_until(Expr::not(..))`. The optimizer makes sure that no instruction overhead is generated.

Inside loop blocks you are free to use `Break` and `Continue` to precisely control the flow. As in every programming language, `Break` terminates the loop while `Continue` jumps to its start again. 

The `.repeat_iterating(collection, item)` constructor is able to consecutively assign every entity to the variable passed as `item` as long as the `collection` value supports iteration. Check the [Iterators](./iterators.md) chapter if you want to find out more about this.

``` rust,no_run
// repeat for all items in an iterator, assign item to variable `i`
let repeat_iterating = main_hir.repeat_iterating(lv2_list!(1, 2, 3), lv2_var!(i));

// ... and this is the elaborate variant
let it = Iter::create(lv2_list!(1, 2, 3));
let repeat_iterating = main_hir.repeat_iterating(it, lv2_var!(i));
```

## Example

We want to print the odd numbers between 0 and 10. This is an unbeautified implementation in pythonic pseudocode.

``` python
i = 0
while True:
    if i == 10:
        break
    i += 1
    if i % 2 == 0:
        continue
    print(i)
```

Translating it to `HIR` one by one could look like this:

``` rust,no_run
let i = &lv2_var!(i);

// i = 0
main_hir.step(Assign::local(i, 0));

let repeat = main_hir.repeat();

// if i == 10: break
repeat
    .branch()
    .add_condition(Expr::eq(i, 10))
    .step(Break::new());

// i += 1
repeat.step(Assign::increment(i));

// if i % 2 == 0: continue
repeat
    .branch()
    .add_condition(Expr::eq(Expr::rem(i, 2), 0))
    .step(Continue::new());

// print(i)
repeat.step(lv2_call!(print, i, "\n"));
```

You can imagine that the resulting `LIR` is a lot more elaborate than the previous examples so we will only focus on the essential parts. From a intermediate perspective an endless loop is implemented by appending an unconditional jump to the loops body.

``` lir
main:
.rep_0_start:
    ...
	Jump(.rep_0_start)
.rep_0_end:
```

To terminate the loop once the counter variable reaches 10, we add a conditional break to the body. This is solely a jump targeting the loops end label.

``` lir
.cond_0_start:
	PushLocal(i)
	CPush(10)
	Operator2(Equal)
	JumpIfFalse(.cond_0_end)
	Jump(.rep_0_end)
.cond_0_end:
```

On the other hand `Continue` targets the start label.