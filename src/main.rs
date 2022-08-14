use image::{RgbImage, Rgb};

mod vector;


fn main() {
    let (w, h) = (256, 256);
    let mut img = RgbImage::from_fn(w, h, |x, y| {
        let i = (x as f64) / (w as f64);
        let j = (y as f64) / (h as f64);
        Rgb([i, j, 0.25].map(vector::f2b))
    });
    img.save("test.png").expect("Eror writing image");
}
