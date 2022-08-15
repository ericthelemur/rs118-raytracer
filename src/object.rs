use derive_more::Constructor;

use crate::ray::Ray;
use crate::vector::{Point, Vec3};

#[derive(Debug, Constructor)]
pub struct Hit {
    pub p: Point,
    pub n: Vec3,
    pub t: f64,
    pub front: bool,
}

pub trait Object {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit>;
}

pub type Scene = Vec<Box<dyn Object + Sync>>;

impl Object for Scene {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.iter()
            .map(|o| o.hit(ray, bounds))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}

#[derive(Debug, Constructor, Copy, Clone)]
pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        let a = ray.dir.dot(ray.dir);
        let diff = ray.origin - self.centre;
        let b = 2.0 * ray.dir.dot(diff);
        let c = diff.dot(diff) - self.radius * self.radius;

        let det = b * b - 4.0 * a * c;
        if det < 0.0 {
            return None;
        }

        let t1 = (-b - det.sqrt()) / (2.0 * a);
        let t2 = (-b + det.sqrt()) / (2.0 * a);
        let top = if bounds.0 <= t1 && t1 <= bounds.1 {
            Some(t1)
        } else if bounds.0 <= t2 && t2 <= bounds.1 {
            Some(t2)
        } else {
            None
        };
        if let Some(t) = top {
            let p = ray.at(t);
            let n = (p - self.centre) / self.radius;
            let front = ray.dir.dot(n) < 0.0;
            Some(Hit::new(p, if front { n } else { -n }, t, front))
        } else {
            None
        }
    }
}
