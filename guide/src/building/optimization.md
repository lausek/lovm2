# Optimization

A rudimentary peephole optimizer is enabled by default. It acts upon the generated `LIR`. If you want to disable optimization at all, use the method `build_with_options` and set the attribute `optimize` to false.

`lovm2` gives you a guarantee that your `HIR` will not be unexpectedly altered when lowering.

## Dead Code elimination

After lowering the `LIR`, the optimizer will eliminate code that is not reachable.

``` lir
main:               
    CPush(0)        main:
    Ret        =>       CPush(0)
    CPush(1)            Ret
    ...
```

## Constant evaluation

Computing constant operations ahead can not only improve the programs performance, but also drop certain constants out of the `CodeObject` overall therefore reducing its size. Bytecode sequences like will be tranformed like this:

``` bytecode
CPush(0)
CPush(1)   =>   CPush(2)
Add
```

> **Note:** The optimizer currently only optimizes expressions if all operands are constant and does not cover neutral elements like `+ 0` or `* 1` as such.

## Logical short-curcuit

It is common for languages to avoid evaluating the second operand of `Or`/`And` operations if the first term is already sufficient for the expressions outcome.

## Logical negation

The optimizer will merge instruction sequences like this:

``` bytecode
...
Not   =>   Jf
Jt
```
