use crate::vector::{Vec3, Point};

#[derive(Debug, Copy, Clone)]
struct Ray {
    origin: Point,
    dir: Vec3
}

impl Ray {
    fn new(origin: Point, dir: Vec3) -> Self { 
        Self { origin: origin, dir: dir.norm() } 
    }

    fn at(&self, t: f64) -> Point {
        self.origin + (self.dir * t)
    }
}
