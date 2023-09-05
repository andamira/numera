// numera::number::integer::prime::trait
//
//!
//

use super::{
    data::{PRIMES_U16, PRIMES_U8},
    Prime128, Prime16, Prime32, Prime64, Prime8,
};

/// Common trait for all primes.
pub trait Prime {
    /// Returns the `nth` prime, or `None` if out of bounds.
    fn get_nth(nth: usize) -> Option<Self>
    where
        Self: Sized;

    /// Returns `true` if this is a Bell prime.
    ///
    /// Bell primes are those that are the number of partitions of a set with *n* members.
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
        matches![self.0, 2 | 5]
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
        matches![self.0, 2 | 5 | 877]
    }
}

impl Prime for Prime32 {
    /// WIP: For now this function only returns Some values if `nth` <= 6,541.
    fn get_nth(nth: usize) -> Option<Self> {
        match nth {
            0..=53 => Some(Prime32(PRIMES_U8[nth].into())),
            54..=6541 => Some(Prime32(PRIMES_U16[nth - 54].into())),
            6542..=203_280_220 => {
                None // TODO
            }
            _ => None,
        }
    }

    fn is_bell(&self) -> bool {
        matches![self.0, 2 | 5 | 877 | 27_644_437]
    }
}

impl Prime for Prime64 {
    /// WIP: For now this function only returns Some values if `nth` <= 6,541.
    #[allow(clippy::match_same_arms)] // TEMP
    fn get_nth(nth: usize) -> Option<Self> {
        match nth {
            0..=53 => Some(Prime64(PRIMES_U8[nth].into())),
            54..=6_541 => Some(Prime64(PRIMES_U16[nth - 54].into())),
            6_542..=203_280_220 => {
                None // TODO
            }
            #[allow(overflowing_literals)] // usize on < 64-bit platforms
            203_280_221..=425_656_284_035_217_742 => {
                None // TODO
            }
            _ => None,
        }
    }

    fn is_bell(&self) -> bool {
        matches![self.0, 2 | 5 | 877 | 27_644_437]
    }
}

impl Prime for Prime128 {
    /// WIP: For now this function only returns Some values if `nth` <= 6,541.
    #[allow(clippy::match_same_arms)] // TEMP
    fn get_nth(nth: usize) -> Option<Self> {
        match nth {
            0..=53 => Some(Prime128(PRIMES_U8[nth].into())),
            54..=6_541 => Some(Prime128(PRIMES_U16[nth - 54].into())),
            6_542..=203_280_220 => {
                None // TODO
            }
            #[allow(overflowing_literals)] // usize on < 64-bit platforms
            203_280_221..=425_656_284_035_217_742 => {
                None // TODO
            }
            _ => None,
        }
    }

    fn is_bell(&self) -> bool {
        matches![
            self.0,
            2 | 5 | 877 | 27_644_437 | 35_742_549_198_872_617_291_353_508_656_626_642_567
        ]
    }
}
