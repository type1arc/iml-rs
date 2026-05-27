use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy> Mul for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl<T: fmt::Display> Complex<T> {
    pub fn log(&self) {
        println!("({}, {}i)", self.re, self.im);
    }
}

impl<T: Default> Default for Complex<T> {
    fn default() -> Self {
        Complex {
            re: T::default(),
            im: T::default(),
        }
    }
}
