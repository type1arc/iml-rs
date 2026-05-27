use crate::constants::constant;
use crate::types::int32;
use crate::util;

pub fn sin(x: f64) -> f64 {
    let c1: f64 = -1.66666666666666324348e-01;
    let c2: f64 = 8.33333333332248946124e-03;
    let c3: f64 = -1.98412698298579493134e-04;
    let c4: f64 = 2.75573137070700676789e-06;
    let c5: f64 = -2.50507602534068634195e-08;
    let c6: f64 = 1.58969099521155010221e-10;

    if x.is_nan() || x.is_infinite() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }

    let half_pi: f64 = constant::HALF_PI;
    let quad: f64 = x / half_pi;
    let quad_val: int32 = quad as int32;
    let reduced: f64 = x - quad_val as f64 * half_pi;

    let z: f64 = reduced * reduced;
    let v: f64 = z * reduced;
    let r: f64 = c2 + z * (c3 + z * (c4 + z * (c5 + z * c6)));
    let approx: f64 = reduced + v * (c1 + z * r);

    match quad_val & 3 {
        0 => approx,
        1 => util::sqroot(1.0 - approx * approx),
        2 => -approx,
        3 => -util::sqroot(1.0 - approx * approx),
        _ => approx,
    }
}

pub fn cos(x: f64) -> f64 {
    let c2: f64 = -5.00000000000000000000e-01;
    let c4: f64 = 4.16666666666666666647e-02;
    let c6: f64 = -1.38888888888887588854e-03;
    let c8: f64 = 2.48015872888505653021e-05;
    let c10: f64 = -2.75573192192997980483e-07;
    let c12: f64 = 2.08767557072843831027e-09;

    if x.is_nan() || x.is_infinite() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 1.0;
    }

    let half_pi: f64 = constant::HALF_PI;
    let quad: f64 = x / half_pi;
    let quad_val: int32 = quad as int32;
    let reduced: f64 = x - quad_val as f64 * half_pi;

    if util::abs(reduced) < 1e-19 {
        if (quad_val & 1) == 1 {
            return 0.0;
        }
        return if (quad_val & 2) == 0 { 1.0 } else { -1.0 };
    }

    let z: f64 = reduced * reduced;
    let r: f64 = c4 + z * (c6 + z * (c8 + z * (c10 + z * c12)));
    let approx: f64 = 1.0 + z * (c2 + z * r);

    if (quad_val & 1) == 0 {
        approx
    } else {
        -approx
    }
}

pub fn tan(x: f64) -> f64 {
    sin(x) / cos(x)
}
