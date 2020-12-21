# Bytecode

`lovm2` is centered around the value stack. This is where the actual computation happens, parameters are passed to functions and data is shared with interrupts. There are instructions that put values on top of the stack like `CPush`, `LPush`, and `GPush`. Some just take a value off and store it somewhere like `LMove`, `GMove`. Almost all other instructions will take a given amount of values from it and leave a return value in place.

For example, the term `1 + (2 * 3)` will be compiled to this sequence:

``` bytecode
 instruction    | stack
----------------------------
 CPush          | [1]
 CPush          | [1, 2]
 CPush          | [1, 2, 3]
 Mul            | [1, 6]
 Add            | [7]
```

You do not need to micromanage the bytecode itself. There are common language constructs with which you can built pretty much everything. These constructs are composed on a function level as `HIR` so every new function gets its own **h**igh-level **i**ntermediate **r**epresentation. Below you can see the transformation process of a function into a runnable `CodeObject` where each arrow means "lowering" to the next level.

```
HIR -> LIR -> CodeObject
```

`LIR` or **l**ow-level **i**ntermediate **r**epresentation is not directly exposed to the user but it is quite crucial if you want to understand how the bytecode generator works under the hood.

`CodeObject`'s on their own are already valid programs, but - as usual in every language - functions can be bundled together and exposed to the virtual machine in a collection called `Module`.

```
HIR -> LIR 
            \
HIR -> LIR    -> CodeObject -> Module
            /
HIR -> LIR 
```
