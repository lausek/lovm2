use lovm2_extend::prelude::*;

#[lovm2_module]
mod shared {
    fn gofunky(ctx: &mut Context) -> Option<Lovm2CError> {
        (0..100).for_each(|_| println!("yeah"));
        ctx.push_value(Value::Nil);
        None
    }
}
