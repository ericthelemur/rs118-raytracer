use std::ops;

use derive_more::{Add, Sub, Mul, Div, Neg};
use image::Rgb;


#[derive(Debug, Copy, Clone, Add, Sub, Mul, Div, Neg, PartialEq, PartialOrd)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

pub type Colour = Vec3;
pub type Point = Vec3;

// Converts [0.0, 1.0] to [0, 255]
pub fn f2b(v: f64) -> u8 {
    (v * 256.0) as u8
}

// Converts Vec3 to image::Rgb
impl From<&Vec3> for Rgb<u8> {
    fn from(v: &Vec3) -> Self {
        Self(v.map(f2b))
    }
}

// f64 * vec
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// vec * vec element-wise
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3{ x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}


impl Vec3 {
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn cross(&self, other: Self) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn mapv<F: FnMut(f64) -> f64>(self, mut f: F) -> Vec3 {
        Vec3 { x: f(self.x), y: f(self.y), z: f(self.z) }
    }

    pub fn map<F: FnMut(f64) -> U, U>(self, mut f: F) -> [U; 3] {
        [f(self.x), f(self.y), f(self.z)]
    }
}