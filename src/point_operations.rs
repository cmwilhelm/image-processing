extern crate image;

use image::{GenericImage, DynamicImage, ImageBuffer, Rgb};

pub fn contrast(img: &DynamicImage, factor: f32) -> image::RgbImage {
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


pub fn brighten(img: &DynamicImage, steps: u8) -> image::RgbImage {
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

fn new_buffer(img: &DynamicImage) -> image::RgbImage {
    let (width, height) = img.dimensions();
    image::RgbImage::new(width, height)
}

pub fn invert(img: &DynamicImage) -> image::RgbImage {
    let mut buffer = new_buffer(&img);

    for pixel in img.pixels() {
        match pixel {
            (x, y, pixel) => buffer.put_pixel(x, y, Rgb {data: [
                invert_channel(*pixel.data.get(0).unwrap()),
                invert_channel(*pixel.data.get(1).unwrap()),
                invert_channel(*pixel.data.get(2).unwrap())
            ]})
        }
    }

    buffer
}

fn invert_channel(value: u8) -> u8 {
    (!0 as u8) - value
}
