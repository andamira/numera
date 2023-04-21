// numera::number::integer::prime
//
//! Prime numbers.
//

use primal_sieve::Sieve;

mod arrays;
mod fns;
mod impl_traits;
mod trait_prime;

pub use arrays::{PRIMES_BELL, PRIMES_U16, PRIMES_U8};
pub use fns::{is_prime_brute, nth_prime_brute};
pub use trait_prime::Prime;

/// An 8-bit prime number that can represent the first 54 prime numbers.
///
// pub struct Prime8(PositiveInteger8);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prime8(u8);

/// A 16-bit prime number that can represent the first 6,542 prime numbers.
///
// pub struct Prime16(PositiveInteger16);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prime16(u16);

/// A 32-bit prime number that can represent the first 203,280,219 prime numbers.
///
// pub struct Prime32(PositiveInteger32);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prime32(u32);

impl Prime8 {
    /// Returns the number of primes upto and including the current one.
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime8};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime8::new(2)?.nth()];
    /// assert_eq![54, Prime8::new(251)?.nth()];
    /// # Ok(()) }
    /// ```
    pub fn nth(&self) -> usize {
        for (i, &p) in PRIMES_U8.iter().enumerate() {
            if p == self.0 {
                return i + 1;
            }
        }
        return 54;
    }
}

impl Prime16 {
    /// Returns the number of primes upto and including the current one.
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime16};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime16::new(2)?.nth()];
    /// assert_eq![54, Prime16::new(251)?.nth()];
    /// assert_eq![55, Prime16::new(257)?.nth()];
    /// assert_eq![6_542, Prime16::new(65_521)?.nth()];
    /// # Ok(()) }
    /// ```
    pub fn nth(&self) -> usize {
        // A) CHECK BENCH
        // for (i, &p) in PRIMES_U8.iter().enumerate() {
        //     if p == core::cmp::min(u8::MAX as u16, self.0) as u8 {
        //         return i + 1
        //     }
        // }
        // B)
        if self.0 < u16::from(u8::MAX) {
            for (i, &p) in PRIMES_U8.iter().enumerate() {
                if u16::from(p) == self.0 {
                    return i + 1;
                }
            }
        } else {
            for (i, &p) in PRIMES_U16.iter().enumerate() {
                if p == self.0 {
                    return i + 55;
                }
            }
        }
        return 6_542;
    }
}
impl Prime32 {
    /// Returns the number of primes upto and including the current one.
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime32};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime32::new(2)?.nth()];
    /// assert_eq![54, Prime32::new(251)?.nth()];
    /// assert_eq![55, Prime32::new(257)?.nth()];
    /// assert_eq![6_542, Prime32::new(65_521)?.nth()];
    /// assert_eq![6_543, Prime32::new(65_537)?.nth()];
    /// assert_eq![40_000_000, Prime32::new(776_531_401)?.nth()];
    /// # Ok(()) }
    /// ```
    pub fn nth(&self) -> usize {
        if self.0 < u32::from(u8::MAX) {
            for (i, &p) in PRIMES_U8.iter().enumerate() {
                if u32::from(p) == self.0 {
                    return i + 1;
                }
            }
        } else if self.0 < u32::from(u16::MAX) {
            for (i, &p) in PRIMES_U16.iter().enumerate() {
                if u32::from(p) == self.0 {
                    return i + 55;
                }
            }
        } else {
            // this can be slow for high 32-bit numbers:
            let sieve = Sieve::new(self.0 as usize);
            return sieve.prime_pi(self.0 as usize);
        }
        return 203_280_221;
    }
}

/* conversions */

impl From<Prime8> for Prime16 {
    fn from(p: Prime8) -> Prime16 {
        Prime16(p.0.into())
    }
}
impl From<Prime8> for Prime32 {
    fn from(p: Prime8) -> Prime32 {
        Prime32(p.0.into())
    }
}
impl From<Prime16> for Prime32 {
    fn from(p: Prime16) -> Prime32 {
        Prime32(p.0.into())
    }
}

impl From<Prime8> for u8 {
    fn from(p: Prime8) -> u8 {
        p.0
    }
}
impl From<Prime8> for u16 {
    fn from(p: Prime8) -> u16 {
        p.0.into()
    }
}
impl From<Prime8> for u32 {
    fn from(p: Prime8) -> u32 {
        p.0.into()
    }
}
impl From<Prime8> for u64 {
    fn from(p: Prime8) -> u64 {
        p.0.into()
    }
}
impl From<Prime8> for u128 {
    fn from(p: Prime8) -> u128 {
        p.0.into()
    }
}
impl From<Prime8> for usize {
    fn from(p: Prime8) -> usize {
        p.0.into()
    }
}

impl From<Prime16> for u16 {
    fn from(p: Prime16) -> u16 {
        p.0
    }
}
impl From<Prime16> for u32 {
    fn from(p: Prime16) -> u32 {
        p.0.into()
    }
}
impl From<Prime16> for u64 {
    fn from(p: Prime16) -> u64 {
        p.0.into()
    }
}
impl From<Prime16> for u128 {
    fn from(p: Prime16) -> u128 {
        p.0.into()
    }
}
impl From<Prime16> for usize {
    fn from(p: Prime16) -> usize {
        p.0.into()
    }
}

impl From<Prime32> for u32 {
    fn from(p: Prime32) -> u32 {
        p.0
    }
}
impl From<Prime32> for u64 {
    fn from(p: Prime32) -> u64 {
        p.0.into()
    }
}
impl From<Prime32> for u128 {
    fn from(p: Prime32) -> u128 {
        p.0.into()
    }
}
