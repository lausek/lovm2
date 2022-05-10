# lol

S-expressions for lovm2.

## Example

```
(def fib (n)
    (if (or (eq n 0) (eq n 1))
        (ret n)
        (ret (+ (fib (- n 1)) (fib (- n 2))))))
```

## Builtin Macros

```
+
-
*
/
%
eq
ne
ge
gt
le
lt
and
bool
or
break
continue
dict
do
float
foreach
if
import
import-global
int
let
list
loop
range
ret
str
```
