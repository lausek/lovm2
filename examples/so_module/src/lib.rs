use lovm2_extend::prelude::*;

#[lovm2_module]
mod shared {
    fn gofunky(_ctx: &mut Context) -> Result<(), String> {
        (0..100).for_each(|_| println!("yeah"));
        Ok(())
    }
}
