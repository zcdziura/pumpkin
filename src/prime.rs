//! Generates cryptographically secure prime numbers.

use ramp::Int;
use rand::OsRng;

pub use common::gen_prime as from_rng;

/// Constructs a new prime number with a size of `bit_length` bits.
///
/// This will initialize an `OsRng` instance and call the
/// `from_rng()` function.
///
/// Note: the `bit_length` MUST be at least 512-bits.
pub fn new(bit_length: usize) -> Int {
    assert!(bit_length >= 512);
    let mut rngesus = match OsRng::new() {
        Ok(rng) => rng,
        Err(reason) => panic!("Error initializing RNG: {}", reason),
    };

    from_rng(bit_length, &mut rngesus)
}

#[cfg(test)]
mod tests {
    use ramp::Int;
    use super::{fermat, miller_rabin};

    #[test]
    fn test_fermat_pass() {
        assert!(fermat(&Int::from(7919)));
    }

    #[test]
    #[should_panic]
    fn test_fermat_fail() {
        assert!(fermat(&Int::from(7920)));
    }

    #[test]
    fn test_miller_rabin_pass() {
        assert!(miller_rabin(&Int::from(7919), 5));
    }

    #[test]
    #[should_panic]
    fn test_miller_rabin_fail() {
        assert!(miller_rabin(&Int::from(7920), 5));
    }
}
