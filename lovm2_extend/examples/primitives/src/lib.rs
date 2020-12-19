use lovm2_extend::prelude::*;

// lmove 0
// lmove 1
// ------- start this function
// lpush 0
// lpush 1
// add
// ------- end this function
// ret
#[lovm2_function]
fn native_add(a: i64, b: i64) -> i64 {
    a + b
}

#[lovm2_function]
fn negate(b: bool) -> bool {
    !b
}

#[lovm2_function]
fn to_string(n: f64, ext: String) -> String {
    format!("{}.{}", n, ext)
}

// this algorithm calculates the amount of ends
// on a sausage with variable length and mass
#[lovm2_function]
fn enden_der_wurst() -> i64 {
    1 + 1
}

#[lovm2_function]
fn assert_this(b: bool) {
    assert!(b);
}

#[lovm2_function]
fn use_context(context: &mut Vm) -> i64 {
    assert!(context.context_mut().frame_mut().is_ok());
    2
}

lovm2_module_init!();
