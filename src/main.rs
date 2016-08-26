extern crate image;

use std::path::Path;

use image::{GenericImage, DynamicImage, ImageBuffer, Rgb};

fn main() {
    let img = image::open(&Path::new("popeye.jpg")).unwrap();

    let bright_buffer: image::RgbImage = brighten(&img, 100);
    let contrast_buffer: image::RgbImage = contrast(&img, 1.5);

    let _ = bright_buffer.save("brighter.jpg").unwrap();
    let _ = contrast_buffer.save("contrasty.jpg").unwrap();
}

fn contrast(img: &DynamicImage, factor: f32) -> image::RgbImage {
    let (width, height) = img.dimensions();
    let mut buffer: image::RgbImage = ImageBuffer::new(width, height);

    for pixel in img.pixels() {
        match pixel {
            (x, y, pixel) => buffer.put_pixel(x, y, Rgb{data: [
                apply_contrast(*pixel.data.get(0).unwrap(), factor),
                apply_contrast(*pixel.data.get(1).unwrap(), factor),
                apply_contrast(*pixel.data.get(2).unwrap(), factor)
            ]})
        }
    }

    buffer
}

fn apply_contrast(value: u8, factor: f32) -> u8 {
    let new_value = value as f32 * factor;
    if new_value > (!0 as u8) as f32 {
        !0 as u8
    } else {
        new_value as u8
    }
}


fn brighten(img: &DynamicImage, steps: u8) -> image::RgbImage {
    let (width, height) = img.dimensions();

    let mut buffer: image::RgbImage = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);

            buffer.put_pixel(x, y, Rgb {data: [
                brighten_channel(*pixel.data.get(0).unwrap(), steps),
                brighten_channel(*pixel.data.get(1).unwrap(), steps),
                brighten_channel(*pixel.data.get(2).unwrap(), steps)
            ]});
        }
    }

    buffer
}

fn brighten_channel(value: u8, steps: u8) -> u8 {
    let new_val = value as u16 + steps as u16;
    if new_val > (!0 as u8) as u16 {
        !0 as u8
    } else {
        new_val as u8
    }
}
