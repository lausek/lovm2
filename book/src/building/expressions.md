# Expressions

The `Expr` represents any computation that leads to a new value on the stack. Expressions can be nested arbitrarily.

To give you an overview of what an expression could look like, here is the stripped down version of its actual implementation.

``` rust,no_run
pub enum Expr {
    // a constant value
    Value,
    // variable in read position
    Variable,
    // call to a function
    Call,
    // operations with one operand
    Operation1,
    // operations with two operands
    Operation2,
    // result of a type conversion
    Cast,
    // attribute read on a list or dict
    Access,
    // create a mutable subpart of a list
    Slice,
    // special variant for creating lists and dicts
    DynamicValue,
}
```

## Example

We want to transform the formula `(1 + 2) * f(2)` to `lovm2` bytecode. Use a parser of your choice to generate an abstract syntax tree from the textual representation. Note that `lovm2` does not care about operator priorities so its your parsers duty to correctly handle them. After processing the input, your ast should look something like this:

```
Value(1)
        \
         -- Operation(+)
        /               \
Value(2)                 -- Operation(*)
                        / 
              Call(f, 2)
```

And here is the compiletime representation of said formula. As you can see, every operation has an equivalent static method on `Expr`.

```
let formula = Expr::mul(
    Expr::add(1, 2),
    Call::new("f").arg(2),
);
```
