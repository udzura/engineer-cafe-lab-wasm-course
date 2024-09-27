use std::error::Error;

use fukuoka_ecl_imagetest::*;

fn main() -> Result<(), Box<dyn Error>> {
    grayscale2("./source.png", "./result2.png")
}
