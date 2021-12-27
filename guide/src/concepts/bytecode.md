# Bytecode

`lovm2` is centered around the value stack. This data structure has a variety of tasks such as passing parameters to functions or acting as a computation buffer for operations. There are instructions that put values on top of the stack like `CPush`, `LPush`, and `GPush`. Some just take a value off and store it somewhere like `LMove`, `GMove`. Almost all other instructions will take a given amount of values from it and leave a return value in place. You can also implement your own bytecode instructions (see [Interrupt](../vm/interrupt.md) chapter for more).

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

You do not need to micromanage the bytecode itself. There are common language constructs with which you can built pretty much everything. These constructs are composed into a **h**igh-level **i**ntermediate **r**epresentation so every new function gets its own `LV2Function` structure. Below you can see the transformation process of a function into a runnable `LV2CodeObject` where each arrow means "lowering" to the next level.

```
LV2Function -> LIR -> LV2CodeObject
```

`LIR` or **l**ow-level **i**ntermediate **r**epresentation is not directly exposed to the user but it is quite crucial if you want to understand how the bytecode generator works under the hood.

`LV2CodeObject`'s on their own are already valid programs bundling together instructions, constants and variables. As usual in every language, functions can be grouped together and exposed to the virtual machine in a construct called `LV2Module`. Introducing this module level abstraction makes sense as we will later work with collections that are not backed by `lovm2` bytecode but native functions.

```
LV2Function -> LIR 
                    \
LV2Function -> LIR    -> LV2CodeObject -> LV2Module --> LV2Vm
                    /
LV2Function -> LIR 
```
