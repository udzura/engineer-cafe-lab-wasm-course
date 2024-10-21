use std::{default::Default, error::Error, ffi::CStr, fs::File, io::Write, slice::from_raw_parts};

use base64::{engine::general_purpose, Engine};
use fukuoka_ecl_imagetest::*;

fn main() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("./original.png");
    let enc = general_purpose::STANDARD.encode(src);
    let enc = format!("data:image/png;base64,{}", enc);
    // dbg!(&enc);
    unsafe {
        let d = grayscale_blob(512, 512, enc.as_ptr(), enc.len() as i32);
        // let d = pixelate(512, 512, enc.as_ptr(), enc.len() as i32);
        let b64 = CStr::from_ptr(d as *const i8).to_str().unwrap();
        println!("data:image/png;base64,{}", b64)
    }
    Ok(())
}
