use derive_more::Constructor;
use rand::{Rng, random};

use crate::ray::Ray;
use crate::vector::{Vec3, Point};
use crate::v;

#[derive(Debug, Constructor)]
pub struct Axes {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

#[derive(Debug)]
pub struct Camera {
    pub pos: Point,
    pub axes: Axes,
    pub u: Vec3,
    pub v: Vec3,
    pub lens_rad: f64,
}

impl Camera {
    pub fn new(fov: f64, aspect_ratio: f64, look_from: Point, look_at: Point, up: Vec3, aperture: f64, focus_dist: f64) -> Self {
        dbg!(look_from, look_at);
        let angle = fov.to_radians();
        let vh = (angle / 2.).tan() * 2.;
        let vw = vh * aspect_ratio;

        let w = (look_from - look_at).norm();
        let u = up.cross(w).norm();
        let v = w.cross(u);
        dbg!(u, v, w);

        dbg!(Self { 
            pos: look_from,
            axes: Axes::new(focus_dist * u * vw, focus_dist * v * vh, focus_dist * w),
            u, v, lens_rad: aperture / 2.
        })
    }

    fn circle_sample() -> Vec3 {
        let r = random::<f64>().sqrt();
        let theta = random::<f64>() * std::f64::consts::TAU;
        
        v!(r * theta.cos(), r * theta.sin(), 0)
    }

    pub fn tl(&self) -> Vec3 {
        self.pos - self.axes.x/2. + self.axes.y/2. - self.axes.z
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let r = Self::circle_sample() * self.lens_rad;
        let o = self.pos + self.u * r.x + self.v * r.y;
        let vxy = self.tl() + x * self.axes.x - y * self.axes.y;
        Ray::towards(o, vxy)
    }
}
