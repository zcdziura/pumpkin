#![feature(test)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! A crate for generating large, cryptographically secure prime numbers.
//! These numbers are seeded from the operating system's main source of
//! entropy, ensuring proper randomness.
//!
//! Numbers are verified to be prime by running the following three tests
//! during initialization:
//!
//! 1. Dividing the initial prime number candidate by the first 1,000 prime
//! numbers, checking the remainder. Should the remainder ever be zero, then
//! add two to the candidate and try again.
//!
//! 2. Run a Fermat Primality Test on the candidate. If it doesn't pass, add
//! two to the candidate and goto Step 1.
//!
//! 3. Finally, complete five rounds of the Miller-Rabin Primality Test.
//! Should any of the tests pass, add two to the candidate and goto Step 1.
//!
//! The preceding steps mirror those used by GnuPG, a leading PGP implementation
//! used by thousands of users all across the world.
//!
//! The prime numbers must be AT LEAST 512-bits long. Attempting to generate a
//! number less than 512-bits long will cause a panic.
//!
//! ## Example
//!
//! ```
//! extern crate pumpkin;
//!
//! use pumpkin::prime;
//!
//! fn main() {
//!     // Generate 2, 2048-bit primes
//!     let p = prime::new(2048);
//!     let q = prime::new(2048);
//!
//!     let n = p * q;
//!     println!("{}", n); // Some 4096-bit composite number
//! }
//! ```

#[allow(unused_imports)]
#[macro_use]
extern crate custom_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate newtype_derive;
extern crate ramp;
extern crate rand;
extern crate test;

mod common;
pub mod error;
pub mod prime;
pub mod safe_prime;

#[cfg(test)]
mod tests {
    use super::{prime, safe_prime};
    use rand::rngs::OsRng;
    use test::Bencher;

    #[bench]
    fn bench_generate_512_bit_prime(b: &mut Bencher) {
        let mut rngesus = OsRng::new().unwrap();
        b.iter(|| prime::from_rng(512, &mut rngesus));
    }

    #[bench]
    fn bench_generate_1024_bit_prime(b: &mut Bencher) {
        let mut rngesus = OsRng::new().unwrap();
        b.iter(|| prime::from_rng(1024, &mut rngesus));
    }

    #[bench]
    fn bench_generate_2048_bit_prime(b: &mut Bencher) {
        let mut rngesus = OsRng::new().unwrap();
        b.iter(|| prime::from_rng(2048, &mut rngesus));
    }

    #[bench]
    fn bench_generate_512_bit_safe_prime(b: &mut Bencher) {
        let mut rngesus = OsRng::new().unwrap();
        b.iter(|| safe_prime::from_rng(512, &mut rngesus));
    }
}
