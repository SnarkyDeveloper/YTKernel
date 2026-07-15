// use heapless::Vec;
// use minipng::{decode_png, decode_png_header};
//
// use crate::BootInfo;
//
// const BUFFER_SIZE: usize = 246 * 185 * 4;
// static mut IMAGE_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
//
//
// #[derive(Debug, Clone, Copy)]
// pub struct Pixel {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
//     pub a: u8,
// }
//
// pub struct Pixels<const N: usize> {
//     pub pixels: Vec<Pixel, N>,
// }
//
// impl<const N: usize> Pixels<N> {
//     pub fn new(raw_bytes: &[u8]) -> Self {
//         let mut pixels = Vec::new();
//
//         for chunk in raw_bytes.chunks_exact(4) {
//             let pixel = Pixel {
//                 r: chunk[0],
//                 g: chunk[1],
//                 b: chunk[2],
//                 a: chunk[3],
//             };
//             
//             if pixels.push(pixel).is_err() {
//                 break;
//             }
//         }
//
//         Pixels { pixels }
//     }
//     
//     pub fn get_pixel(&self, x: usize, y: usize, width: usize) -> Option<&Pixel> {
//         let index = y * width + x;
//         self.pixels.get(index)
//     }
// }
//
//
// pub unsafe fn draw_image(info: &BootInfo, image: &[u8], x: usize, y: usize) {
//     let header = decode_png_header(image).expect("bad png");
//     if header.required_bytes_rgba8bpc() > BUFFER_SIZE {
//         panic!("image too large");
//     }
//
//     let mut image = decode_png(image, &mut IMAGE_BUFFER).expect("couldnt decode :c");
//     
//     image.convert_to_rgba8bpc();
//
//     let pixels = Pixels::<BUFFER_SIZE>::new(image.pixels());
//
//     if info.fb_base.is_null() {
//         return;
//     }
//
//     // wait for PMM/VMM
// }
