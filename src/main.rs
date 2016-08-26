extern crate image;

mod point_operations;

use std::path::Path;
use point_operations::{brighten, contrast};

fn main() {
    let img = image::open(&Path::new("popeye.jpg")).unwrap();

    let bright_buffer: image::RgbImage = brighten(&img, 100);
    let contrast_buffer: image::RgbImage = contrast(&img, 1.5);

    let _ = bright_buffer.save("brighter.jpg").unwrap();
    let _ = contrast_buffer.save("contrasty.jpg").unwrap();
}
