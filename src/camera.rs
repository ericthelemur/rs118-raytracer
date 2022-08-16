use crate::ray::Ray;
use crate::vector::{Vec3, Point};
use crate::v;

#[derive(Debug)]
pub struct Camera {
    pub vw: f64,
    pub vh: f64,
    pub pxw: u32,
    pub pxh: u32,
    pub f: f64,
}

impl Camera {
    pub fn new(px_w: u32, aspect_ratio: f64) -> Self { 
        Self { pxw: px_w, pxh: ((px_w as f64) / aspect_ratio) as u32, vw: 2.0 * aspect_ratio, vh: 2.0, f: 1.0 }
    }

    pub fn tl(&self) -> Vec3 {
        v!(-self.vw/2.0, self.vh/2.0, -self.f)
    }
    
    pub fn br(&self) -> Vec3 {
        v!(self.vw/2.0, -self.vh/2.0, -self.f)
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let vxy = v!(x, y, 0).rescale(v!(), v!(self.pxw, self.pxh, 0), self.tl(), self.br());
        Ray::towards(v!(), vxy)
    }
}
