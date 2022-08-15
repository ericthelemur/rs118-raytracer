use crate::ray::Ray;
use crate::vector::{Vec3, Point};
use crate::v;

#[derive(Debug)]
pub struct Camera {
    pub ww: f64,
    pub wh: f64,
    pub vw: u32,
    pub vh: u32,
    pub f: f64,
}

impl Camera {
    pub fn new(px_w: u32, aspect_ratio: f64) -> Self { 
        Self { vw: px_w, vh: ((px_w as f64) / aspect_ratio) as u32, ww: 2.0 * aspect_ratio, wh: 2.0, f: 1.0 }
    }

    pub fn tl(&self) -> Vec3 {
        v!(-self.ww/2.0, self.wh/2.0, -self.f)
    }
    
    pub fn br(&self) -> Vec3 {
        v!(self.ww/2.0, -self.wh/2.0, -self.f)
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let vxy = v!(x, y, 0).rescale(v!(), v!(self.vw, self.vh, 0), self.tl(), self.br());
        Ray::towards(v!(), vxy)
    }
}
