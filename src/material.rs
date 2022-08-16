use derive_more::Constructor;
use rand::Rng;

use crate::object::Hit;
use crate::ray::Ray;
use crate::v;
use crate::vector::{Colour, Vec3};


#[derive(Debug, Constructor)]
pub struct Reflection {
    refl: Ray,
    colour: Colour,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

#[derive(Debug, Constructor)]
pub struct Lambertian {
    colour: Colour
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
        let new_ray = Ray::new(hit.p, hit.n + refl);

        Some(Reflection::new(new_ray, self.colour))
    }
}