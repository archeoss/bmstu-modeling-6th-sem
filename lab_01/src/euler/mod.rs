use crate::constants::{SIZE, STEPS};

pub fn solve(f: fn(f64, f64) -> f64, x0: f64, y0: f64) -> Vec<f64> {
    let mut result = vec![y0];
    let mut x = x0;
    let mut y = y0;
    for _i in 0..STEPS {
        y += SIZE * f(x, y);
        x += SIZE;
        result.push(y);
    }

    result
}
