use crate::vector::{Vec3, Point};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self { 
        Self { origin: origin, dir: dir/*.norm()*/ } 
    }

    pub fn towards(origin: Point, target: Point) -> Self {
        Self::new(origin, target - origin)
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + (self.dir * t)
    }
}
