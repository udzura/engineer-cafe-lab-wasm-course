extern crate image;

use core::str;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use std::error::Error;
use base64::{engine::general_purpose, Engine};

use image::{codecs::png, load_from_memory_with_format, ImageBuffer, ImageReader};

// #[cfg(target_arch="wasm32")]
// extern "C" {
//     fn dbg_error_code(value: u32);
// }
// #[cfg(not(target_arch="wasm32"))]
// fn dbg_error_code(_value: u32) {
//     // nop
// }

#[no_mangle]
pub unsafe fn grayscale_blob(
    width: u32, height: u32, src: *const u8, slen: i32,
) -> *const u8 {
    let src = from_raw_parts(src, slen as usize);
    let mut tmp_buf: Vec<u8> = Vec::<u8>::new();
    tmp_buf.resize(slen as usize, 0);
    tmp_buf.copy_from_slice(src);

    let mut result_buf: Vec<u8> = Vec::<u8>::new();
    result_buf.resize(1<<22, 0);

    // data url: e.g.
    // data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAeAA
    // AAFACAYAAABkyK97AAAAAXNSR0IArs4c6QAAIABJREFUeF7svU...
    let url = str::from_utf8(&tmp_buf).unwrap();
    let collected = url.split(",").collect::<Vec<&str>>();
    let src = collected[1].as_bytes();
    
    let blob: Vec<u8> = general_purpose::STANDARD.decode(src).unwrap();

    let img: image::DynamicImage = load_from_memory_with_format(&blob, image::ImageFormat::Png).unwrap();
    let img: ImageBuffer<image::Rgb<u8>, _> = img.to_rgb8();
    let mut dest: ImageBuffer<image::Luma<u8>, Vec<u8>> = image::GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel: &image::Rgb<u8> = img.get_pixel(x, y);
            // simple avagare
            let val = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
            let val = [val as u8; 1];
            dest.put_pixel(x, y, image::Luma(val));
        }
    }

    #[cfg(not(target_arch="wasm32"))]
    {
        dest.save("/tmp/debug.png").unwrap();
    }

    let mut inter_buf = Vec::<u8>::new();
    let enc = png::PngEncoder::new(&mut inter_buf);
    dest.write_with_encoder(enc).unwrap_or_else(|_| panic!("encode failed") );
    general_purpose::STANDARD.encode_slice(&inter_buf, &mut result_buf).unwrap();
    result_buf.as_ptr()
}

// #[no_mangle]
// pub unsafe fn pixelate(width: u32, height: u32, src: *const u8, slen: i32) -> *const u8 {
//     let mut result_buf: Vec<u8> = Vec::<u8>::new();
//     result_buf.resize(1<<22, 0);

//     let src = from_raw_parts(src, slen as usize);
//     // data url: e.g.
//     // data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAeAA
//     // AAFACAYAAABkyK97AAAAAXNSR0IArs4c6QAAIABJREFUeF7svU...
//     let url = str::from_utf8(src).unwrap();
//     let collected = url.split(",").collect::<Vec<&str>>();
//     let src = collected[1].as_bytes();
    
//     let blob = general_purpose::STANDARD.decode(src).unwrap();

//     let img: image::DynamicImage = 
//         load_from_memory_with_format(&blob, image::ImageFormat::Png).unwrap();
//     let img: ImageBuffer<image::Rgb<u8>, _> = img.to_rgb8();
//     let mut dest: ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::RgbImage::new(width, height);

//     for y in (0..height).step_by(20) {
//         for x in (0..width).step_by(20) {
//             let pixel0= img.get_pixel(x, y);
//             let val: [u8; 3] = [
//                 pixel0[0],
//                 pixel0[1],
//                 pixel0[2],
//             ];
//             for i in 0..400 {
//                 let (ex, ey) = (x+i/20, y+i%20);
//                 if ex >= width || ey >= height {
//                     continue;
//                 }

//                 dest.put_pixel(ex, ey, image::Rgb(val));
//             }
//         }
//     }

//     #[cfg(not(target_arch="wasm32"))]
//     {
//         dest.save("/tmp/debug.png").unwrap();
//     }

//     let mut inter_buf = Vec::<u8>::new();
//     let enc = png::PngEncoder::new(&mut inter_buf);
//     dest.write_with_encoder(enc).unwrap();
//     general_purpose::STANDARD.encode_slice(&inter_buf, &mut result_buf).unwrap();
//     result_buf.as_ptr()
// }

// pub fn grayscale(path: &'static str, dest_path: &'static str) -> Result<(), Box<dyn Error>> {
//     let img = ImageReader::open(path)?.decode()?;
//     let img = img.to_rgb8();
//     let size_x = img.width();
//     let size_y = img.height();

//     let mut dest = image::GrayImage::new(size_x, size_y);
//     for y in 0..size_y {
//         for x in 0..size_x {
//             let pixel = img.get_pixel(x, y);
//             // simple avagare
//             let val = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
//             let val = [val as u8; 1];
//             dest.put_pixel(x, y, image::Luma(val));
//         }
//     }

//     dest.save(dest_path)?;
//     Ok(())
// }

// /// This uses BT.601
// pub fn grayscale2(path: &'static str, dest_path: &'static str) -> Result<(), Box<dyn Error>> {
//     let img = ImageReader::open(path)?.decode()?;
//     let img = img.to_rgb8();
//     let size_x = img.width();
//     let size_y = img.height();

//     let mut dest = image::GrayImage::new(size_x, size_y);
//     for y in 0..size_y {
//         for x in 0..size_x {
//             let pixel = img.get_pixel(x, y);
//             // This uses BT.601
//             let val = pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114;
//             let val = [val as u8; 1];
//             dest.put_pixel(x, y, image::Luma(val));
//         }
//     }

//     dest.save(dest_path)?;
//     Ok(())
// }
