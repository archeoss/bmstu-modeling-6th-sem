use crate::constants::{EULER_SIZE, EULER_STEPS};

pub fn solve(f: fn(f64) -> f64, mut x0: f64, y0: f64) -> Vec<f64> {
    let mut result = vec![y0];
    for _ in 0..EULER_STEPS {
        let res = f(x0);
        x0 += EULER_SIZE;
        result.push(res);
    }

    result
}
