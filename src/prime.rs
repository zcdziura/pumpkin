//! Generates cryptographically secure prime numbers.

use rand::rngs::OsRng;

pub use common::gen_prime as from_rng;
use error::{Error, Result};

/// Constructs a new prime number with a size of `bit_length` bits.
///
/// This will initialize an `OsRng` instance and call the
/// `from_rng()` function.
///
/// Note: the `bit_length` MUST be at least 512-bits.
pub fn new(bit_length: usize) -> Result {
    if bit_length < 512 {
        Err(Error::BitLength(bit_length))
    } else {
        let mut rngesus = OsRng::new()?;
        Ok(from_rng(bit_length, &mut rngesus)?)
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn test_prime_bit_length_too_small() {
        let p = new(511);
        assert_eq!(false, match p {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    #[test]
    fn test_prime() {
        let p = new(512);
        assert_eq!(true, match p {
            Ok(_) => true,
            Err(_) => false
        });
    }
}
