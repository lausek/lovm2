use lovm2_extend::prelude::*;

#[lovm2_function]
fn e() -> f64 {
    std::f64::consts::E
}

#[lovm2_function]
fn pi() -> f64 {
    std::f64::consts::PI
}

#[lovm2_function]
fn sin(val: f64) -> f64 {
    val.sin()
}

#[lovm2_function]
fn cos(val: f64) -> f64 {
    val.cos()
}

#[lovm2_function]
fn tan(val: f64) -> f64 {
    val.tan()
}

#[lovm2_function]
fn asin(val: f64) -> f64 {
    val.asin()
}

#[lovm2_function]
fn acos(val: f64) -> f64 {
    val.acos()
}

#[lovm2_function]
fn atan(val: f64) -> f64 {
    val.atan()
}

#[lovm2_function]
fn atan2(val: f64, other: f64) -> f64 {
    val.atan2(other)
}

#[lovm2_function]
fn clamp(val: f64, min: f64, max: f64) -> f64 {
    // TODO: this is unstable
    //val.clamp(min, max)
    if val < min {
        min
    } else if max < val {
        max
    } else {
        val
    }
}

#[lovm2_function]
fn ceil(val: f64) -> f64 {
    val.ceil()
}

#[lovm2_function]
fn floor(val: f64) -> f64 {
    val.floor()
}

#[lovm2_function]
fn round(val: f64) -> f64 {
    val.round()
}

#[lovm2_function]
fn log(val: f64, base: f64) -> f64 {
    val.log(base)
}

#[lovm2_function]
fn sqrt(val: f64) -> f64 {
    val.sqrt()
}

lovm2_module_init!(math);
