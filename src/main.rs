mod vector;
mod ray;
mod object;

use image::{RgbImage};
use lerp::Lerp;
use vector::{Vec3, Colour};
use ray::Ray;
use rayon::prelude::*;
use object::{Object, Sphere, Hit, Scene};

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

fn colour(ray: &Ray, scene: &Scene) -> Colour {
    if let Some(h) = scene.hit(ray, (0.0, f64::INFINITY)) {
        return h.n.rescale(v!(-1), v!(1), v!(0), v!(1));
    }
    v!(1).lerp(v!(0.5, 0.7, 1.0), (ray.dir.norm().y+1.0)/2.0)
}

fn main() {
    let aspect_ratio = 16.0/9.0;
    let w = 400;
    let h = (w as f64 / aspect_ratio) as u32;

    let vh = 2.0;
    let v = Viewport { w: vh * aspect_ratio, h: vh, f: 1.0 };

    let scene: Scene = vec![
        Box::new(Sphere::new(v!(0, 0, -1.0), 0.5)),
        Box::new(Sphere::new(v!(0.2, 0, -0.6), 0.2)),
    ];

    let mut img = RgbImage::new(w, h);
    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, p)| {
        let vxy = v!(x, y, 0).rescale(v!(), v!(w, h, 0), v.tl(), v.br());
        let ray = Ray::towards(v!(), vxy);
        let colour = colour(&ray, &scene);
        *p = colour.into()
    });

    img.save("test.png").expect("Eror writing image");
}
