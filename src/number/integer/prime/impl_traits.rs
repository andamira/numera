// numera::number::integer::prime::impl_traits
//
//!
//
// NOTE: chosen the is_prime_brute for Prime8 & Prime16 because it's faster.

use super::{is_prime_brute, Prime16, Prime32, Prime8, PRIMES_U16, PRIMES_U8};
use crate::{
    error::{IntegerError, NumeraResult},
    number::traits::{
        Bound, ConstLowerBounded, ConstUpperBounded, Count, Countable, Ident, LowerBounded,
        NonNegOne, NonOne, NonZero, Number, Sign, Unsigned, UpperBounded,
    },
};

#[cfg(feature = "std")]
use {super::Sieve, crate::all::is_prime_sieve, az::CheckedAs, core::cmp::min};

/* Prime8 */

impl Bound for Prime8 {
    fn is_lower_bounded(&self) -> bool {
        true
    }
    fn is_upper_bounded(&self) -> bool {
        true
    }
    fn lower_bound(&self) -> Option<Self> {
        Some(Prime8(2))
    }
    fn upper_bound(&self) -> Option<Self> {
        Some(Prime8(251))
    }
}
impl LowerBounded for Prime8 {
    fn new_min() -> Self {
        Prime8(2)
    }
}
impl UpperBounded for Prime8 {
    fn new_max() -> Self {
        Prime8(251)
    }
}
impl ConstLowerBounded for Prime8 {
    const MIN: Self = Prime8(2);
}
impl ConstUpperBounded for Prime8 {
    const MAX: Self = Prime8(251);
}

impl Count for Prime8 {
    fn is_countable(&self) -> bool {
        true
    }
}

impl Countable for Prime8 {
    /// Returns the next prime.
    ///
    /// # Examples
    /// ```
    /// use numera::number::{Number, integer::Prime8, traits::Countable};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime8::new(2)?.next()?, Prime8::new(3)?];
    /// assert_eq![Prime8::new(241)?.next()?, Prime8::new(251)?];
    /// assert![Prime8::new(251)?.next().is_err()];
    /// # Ok(()) }
    /// ```
    fn next(&self) -> NumeraResult<Self> {
        let nth = self.nth();
        match nth {
            54 => Err(IntegerError::Overflow.into()),
            _ => Ok(Prime8(PRIMES_U8[nth])),
        }
    }
    /// Returns the previous prime.
    ///
    /// # Examples
    /// ```
    /// use numera::number::{Number, integer::Prime8, traits::Countable};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime8::new(3)?.previous()?, Prime8::new(2)?, ];
    /// assert_eq![Prime8::new(251)?.previous()?, Prime8::new(241)?];
    /// assert![Prime8::new(2)?.previous().is_err()];
    /// # Ok(()) }
    /// ```
    fn previous(&self) -> NumeraResult<Self> {
        let nth = self.nth();
        match nth {
            1 => Err(IntegerError::Underflow.into()),
            _ => Ok(Prime8(PRIMES_U8[nth - 2])),
        }
    }
}

