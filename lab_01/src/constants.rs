pub const EULER_SIZE: f64 = 0.03;
pub const EULER_STEPS: usize = 50;

// TASK 1
pub const TASK1_ANALYTIC: fn(f64) -> f64 = |x: f64| 3. * x.exp() - x.powi(2) - 2. * x - 2.;
pub const TASK1_EULER: fn(f64, f64) -> f64 = |x: f64, u: f64| 1.0 / (x + u.powi(2));
pub const TASK1_PICARD1: fn(f64) -> f64 = |u: f64| 1. + u + u.powi(3) / 3.;
pub const TASK1_PICARD2: fn(f64) -> f64 =
    |u: f64| TASK1_PICARD1(u) + u.powi(2) / 2. + u.powi(4) / 12.;
pub const TASK1_PICARD3: fn(f64) -> f64 =
    |u: f64| TASK1_PICARD2(u) + u.powi(3) / 6. + u.powi(5) / 60.;
pub const TASK1_PICARD4: fn(f64) -> f64 =
    |u: f64| TASK1_PICARD3(u) + u.powi(4) / 24. + u.powi(6) / 360.;
pub const TASK1_X0: f64 = 1.;
pub const TASK1_U0: f64 = 0.;

// TASK 2
pub const TASK2_ANALYTIC: fn(f64) -> f64 = |x: f64| (x.powi(2) + 1.) / 2.;
pub const TASK2_EULER: fn(f64, f64) -> f64 = |x: f64, u: f64| 1. / (u.powi(3) + 2. * x * u);
pub const TASK2_PICARD1: fn(f64) -> f64 = |u: f64| 0.5 + u.powi(2) / 2. + u.powi(4) / 4.0;
pub const TASK2_PICARD2: fn(f64) -> f64 =
    |u: f64| TASK2_PICARD1(u) + u.powi(4) / 4. + u.powi(6) / 12.;
pub const TASK2_PICARD3: fn(f64) -> f64 =
    |u: f64| TASK2_PICARD2(u) + u.powi(6) / 12. + u.powi(8) / 48.;
pub const TASK2_PICARD4: fn(f64) -> f64 =
    |u: f64| TASK2_PICARD3(u) + u.powi(8) / 48. + u.powi(6) / 240.;
pub const TASK2_X0: f64 = 0.5;
pub const TASK2_U0: f64 = 0.;

// TASK 3
pub const TASK3_EULER: fn(f64, f64) -> f64 = |x: f64, u: f64| x.powi(2) + u.powi(2);
pub const TASK3_PICARD1: fn(f64) -> f64 = |u: f64| u.powi(3) / 3.;
pub const TASK3_PICARD2: fn(f64) -> f64 = |u: f64| TASK3_PICARD1(u) + u.powi(7) / 63.;
pub const TASK3_PICARD3: fn(f64) -> f64 =
    |u: f64| TASK3_PICARD2(u) + 2. * u.powi(11) / 2079. + u.powi(15) / 59535.;
pub const TASK3_PICARD4: fn(f64) -> f64 = |u: f64| {
    TASK3_PICARD3(u)
        + (2. / 93_555.) * u.powi(15)
        + (2. / 3_393_495.) * u.powi(19)
        + (2. / 2_488_563.) * u.powi(19)
        + (2. / 86_266_215.) * u.powi(23)
        + (1. / 99_411_543.) * u.powi(23)
        + (2. / 3_341_878_155.) * u.powi(27)
        + (1. / 109_876_902_975.) * u.powi(31)
};

pub const TASK3_X0: f64 = 0.;
pub const TASK3_U0: f64 = 0.;
