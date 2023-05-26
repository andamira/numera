// numera::number::integer::prime::impl_traits
//
//!
//
// NOTE: chosen the is_prime for Prime8 & Prime16 because it's faster.

use super::{
    data::{PRIMES_U16, PRIMES_U8},
    is_prime, Prime128, Prime16, Prime32, Prime64, Prime8,
};
use crate::{
    error::{IntegerError, NumeraResult},
    number::traits::{
        Bound, ConstLowerBounded, ConstUpperBounded, Count, Countable, Ident, LowerBounded,
        NonNegOne, NonOne, NonZero, Number, Sign, Unsigned, UpperBounded,
    },
};

#[cfg(feature = "std")]
use {devela::az::CheckedAs, crate::all::is_prime_sieve, core::cmp::min, primal_sieve::Sieve};

/* Prime8 */

#[rustfmt::skip]
impl Bound for Prime8 {
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    fn is_upper_bounded(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Option<Self> { Some(Prime8::MIN) }
    #[inline]
    fn upper_bound(&self) -> Option<Self> { Some(Prime8::MAX) }
}
impl LowerBounded for Prime8 {
    #[inline]
    fn new_min() -> Self {
        Prime8::MIN
    }
}
impl UpperBounded for Prime8 {
    #[inline]
    fn new_max() -> Self {
        Prime8::MAX
    }
}
impl ConstLowerBounded for Prime8 {
    const MIN: Self = Prime8(2);
}
impl ConstUpperBounded for Prime8 {
    /// The largest 8-bit prime equals [`u8::MAX`] - 4.
    const MAX: Self = Prime8(251);
}

impl Count for Prime8 {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}

impl Countable for Prime8 {
    /// Returns the next prime.
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Countable, Number, Prime8};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime8::from_parts(2)?.next()?, Prime8::from_parts(3)?];
    /// assert_eq![Prime8::from_parts(241)?.next()?, Prime8::from_parts(251)?];
    /// assert![Prime8::from_parts(251)?.next().is_err()];
    /// # Ok(()) }
    /// ```
    #[inline]
    fn next(&self) -> NumeraResult<Self> {
        let nth = self.pi();
        match nth {
            54 => Err(IntegerError::Overflow.into()),
            _ => Ok(Prime8(PRIMES_U8[nth])),
        }
    }
    /// Returns the previous prime.
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Countable, Number, Prime8};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime8::from_parts(3)?.previous()?, Prime8::from_parts(2)?, ];
    /// assert_eq![Prime8::from_parts(251)?.previous()?, Prime8::from_parts(241)?];
    /// assert![Prime8::from_parts(2)?.previous().is_err()];
    /// # Ok(()) }
    /// ```
    #[inline]
    fn previous(&self) -> NumeraResult<Self> {
        let nth = self.pi();
        match nth {
            1 => Err(IntegerError::Underflow.into()),
            _ => Ok(Prime8(PRIMES_U8[nth - 2])),
        }
    }
}

#[rustfmt::skip]
impl Ident for Prime8 {
    #[inline]
    fn can_zero(&self) -> bool { false }
    #[inline]
    fn can_one(&self) -> bool { false }
    #[inline]
    fn can_neg_one(&self) -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}
impl NonZero for Prime8 {}
impl NonOne for Prime8 {}
impl NonNegOne for Prime8 {}

#[rustfmt::skip]
impl Sign for Prime8 {
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
impl Unsigned for Prime8 {}

impl Number for Prime8 {
    type Parts = u8;

