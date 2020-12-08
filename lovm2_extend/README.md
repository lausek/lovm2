# lovm2_extend

libraries (shared objects on linux) created with this can be imported by lovm2 and used like regular modules. You just need to add the shared object to lovm2s module search path e.g. `~/.local/lib/lovm2/`. When searching a module, the file extension is stripped. This means that a file named `libmymodule.so` will only be imported if you have a `Load("libmymodule")` instruction.

See [so_module](./examples/so_module) and [primitives](./examples/primitives) for showcase modules.
