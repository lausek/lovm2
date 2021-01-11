use lovm2_extend::prelude::*;

// lmove 0
// lmove 1
// ------- start this function
// lpush 0
// lpush 1
// add
// ------- end this function
// ret
#[lovm2_function(extern)]
fn native_add(a: i64, b: i64) -> i64 {
    a + b
}

#[lovm2_function(extern)]
fn negate(b: bool) -> bool {
    !b
}

#[lovm2_function(extern)]
fn to_string(n: f64, ext: String) -> String {
    format!("{}.{}", n, ext)
}

// this algorithm calculates the amount of ends
// on a sausage with variable length and mass
#[lovm2_function(extern)]
fn enden_der_wurst() -> i64 {
    1 + 1
}

#[lovm2_function(extern)]
fn assert_this(b: bool) {
    assert!(b);
}

#[lovm2_function(extern)]
fn use_context(context: &mut Vm) -> i64 {
    assert!(context.context_mut().frame_mut().is_ok());
    2
}

/// Use this to store a name
#[lovm2_object]
pub struct Session {
    name: Option<String>,
}

#[lovm2_function(extern)]
fn new() -> Session {
    Session { name: None }
}

#[lovm2_function(extern)]
fn get_name(session: &Session) -> Option<String> {
    session.name.clone()
}

#[lovm2_function(extern)]
fn set_name(session: &mut Session, name: String) {
    session.name = Some(name);
}

lovm2_module_init!();
