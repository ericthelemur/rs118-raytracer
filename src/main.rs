mod vector;
mod ray;
mod object;
mod camera;
mod material;

use std::f64::consts::PI;

use camera::Camera;
use derive_more::Constructor;
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

fn random_scene() -> Scene {
    let mut objects: Scene = vec![];

    let ground = Box::new(Sphere::new(
        v!(0, -1000, 0),
        1000.0,
        Lambertian::new(v!(0.5, 0.5, 0.5)),
    ));
    objects.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let material_choice: f64 = rand::random();
            let center = v!(
                a + 0.9 * rand::random::<f64>(),
                0.2,
                b + 0.9 * rand::random::<f64>()
            );

            if material_choice < 0.8 {
                //diffuse
                let material = Lambertian::new(v!(rand::random::<f64>()));
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else if material_choice < 0.95 {
                //metal
                let colour = v!(rand::random::<f64>() / 2.0 + 0.5);
                let fuzz = rand::random::<f64>() / 2.0;
                let material = Metal::new(colour);
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else {
                //glass
                objects.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        v!(0, 1, 0),
        1.0,
        Dielectric::new(1.5),
    )));
    objects.push(Box::new(Sphere::new(
        v!(-4, 1, 0),
        1.0,
        Lambertian::new(v!(0.4, 0.2, 0.1)),
    )));
    objects.push(Box::new(Sphere::new(
        v!(4, 1, 0),
        1.0,
        Metal::new(v!(0.7, 0.6, 0.5)),
    )));
    objects
}

fn main() {
    let samples = 100;
    let max_depth = 20;
    let asp_ratio = 1.5;
    let pxw = 400;
    let pxh = ((pxw as f64) / asp_ratio) as u32;
    let from = v!(13, 2, 3);
    let to = v!(0, 0, 0);
    let c = Camera::new(20., asp_ratio, from, to, v!(0, 1, 0), 0.1, 10.);

    let R = (PI/4.).cos();
    let scene: Scene = random_scene();
    let bar = indicatif::ProgressBar::new((pxw * pxh) as u64);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
            )
            .progress_chars("#>-")
            .on_finish(indicatif::ProgressFinish::WithMessage("-- Done!".into()))
    );
    bar.set_draw_rate(5);

    let mut img = RgbImage::new(pxw, pxh);
    img.enumerate_pixels_mut()
        .par_bridge()
        .progress_with(bar)
        .for_each(|(x, y, p)| {
            let mut rng = rand::thread_rng();
            let colour = (0..samples).map(|_| {
                    let (rx, ry): (f64, f64) = (rng.gen(), rng.gen());
                    let (sx, sy) = (x as f64 - 0.5 + rx, y as f64 - 0.5 + ry);
                    let (ux, uy) = (sx / pxw as f64, sy / pxh as f64);
                    let ray = c.get_ray(ux, uy);
                    colour(&ray, &scene, max_depth)
                }).fold(v!(), |acc, x| acc + x) / (samples as f64);
            *p = colour.into()
        });

    img.save("test.png").expect("Eror writing image");
}