    #[inline]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        if is_prime(value.into()) {
            Ok(Prime8(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
        Self(value)
    }
}

/* Prime16 */

#[rustfmt::skip]
impl Bound for Prime16 {
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    fn is_upper_bounded(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Option<Self> { Some(Prime16::MIN) }
    #[inline]
    fn upper_bound(&self) -> Option<Self> { Some(Prime16::MAX) }
}
impl LowerBounded for Prime16 {
    #[inline]
    fn new_min() -> Self {
        Prime16::MIN
    }
}
impl UpperBounded for Prime16 {
    #[inline]
    fn new_max() -> Self {
        Prime16::MAX
    }
}
impl ConstLowerBounded for Prime16 {
    const MIN: Self = Prime16(2);
}
impl ConstUpperBounded for Prime16 {
    /// The largest 16-bit prime equals [`u16::MAX`] - 14.
    const MAX: Self = Prime16(65_521);
}

impl Count for Prime16 {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}
impl Countable for Prime16 {
    /// Returns the next prime.
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Countable, Number, Prime16};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime16::from_parts(5)?.next()?, Prime16::from_parts(7)?];
    /// assert_eq![Prime16::from_parts(251)?.next()?, Prime16::from_parts(257)?];
    /// assert_eq![Prime16::from_parts(257)?.next()?, Prime16::from_parts(263)?];
    /// assert_eq![Prime16::from_parts(65_519)?.next()?, Prime16::from_parts(65_521)?];
    /// assert![Prime16::from_parts(65_521)?.next().is_err()];
    /// # Ok(()) }
    /// ```
    #[inline]
    fn next(&self) -> NumeraResult<Self> {
        let nth = self.pi();
        match nth {
            // can't be 0
            1..=53 => Ok(Prime16(u16::from(PRIMES_U8[nth]))),
            54..=6_541 => Ok(Prime16(PRIMES_U16[nth - 54])),
            // otherwise it can only be 6_542
            _ => Err(IntegerError::Overflow.into()),
        }

        // ALTERNATIVE:
        // if self.0 == 65_521 { Err(IntegerError::Overflow.into()) } else {
        //     let sieve = Sieve::new(min(self.0.saturating_add(1000), u16::MAX) as usize);
        //     let nth = sieve.prime_pi(self.0 as usize);
        //     let next_prime = sieve.nth_prime(nth + 1);
        //     Ok(Prime16(next_prime.try_into().unwrap()))
        // }
    }

    /// Returns the previous prime.
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Countable, Number, Prime16};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime16::from_parts(7)?.previous()?, Prime16::from_parts(5)?];
    /// assert_eq![Prime16::from_parts(251)?.previous()?, Prime16::from_parts(241)?];
    /// assert_eq![Prime16::from_parts(257)?.previous()?, Prime16::from_parts(251)?];
    /// assert_eq![Prime16::from_parts(65_521)?.previous()?, Prime16::from_parts(65_519)?];
    /// assert![Prime16::from_parts(2)?.previous().is_err()];
    /// # Ok(()) }
    /// ```
    #[inline]
    fn previous(&self) -> NumeraResult<Self> {
        let nth = self.pi();
        match nth {
            2..=55 => Ok(Prime16(u16::from(PRIMES_U8[nth - 2]))),
            56..=6_542 => Ok(Prime16(PRIMES_U16[nth - 54 - 2])),
            // otherwise it can only be 1
            _ => Err(IntegerError::Underflow.into()),
        }

        // ALTERNATIVE:
        // if self.0 == 2 { Err(IntegerError::Underflow.into()) } else {
        //     let sieve = Sieve::new(min(self.0.saturating_add(1000), u16::MAX) as usize);
        //     let nth = sieve.prime_pi(self.0 as usize);
        //     let prev_prime = sieve.nth_prime(nth - 1);
        //     Ok(Prime16(prev_prime.try_into().unwrap()))
        // }
    }
}

#[rustfmt::skip]
impl Ident for Prime16 {
    #[inline]
    fn can_zero(&self) -> bool { false }
    #[inline]
    fn can_one(&self) -> bool { false }
    #[inline]
    fn can_neg_one(&self) -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}
impl NonZero for Prime16 {}
impl NonOne for Prime16 {}
impl NonNegOne for Prime16 {}

#[rustfmt::skip]
impl Sign for Prime16 {
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
impl Unsigned for Prime16 {}

impl Number for Prime16 {
    type Parts = u16;

