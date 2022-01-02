# Expressions

The `LV2Expr` structure represents any computation that leads to a new value on the stack. Expressions can be nested arbitrarily.

To give you an overview of what an expression could look like, here is the stripped down version of its actual implementation.

``` rust,no_run
pub enum LV2Expr {
    // a constant value
    Value,
    // variable in read position
    Variable,
    // a branch that evaluates expressions conditionally
    Branch,
    // call to a function
    Call,
    // operations with one operand
    Operation1,
    // operations with two operands
    Operation2,
    // ... and many more.
}
```

## Resolvement of Variables

The scope of a variable is determined by the previously emitted `Global` and `Local` declarations.
In a block, the variable's scope is not fixed but can change after each statement.

> **Behavior before v0.5.0:** While lowering, the runtime keeps track of locally assigned identifiers. This is crucial for determining the scope during read access later. If a variable is not known locally, a fallback to global scope happens.

## Example

We want to transform the formula `(1 + 2) * f(2)` to `lovm2` bytecode. Use a parser of your choice to generate an abstract syntax tree from the textual representation. Note that `lovm2` does not care about operator priorities so its your parsers duty to correctly handle them. After processing the input, your AST should look something like this:

```
Value(1)
        \
         -- Operation(+)
        /               \
Value(2)                 -- Operation(*)
                        / 
              Call(f, 2)
```

And here is the compiletime representation of said formula:

``` rust,no_run
let formula = LV2Expr::from(1)
                .add(2)
                .mul(LV2Call::new("f").arg(2));
```

The unoptimized `LIR` now looks like this:

``` lir
CPush(1)
CPush(2)
Operator2(Add)
CPush(2)
Call(f, 1)
Operator2(Mul)
```