impl Ident for Prime8 {
    fn can_zero(&self) -> bool {
        false
    }
    fn can_one(&self) -> bool {
        false
    }
    fn can_neg_one(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_neg_one(&self) -> bool {
        false
    }
}
impl NonZero for Prime8 {}
impl NonOne for Prime8 {}
impl NonNegOne for Prime8 {}

impl Sign for Prime8 {
    fn can_positive(&self) -> bool {
        true
    }
    fn can_negative(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}
impl Unsigned for Prime8 {}

impl Number for Prime8 {
    type Inner = u8;
    fn new(value: Self::Inner) -> NumeraResult<Self> {
        if is_prime_brute(value.into()) {
            Ok(Prime8(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
}

/* Prime16 */

impl Bound for Prime16 {
    fn is_lower_bounded(&self) -> bool {
        true
    }
    fn is_upper_bounded(&self) -> bool {
        true
    }
    fn lower_bound(&self) -> Option<Self> {
        Some(Prime16(2))
    }
    fn upper_bound(&self) -> Option<Self> {
        Some(Prime16(65_521))
    }
}
impl LowerBounded for Prime16 {
    fn new_min() -> Self {
        Prime16(2)
    }
}
impl UpperBounded for Prime16 {
    fn new_max() -> Self {
        Prime16(65_512)
    }
}
impl ConstLowerBounded for Prime16 {
    const MIN: Self = Prime16(2);
}
impl ConstUpperBounded for Prime16 {
    const MAX: Self = Prime16(65_521);
}

impl Count for Prime16 {
    fn is_countable(&self) -> bool {
        true
    }
}
impl Countable for Prime16 {
    /// Returns the next prime.
    ///
    /// # Examples
    /// ```
    /// use numera::number::{Number, integer::Prime16, traits::Countable};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime16::new(5)?.next()?, Prime16::new(7)?];
    /// assert_eq![Prime16::new(251)?.next()?, Prime16::new(257)?];
    /// assert_eq![Prime16::new(257)?.next()?, Prime16::new(263)?];
    /// assert_eq![Prime16::new(65_519)?.next()?, Prime16::new(65_521)?];
    /// assert![Prime16::new(65_521)?.next().is_err()];
    /// # Ok(()) }
    /// ```
    fn next(&self) -> NumeraResult<Self> {
        let nth = self.nth();
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
    /// use numera::number::{Number, integer::Prime16, traits::Countable};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime16::new(7)?.previous()?, Prime16::new(5)?];
    /// assert_eq![Prime16::new(251)?.previous()?, Prime16::new(241)?];
    /// assert_eq![Prime16::new(257)?.previous()?, Prime16::new(251)?];
    /// assert_eq![Prime16::new(65_521)?.previous()?, Prime16::new(65_519)?];
    /// assert![Prime16::new(2)?.previous().is_err()];
    /// # Ok(()) }
    /// ```
    fn previous(&self) -> NumeraResult<Self> {
        let nth = self.nth();
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

impl Ident for Prime16 {
    fn can_zero(&self) -> bool {
        false
    }
    fn can_one(&self) -> bool {
        false
    }
    fn can_neg_one(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_neg_one(&self) -> bool {
        false
    }
}
impl NonZero for Prime16 {}
impl NonOne for Prime16 {}
impl NonNegOne for Prime16 {}

impl Sign for Prime16 {
    fn can_positive(&self) -> bool {
        true
    }
    fn can_negative(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}
impl Unsigned for Prime16 {}

impl Number for Prime16 {
    type Inner = u16;
    fn new(value: Self::Inner) -> NumeraResult<Self> {
        if is_prime_brute(value.into()) {
            Ok(Prime16(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
}

/* Prime32 */

impl Bound for Prime32 {
    fn is_lower_bounded(&self) -> bool {
        true
    }
    fn is_upper_bounded(&self) -> bool {
        true
    }
    fn lower_bound(&self) -> Option<Self> {
        Some(Prime32(2))
    }
    fn upper_bound(&self) -> Option<Self> {
        Some(Prime32(4_294_967_291))
    }
}
impl LowerBounded for Prime32 {
    fn new_min() -> Self {
        Prime32(2)
    }
}
impl UpperBounded for Prime32 {
    fn new_max() -> Self {
        Prime32(4_294_967_291)
    }
}
impl ConstLowerBounded for Prime32 {
    const MIN: Self = Prime32(2);
}
impl ConstUpperBounded for Prime32 {
    const MAX: Self = Prime32(4_294_967_291);
}

impl Count for Prime32 {
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
    /// use numera::number::{Number, integer::Prime32, traits::Countable};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime32::new(5)?.next()?, Prime32::new(7)?];
    /// assert_eq![Prime32::new(251)?.next()?, Prime32::new(257)?];
    /// assert_eq![Prime32::new(65_521)?.next()?, Prime32::new(65_537)?];
    /// assert_eq![Prime32::new(50_000_017)?.next()?, Prime32::new(50_000_021)?];
    /// // assert![Prime32::new(4_294_967_291)?.next().is_err()]; // SLOW
    /// # Ok(()) }
    /// ```
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
    /// use numera::number::{Number, integer::Prime32, traits::Countable};
    /// # use numera::error::NumeraResult;
    /// # fn main() -> NumeraResult<()> {
    /// assert_eq![Prime32::new(7)?.previous()?, Prime32::new(5)?];
    /// assert_eq![Prime32::new(257)?.previous()?, Prime32::new(251)?];
    /// assert_eq![Prime32::new(65_537)?.previous()?, Prime32::new(65_521)?];
    /// assert_eq![Prime32::new(50_000_021)?.previous()?, Prime32::new(50_000_017)?];
    /// // assert![Prime32::new(4_294_967_291)?.previous().is_err()]; // SLOW
    /// # Ok(()) }
    /// ```
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

impl Ident for Prime32 {
    fn can_zero(&self) -> bool {
        false
    }
    fn can_one(&self) -> bool {
        false
    }
    fn can_neg_one(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_neg_one(&self) -> bool {
        false
    }
}
impl NonZero for Prime32 {}
impl NonOne for Prime32 {}
impl NonNegOne for Prime32 {}

impl Sign for Prime32 {
    fn can_positive(&self) -> bool {
        true
    }
    fn can_negative(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}
impl Unsigned for Prime32 {}

impl Number for Prime32 {
    type Inner = u32;
    #[cfg(not(feature = "std"))]
    fn new(value: Self::Inner) -> NumeraResult<Self> {
        if is_prime_brute(value) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    #[cfg(feature = "std")]
    fn new(value: Self::Inner) -> NumeraResult<Self> {
        if is_prime_sieve(value.checked_as::<usize>().ok_or(IntegerError::Overflow)?) {
            Ok(Prime32(value))
        } else {
            Err(IntegerError::NotPrime.into())
        }
    }
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
}
