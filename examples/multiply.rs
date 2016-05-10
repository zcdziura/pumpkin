extern crate pumpkin;

use pumpkin::Prime;

fn main() {
    let p = Prime::new(2048);
    println!("{:x}", p);

    let q = Prime::new(2048);
    println!("\n{:x}", q);

    println!("\n{:X}", p * q);
}
