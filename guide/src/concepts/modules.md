# Modules

While you are already familiar with the "lovm2 native" representation of executable code, `Modules` are far more abstract under the hood. `lovm2` is able to load specifically compiled shared objects - or dlls as you would call them in the Windows world - at runtime and execute real native functions as well.

And that's not all. As long as your structure implements the `CallProtocol` trait you are free to even implement native functions inside your own compiler or runtime. This job can be done using the [lovm2_extend](..) module which allows you to write your own modules in Rust.
