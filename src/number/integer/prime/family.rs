// numera::number::integer::prime::family
//
//!
//

use super::{Prime128, Prime16, Prime32, Prime64, Prime8};
use crate::all::{
    Bound, ConstLowerBounded, ConstUpperBounded, Count, Countable, Ident, LowerBounded,
    NonNegative, NonOne, NonZero, Number, NumeraResult, Positive, Sign, UpperBounded,
};

/// The family of primes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Primes {
    Prime8(Prime8),
    Prime16(Prime16),
    Prime32(Prime32),
    Prime64(Prime64),
    Prime128(Prime128),
    // PrimeBig(PrimeBig),
}

/* number impl */

/// This implementation is no-op.
impl Number for Primes {
    type Parts = Self;

    /// Returns `value` unchanged.
    #[inline]
    fn from_parts(value: Primes) -> NumeraResult<Self> {
        Ok(value)
    }

    /// Returns `value` unchanged.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Primes) -> Self {
        value
    }
}

/// This implementation defers to the actual integer variant.
impl Bound for Primes {
    fn is_lower_bounded(&self) -> bool {
        use Primes::*;
        match self {
            Prime8(p) => p.is_lower_bounded(),
            Prime16(p) => p.is_lower_bounded(),
            Prime32(p) => p.is_lower_bounded(),
            Prime64(p) => p.is_lower_bounded(),
            Prime128(p) => p.is_lower_bounded(),
        }
    }
    fn is_upper_bounded(&self) -> bool {
        use Primes::*;
        match self {
            Prime8(p) => p.is_upper_bounded(),
            Prime16(p) => p.is_upper_bounded(),
            Prime32(p) => p.is_upper_bounded(),
            Prime64(p) => p.is_upper_bounded(),
            Prime128(p) => p.is_upper_bounded(),
        }
    }
    fn lower_bound(&self) -> Option<Self> {
        use Primes::*;
        match self {
            Prime8(p) => p.lower_bound().map(|p| p.into()),
            Prime16(p) => p.lower_bound().map(|p| p.into()),
            Prime32(p) => p.lower_bound().map(|p| p.into()),
            Prime64(p) => p.lower_bound().map(|p| p.into()),
            Prime128(p) => p.lower_bound().map(|p| p.into()),
        }
    }
    fn upper_bound(&self) -> Option<Self> {
        use Primes::*;
        match self {
            Prime8(p) => p.upper_bound().map(|p| p.into()),
            Prime16(p) => p.upper_bound().map(|p| p.into()),
            Prime32(p) => p.upper_bound().map(|p| p.into()),
            Prime64(p) => p.upper_bound().map(|p| p.into()),
            Prime128(p) => p.upper_bound().map(|p| p.into()),
        }
    }
}

impl LowerBounded for Primes {
    /// Returns a [`Prime8::new_min()`][Prime8#method.new_min].
    #[inline]
    fn new_min() -> Primes {
        Primes::Prime8(Prime8::new_min())
    }
}
impl ConstLowerBounded for Primes {
    const MIN: Self = Primes::Prime8(Prime8::MIN);
}
impl UpperBounded for Primes {
    /// Returns a [`Prime128::new_max()`][Prime128#method.new_max].
    #[inline]
    fn new_max() -> Primes {
        Primes::Prime128(Prime128::new_max())
    }
}
impl ConstUpperBounded for Primes {
    const MAX: Self = Primes::Prime32(Prime32::MAX);
}

impl Count for Primes {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}
impl Countable for Primes {
    fn next(&self) -> NumeraResult<Self> {
        use Primes::*;
        match self {
            // Prime8(p) => Prime8(p.),
            Prime8(p) => p.next().map(|p| p.into()),
            Prime16(p) => p.next().map(|p| p.into()),
            #[cfg(feature = "std")]
            Prime32(p) => p.next().map(|p| p.into()),
            // IMPROVE for bigger sized
            // IMPROVE for no-std
            // #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraError::NotImplemented),
        }
    }
    fn previous(&self) -> NumeraResult<Self> {
        use Primes::*;
        match self {
            // Prime8(p) => Prime8(p.),
            Prime8(p) => p.previous().map(|p| p.into()),
            Prime16(p) => p.previous().map(|p| p.into()),
            #[cfg(feature = "std")]
            Prime32(p) => p.previous().map(|p| p.into()),
            // IMPROVE for bigger sized
            // IMPROVE for no-std
            // #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraError::NotImplemented),
        }
    }
}

#[rustfmt::skip]
impl Ident for Primes {
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
impl NonZero for Primes {}
impl NonOne for Primes {}

#[rustfmt::skip]
impl Sign for Primes {
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
impl Positive for Primes {}
impl NonNegative for Primes {}
