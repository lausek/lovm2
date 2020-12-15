use lovm2_extend::prelude::*;

#[lovm2_module]
mod shared {
    // lmove 0
    // lmove 1
    // ------- start this function
    // lpush 0
    // lpush 1
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

    fn use_context(context: &mut Vm) -> i64 {
        assert!(context.ctx.frame_mut().is_ok());
        2
    }
}
