# Expressions

The `Expr` represents any computation that leads to a new value on the stack. Expressions can be nested arbitrarily. For example, the formula `f(2) * (1 + 2)` gets transformed into something like this:

```
Value(1)
        \
         -- Operation(+)
        /               \
Value(2)                 -- Operation(*)
                        / 
              Call(f, 2)
```

Note that `lovm2` does not care about operator priorities so its your parsers duty to correctly handle them.

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
