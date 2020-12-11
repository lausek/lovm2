# Assignment

To create or change a local variable, it is sufficient to use this construct:

``` rust,no_run
Assign::local(&lv2_var!(n), expr)
```

It is possible to manipulate the global context using this variant:

``` rust,no_run
Assign::global(&lv2_var!(n), expr)
```

There is even a way of setting the values on lists and dictionaries. Under the hood, `Set` is actually expecting a `Ref` as the target location - which is retrieved by `Access` - and overwrites the value inside. This is compatible with the way dictionaries and lists are internally constructed.

``` rust,no_run
Assign::local(&lv2_var!(point), lv2_dict!());
Assign::set(&lv2_access!(point, x), x_coord);
Assign::set(&lv2_access!(point, y), y_coord);
```
