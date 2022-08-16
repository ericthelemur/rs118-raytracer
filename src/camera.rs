use derive_more::Constructor;

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
}

impl Camera {
    pub fn new(fov: f64, aspect_ratio: f64) -> Self {
        let angle = fov.to_radians();
        let vh = (angle / 2.).tan() * 2.;

        dbg!(Self { 
            pos: v!(),
            axes: Axes::new(v!(vh * aspect_ratio, 0, 0), v!(0, vh, 0), v!(0, 0, 1)),
        })
    }

    pub fn tl(&self) -> Vec3 {
        self.pos - self.axes.x/2. + self.axes.y/2. - self.axes.z
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let vxy = self.tl() + x * self.axes.x - y * self.axes.y;
        Ray::towards(v!(), vxy)
    }
}
