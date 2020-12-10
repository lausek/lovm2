# Bytecode

`lovm2` is centered around the value stack. This is where the actual computation happens, parameters are passed to functions and data is shared with interrupts. There are instructions that put values on top of the stack like `Pushc`, `Pushl`, and `Pushg`. Some just take a value off and store it somewhere like `Movel`, `Moveg`. Almost all other instructions will take a given amount of values from it and leave a return value in place.

For example, the term `1 + (2 * 3)` will be compiled to this sequence:

```
 instruction    | stack
----------------------------
 Pushc          | [1]
 Pushc          | [1, 2]
 Pushc          | [1, 2, 3]
 Mul            | [1, 6]
 Add            | [7]
```

You do not need to micromanage the bytecode itself. There are common language constructs with which you can built pretty much everything. These constructs are composed on a function level as `Hir` so every new function gets its own **h**igh-level **i**ntermediate **r**epresentation. Below you can see the transformation process of a function into a runnable `CodeObject`.

```
Hir -> Lir -> CodeObject
```

`CodeObject`'s on their own are already valid programs, but - as usual in every language - functions can be bundled together in some sort of collection - called `Module`.

```
Hir -> Lir -> CodeObject
                         \
Hir -> Lir -> CodeObject  --> Module
                         /
Hir -> Lir -> CodeObject
```
