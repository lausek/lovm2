# Concepts

This chapter aims to give you a brief overview of the internal workings. Even though `lovm2` is designed to be as simple as possible, it is still quite important to grasp the implementation concepts behind it.

The general steps of coming to a runnable program are roughly:

- Create a new `ModuleBuilder` and populate it with functions aka. `HIR` data
- Call `module_builder.build()` returning a runnable `Module` from the current state
- Load the module into an instance of a virtual machine `Vm` using `add_main_module()`
- Start the program by calling `run` on the virtual machine
