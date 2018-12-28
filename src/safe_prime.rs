//! Generates [safe prime numbers](https://www.wikiwand.com/en/Sophie_Germain_prime).

use ramp::Int;

use rand::rngs::OsRng;

pub use common::{gen_prime, is_prime};
use common::{two, three};
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
        let mut rngesus = OsRng::new()?;
        Ok(from_rng(bit_length, &mut rngesus)?)
    }
}

/// Checks if number is a safe prime
pub fn is_safe_prime(candidate: &Int) -> bool {
    // according to https://eprint.iacr.org/2003/186.pdf
    // a safe prime is congruent to 2 mod 3
    if (candidate % three()) == two() {
        if is_prime(&candidate) {
            // a safe prime satisfies (p-1)/2 is prime. Since a
            // prime is odd, We just need to divide by 2
            let candidate_p = candidate >> 1;
            return is_prime(&candidate_p)
        }
    }
    false
}

/// Constructs a new `SafePrime` with the size of `bit_length` bits, sourced
/// from an already-initialized `OsRng`.
pub fn from_rng(bit_length: usize, mut rngesus: &mut OsRng) -> Result {
    if bit_length < 512 {
        Err(Error::BitLength(bit_length))
    } else {
        let mut candidate: Int;

        loop {
            candidate = gen_prime(bit_length, &mut rngesus)?;

            if is_safe_prime(&candidate) {
                break;
            }

            candidate <<= 1;
            candidate += 1;

            if is_prime(&candidate) {
                break;
            }
        }

        Ok(candidate)
    }
}

#[cfg(test)]
mod tests {
    use super::{new, is_safe_prime};
    use ramp::Int;

    #[test]
    fn test_safe_prime_bit_length_too_small() {
        let sp = new(511);
        assert_eq!(false, match sp {
            Ok(_) => true,
            Err(_) => false
        });
    }

    #[test]
    fn test_safe_prime() {
        let sp = new(512);
        assert_eq!(true, match sp {
            Ok(_) => true,
            Err(_) => false
        });
    }

    #[test]
    fn test_is_safe_prime() {
        //Numbers pulled from https://github.com/mikelodder7/cunningham_chain/blob/master/findings.md
        //p0 is sophie german prime
        let p0 = Int::from_str_radix("37313426856874901938110133384605074194791927500210707276948918975046371522830901596065044944558427864187196889881993164303255749681644627614963632713725183364319410825898054225147061624559894980555489070322738683900143562848200257354774040241218537613789091499134051387344396560066242901217378861764936185029", 10).unwrap();
        assert!(!is_safe_prime(&p0));
        let p1 = Int::from_str_radix("74626853713749803876220266769210148389583855000421414553897837950092743045661803192130089889116855728374393779763986328606511499363289255229927265427450366728638821651796108450294123249119789961110978140645477367800287125696400514709548080482437075227578182998268102774688793120132485802434757723529872370059", 10).unwrap();
        assert!(is_safe_prime(&p1));
        let p2 = Int::from_str_radix("149253707427499607752440533538420296779167710000842829107795675900185486091323606384260179778233711456748787559527972657213022998726578510459854530854900733457277643303592216900588246498239579922221956281290954735600574251392801029419096160964874150455156365996536205549377586240264971604869515447059744740119", 10).unwrap();
        assert!(is_safe_prime(&p2));
        let p3 = Int::from_str_radix("298507414854999215504881067076840593558335420001685658215591351800370972182647212768520359556467422913497575119055945314426045997453157020919709061709801466914555286607184433801176492996479159844443912562581909471201148502785602058838192321929748300910312731993072411098755172480529943209739030894119489480239", 10).unwrap();
        assert!(is_safe_prime(&p3));
        let p4 = Int::from_str_radix("4806876214089177439121678559764069543282270755154137981051366776821330958611719328037311759924923156830623290278296826263863902327008664143707117531049168010908663795201825132050017581985031718536424081509084930569115857201636971728388275433540277562846153879803474020036767852693656753257597801227199822164846876100177774044259379232968071371318658371230787073384750022830829873718254139779006439569882904712552834431199870749249168775012460891012776977366721903", 10).unwrap();
        assert!(is_safe_prime(&p4));
    }
}
