use lovm2_extend::prelude::*;

#[lovm2_module]
mod shared {
    // popl 0
    // popl 1
    // ------- start this function
    // pushl 0
    // pushl 1
    // add
    // ------- end this function
    // ret
    fn native_add(a: i64, b: i64) -> i64 {
        a + b
    }

    fn negate(b: bool) -> bool {
        !b
    }

    fn to_string(n: f64, ext: String) -> String {
        format!("{}.{}", n, ext)
    }

    // this algorithm calculates the amount of ends
    // on a sausage with variable length and mass
    fn enden_der_wurst() -> i64 {
        1 + 1
    }

    fn assert_this(b: bool) {
        assert!(b);
    }

    fn use_context(context: &mut Context) -> i64 {
        assert!(context.frame_mut().is_ok());
        2
    }
}
