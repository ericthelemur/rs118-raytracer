mod vector;
mod ray;
mod object;
mod camera;

use camera::Camera;
use image::{RgbImage};
use lerp::Lerp;
use rand::Rng;
use vector::{Vec3, Colour};
use ray::Ray;
use rayon::prelude::*;
use object::{Object, Sphere, Hit, Scene};

fn colour(ray: &Ray, scene: &Scene) -> Colour {
    if let Some(h) = scene.hit(ray, (0.0, f64::INFINITY)) {
        return h.n.rescale(v!(-1), v!(1), v!(0), v!(1));
    }
    v!(1).lerp(v!(0.5, 0.7, 1.0), (ray.dir.norm().y+1.0)/2.0)
}

fn main() {
    let samples = 100;
    let c = Camera::new(400, 16. / 9.);
    
    let scene: Scene = vec![
        Box::new(Sphere::new(v!(0, 0, -1.0), 0.5)),
        Box::new(Sphere::new(v!(0.2, 0, -0.6), 0.2)),
        Box::new(Sphere::new(v!(0, -100.5, -1), 100.0)),
    ];
    let bar = indicatif::ProgressBar::new((c.vw * c.vh * samples) as u64);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
            )
            .progress_chars("#>-")
            .on_finish(indicatif::ProgressFinish::WithMessage("-- Done!".into()))
    );

    let mut img = RgbImage::new(c.vw, c.vh);
    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, p)| {
        let mut rng = rand::thread_rng();
        let colour = (0..samples).map(|_| {
            let (rx, ry): (f64, f64) = (rng.gen(), rng.gen());
            let (sx, sy) = (x as f64 - 0.5 + rx, y as f64 - 0.5 + ry);
            let ray = c.get_ray(sx, sy);
            colour(&ray, &scene)
        }).fold(v!(), |acc, x| acc + x) / (samples as f64);
        *p = colour.into()
    });

    img.save("test.png").expect("Eror writing image");
}
