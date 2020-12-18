# Setup

1. Modify your `Cargo.toml`

    - Add the latest crates.io version
    
    ``` toml
    lovm2 = "0.4.6"
    ```
    
    - ... or - if you feel lucky - use the current master branch directly

    ``` toml
    lovm2 = { git = "https://github.com/lausek/lovm2" }
    ```

2. Run `cargo update` on your terminal

3. Import the useful `lovm2` components into scope using `use lovm2::prelude::*;`
