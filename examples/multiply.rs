extern crate pumpkin;

use pumpkin::Prime;

fn main() {
    let p = Prime::new(2048);
    let q = Prime::new(2048);
    let s = p * q;

    println!("{}", s);
}
