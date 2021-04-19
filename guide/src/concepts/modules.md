# Modules

While you are already familiar with `lovm2`'s own representation of executable code, `Modules` are far more abstract under the hood. `lovm2` is able to load specifically compiled shared objects - or DLLs as you would call them in the Windows world - at runtime and execute real native functions as well.

And that's not all. As long as your structure complies with the `CallProtocol` trait you are free to even implement native functions inside your own compiler or runtime. This job can be done using the [lovm2_extend](..) package which allows you to write your own modules in Rust.
