extern crate image;

mod point_operations;

use std::path::Path;
use point_operations::{brighten, contrast, invert, threshold};

fn main() {
    let img = image::open(&Path::new("popeye.jpg")).unwrap();

    let bright_buffer: image::RgbImage = brighten(&img, 100);
    let contrast_buffer: image::RgbImage = contrast(&img, 1.5);
    let inverted_buffer: image::RgbImage = invert(&img);
    let threshold_buffer: image::RgbImage = threshold(&img, 200);

    let _ = bright_buffer.save("brighter.jpg").unwrap();
    let _ = contrast_buffer.save("contrasty.jpg").unwrap();
    let _ = inverted_buffer.save("inverted.jpg").unwrap();
    let _ = threshold_buffer.save("thresholded.jpg").unwrap();
}
