use crate::constants::{EULER_SIZE, EULER_STEPS};

pub fn solve(f: fn(f64, f64) -> f64, x0: f64, y0: f64) -> Vec<f64> {
    let mut result = vec![y0];
    let mut x = x0;
    let mut y = y0;
    for _i in 0..EULER_STEPS {
        y += EULER_SIZE * f(x, y);
        x += EULER_SIZE;
        result.push(y);
    }

    result
}
