use num::{BigUint, FromPrimitive, One, Zero, pow};
use num::bigint::RandBigInt;
use num::integer::Integer;

use rand::{OsRng, thread_rng};

pub fn generate_prime() -> BigUint {
    /* Generates a large prime number within the range [2^2048, 2^2049);
     * that is, generates a large prime number between 2^2048 inclusive and
     * 2^2049 exclusive.
     */

    // Generate the RNG, which will be sourced from the OS's secure source
    // of entropy. Also, give it a sweet name.
    let one: BigUint = One::one();
    let two = one.clone() + one.clone();
    let mut rngesus = match OsRng::new() {
        Ok(o) => o,
        Err(err) => panic!("Error initializing RNG: {}", err)
    };

    let lower_bound = pow(BigUint::from_u8(2).unwrap(), 2048);
    let upper_bound = pow(BigUint::from_u8(2).unwrap(), 2049);

    let mut candidate = rngesus.gen_biguint_range(&lower_bound,
                                                     &upper_bound);
    if candidate.is_even() {
        candidate = candidate + one.clone();
    }
    while !test_prime(&candidate, 128) {
        candidate = candidate + two.clone();
    }
    
    candidate
}

fn test_prime(candidate: &BigUint, limit: u8) -> bool {
    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
    let two = one.clone() + one.clone();
    
    if *candidate < two {
        false
    } else if *candidate == two {
        true
    } else if candidate.is_even() {
        false
    } else {
        let (s, d) = rewrite(&(candidate - one.clone()));
        let mut k = 0;

        while k < limit {
            let basis = thread_rng().gen_biguint_range(&two, candidate);
            let mut v = modulo(&basis, &d, candidate);

            if v != one.clone() && v != (candidate - one.clone()) {
                let mut i = zero.clone();
                loop {
                    v = modulo(&v, &two, candidate);
                    if v == (candidate - one.clone()) {
                        break
                    } else if v == one.clone() || i == (s.clone() - one.clone()) {
                        return false;
                    }
                    
                    i = i + one.clone();
                }
            }  

            k = k + 2;
        }

        true
    }
}

fn rewrite(n: &BigUint) -> (BigUint, BigUint) {
    let mut d = n.clone();
    let mut s: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = one.clone() + one.clone();

    while d.is_even() {
        d = d.clone() / two.clone();
        s = s.clone() + one.clone();
    }

    (s, d)
}

fn modulo(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    // Modular exponentiation function, with BigUints!
    // Warning!! Ahead, thar be clones!
    let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
    let mut result = one.clone();
    let mut b = base.clone();
    let mut e = exponent.clone();

    while e > zero {
        if (exponent & one.clone()) == one {
            result = (result * b.clone()) % modulus;
        }

        b = (b.clone() * b.clone()) % modulus;
        e = e >> 1;
    }

    result
}
