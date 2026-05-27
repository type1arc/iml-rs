use std::ops::{Add, Mul, Sub};

use crate::util;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Quaternion<T> {
    pub fn new(w: T, x: T, y: T, z: T) -> Self {
        Quaternion { w, x, y, z }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy> Mul for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, other: Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}

impl Quaternion<f64> {
    pub fn normalize(&mut self) -> Quaternion<f64> {
        let mag_sq = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
        if mag_sq > 0.0 {
            let inv_mag = 1.0 / util::sqroot(mag_sq);
            self.w *= inv_mag;
            self.x *= inv_mag;
            self.y *= inv_mag;
            self.z *= inv_mag;
        }
        Quaternion::new(self.w, self.x, self.y, self.z)
    }

    pub fn inverse(&self) -> Quaternion<f64> {
        Quaternion::new(self.w, -self.x, -self.y, -self.z)
    }

    pub fn rotate(&self, vx: &mut f64, vy: &mut f64, vz: &mut f64) {
        let v = Quaternion::new(0.0, *vx, *vy, *vz);
        let result = *self * v * self.inverse();
        *vx = result.x;
        *vy = result.y;
        *vz = result.z;
    }
}
