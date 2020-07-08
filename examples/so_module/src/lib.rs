use lovm2_extend::prelude::*;

#[lovm2_module]
mod shared {
    fn gofunky() -> Result<(), String> {
        Ok(())
    }
}
