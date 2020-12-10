# Expressions

The `Expr` represents any computation that leads to a new value on the stack. Expressions can be nested arbitrarily.

```
Value(1)
        \
         -- Operation(+)
        /               \
Value(2)                 -- Operation(*)
                        / 
              Call(f, 2)
```


``` rust,no_run
pub enum Expr {
    // a constant value
    Value { val: Value, boxed: bool },
    // variable in read position
    Variable(Variable),
    // call to a function
    Call(Call),
    // operations with one operand
    Operation1(Operator1, Box<Expr>),
    // operations with two operands
    Operation2(Operator2, Box<Expr>, Box<Expr>),
    // result of a type conversion
    Cast(Cast),
    // attribute read on a list or dict
    Access(Access),
    // create a mutable subpart of a list
    Slice(Slice),
    // special variant for creating lists and dicts
    DynamicValue(Initialize),
}
```
