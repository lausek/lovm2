# lovm2_core::extend

`lovm2_core::extend` bundles functionality for writing `lovm2` extensions using Rust. You can either statically import functions or produce a shared object that can be loaded at runtime.

Shared object libraries created using this crate can be imported by `lovm2` and used like regular modules. You just need to add the shared object to `lovm2`s module search path e.g. `~/.local/lib/lovm2/`. When searching for a module, the file extension is stripped. This means that a file named `libmymodule.so` will only be imported if you have a `Import("libmymodule")` instruction.

## Examples

- [shared-module](./examples/shared-module)
- [static-module](./examples/static-module)

## Shared Objects

### Setup

1. Create a new library crate `cargo init <name> --lib`

2. Change your crate-type inside `Cargo.toml`

``` toml
[lib]
crate-type = ["cdylib"]
```

3. Write your functions and use `cargo build --release` to produce
a shared object inside `target/release/`

### Usage

This crate exports three macros:

- `lovm2_module_init!()` - creates a module initializer. Required at end of file for all shared object modules.
- `#[lovm2_function]` - easy to use wrapper function in combination with `create_callable`.
- `#[lovm2_function(extern)]` - an attribute for exporting functions in shared object modules.
- `#[lovm2_object]` - a `struct` attribute for making data structures available to functions.

``` rust
// Import all required types for writing a module
use lovm2_core::extend::prelude::*;

#[lovm2_function(extern)]
fn div(a: i64, b: i64) -> Lovm2Result<i64> {
    if b == 0 {
        return err_from_string("div by zero");
    }
    Ok(a / b)
}

// This attribute generates wrapper code for Rust structures
#[lovm2_object]
pub struct Session {
    name: Option<String>,
}

// Constructor for new values of `Session`
#[lovm2_function(extern)]
fn new() -> Session {
    Session { name: None }
}

// Returning `Option`s is allowed
#[lovm2_function(extern)]
fn get_name(session: &Session) -> Option<String> {
    session.name.clone()
}

// You can modify `Session`
#[lovm2_function(extern)]
fn set_name(session: &mut Session, name: String) {
    session.name = Some(name);
}

// Generate module bloat (required)
lovm2_module_init!();
```

## Supported Types 

- Functions can take the generic `Value` type as argument. `Value` is also allowed in return position. If `&Value` or `&mut Value` is used, `lovm2` references are **automatically dereferenced** so you never have to worry about them in your functions body. You probably want this behavior in most cases.
- `bool`, `i64`, `f64`, `String` support conversion from `lovm2` values and can be used for arguments and as return type.
- Functions are allowed to return nothing aka. `()`.
- Wrapping the types above in `Option<_>` or `Lovm2Result<_>` also produces an accepted return type.
- A function is allowed to have at most one argument taking a mutable or immutable reference to the virtual machine itself e.g. `vm: &mut Vm`.
