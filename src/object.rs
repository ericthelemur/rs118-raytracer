use derive_more::Constructor;

use crate::vector::Point;
use crate::ray::Ray;


#[derive(Debug, Constructor)]
pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> bool {
        let a = ray.dir.dot(ray.dir);
        let diff = ray.origin - self.centre;
        let b = 2.0 * ray.dir.dot(diff);
        let c = diff.dot(diff) - self.radius * self.radius;

        let det = b*b - 4.0 * a * c;
        det >= 0.0
    }
}