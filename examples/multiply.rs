extern crate pumpkin;

use pumpkin::prime;

fn main() {
    let p = prime::new(1024).unwrap();
    let q = prime::new(1024).unwrap();
    let n = p * q;

    println!("The product of 'p' and 'q' is: {}", n);
}
