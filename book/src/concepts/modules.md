# Modules

While you are already familiar with the "lovm2 native" representation of executable code, `Modules` are far more abstract under the hood. `lovm2` is able to load specifically compiled shared objects at runtime and execute real native functions as well.

And that's not all. As long as your structure implements the `CallProtocol` trait you are free to even implement native functions inside your own compiler.

## Shared Libraries
