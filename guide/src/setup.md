# Setup

1. Modify your `Cargo.toml`.

    - Add the latest crates.io version
    
    ``` toml
    lovm2 = "0.5.0"
    ```
    
    - ... or use the current master branch directly

    ``` toml
    lovm2 = { git = "https://github.com/lausek/lovm2" }
    ```

2. Run `cargo update`.

3. Import `lovm2` components into scope using `use lovm2::prelude::*;`.
