// numera::number::integer::prime
//
//! Prime numbers ([m][0m]/[o][0o]/[w][0w]), from the set $\Bbb{P}$.
//!
//! $ \Bbb{P} = \lbrace 2, 3, 5, 7, 11, 13, 17, 19, 23, … \rbrace $ ([oeis])
//!
//! [0m]: https://mathworld.wolfram.com/PrimeNumber.html
//! [0o]: https://oeis.org/wiki/Prime_numbers
//! [0w]: https://en.wikipedia.org/wiki/Prime_number
//! [oeis]: https://oeis.org/A000040
//

use crate::all::{IntegerError, NumeraError, NumeraResult};
use core::fmt;

#[cfg(feature = "std")]
use {devela::az::CheckedAs, primal_sieve::Sieve};

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
        fns::{is_prime, nth_prime, prime_pi},
        trait_prime::Prime,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::fns::{is_prime_sieve, nth_prime_sieve, prime_number_theorem, prime_pi_sieve};
}

use crate::number::macros::define_abbreviations;
define_abbreviations![many P, Prime, 8, 16, 32, 64, 128];

/// An 8-bit prime number, from the set $\Bbb{P}$,
/// also known as [`P8`].
///
/// Can represent the first 54 prime numbers.
///
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Prime8(PositiveInteger8);
pub struct Prime8(pub(super) u8);

/// A 16-bit prime number, from the set $\Bbb{P}$,
/// also known as [`P16`].
///
/// Can represent the first 6,542 prime numbers.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Prime16(PositiveInteger16);
pub struct Prime16(pub(super) u16);

/// A 32-bit prime number, from the set $\Bbb{P}$,
/// also known as [`P32`].
///
/// Can represent the first 203,280,219 prime numbers.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Prime32(PositiveInteger32);
pub struct Prime32(pub(super) u32);

/// A 64-bit prime number, from the set $\Bbb{P}$,
/// also known as [`P64`].
///
/// Can represent the first *approximately* 415,828,534,307,635,072 prime
/// numbers (1 per 44).
/// This is calculated using the *prime number theorem* formula.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Prime64(PositiveInteger64);
pub struct Prime64(pub(super) u64);

/// A 128-bit prime number, from the set $\Bbb{P}$,
/// also known as [`P64`].
///
/// Can represent the first *approximately*
/// 3,835,341,275,459,348,115,779,911,081,237,938,176 prime numbers (1 per 88).
/// This is calculated using the *prime number theorem* formula.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Prime128(PositiveInteger128);
pub struct Prime128(pub(super) u128);

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
impl fmt::Display for Prime64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Prime128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for Prime8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P8({})", self.0)
    }
}
impl fmt::Debug for Prime16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P16({})", self.0)
    }
}
impl fmt::Debug for Prime32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P32({})", self.0)
    }
}
impl fmt::Debug for Prime64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P64({})", self.0)
    }
}
impl fmt::Debug for Prime128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P128({})", self.0)
    }
}

