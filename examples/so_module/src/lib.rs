use lovm2_internals::lovm2_module;

#[lovm2_module]
mod shared {
    fn gofunky() -> Result<(), String> {
        Ok(())
    }
}
