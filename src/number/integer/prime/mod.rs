// numera::number::integer::prime
//
//! Prime numbers.
//

#[cfg(feature = "std")]
use primal_sieve::Sieve;

use crate::all::{IntegerError, NumeraError, NumeraResult};
use core::fmt;

#[cfg(all(debug_assertions, feature = "std"))]
use az::CheckedAs;

mod consts;
mod family;
mod fns;
mod impl_from;
mod impl_traits;
mod trait_prime;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        consts::{PRIMES_BELL, PRIMES_U16, PRIMES_U8},
        family::Primes,
        fns::{is_prime_brute, nth_prime_brute},
        trait_prime::Prime,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::fns::{is_prime_sieve, nth_prime_sieve};
}

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

impl fmt::Display for Prime8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Prime16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Prime32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Prime8 {
    /// Returns a new `Prime8`.
    ///
    /// # Errors
    /// If the `value` provided is not a prime number.
    #[inline]
    pub fn new(value: u8) -> NumeraResult<Self> {
        if is_prime_brute(value.into()) {
            Ok(Prime8(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    // NOTE: the following 2 functions are duplicated because calling
    // is_prime_brute makes it non-const.

    /// Returns a new `Prime8`.
    ///
    /// # Safety
    /// The provided `value` must be a prime number.
    /// # Panics
    /// Panics in debug if `value` is not a prime number.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
    pub unsafe fn new_unchecked(value: u8) -> Self {
        debug_assert![is_prime_brute(value.into())];
        Self(value)
    }
    /// Returns a new `Prime8`.
    ///
    /// # Safety
    /// The provided `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime8};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime8::from_parts(2)?.nth()];
    /// assert_eq![54, Prime8::from_parts(251)?.nth()];
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
    /// Returns a new `Prime16`.
    ///
    /// # Errors
    /// If the `value` provided is not a prime number.
    #[inline]
    pub fn new(value: u16) -> NumeraResult<Self> {
        if is_prime_brute(value.into()) {
            Ok(Prime16(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    // NOTE: the following 2 functions are duplicated because calling
    // is_prime_brute makes it non-const.

    /// Returns a new `Prime16`.
    ///
    /// # Safety
    /// The provided `value` must be a prime number.
    /// # Panics
    /// Panics in debug if `value` is not a prime number.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
    pub unsafe fn new_unchecked(value: u16) -> Self {
        debug_assert![is_prime_brute(value.into())];
        Self(value)
    }
    /// Returns a new `Prime16`.
    ///
    /// # Safety
    /// The provided `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
    pub const unsafe fn new_unchecked(value: u16) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime16};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime16::from_parts(2)?.nth()];
    /// assert_eq![54, Prime16::from_parts(251)?.nth()];
    /// assert_eq![55, Prime16::from_parts(257)?.nth()];
    /// assert_eq![6_542, Prime16::from_parts(65_521)?.nth()];
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
    /// Returns a new `Prime32`.
    ///
    /// This can be very slow for big numbers, specially with the `no-std`
    /// implementation that calls [`is_prime_brute`].
    ///
    /// # Errors
    /// If the `value` provided is not a prime number.
    #[inline]
    #[cfg(not(feature = "std"))]
    pub fn new(value: u32) -> NumeraResult<Self> {
        if is_prime_brute(value) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    /// Returns a new `Prime32`.
    ///
    /// This can be very slow for big numbers, specially with the `no-std`
    /// implementation that calls [`is_prime_brute`].
    ///
    /// # Errors
    /// If the `value` provided is not a prime number or can't fit in a `usize`.
    #[inline]
    #[cfg(feature = "std")]
    pub fn new(value: u32) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    // NOTE: the next 2 functions are mostly a duplication, because calling
    // `is_prime_brute` doesn't allow the function to be `const`.

    /// Returns a new `Prime32`.
    ///
    /// # Safety
    /// The provided `value` must be a prime number.
    /// # Panics
    /// Panics in debug if `value` is not a prime number, or if can't fit in
    /// a `usize`.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe"), feature = "std"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
    pub unsafe fn new_unchecked(value: u32) -> Self {
        debug_assert![is_prime_sieve(value.checked_as::<usize>().unwrap())];
        Self(value)
    }
    /// Returns a new `Prime32`.
    ///
    /// # Safety
    /// The provided `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// Note that this operation can be slow for big 32-bit numbers,
    /// specially in no-std context.
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime32};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime32::from_parts(2)?.nth()];
    /// assert_eq![54, Prime32::from_parts(251)?.nth()];
    /// assert_eq![55, Prime32::from_parts(257)?.nth()];
    /// assert_eq![6_542, Prime32::from_parts(65_521)?.nth()];
    /// assert_eq![6_543, Prime32::from_parts(65_537)?.nth()];
    /// assert_eq![40_000_000, Prime32::from_parts(776_531_401)?.nth()];
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
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
            #[cfg(feature = "std")]
            {
                let sieve = Sieve::new(self.0 as usize);
                return sieve.prime_pi(self.0 as usize);
            }
            // this can be VERY slow for high 32-bit numbers:
            #[cfg(not(feature = "std"))]
            {
                todo![] // TODO
            }
        }
        return 203_280_221;
    }
}
