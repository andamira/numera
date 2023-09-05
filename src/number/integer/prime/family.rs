// numera::number::integer::prime::family
//
//!
//

use super::{Prime128, Prime16, Prime32, Prime64, Prime8};
use crate::all::{
    Bound, ConstLowerBounded, ConstUpperBounded, Count, Countable, Ident, LowerBounded,
    NonNegative, NonOne, NonZero, Number, NumeraResult, Positive, Sign, UpperBounded,
};

/// The family of [prime][super] numbers, also known as [`P`][super::P].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Primes {
    _8(Prime8),
    _16(Prime16),
    _32(Prime32),
    _64(Prime64),
    _128(Prime128),
    // Big(PrimeBig),
}

/* Number impl */

/// This implementation is no-op.
impl Number for Primes {
    type InnerRepr = Self;
    type InnermostRepr = Self;

    /// Returns `value` unchanged.
    #[inline]
    fn from_inner_repr(value: Primes) -> NumeraResult<Self> {
        Ok(value)
    }
    /// Returns `value` unchanged.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
    unsafe fn from_inner_repr_unchecked(value: Primes) -> Self {
        value
    }

    /// Returns `value` unchanged.
    #[inline]
    fn from_innermost_repr(value: Primes) -> NumeraResult<Self> {
        Ok(value)
    }
    /// Returns `value` unchanged.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
    unsafe fn from_innermost_repr_unchecked(value: Primes) -> Self {
        value
    }

    /// Returns `self`.
    #[inline]
    fn into_inner_repr(self) -> Self {
        self
    }
    /// Returns `self`.
    #[inline]
    fn into_innermost_repr(self) -> Self {
        self
    }
}

/// This implementation defers to the actual integer variant.
impl Bound for Primes {
    fn is_lower_bounded(&self) -> bool {
        match self {
            Primes::_8(p) => p.is_lower_bounded(),
            Primes::_16(p) => p.is_lower_bounded(),
            Primes::_32(p) => p.is_lower_bounded(),
            Primes::_64(p) => p.is_lower_bounded(),
            Primes::_128(p) => p.is_lower_bounded(),
        }
    }
    fn is_upper_bounded(&self) -> bool {
        match self {
            Primes::_8(p) => p.is_upper_bounded(),
            Primes::_16(p) => p.is_upper_bounded(),
            Primes::_32(p) => p.is_upper_bounded(),
            Primes::_64(p) => p.is_upper_bounded(),
            Primes::_128(p) => p.is_upper_bounded(),
        }
    }
    fn lower_bound(&self) -> Option<Self> {
        match self {
            Primes::_8(p) => p.lower_bound().map(|p| p.into()),
            Primes::_16(p) => p.lower_bound().map(|p| p.into()),
            Primes::_32(p) => p.lower_bound().map(|p| p.into()),
            Primes::_64(p) => p.lower_bound().map(|p| p.into()),
            Primes::_128(p) => p.lower_bound().map(|p| p.into()),
        }
    }
    fn upper_bound(&self) -> Option<Self> {
        match self {
            Primes::_8(p) => p.upper_bound().map(|p| p.into()),
            Primes::_16(p) => p.upper_bound().map(|p| p.into()),
            Primes::_32(p) => p.upper_bound().map(|p| p.into()),
            Primes::_64(p) => p.upper_bound().map(|p| p.into()),
            Primes::_128(p) => p.upper_bound().map(|p| p.into()),
        }
    }
}

impl LowerBounded for Primes {
    /// Returns a [`Prime8::new_min()`][Prime8#method.new_min].
    #[inline]
    fn new_min() -> Primes {
        Primes::_8(Prime8::new_min())
    }
}
impl ConstLowerBounded for Primes {
    const MIN: Self = Primes::_8(Prime8::MIN);
}
impl UpperBounded for Primes {
    /// Returns a [`Prime128::new_max()`][Prime128#method.new_max].
    #[inline]
    fn new_max() -> Primes {
        Primes::_128(Prime128::new_max())
    }
}
impl ConstUpperBounded for Primes {
    const MAX: Self = Primes::_32(Prime32::MAX);
}

impl Count for Primes {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}
impl Countable for Primes {
    fn next(&self) -> NumeraResult<Self> {
        match self {
            Primes::_8(p) => p.next().map(|p| p.into()),
            Primes::_16(p) => p.next().map(|p| p.into()),
            #[cfg(feature = "std")]
            Primes::_32(p) => p.next().map(|p| p.into()),
            // IMPROVE for bigger sized
            // IMPROVE for no-std
            // #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraErrors::NotImplemented),
        }
    }
    fn previous(&self) -> NumeraResult<Self> {
        match self {
            // Prime8(p) => Prime8(p.),
            Primes::_8(p) => p.previous().map(|p| p.into()),
            Primes::_16(p) => p.previous().map(|p| p.into()),
            #[cfg(feature = "std")]
            Primes::_32(p) => p.previous().map(|p| p.into()),
            // IMPROVE for bigger sized
            // IMPROVE for no-std
            // #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraErrors::NotImplemented),
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
