// extern "C" {
//     fn hoge(hoge: i32);
// }

#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    // hoge(left);
    left + right
}
