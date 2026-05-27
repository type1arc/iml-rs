pub mod opt {
    use std::fmt;
    use std::ops::{BitXor, Shr};

    #[derive(Debug, Clone, Copy)]
    pub struct Shift<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Shift<T> {
        pub fn new(x: T, y: T, z: T) -> Self {
            Shift { x, y, z }
        }
    }

    impl<T: Copy> Shr<Shift<T>> for Shift<T> {
        type Output = Shift<T>;

        fn shr(mut self, other: Shift<T>) -> Shift<T> {
            self.x = other.y;
            self.y = other.z;
            self.z = other.x;
            self
        }
    }

    impl<T: fmt::Display> Shift<T> {
        pub fn log(&self) {
            println!("({}, {}, {})", self.x, self.y, self.z);
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Expo<T> {
        pub x: T,
    }

    impl<T> Expo<T> {
        pub fn new(x: T) -> Self {
            Expo { x }
        }
    }

    impl BitXor<f64> for Expo<f64> {
        type Output = f64;

        fn bitxor(self, exponent: f64) -> f64 {
            self.x.powf(exponent)
        }
    }

    impl BitXor<i32> for Expo<f64> {
        type Output = f64;

        fn bitxor(self, exponent: i32) -> f64 {
            self.x.powi(exponent)
        }
    }
}
