use crate::constants::{SIZE, STEPS};

pub fn solve(f: fn(f64) -> f64, mut x0: f64, y0: f64) -> Vec<f64> {
    let mut result = vec![y0];
    for _ in 0..STEPS {
        let res = f(x0);
        x0 += SIZE;
        result.push(res);
    }

    result
}
