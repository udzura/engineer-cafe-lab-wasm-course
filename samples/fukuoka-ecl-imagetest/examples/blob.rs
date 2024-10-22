use std::error::Error;
use std::ffi::CStr;
use base64::{engine::general_purpose, Engine};
use fukuoka_ecl_imagetest::*;

fn main() -> Result<(), Box<dyn Error>> {
    // 画像をexamplesディレクトリにコピーして用意
    let src = include_bytes!("./source.png");
    let enc = general_purpose::STANDARD.encode(src);
    let enc = format!("data:image/png;base64,{}", enc);
    unsafe { // width, height は画像に合わせる
        let d = grayscale_blob(660, 495, enc.as_ptr(), enc.len() as i32);
        let b64 = CStr::from_ptr(d as *const i8).to_str().unwrap();
        println!("data:image/png;base64,{}", b64)
    }
    Ok(())
}
