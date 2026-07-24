extern crate alloc;

use embedded_graphics::{Pixel, pixelcolor::Rgb888, prelude::{IntoStorage, Point, RgbColor}};
use minipng::{decode_png, decode_png_header};
use tinybmp::Bmp;

use alloc::vec;
use crate::{BootInfo, renderer::draw_pixel};


pub unsafe fn draw_png(info: &BootInfo, image_bytes: &[u8], x: usize, y: usize) {
    if info.fb_base.is_null() {
        return;
    }

    let header = decode_png_header(image_bytes).expect("Failed to read PNG header");
    let width = header.width() as usize;
    let height = header.height() as usize;
    
    let required_bytes = header.required_bytes_rgba8bpc();

    let mut buffer = vec![0u8; required_bytes];

    let mut image = decode_png(image_bytes, &mut buffer).expect("Failed to decode PNG");
    let _ = image.convert_to_rgba8bpc();

    let pixels = image.pixels();

    for row in 0..height {
        for col in 0..width {
            let idx = (row * width + col) * 4;
            
            let r = pixels[idx];
            let g = pixels[idx + 1];
            let b = pixels[idx + 2];
            let _a = pixels[idx + 3];

            let color = Rgb888::new(r, g, b);

            let pixel = Pixel(
                Point::new(col as i32, row as i32), 
                color
            );

            let Pixel(pos, colordat) = pixel;

            let color = ((colordat.r() as u32) << 16) 
                          | ((colordat.g() as u32) << 8) 
                          | (colordat.b() as u32);

            let screen_x = (x + pos.x as usize) as u32;
            let screen_y = (y + pos.y as usize) as u32;

            draw_pixel(info, screen_x, screen_y, color);
        }
    }
}

pub unsafe fn draw_image(info: &BootInfo, image: &[u8], x: usize, y: usize) {
    let bmp = Bmp::<Rgb888>::from_slice(image).expect("Failed to decode the png");
    let head = bmp.as_raw().header();
    let height = head.image_size.height as usize;

    for Pixel(pos, color) in bmp.pixels() {
        unsafe {
            let inverted_y = (height - 1) - pos.y as usize;
            draw_pixel(info, (x + pos.x as usize) as u32, (y + inverted_y) as u32, color.into_storage())
        }
    }
}
