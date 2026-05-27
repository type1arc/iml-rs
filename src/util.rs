#[macro_export]
macro_rules! log {
    ($x:expr) => {
        println!("{}", $x)
    };
}

#[macro_export]
macro_rules! errlog {
    ($x:expr) => {
        eprintln!("{}", $x)
    };
}

pub fn sqroot(x: f64) -> f64 {
    if x < 0.0 {
        eprintln!("complex-plane: sqroot arg < 0");
        return f64::NAN;
    }

    let mut guess = x / 2.0;
    let epsilon = 0.00000001;

    while (guess * guess - x).abs() > epsilon {
        guess = (guess + x / guess) / 2.0;
    }

    guess
}

pub fn abs(x: f64) -> f64 {
    x.abs()
}
