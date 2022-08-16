use std::ops;

use derive_more::{Constructor, Add, Sub, Mul, Div, Neg};
use image::Rgb;
use lerp::Lerp;


#[derive(Debug, Copy, Clone, Constructor, Add, Sub, Mul, Div, Neg, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Colour = Vec3;
pub type Point = Vec3;

#[macro_export] 
macro_rules! v {
    ($x:expr, $y: expr, $z: expr) => {
        Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
    ($x:expr) => {
        Vec3::new(f64::from($x), f64::from($x), f64::from($x))
    };
    
    () => {
        Vec3::new(0.0, 0.0, 0.0)
    };
}

// Converts [0.0, 1.0] to [0, 255]
pub fn f2b(v: f64) -> u8 {
    (v * 256.0) as u8
}

// Converts Vec3 to image::Rgb
impl From<Vec3> for Rgb<u8> {
    fn from(v: Vec3) -> Self {
        let arr: [f64; 3] = v.into();
        Self(arr.map(f2b))
    }
}

// Converts Vec3 to [f, f, f]
impl From<Vec3> for [f64; 3] {
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
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

// vec / vec element-wise
impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        Vec3{ x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}


impl Vec3 {
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dist(&self, other: Self) -> f64 {
        (other - *self).mag()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn map<F: FnMut(f64) -> f64>(self, mut f: F) -> Self {
        Self { x: f(self.x), y: f(self.y), z: f(self.z) }
    }

    pub fn norm(&self) -> Self {
        let mag = self.mag();
        self.map(|v| v / mag)
    }

    fn invlerp(&self, low: Self, high: Self) -> Self {
        let mut r = (*self - low) / (high - low);
        if high.x == low.x { r.x = 0.5 }
        if high.y == low.y { r.y = 0.5 }
        if high.z == low.z { r.z = 0.5 }
        r
    }

    pub fn rescale(&self, oldmin: Self, oldmax: Self, newmin: Self, newmax: Self) -> Self {
        let v = self.invlerp(oldmin, oldmax);
        newmin.lerp(newmax, v)
    }
}

impl Lerp<Vec3> for Vec3 {
    fn lerp(self, other: Self, t: Vec3) -> Self {
        Self {
            x: self.x.lerp(other.x, t.x),
            y: self.y.lerp(other.y, t.y),
            z: self.z.lerp(other.z, t.z),
        }
    }
}