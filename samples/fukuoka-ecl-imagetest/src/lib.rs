use std::error::Error;

use image::ImageReader;

pub fn grayscale(path: &'static str, dest_path: &'static str) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let img = img.to_rgb8();
    let size_x = img.width();
    let size_y = img.height();

    let mut dest = image::GrayImage::new(size_x, size_y);
    for y in 0..size_y {
        for x in 0..size_x {
            let pixel = img.get_pixel(x, y);
            // simple avagare
            let val = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
            let val = [val as u8; 1];
            dest.put_pixel(x, y, image::Luma(val));
        }
    }

    dest.save(dest_path)?;
    Ok(())
}

/// This uses BT.601
pub fn grayscale2(path: &'static str, dest_path: &'static str) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let img = img.to_rgb8();
    let size_x = img.width();
    let size_y = img.height();

    let mut dest = image::GrayImage::new(size_x, size_y);
    for y in 0..size_y {
        for x in 0..size_x {
            let pixel = img.get_pixel(x, y);
            // simple avagare
            let val = pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114;
            let val = [val as u8; 1];
            dest.put_pixel(x, y, image::Luma(val));
        }
    }

    dest.save(dest_path)?;
    Ok(())
}
