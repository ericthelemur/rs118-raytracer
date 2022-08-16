use derive_more::Constructor;
use rand::Rng;

use crate::object::Hit;
use crate::ray::Ray;
use crate::v;
use crate::vector::{Colour, Vec3};


#[derive(Debug, Constructor)]
pub struct Reflection {
    pub ray: Ray,
    pub colour: Colour,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

#[derive(Debug, Constructor)]
pub struct Lambertian {
    pub colour: Colour
}

impl Lambertian {
    fn generate_reflection() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let v = Vec3::new(rng.gen(), rng.gen(), rng.gen())
                .rescale(v!(0), v!(1), v!(-1), v!(1));
            if v.mag() <= 1.0 {
                return v
            }
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let refl = Self::generate_reflection();
        let new_ray = if refl.is_tiny() {
            Ray::new(hit.p, hit.n)
        } else {
            Ray::new(hit.p, hit.n + refl)
        };

        Some(Reflection::new(new_ray, self.colour))
    }
}

#[derive(Debug, Constructor)]
pub struct Metal {
    pub colour: Colour
}

impl Metal {
    fn reflect_ray(v: Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * v.dot(*n) * *n
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let refl = Self::reflect_ray(incident_ray.dir, &hit.n);
        
        if refl.dot(hit.n) > 0.0 {
            Some(Reflection::new(Ray::new(hit.p, refl), self.colour))
        } else {
            None
        }
    }
}

#[derive(Debug, Constructor)]
pub struct Dielectric {
    ratio: f64,
}

impl Dielectric {
    fn refract(inc: Vec3, n: Vec3, ratio: f64) -> Vec3 {
        let co = -inc.dot(n);
        let si = (1.0 - co*co).sqrt();
        if ratio * si > 1.0 { 
            return Metal::reflect_ray(inc, &n);
        }
        let perp = ratio * (inc + n * co);
        let para = -(1.0 - perp.dot(perp)).abs().sqrt() * n;
        perp + para
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let ratio = if hit.front { 1.0 / self.ratio } else { self.ratio };
        let refr = Dielectric::refract(incident_ray.dir, hit.n, ratio);
        Some(Reflection::new(Ray::new(hit.p, refr), v!(1)))
    }
}