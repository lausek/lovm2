use lovm2_extend::prelude::*;

#[lovm2_function]
fn gofunky() {
    (0..100).for_each(|_| println!("yeah"));
}

lovm2_module_init!();
