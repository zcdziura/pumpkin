#![feature(test)]

/// # The Pumpkin Prime Number Generator
///
/// The `pumpkin` prime number generator library can be used to generate
/// prime numbers of any reasonable length, suitable for any cryptographic
/// purpose. All numbers generated are seeded from the operating system's
/// secure source of entrophy and are verified using three different primality
/// tests.
///
/// Primes have to be AT LEAST 512-bits long. Any lower bit-length will
/// immediately fail.
///
/// ## Examples
///
/// ```
/// extern crate pumpkin;
///
/// use pumpkin::Prime;
///
/// fn main() {
///     // Generate a 2048-bit prime number
///     let p = Prime::new(2048);
///     let q = Prime::new(2048);
///
///     let r = p * q;
///     println!("{}", r); // Some ridiculously large number
/// }
/// ```

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;
extern crate ramp;
extern crate rand;
extern crate test;

mod prime;
pub use prime::Prime;

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
        let mut rngesus = match OsRng::new() {
            Ok(rng) => rng,
            Err(reason) => panic!("An error occurred when initializing the RNG: {}", reason)
        };

        Prime::from_rng(511, &mut rngesus);
    }

    #[bench]
    #[ignore]
    fn bench_generate_512_bit_prime(b: &mut Bencher) {
        b.iter(|| Prime::new(512));
    }

    #[bench]
    #[ignore]
    fn bench_generate_1024_bit_prime(b: &mut Bencher) {
        b.iter(|| Prime::new(1024));
    }

    #[bench]
    #[ignore]
    fn bench_generate_2048_bit_prime(b: &mut Bencher) {
        b.iter(|| Prime::new(2048));
    }
}
