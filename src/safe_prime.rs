//! Generates [safe prime numbers](https://www.wikiwand.com/en/Sophie_Germain_prime).

use ramp::Int;
use rand::OsRng;

pub use common::{gen_prime, is_prime};

/// Constructs a new `SafePrime` with a size of `bit_length` bits.
///
/// This will initialize an `OsRng` instance and call the
/// `SafePrime::from_rng()` method.
///
/// Note: the `bit_length` MUST be at least 512-bits.
pub fn new(bit_length: usize) -> Int {
    debug_assert!(bit_length >= 512);
    let mut rngesus = match OsRng::new() {
        Ok(rng) => rng,
        Err(reason) => panic!("Error initializing RNG: {}", reason),
    };

    from_rng(bit_length, &mut rngesus)
}

/// Constructs a new `SafePrime` with the size of `bit_length` bits, sourced
/// from an already-initialized `OsRng`.
pub fn from_rng(bit_length: usize, mut rngesus: &mut OsRng) -> Int {
    debug_assert!(bit_length >= 512);
    let mut candidate: Int;

    // Circumvent uninitialized warning (technically valid but compiler
    // cannot determine that `clone_from` will fill the value.
    let mut candidate_p: Int = Int::zero();

    loop {
        candidate = gen_prime(bit_length, &mut rngesus);

        candidate_p.clone_from(&candidate);
        candidate_p -= 1_usize;
        candidate_p /= 2_usize;

        if is_prime(&candidate_p) {
            break;
        }
    }

    candidate
}
