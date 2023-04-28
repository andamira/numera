// numera::number::integer::prime::trait_prime
//
//!
//

use super::{Prime16, Prime32, Prime8, PRIMES_U16, PRIMES_U8};

/// Common trait for all primes.
pub trait Prime {
    /// Returns the `nth` prime, or `None` if out of bounds.
    fn get_nth(nth: usize) -> Option<Self>
    where
        Self: Sized;

    /// Returns `true` if the prime is a Bell prime.
    ///
    /// Bell primes are those that are the number of partitions of a set with n members.
    fn is_bell(&self) -> bool;

    // /// Returns Some(true) if the prime is balanced.
    // ///
    // /// Balanced primes are primes with equal-sized prime gaps above and below
    // /// them, so that they are equal to the arithmetic mean of the nearest
    // /// primes above and below.
    // // https://en.wikipedia.org/wiki/Balanced_prime
    // fn is_some_balanced(&self) -> bool;
}

impl Prime for Prime8 {
    fn get_nth(nth: usize) -> Option<Self> {
        match nth {
            0..=53 => Some(Prime8(PRIMES_U8[nth])),
            _ => None,
        }
    }

    fn is_bell(&self) -> bool {
        matches![self.0, 5 | 8]
    }
}

impl Prime for Prime16 {
    fn get_nth(nth: usize) -> Option<Self> {
        match nth {
            0..=53 => Some(Prime16(PRIMES_U8[nth].into())),
            54..=6541 => Some(Prime16(PRIMES_U16[nth - 54])),
            _ => None,
        }
    }

    fn is_bell(&self) -> bool {
        matches![self.0, 5 | 8 | 877]
    }
}

impl Prime for Prime32 {
    fn get_nth(nth: usize) -> Option<Self> {
        match nth {
            0..=53 => Some(Prime32(PRIMES_U8[nth].into())),
            54..=6541 => Some(Prime32(PRIMES_U16[nth - 54].into())),
            6542..=203_280_219 => {
                todo![]
            }
            _ => None,
        }
    }

    fn is_bell(&self) -> bool {
        matches![self.0, 5 | 8 | 877 | 27_644_437]
    }
}
