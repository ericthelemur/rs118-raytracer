use image::{RgbImage, Rgb};

fn f2b(v: f32) -> u8 {
    return (v * 256.0) as u8;
}

fn main() {
    let (w, h) = (256, 256);
    let mut img = RgbImage::from_fn(w, h, |x, y| {
        let i = (x as f32) / (w as f32);
        let j = (y as f32) / (h as f32);
        Rgb([i, j, 0.25].map(f2b))
    });
    img.save("test.png").expect("Eror writing image");
}
