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
    // special variant for creating lists and dicts
    DynamicValue,
    // create a mutable subpart of a list
    Slice,
}
```

## Resolvement of Variables

While lowering, the runtime keeps track of locally assigned identifiers. This is crucial for determining the scope during read access later. If a variable is not known locally, a fallback to global scope happens.

`Expr` variants relying on this functionality are `Variable` and `Access`. As such, their macro helper functions `lv2_var!` and `lv2_access!` follow the same rules.

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

And here is the compiletime representation of said formula. As you can see, every operation has an equivalent static method on `Expr`. Also note that calling a function has its own construct. This is due to calls being allowed in statement position as well.

``` rust,no_run
let formula = Expr::mul(
    Expr::add(1, 2),
    Call::new("f").arg(2),
);
```

The (unoptimized) `LIR` now looks like this:

``` lir
CPush(1)
CPush(2)
Operator2(Add)
CPush(2)
Call(f, 1)
Operator2(Mul)
```
