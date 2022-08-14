mod vector;
mod ray;

use image::{RgbImage};
use vector::{Vec3, Colour};
use ray::Ray;

struct Viewport {
    w: f64,
    h: f64,
    f: f64,
}

impl Viewport {
    fn tl(&self) -> Vec3 {
        v!(-self.w/2.0, -self.h/2.0, -self.f)
    }
}

fn colour(ray: &Ray) -> Colour {
    v!(0, 1.0, 0)
}

fn main() {
    let aspect_ratio = 16.0/9.0;
    let w = 400;
    let h = (w as f64 / aspect_ratio) as u32;

    let vh = 2.0;
    let v = Viewport { w: vh * aspect_ratio, h: vh, f: 1.0 };

    let img = RgbImage::from_fn(w, h, |x, y| {
        let (i, j) = ((x as f64) / (w as f64), (y as f64) / (h as f64));
        let vxy = v.tl() + v!(i * v.w, j * v.h, 0);
        let ray = Ray::between(v!(), vxy);
        let colour = colour(&ray);
        colour.into()
    });
    img.save("test.png").expect("Eror writing image");
}
