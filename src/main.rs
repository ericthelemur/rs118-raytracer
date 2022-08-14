use image::{RgbImage, Rgb};
use vector::Vec3;

mod vector;


fn main() {
    let (w, h) = (256, 256);
    let mut img = RgbImage::from_fn(w, h, |x, y| {
        let i = (x as f64) / (w as f64);
        let j = (y as f64) / (h as f64);
        Vec3::new(i, j, 0.25).into()
    });
    img.save("test.png").expect("Eror writing image");
}
