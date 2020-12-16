use lovm2_extend::prelude::*;

#[lovm2_object]
pub struct Session {
    name: Option<String>,
}

#[lovm2_function]
fn new() -> Session {
    Session { name: None }
}

#[lovm2_function]
fn get_name(session: &Session) -> Option<String> {
    session.name.clone()
}

#[lovm2_function]
fn set_name(session: &mut Session, name: String) {
    session.name = Some(name);
}

lovm2_module_init!();
