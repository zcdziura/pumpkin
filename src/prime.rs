//! Generates cryptographically secure prime numbers.

use rand::OsRng;

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
        let mut rngesus = try!(OsRng::new());
        Ok(try!(from_rng(bit_length, &mut rngesus)))
    }
}

#[cfg(test)]
mod tests {
    use super::{new, from_rng};

    #[test]
    #[should_panic]
    fn test_prime_bad_bit_length() {
        new(511);
    }
}
