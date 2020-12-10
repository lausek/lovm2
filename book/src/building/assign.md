# Assignment

``` rust,no_run
Assign::local(lv2_var!(n), expr)
```

``` rust,no_run
Assign::global(lv2_var!(n), expr)
```

There is also a special variant for setting the values on lists and dictionaries.

``` rust,no_run
Assign::local(lv2_var!(point), lv2_dict!());
Assign::set(lv2_access!(point, x), x_coord);
Assign::set(lv2_access!(point, y), y_coord);
```
