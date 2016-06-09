//! Generates [safe prime numbers](https://www.wikiwand.com/en/Sophie_Germain_prime).

use ramp::Int;
use rand::OsRng;

pub use common::{gen_prime, is_prime};
use error::{Error, Result};

/// Constructs a new `SafePrime` with a size of `bit_length` bits.
///
/// This will initialize an `OsRng` instance and call the
/// `SafePrime::from_rng()` method.
///
/// Note: the `bit_length` MUST be at least 512-bits.
pub fn new(bit_length: usize) -> Result {
    if bit_length < 512 {
        Err(Error::BitLength(bit_length))
    } else {
        let mut rngesus = try!(OsRng::new());
        Ok(try!(from_rng(bit_length, &mut rngesus)))
    }
}

/// Constructs a new `SafePrime` with the size of `bit_length` bits, sourced
/// from an already-initialized `OsRng`.
pub fn from_rng(bit_length: usize, mut rngesus: &mut OsRng) -> Result {
    if bit_length < 512 {
        Err(Error::BitLength(bit_length))
    } else {
        let mut candidate: Int;

        loop {
            candidate = try!(gen_prime(bit_length, &mut rngesus));

            let mut candidate_p = (&candidate).clone();
            candidate_p -= 1_usize;
            candidate_p /= 2_usize;

            if is_prime(&candidate_p) {
                break;
            }
        }

        Ok(candidate)
    }
}
