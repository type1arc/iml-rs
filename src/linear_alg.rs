use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

// --- Generic N-dimensional Vector (existing) ---

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize> {
    pub mem: [T; N],
}

impl<T: Copy, const N: usize> Vector<T, N> {
    pub const fn new(mem: [T; N]) -> Self {
        Self { mem }
    }
}

impl<'a, 'b, T: Add<Output = T> + Copy + Default, const N: usize> Add<&'b Vector<T, N>>
    for &'a Vector<T, N>
{
    type Output = Vector<T, N>;

    fn add(self, other: &'b Vector<T, N>) -> Vector<T, N> {
        let mut res = [T::default(); N];
        for i in 0..N {
            res[i] = self.mem[i] + other.mem[i];
        }
        Vector::new(res)
    }
}

pub trait Dot<RHS = Self> {
    type Output;
    fn dot(self, rhs: RHS) -> Self::Output;
}

impl<'a, 'b, T: Add<Output = T> + Copy + Default + Mul<Output = T>, const N: usize>
    Dot<&'b Vector<T, N>> for &'a Vector<T, N>
{
    type Output = T;

    fn dot(self, rhs: &'b Vector<T, N>) -> T {
        let mut res = T::default();
        for i in 0..N {
            res = res + self.mem[i] * rhs.mem[i];
        }
        res
    }
}

pub trait Negate {
    type Output;
    fn negate(self) -> Self::Output;
}

impl<T: Neg<Output = T> + Copy + Default, const N: usize> Negate for Vector<T, N> {
    type Output = Vector<T, N>;

    fn negate(self) -> Vector<T, N> {
        let mut res = [T::default(); N];
        for i in 0..N {
            res[i] = -self.mem[i];
        }
        Vector::new(res)
    }
}

pub trait NegateInPlace {
    fn negate_assign(&mut self);
}

impl<T: Neg<Output = T> + Copy, const N: usize> NegateInPlace for Vector<T, N> {
    fn negate_assign(&mut self) {
        for i in 0..N {
            self.mem[i] = -self.mem[i];
        }
    }
}

pub trait Cross<RHS = Self> {
    type Output;
    fn cross_product(self, other: &Self) -> Self::Output;
}

impl<T: Copy + Default + Mul<Output = T> + Sub<Output = T> + Add<Output = T>> Cross
    for Vector<T, 3>
{
    type Output = Vector<T, 3>;

    fn cross_product(self, other: &Self) -> Vector<T, 3> {
        let mut res = [T::default(); 3];
        res[0] = self.mem[1] * other.mem[2] - self.mem[2] * other.mem[1];
        res[1] = self.mem[2] * other.mem[0] - self.mem[0] * other.mem[2];
        res[2] = self.mem[0] * other.mem[1] - self.mem[1] * other.mem[0];
        Vector::new(res)
    }
}

// --- Vec3 (3D vector matching C++ interface) ---

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T: fmt::Display> Vec3<T> {
    pub fn logtty(&self) {
        println!("({}, {}, {})", self.x, self.y, self.z);
    }
}

impl<T: Add<Output = T> + Copy> Vec3<T> {
    pub fn mag(&self) -> T {
        self.x + self.y + self.z
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// --- Vec2 (2D vector matching C++ interface) ---

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T: fmt::Display> Vec2<T> {
    pub fn logtty(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

impl<T: Add<Output = T> + Copy> Vec2<T> {
    pub fn mag(&self) -> T {
        self.x + self.y
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// --- Mat4 ---

#[derive(Debug, Clone, Copy)]
pub struct Mat4<T> {
    pub matrix: [[T; 4]; 4],
}

impl<T: Default + Copy> Mat4<T> {
    pub fn new() -> Self {
        Mat4 {
            matrix: [[T::default(); 4]; 4],
        }
    }
}

impl Mat4<f32> {
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / crate::trig::tan(fov as f64 * 0.5) as f32;
        let mut m = Mat4::new();
        m.matrix[0][0] = f / aspect;
        m.matrix[1][1] = f;
        m.matrix[2][2] = (far + near) / (near - far);
        m.matrix[2][3] = (2.0 * far * near) / (near - far);
        m.matrix[3][2] = -1.0;
        m
    }
}

// --- Free functions for Vec3 ---

pub fn add_v3<T: Add<Output = T> + Copy>(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
    Vec3::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
}

pub fn normalize_v3(v: &mut Vec3<f64>) -> Vec3<f64> {
    let mut out = Vec3::new(
        v.x / v.x.abs(),
        v.y / v.y.abs(),
        v.z / v.z.abs(),
    );
    if out.x.is_nan() { out.x = 0.0; }
    if out.y.is_nan() { out.y = 0.0; }
    if out.z.is_nan() { out.z = 0.0; }
    out
}

pub fn negate_v3<T: Neg<Output = T> + Copy>(v: &mut Vec3<T>) -> Vec3<T> {
    v.x = -v.x;
    v.y = -v.y;
    v.z = -v.z;
    *v
}

pub fn dot_product_v3<T: Add<Output = T> + Mul<Output = T> + Copy>(v1: &Vec3<T>, v2: &Vec3<T>) -> T {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross_product_v3<T: Mul<Output = T> + Sub<Output = T> + Copy>(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
    Vec3::new(
        v1.y * v2.z - v1.z * v2.y,
        v1.x * v2.z - v1.z * v2.x,
        v1.x * v2.y - v1.y * v2.x,
    )
}

// --- Free functions for Vec2 ---

pub fn add_v2<T: Add<Output = T> + Copy>(v1: &Vec2<T>, v2: &Vec2<T>) -> Vec2<T> {
    Vec2::new(v1.x + v2.x, v1.y + v2.y)
}

pub fn normalize_v2(v: &mut Vec2<f64>) -> Vec2<f64> {
    let mut out = Vec2::new(
        v.x / v.x.abs(),
        v.y / v.y.abs(),
    );
    if out.x.is_nan() { out.x = 0.0; }
    if out.y.is_nan() { out.y = 0.0; }
    out
}

pub fn negate_v2<T: Neg<Output = T> + Copy>(v: &mut Vec2<T>) -> Vec2<T> {
    v.x = -v.x;
    v.y = -v.y;
    *v
}

pub fn dot_product_v2(v1: &Vec2<f64>, v2: &Vec2<f64>) -> f64 {
    v1.mag() * v2.mag() * (v1.mag() / v2.mag()).atan().cos()
}

pub fn cross_product_v2(v1: &Vec2<f64>, v2: &Vec2<f64>) -> Vec2<f64> {
    let angle = (v1.mag() / v2.mag()).atan();
    let s = angle.sin();
    Vec2::new(v1.x * v2.x * s, v1.y * v2.y * s)
}
