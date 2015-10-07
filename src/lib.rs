#![feature(augmented_assignments)]
#![feature(core)]
#![feature(test)]

/// # The Pumpkin Prime Number Generator
///
/// The `pumpkin` prime number generator library can be used to generate
/// prime numbers of any reasonable length, suitable for any cryptographic
/// purpose. All numbers generated are seeded from the operating system's
/// secure source of entrophy and are verified using three different primality
/// tests.
///
/// ## Installation
///
/// To include `pumpkin` in your project add the following line to your
/// Cargo.toml file:
///
/// ```
/// [dependencies]
/// pumpkin = "*"
/// ```
///
/// ## Examples
///
/// ```
/// fn main() {
///     // Generate a 2048-bit prime number
///     let prime = pumpkin::Prime::new(2048);
///
///     // Want to very the prime you generated is ACTUALLY prime, and not
///     // simply a probable prime? Easy!
///     assert_eq!(prime.verify(), true);
/// }
/// ```

extern crate core;
extern crate ramp;
extern crate rand;

pub mod prime;
