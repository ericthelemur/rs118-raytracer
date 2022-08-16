mod vector;
mod ray;
mod object;
mod camera;
mod material;

use std::f64::consts::PI;

use camera::Camera;
use image::{RgbImage};
use indicatif::ParallelProgressIterator;
use lerp::Lerp;
use material::{Lambertian, Metal, Dielectric};
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use object::{Object, Sphere, Hit, Scene};
use vector::{Colour, Vec3};

fn colour(ray: &Ray, scene: &Scene, depth: u32) -> Colour {
    if depth <= 0 {
        return v!(0)
    }
    if let Some(h) = scene.hit(ray, (0.00001, f64::INFINITY)) {
        if let Some(refl) = h.refl {
            return refl.colour * colour(&refl.ray, scene, depth - 1);
        }
    }
    v!(1).lerp(v!(0.5, 0.7, 1.0), (ray.dir.norm().y + 1.0) / 2.0)
}

fn main() {
    let samples = 50;
    let max_depth = 20;
    let c = Camera::new(400, 90., 16. / 9.);

    let R = (PI/4.).cos();
    let scene: Scene = vec![
        Box::new(Sphere::new(
            v!(-R, 0, -1),
            R,
            Lambertian::new(v!(0, 0, 1)),
        )),
        Box::new(Sphere::new(
            v!(R, 0, -1),
            R,
            Lambertian::new(v!(1, 0, 0)),
        )),
    ];
    let bar = indicatif::ProgressBar::new((c.pxw * c.pxh) as u64);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
            )
            .progress_chars("#>-")
            .on_finish(indicatif::ProgressFinish::WithMessage("-- Done!".into()))
    );
    bar.set_draw_rate(5);

    let mut img = RgbImage::new(c.pxw, c.pxh);
    img.enumerate_pixels_mut()
        .par_bridge()
        .progress_with(bar)
        .for_each(|(x, y, p)| {
            let mut rng = rand::thread_rng();
            let colour = (0..samples).map(|_| {
                    let (rx, ry): (f64, f64) = (rng.gen(), rng.gen());
                    let (sx, sy) = (x as f64 - 0.5 + rx, y as f64 - 0.5 + ry);
                    let ray = c.get_ray(sx, sy);
                    colour(&ray, &scene, max_depth)
                }).fold(v!(), |acc, x| acc + x) / (samples as f64);
            *p = colour.into()
        });

    img.save("test.png").expect("Eror writing image");
}