    #[inline]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        if is_prime(value.into()) {
            Ok(Prime16(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
        Self(value)
    }
}

/* Prime32 */

#[rustfmt::skip]
impl Bound for Prime32 {
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    fn is_upper_bounded(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Option<Self> { Some(Prime32::MIN) }
    #[inline]
    fn upper_bound(&self) -> Option<Self> { Some(Prime32::MAX) }
}
impl LowerBounded for Prime32 {
    #[inline]
    fn new_min() -> Self {
        Prime32::MIN
    }
}
impl UpperBounded for Prime32 {
    #[inline]
    fn new_max() -> Self {
        Prime32::MAX
    }
}
impl ConstLowerBounded for Prime32 {
    const MIN: Self = Prime32(2);
}
impl ConstUpperBounded for Prime32 {
    /// The largest 32-bit prime equals [`u32::MAX`] - 4.
    const MAX: Self = Prime32(4_294_967_291);
}

impl Count for Prime32 {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}

// TODO:IMPROVE for no-std
#[rustfmt::skip]
#[cfg(not(feature = "std"))]
impl Countable for Prime32 {
    /// Not implemented for no-std.
    fn next(&self) -> NumeraResult<Self> { Err(crate::all::NumeraError::NotImplemented) }
    /// Not implemented for no-std.
    fn previous(&self) -> NumeraResult<Self> { Err(crate::all::NumeraError::NotImplemented) }
}
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl Countable for Prime32 {
    /// Returns the next prime.
    ///
    /// Note that this operation can be slow for high values.
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Countable, Number, Prime32};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime32::from_parts(5)?.next()?, Prime32::from_parts(7)?];
    /// assert_eq![Prime32::from_parts(251)?.next()?, Prime32::from_parts(257)?];
    /// assert_eq![Prime32::from_parts(65_521)?.next()?, Prime32::from_parts(65_537)?];
    /// assert_eq![Prime32::from_parts(50_000_017)?.next()?, Prime32::from_parts(50_000_021)?];
    /// // assert![Prime32::from_parts(4_294_967_291)?.next().is_err()]; // SLOW
    /// # Ok(()) }
    /// ```
    #[inline]
    fn next(&self) -> NumeraResult<Self> {
        if self.0 == 4_294_967_291 {
            Err(IntegerError::Overflow.into())
        } else {
            let sieve = Sieve::new(min(self.0.saturating_add(1000), u32::MAX) as usize);
            let nth = sieve.prime_pi(self.0 as usize);
            let next_prime = sieve.nth_prime(nth + 1);
            Ok(Prime32(next_prime.try_into().unwrap()))
        }
    }
    /// Returns the previous prime.
    ///
    /// Note that this operation can be slow for high values.
    ///
    /// # Examples
    /// ```
    /// use numera::all::{Countable, Number, Prime32};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime32::from_parts(7)?.previous()?, Prime32::from_parts(5)?];
    /// assert_eq![Prime32::from_parts(257)?.previous()?, Prime32::from_parts(251)?];
    /// assert_eq![Prime32::from_parts(65_537)?.previous()?, Prime32::from_parts(65_521)?];
    /// assert_eq![Prime32::from_parts(50_000_021)?.previous()?, Prime32::from_parts(50_000_017)?];
    /// // assert![Prime32::from_parts(4_294_967_291)?.previous().is_err()]; // SLOW
    /// # Ok(()) }
    /// ```
    #[inline]
    fn previous(&self) -> NumeraResult<Self> {
        if self.0 == 2 {
            Err(IntegerError::Underflow.into())
        } else {
            let sieve = Sieve::new(min(self.0.saturating_add(1000), u32::MAX) as usize);
            let nth = sieve.prime_pi(self.0 as usize);
            let prev_prime = sieve.nth_prime(nth - 1);
            Ok(Prime32(prev_prime.try_into().unwrap()))
        }
    }
}

#[rustfmt::skip]
impl Ident for Prime32 {
    #[inline]
    fn can_zero(&self) -> bool { false }
    #[inline]
    fn can_one(&self) -> bool { false }
    #[inline]
    fn can_neg_one(&self) -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}
impl NonZero for Prime32 {}
impl NonOne for Prime32 {}
impl NonNegOne for Prime32 {}

#[rustfmt::skip]
impl Sign for Prime32 {
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
impl Unsigned for Prime32 {}

impl Number for Prime32 {
    type Parts = u32;
    #[inline]
    #[cfg(not(feature = "std"))]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        if is_prime(value) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    #[inline]
    #[cfg(feature = "std")]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
        Self(value)
    }
}

/* Prime64 */

#[rustfmt::skip]
impl Bound for Prime64 {
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    fn is_upper_bounded(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Option<Self> { Some(Prime64::MIN) }
    #[inline]
    fn upper_bound(&self) -> Option<Self> { Some(Prime64::MAX) }
}
impl LowerBounded for Prime64 {
    #[inline]
    fn new_min() -> Self {
        Prime64::MIN
    }
}
impl UpperBounded for Prime64 {
    #[inline]
    fn new_max() -> Self {
        Prime64::MAX
    }
}
impl ConstLowerBounded for Prime64 {
    const MIN: Self = Prime64(2);
}
impl ConstUpperBounded for Prime64 {
    /// The largest 64-bit prime equals [`u64::MAX`] - 58.
    const MAX: Self = Prime64(18_446_744_073_709_551_557);
}

impl Count for Prime64 {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}

// // TODO
// #[rustfmt::skip]
// #[cfg(not(feature = "std"))]
// impl Countable for Prime64 {
// }
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// impl Countable for Prime64 {
// }

#[rustfmt::skip]
impl Ident for Prime64 {
    #[inline]
    fn can_zero(&self) -> bool { false }
    #[inline]
    fn can_one(&self) -> bool { false }
    #[inline]
    fn can_neg_one(&self) -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}
impl NonZero for Prime64 {}
impl NonOne for Prime64 {}
impl NonNegOne for Prime64 {}

#[rustfmt::skip]
impl Sign for Prime64 {
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
impl Unsigned for Prime64 {}

impl Number for Prime64 {
    type Parts = u64;
    #[inline]
    #[cfg(not(feature = "std"))]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        // IMPROVE
        if is_prime(value.try_into()?) {
            Ok(Prime64(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    #[inline]
    #[cfg(feature = "std")]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime64(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
        Self(value)
    }
}

/* Prime128 */

#[rustfmt::skip]
impl Bound for Prime128 {
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    fn is_upper_bounded(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Option<Self> { Some(Prime128::MIN) }
    #[inline]
    fn upper_bound(&self) -> Option<Self> { Some(Prime128::MAX) }
}
impl LowerBounded for Prime128 {
    #[inline]
    fn new_min() -> Self {
        Prime128::MIN
    }
}
impl UpperBounded for Prime128 {
    #[inline]
    fn new_max() -> Self {
        Prime128::MAX
    }
}
impl ConstLowerBounded for Prime128 {
    const MIN: Self = Prime128(2);
}
impl ConstUpperBounded for Prime128 {
    /// The largest 128-bit prime equals [`u128::MAX`] - 158.
    const MAX: Self = Prime128(340_282_366_920_938_463_463_374_607_431_768_211_297);
}

impl Count for Prime128 {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}

// // TODO
// #[rustfmt::skip]
// #[cfg(not(feature = "std"))]
// impl Countable for Prime128 {
// }
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// impl Countable for Prime128 {
// }

#[rustfmt::skip]
impl Ident for Prime128 {
    #[inline]
    fn can_zero(&self) -> bool { false }
    #[inline]
    fn can_one(&self) -> bool { false }
    #[inline]
    fn can_neg_one(&self) -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}
impl NonZero for Prime128 {}
impl NonOne for Prime128 {}
impl NonNegOne for Prime128 {}

#[rustfmt::skip]
impl Sign for Prime128 {
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
impl Unsigned for Prime128 {}

impl Number for Prime128 {
    type Parts = u128;
    #[inline]
    #[cfg(not(feature = "std"))]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        // IMPROVE
        if is_prime(value.try_into()?) {
            Ok(Prime128(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    #[inline]
    #[cfg(feature = "std")]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime128(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }

    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
        Self(value)
    }
}