impl Prime8 {
    /// Returns a new `Prime8`.
    ///
    /// # Errors
    /// If the given `value` is not a prime number.
    #[inline]
    pub fn new(value: u8) -> NumeraResult<Self> {
        if is_prime(value.into()) {
            Ok(Prime8(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    /// Returns the `nth` prime number.
    ///
    /// # Errors
    /// If `nth` is 0 or greater than 54.
    #[inline]
    pub const fn new_nth(nth: u8) -> NumeraResult<Self> {
        match nth {
            1..=54 => Ok(Self(PRIMES_U8[(nth - 1) as usize])),
            _ => Err(NumeraError::Integer(IntegerError::Overflow)),
        }
    }

    // NOTE: the following 2 functions are duplicated because calling
    // is_prime makes it non-const.

    /// Returns a new `Prime8`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    /// # Panics
    /// Panics in debug if the given `value` is not a prime number.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn new_unchecked(value: u8) -> Self {
        debug_assert![is_prime(value.into())];
        Self(value)
    }
    /// Returns a new `Prime8`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// # Notation
    /// $\pi(x)$
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime8};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime8::new(2)?.pi()];
    /// assert_eq![54, Prime8::new(251)?.pi()];
    /// # Ok(()) }
    /// ```
    ///
    /// # Links
    /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
    /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
    pub fn pi(&self) -> usize {
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
    /// If the given `value` is not a prime number.
    #[inline]
    pub fn new(value: u16) -> NumeraResult<Self> {
        if is_prime(value.into()) {
            Ok(Prime16(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    /// Returns the `nth` prime number.
    ///
    /// # Errors
    /// If `nth` is 0 or greater than 6,542.
    #[inline]
    pub const fn new_nth(nth: u16) -> NumeraResult<Self> {
        match nth {
            #[allow(clippy::cast_possible_truncation)]
            1..=54 => Ok(Self(PRIMES_U8[(nth as u8 - 1) as usize] as u16)),
            55..=6542 => Ok(Self(PRIMES_U16[(nth + 54) as usize])),
            _ => Err(NumeraError::Integer(IntegerError::Overflow)),
        }
    }

    // NOTE: the following 2 functions are duplicated because calling
    // is_prime makes it non-const.

    /// Returns a new `Prime16`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    /// # Panics
    /// Panics in debug if the given `value` is not a prime number.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn new_unchecked(value: u16) -> Self {
        debug_assert![is_prime(value.into())];
        Self(value)
    }
    /// Returns a new `Prime16`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub const unsafe fn new_unchecked(value: u16) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// # Notation
    /// $\pi(x)$
    ///
    /// # Example
    /// ```
    /// use numera::all::{Number, Prime16};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime16::new(2)?.pi()];
    /// assert_eq![54, Prime16::new(251)?.pi()];
    /// assert_eq![55, Prime16::new(257)?.pi()];
    /// assert_eq![6_542, Prime16::new(65_521)?.pi()];
    /// # Ok(()) }
    /// ```
    ///
    /// # Links
    /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
    /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
    pub fn pi(&self) -> usize {
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
    /// implementation that calls [`is_prime`].
    ///
    /// # Errors
    /// If the given `value` is not a prime number.
    #[inline]
    #[cfg(not(feature = "std"))]
    pub fn new(value: u32) -> NumeraResult<Self> {
        if is_prime(value) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    /// Returns a new `Prime32`.
    ///
    /// This can be very slow for big numbers, specially with the `no-std`
    /// implementation that calls [`is_prime`].
    ///
    /// # Errors
    /// If the given `value` is not a prime number or it can't fit in a `usize`.
    #[inline]
    #[cfg(feature = "std")]
    pub fn new(value: u32) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    /// Returns the `nth` prime number.
    ///
    /// # Errors
    /// If `nth` is 0 or greater than 203,280,219.
    #[inline]
    #[allow(clippy::missing_panics_doc)] // NonZeroUsize can't be zero
    pub fn new_nth(nth: u32) -> NumeraResult<Self> {
        match nth {
            #[allow(clippy::cast_possible_truncation)]
            1..=54 => Ok(Self(u32::from(PRIMES_U8[(nth as u8 - 1) as usize]))),
            #[allow(clippy::cast_possible_truncation)]
            55..=6_542 => Ok(Self(u32::from(PRIMES_U16[(nth as u16 + 54) as usize]))),
            6_543..=203_280_219 => {
                #[cfg(feature = "std")]
                {
                    use core::num::NonZeroUsize;

                    #[allow(clippy::cast_possible_truncation)]
                    return Ok(Self(
                        nth_prime_sieve(NonZeroUsize::new(nth as usize).unwrap()) as u32,
                    ));
                }

                #[cfg(not(feature = "std"))]
                {
                    use core::num::NonZeroU32;

                    // Ok(Self(nth_prime(nth)))
                    Ok(Self(nth_prime(NonZeroU32::new(nth).unwrap())))
                }
            }
            _ => Err(NumeraError::Integer(IntegerError::Overflow)),
        }
    }

    // NOTE: the next 2 functions are mostly a duplication, because calling
    // `is_prime` doesn't allow the function to be `const`.

    /// Returns a new `Prime32`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    /// # Panics
    /// Panics in debug if the given `value` is not a prime number, or if it
    /// can't fit in a `usize`.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe"), feature = "std"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn new_unchecked(value: u32) -> Self {
        debug_assert![is_prime_sieve(value.checked_as::<usize>().unwrap())];
        Self(value)
    }
    /// Returns a new `Prime32`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// Note that this operation can be slow for big 32-bit numbers,
    /// specially in a no-std context.
    ///
    /// # Notation
    /// $\pi(x)$
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Number, Prime32};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime32::new(2)?.pi()];
    /// assert_eq![54, Prime32::new(251)?.pi()];
    /// assert_eq![55, Prime32::new(257)?.pi()];
    /// assert_eq![6_542, Prime32::new(65_521)?.pi()];
    /// assert_eq![6_543, Prime32::new(65_537)?.pi()];
    /// assert_eq![40_000_000, Prime32::new(776_531_401)?.pi()];
    /// # Ok(()) }
    /// ```
    ///
    /// # Links
    /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
    /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn pi(&self) -> usize {
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
                nth_prime(core::num::NonZeroU32::new(self.0).unwrap())
            }
        }
        return 203_280_221;
    }
}

impl Prime64 {
    /// Returns a new `Prime64`.
    ///
    /// This can be impossibly slow for big numbers.
    ///
    /// # Errors
    /// If the given `value` is not a prime number or it can't fit in a `usize`.
    #[inline]
    #[cfg(feature = "std")]
    pub fn new(value: u64) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime64(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    /// Returns the `nth` prime number.
    ///
    /// # Errors
    /// If `nth` is 0 or greater than *415,828,534,307,635,072* (a rough approximation).
    #[inline]
    #[allow(clippy::missing_panics_doc)] // NonZeroUsize can't be zero
    pub fn new_nth(nth: u64) -> NumeraResult<Self> {
        match nth {
            #[allow(clippy::cast_possible_truncation)]
            1..=54 => Ok(Self(u64::from(PRIMES_U8[(nth as u8 - 1) as usize]))),
            #[allow(clippy::cast_possible_truncation)]
            55..=6_542 => Ok(Self(u64::from(PRIMES_U16[(nth as u16 + 54) as usize]))),
            6_543..=203_280_219 => {
                #[cfg(feature = "std")]
                {
                    use core::num::NonZeroUsize;

                    #[allow(clippy::cast_possible_truncation)]
                    return Ok(Self(
                        nth_prime_sieve(NonZeroUsize::new(nth as usize).unwrap()) as u64,
                    ));
                }

                #[cfg(not(feature = "std"))]
                {
                    todo![]
                    // use core::num::NonZeroU32;
                    // Ok(Self(nth_prime(NonZeroU32::new(nth.try_into().unwrap()).unwrap())))
                }
            }
            203_280_223..=415_828_534_307_635_072 => {
                todo![] // TODO
            }
            _ => Err(NumeraError::Integer(IntegerError::Overflow)),
        }
    }

    // NOTE: the next 2 functions are mostly a duplication, because calling
    // `is_prime` doesn't allow the function to be `const`.

    /// Returns a new `Prime64`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    /// # Panics
    /// Panics in debug if the given `value` is not a prime number, or if it
    /// can't fit in a `usize`.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe"), feature = "std"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        debug_assert![is_prime_sieve(value.checked_as::<usize>().unwrap())];
        Self(value)
    }
    /// Returns a new `Prime64`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub const unsafe fn new_unchecked(value: u64) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// Note that this operation can be impossibly slow for big 64-bit numbers,
    ///
    /// # Notation
    /// $\pi(x)$
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Number, Prime64};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime64::new(2)?.pi()];
    /// assert_eq![54, Prime64::new(251)?.pi()];
    /// assert_eq![55, Prime64::new(257)?.pi()];
    /// assert_eq![6_542, Prime64::new(65_521)?.pi()];
    /// assert_eq![6_543, Prime64::new(65_537)?.pi()];
    /// assert_eq![40_000_000, Prime64::new(776_531_401)?.pi()];
    /// # Ok(()) }
    /// ```
    ///
    /// # Links
    /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
    /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn pi(&self) -> usize {
        if self.0 < u64::from(u8::MAX) {
            for (i, &p) in PRIMES_U8.iter().enumerate() {
                if u64::from(p) == self.0 {
                    return i + 1;
                }
            }
        } else if self.0 < u64::from(u16::MAX) {
            for (i, &p) in PRIMES_U16.iter().enumerate() {
                if u64::from(p) == self.0 {
                    return i + 55;
                }
            }
        } else {
            // this can be slow for high 64-bit numbers:
            #[cfg(feature = "std")]
            {
                let value = usize::try_from(self.0).expect("usize overflow");
                let sieve = Sieve::new(value);
                return sieve.prime_pi(value);
            }
            // this can be VERY slow for high 64-bit numbers:
            #[cfg(not(feature = "std"))]
            {
                nth_prime(core::num::NonZeroU64::new(self.0).unwrap())
            }
        }
        return 415_828_534_307_635_072; // a rough approximation
    }
}

