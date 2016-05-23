#![feature(test)]
#![deny(missing_docs, missing_debug_implementations,
    missing_copy_implementations, trivial_casts, trivial_numeric_casts,
    unsafe_code, unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! A crate for generating large, cryptographically secure prime numbers.
//! `Primes` are seeded from the operating system's main source of entropy,
//! ensuring proper randomness.
//!
//! `Primes` must be AT LEAST 512-bits long. Attempting to generate a `Prime`
//! less than 512-bits long will cause a panic.
//!
//! ## Example
//!
//! ```
//! extern crate pumpkin;
//!
//! use pumpkin::Prime;
//!
//! fn main() {
//!     // Generate 2048-bit primes
//!     let p = Prime::new(2048);
//!     let q = Prime::new(2048);
//!
//!     let n = p * q;
//!     println!("{}", n); // Some 4096-bit composite number
//! }
//! ```

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;
extern crate ramp;
extern crate rand;
extern crate test;

mod prime;
pub use prime::Prime;
pub use prime::SafePrime;

#[cfg(test)]
mod tests {
    use rand::OsRng;
    use super::*;
    use test::Bencher;

    #[test]
    #[should_panic]
    fn test_new_small_prime() {
        Prime::new(511);
    }

    #[test]
    #[should_panic]
    fn test_new_small_prime_from_rng() {
        let mut rngesus = OsRng::new().unwrap();

        Prime::from_rng(511, &mut rngesus);
    }

    #[test]
    fn test_should_destructure() {
        let Prime(n) = Prime::new(512);
    }

    #[bench]
    fn bench_generate_512_bit_prime(b: &mut Bencher) {
        b.iter(|| Prime::new(512));
    }

    #[bench]
    fn bench_generate_1024_bit_prime(b: &mut Bencher) {
        b.iter(|| Prime::new(1024));
    }

    #[bench]
    fn bench_generate_2048_bit_prime(b: &mut Bencher) {
        b.iter(|| Prime::new(2048));
    }
}
