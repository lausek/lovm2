use super::*;

#[lv2_function]
fn e() -> f64 {
    std::f64::consts::E
}

#[lv2_function]
fn pi() -> f64 {
    std::f64::consts::PI
}

#[lv2_function]
fn sin(val: f64) -> f64 {
    val.sin()
}

#[lv2_function]
fn cos(val: f64) -> f64 {
    val.cos()
}

#[lv2_function]
fn tan(val: f64) -> f64 {
    val.tan()
}

#[lv2_function]
fn asin(val: f64) -> f64 {
    val.asin()
}

#[lv2_function]
fn acos(val: f64) -> f64 {
    val.acos()
}

#[lv2_function]
fn atan(val: f64) -> f64 {
    val.atan()
}

#[lv2_function]
fn atan2(val: f64, other: f64) -> f64 {
    val.atan2(other)
}

#[lv2_function]
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

#[lv2_function]
fn ceil(val: f64) -> f64 {
    val.ceil()
}

#[lv2_function]
fn floor(val: f64) -> f64 {
    val.floor()
}

#[lv2_function]
fn round(val: f64) -> f64 {
    val.round()
}

#[lv2_function]
fn log(val: f64, base: f64) -> f64 {
    val.log(base)
}

#[lv2_function]
fn sqrt(val: f64) -> f64 {
    val.sqrt()
}
