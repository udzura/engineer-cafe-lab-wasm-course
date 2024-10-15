extern "C" {
    fn my_callback(n: i32);
}

#[no_mangle]
pub unsafe fn fib(n: i32) -> i32 {
    let r = match n {
        ..=-1 => {
            0
        }
        ..=1 => {
            1
        }
        _ => {
            fib(n-1) + fib(n-2)
        }
    };
    my_callback(r);
    r
}