# Iterators

Iterators are special values that are used to step through the items of `Str`, `List` and `Dict`. Creating them from a `HIR` perspective is as easy as passing the source expression into `Iter::create(..)`. Depending on the base collection, the return type of `Iter::next(..)` will change:

- `Str`: Characters of the string.
- `Dict`: A `List` in which the first item is the `key` and the second item is the `value`.

Only `List` causes the iterator to return a contained value as is.

## Ranges

You can also use `Iter::create_ranged(from, to)` if you want to have a sequence of numbers.  Note that `from` is inclusive and `to` is exclusive.

``` rust,no_run
let it1 = Iter::create_ranged(0, 5);
// => [0, 1, 2, 3, 4]

let it2 = Iter::create_ranged(-5, 5);
// => [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4]

// passing an iterator expression into `Iter::reverse` does 
// exactly what you would expect

let it3 = Iter::reverse(Iter::create_ranged(0, 5));
// => [4, 3, 2, 1, 0]

// you can also omit `from` and `to` by passing a `Nil` value

let it4 = Iter::create_ranged(Value::Nil, 5)
// => [0, 1, 2, 3, 4]

let it5 = Iter::create_ranged(10, Value::Nil)
// => [10, 11, 12, ...]
```

## Example

If you want to control the iterator more granularly, feel free to use `Iter::has_next(it)` and `Iter::next(it)`.

``` rust,no_run
let (it, item) = &lv2_var!(it, item);

main_hir.assign(it, Iter::create(lv2_list!(1, 2, 3, 4)));

main_hir
    .repeat_until(Expr::not(Iter::has_next(it)))
    .assign(item, Iter::next(it));
```
