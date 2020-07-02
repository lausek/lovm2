use lovm2::context::Context;

pub type ExternFunction = unsafe extern fn(&mut Context) -> Result<(), String>;
