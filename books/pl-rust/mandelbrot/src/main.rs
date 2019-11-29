extern crate num;
use num::Complex;

#[allow(dead_code)]
fn complex_square_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0., im: 0. };
    loop {
        z = z * z + c;
    }
}

fn main() {
    println!("Hello, world!");
}
