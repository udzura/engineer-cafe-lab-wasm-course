// use core::slice;

use core::slice::from_raw_parts_mut;
use core::slice::from_raw_parts;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hello_bg() -> String {
    "Hello, world".to_string()
}

#[no_mangle]
pub unsafe fn hello2() -> *const u8 {
    "Hello, world".as_ptr()
}

#[no_mangle]
pub unsafe fn hello3(buf: *mut u8, buflen: i32) {
    let src: &[u8] = "Hello, world".as_bytes();
    let buf: &mut [u8] = from_raw_parts_mut(buf, buflen as usize);
    buf.copy_from_slice(src);
}

#[wasm_bindgen]
pub fn welcome_bg(src: &str) -> i32 {
    src.as_bytes()[0] as i32
}

#[no_mangle]
pub unsafe fn welcome2(src: *mut u8, srclen: i32) -> i32 {
    let src: &[u8] = from_raw_parts(src, srclen as usize);
    src[0] as i32
}

#[no_mangle]
pub unsafe fn wordcount(src: *mut u8, srclen: i32) -> i32 {
//     return 100;
    let src: &[u8] = from_raw_parts(src, srclen as usize);
    if srclen == 0 {
        return 0; // empty
    }

    let mut count = 1;
    for c in src.iter() {
        let c = *c;
        if c == 0x20 { // space
            count += 1;
        }
    }
    count
}

// #[no_mangle]
// pub unsafe fn reverse(src: &[u8], dst: *mut u8, len: usize) -> i32 {
//     if src.len() != len {
//         // error
//         return -1;
//     }
//     let dst: &mut [u8] = slice::from_raw_parts_mut(dst, len);
//     for i in 0..src.len() {
//         let j = src.len() - i - 1;
//         dst[j] = src[i];
//     }
//     src.len() as i32
// }