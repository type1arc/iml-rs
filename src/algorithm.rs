use crate::complex::Complex;
use crate::constants::constant;

#[allow(non_camel_case_types)]
pub type c_arr = Vec<Complex<f64>>;

pub fn fft(x: &mut c_arr) {
    let n = x.len();
    if n <= 1 {
        return;
    }

    let mut even: c_arr = x.iter().step_by(2).copied().collect();
    let mut odd: c_arr = x.iter().skip(1).step_by(2).copied().collect();

    fft(&mut even);
    fft(&mut odd);

    for k in 0..n / 2 {
        let angle = -2.0 * constant::PI * k as f64 / n as f64;
        let t = Complex::new(angle.cos(), angle.sin()) * odd[k];
        x[k] = even[k] + t;
        x[k + n / 2] = even[k] - t;
    }
}
