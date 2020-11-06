use lovm2_extend::prelude::*;

#[lovm2_module]
mod shared {
    fn gofunky() {
        (0..100).for_each(|_| println!("yeah"));
    }
}
