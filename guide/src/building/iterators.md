# Iterators

Iterators are special values that are used to step through the items of `Str`, `List` and `Dict`. Creating them from a `HIR` perspective is as easy as calling `.to_iter()` on the source expression. Depending on the base collection, the return type of `LV2Expr::from(it).next(..)` will change:

- `Str`: Characters of the string.
- `Dict`: A `List` in which the first item is the `key` and the second item is the `value`.

Only `List` causes the iterator to return a contained value as is.

## Ranges

You can also use `LV2Expr::iter_ranged(from, to)` if you want to have a sequence of numbers.  Note that `from` is inclusive and `to` is exclusive.

``` rust,no_run
let it1 = LV2Expr::iter_ranged(0, 5);
// => [0, 1, 2, 3, 4]

let it2 = LV2Expr::iter_ranged(-5, 5);
// => [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4]

// passing an iterator expression into `Iter::reverse` does 
// exactly what you would expect

let it3 = LV2Expr::create_ranged(0, 5).reverse();
// => [4, 3, 2, 1, 0]

// you can also omit `from` and `to` by passing a `Nil` value

let it4 = LV2Expr::iter_ranged(Value::Nil, 5)
// => [0, 1, 2, 3, 4]

let it5 = LV2Expr::iter_ranged(10, Value::Nil)
// => [10, 11, 12, ...]
```

## Example

If you want to control the iterator more granularly, feel free to use `LV2Expr::from(it).has_next()` and `LV2Expr::from(it).next()`.

``` rust,no_run
let (it, item) = &lv2_var!(it, item);

main_hir.assign(it, lv2_list!(1, 2, 3, 4).to_iter());

main_hir
    .repeat_until(LV2Expr::from(it).has_next().not()))
    .assign(item, LV2Expr::from(it).next());
```
