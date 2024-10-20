// extern "C" {
//     fn hoge(hoge: i32);
// }

// #[no_mangle]
// pub fn add(left: i32, right: i32) -> i32 {
//     // hoge(left);
//     left + right
// }

#[no_mangle]
pub fn fib(n: i32) -> i32 {
    match n {
        ..=-1 => {
            0
        }
        ..=1 => {
            1
        }
        _ => {
            fib(n-1) + fib(n-2)
        }
    }
}

// fn fib2(n: i32) -> i32 {
//     if n <= -1 {
//         return 0;
//     }
//     if n <= 1 {
//         return 1;
//     }
//     return fib(n-1) + fib(n-2);
// }