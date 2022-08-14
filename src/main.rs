use image::{RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::from_fn(256, 256, |x, y| {
        Rgb([255, 0, 0])
    });
    img.save("test.png").expect("Eror writing image");
}
