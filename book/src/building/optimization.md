# Optimization

The rudimentary bytecode optimizer is enabled by default. It acts upon the generated `Lir`.

## Dead Code elimination

After lowering the `Lir`, the optimizer will eliminate code that is not reachable.

``` lir
main:               
    CPush(0)        main:
    Ret        =>       CPush(0)
    CPush(1)            Ret
    ...
```

## Constant evaluation

Computing constant operations ahead can not only improve the programs performance, but also drop certain constants out of the `CodeObject` overall therefore reducing its size. Bytecode sequences like will be tranformed like this:

``` lir
CPush(0)
CPush(1)   =>   CPush(2)
Add
```

> **v0.4.7 Note:** the optimizer currently only optimizes expressions if all operands are constant.

## Logical short-curcuit

All languages should not evaluate the second operand of a `Or` or `And` operation if the first result is already sufficient for the expressions outcome.

## Small adjustments

The optimizer will merge instruction sequences like this:

``` lir
...
Not   =>   Jt
Jt
```
