# Optimization

The rudimentary bytecode optimizer is enabled by default. It acts upon the generated `Lir`.

## Constant evaluation

Computing constant operations ahead can not only improve the programs performance, but also drop certain constants out of the `CodeObject` overall therefore reducing its size.

```
```

## Logical short-curcuit

All languages should not evaluate the second operand of a `Or` or `And` operation if the first result is already sufficient for the expressions outcome.

## Small adjustments

```
```
