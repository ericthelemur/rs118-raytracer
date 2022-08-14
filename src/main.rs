mod vector;
mod ray;
mod object;

use image::{RgbImage};
use lerp::Lerp;
use vector::{Vec3, Colour};
use ray::Ray;

#[derive(Debug)]
pub struct Viewport {
    pub w: f64,
    pub h: f64,
    pub f: f64,
}

impl Viewport {
    fn tl(&self) -> Vec3 {
        v!(-self.w/2.0, self.h/2.0, -self.f)
    }
    
    fn br(&self) -> Vec3 {
        v!(self.w/2.0, -self.h/2.0, -self.f)
    }
}

fn colour(ray: &Ray) -> Colour {
    let sphere = object::Sphere::new(v!(0, 0, -1), 0.5);
    if sphere.hit(ray) {
        return v!(1.0, 0, 0);
    }
    v!(1).lerp(v!(0.5, 0.7, 1.0), (ray.dir.norm().y+1.0)/2.0)
}

fn main() {
    let aspect_ratio = 16.0/9.0;
    let w = 400;
    let h = (w as f64 / aspect_ratio) as u32;

    let vh = 2.0;
    let v = Viewport { w: vh * aspect_ratio, h: vh, f: 1.0 };
    let img = RgbImage::from_fn(w, h, |x, y| {
        let vxy = v!(x, y, 0).rescale(v!(), v!(w, h, 0), v.tl(), v.br());
        let ray = Ray::towards(v!(), vxy);
        let colour = colour(&ray);
        colour.into()
    });
    img.save("test.png").expect("Eror writing image");
}