impl Prime128 {
    /// Returns a new `Prime128`.
    ///
    /// This can be impossibly slow for big numbers.
    ///
    /// # Errors
    /// If the given `value` is not a prime number or it can't fit in a `usize`.
    #[inline]
    #[cfg(feature = "std")]
    pub fn new(value: u128) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime128(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    /// Returns the `nth` prime number.
    ///
    /// # Errors
    /// If `nth` is 0 or greater than *415,828,534,307,635,072* (a rough approximation).
    #[inline]
    // NonZeroUsize can't be zero
    #[allow(clippy::missing_panics_doc, clippy::match_overlapping_arm)]
    pub fn new_nth(nth: u128) -> NumeraResult<Self> {
        match nth {
            #[allow(clippy::cast_possible_truncation)]
            1..=54 => Ok(Self(u128::from(PRIMES_U8[(nth as u8 - 1) as usize]))),
            #[allow(clippy::cast_possible_truncation)]
            55..=6_542 => Ok(Self(u128::from(PRIMES_U16[(nth as u16 + 54) as usize]))),
            6_543..=203_280_219 => {
                #[cfg(feature = "std")]
                {
                    use core::num::NonZeroUsize;

                    #[allow(clippy::cast_possible_truncation)]
                    return Ok(Self(
                        nth_prime_sieve(NonZeroUsize::new(nth as usize).unwrap()) as u128,
                    ));
                }

                #[cfg(not(feature = "std"))]
                {
                    todo![]
                    // use core::num::NonZeroU32;
                    // Ok(Self(nth_prime(NonZeroU32::new(nth.try_into().unwrap()).unwrap())))
                }
            }
            // a rough approximation
            203_280_223..=415_828_534_307_635_072 => {
                todo![] // TODO
            }

            // a rough approximation
            #[allow(overlapping_range_endpoints)]
            415_828_534_307_635_072..=3_835_341_275_459_348_115_779_911_081_237_938_176 => {
                todo![] // TODO
            }
            _ => Err(NumeraError::Integer(IntegerError::Overflow)),
        }
    }

    // NOTE: the next 2 functions are mostly a duplication, because calling
    // `is_prime` doesn't allow the function to be `const`.

    /// Returns a new `Prime128`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    /// # Panics
    /// Panics in debug if the given `value` is not a prime number, or if it
    /// can't fit in a `usize`.
    #[inline]
    #[cfg(all(debug_assertions, not(feature = "safe"), feature = "std"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn new_unchecked(value: u128) -> Self {
        debug_assert![is_prime_sieve(value.checked_as::<usize>().unwrap())];
        Self(value)
    }
    /// Returns a new `Prime128`.
    ///
    /// # Safety
    /// The given `value` must be a prime number.
    #[inline]
    #[cfg(all(not(debug_assertions), not(feature = "safe")))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub const unsafe fn new_unchecked(value: u128) -> Self {
        Self(value)
    }

    /// Returns the number of primes upto and including the current one.
    ///
    /// Note that this operation can be impossibly slow for big 128-bit numbers,
    ///
    /// # Notation
    /// $\pi(x)$
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Number, Prime128};
    ///
    /// # fn main() -> numera::error::NumeraResult<()> {
    /// assert_eq![1, Prime128::new(2)?.pi()];
    /// assert_eq![54, Prime128::new(251)?.pi()];
    /// assert_eq![55, Prime128::new(257)?.pi()];
    /// assert_eq![6_542, Prime128::new(65_521)?.pi()];
    /// assert_eq![6_543, Prime128::new(65_537)?.pi()];
    /// assert_eq![40_000_000, Prime128::new(776_531_401)?.pi()];
    /// # Ok(()) }
    /// ```
    ///
    /// # Links
    /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
    /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn pi(&self) -> usize {
        if self.0 < u128::from(u8::MAX) {
            for (i, &p) in PRIMES_U8.iter().enumerate() {
                if u128::from(p) == self.0 {
                    return i + 1;
                }
            }
        } else if self.0 < u128::from(u16::MAX) {
            for (i, &p) in PRIMES_U16.iter().enumerate() {
                if u128::from(p) == self.0 {
                    return i + 55;
                }
            }
        } else {
            // this can be slow for high 128-bit numbers:
            #[cfg(feature = "std")]
            {
                let sieve = Sieve::new(self.0 as usize);
                return sieve.prime_pi(self.0 as usize);
            }
            // this can be VERY slow for high 128-bit numbers:
            #[cfg(not(feature = "std"))]
            {
                nth_prime(core::num::NonZeroU128::new(self.0).unwrap()) // FIX
            }
        }
        // TEMP
        return usize::MAX;
        // FIX:
        // return 3_835_341_275_459_348_115_779_911_081_237_938_176; // a rough approximation
    }
}
