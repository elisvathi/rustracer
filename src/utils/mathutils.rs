// Math utils
use crate::Float;
pub fn min(a: Float, b: Float) -> Float {
    if a < b {
        return a;
    }
    b
}
pub fn max(a: Float, b: Float) -> Float {
    if a > b {
        return a;
    }
    b
}

pub fn map(x: Float, a1: Float, b1: Float, a2: Float, b2: Float) -> Float {
    let rap = (x - a1) / (b1 - a1);
    (b2 - a2) * rap + a2
}

pub fn clamp(a: Float, minimum: Float, maximum: Float) -> Float {
    if a < minimum {
        return minimum;
    } else if a > maximum {
        return maximum;
    }
    a
}

pub fn to_color(a: Float, start: Float, end: Float) -> i32 {
    clamp(map(a, start, end, 0.0, 255.0), 0.0, 255.0) as i32
}

pub fn sigmoid(f: Float) -> Float {
    use std::f64::consts::E;
    1.0 / (1.0 + E.powf(-f))
}
