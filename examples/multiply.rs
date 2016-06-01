extern crate pumpkin;

use pumpkin::prime;

fn main() {
    let p = prime::new(2048);
    println!("{:x}", p);

    let q = prime::new(2048);
    println!("\n{:x}", q);

    println!("\n{:X}", p * q);
}
