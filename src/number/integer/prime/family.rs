// numera::number::integer::prime::family
//
//!
//

use super::{Prime128, Prime16, Prime32, Prime64, Prime8};
use crate::all::{
    Bound, ConstLowerBounded, ConstUpperBounded, Count, Countable, Ident, LowerBounded,
    NonNegative, NonOne, NonZero, Numbers, NumeraResult, Positive, Sign, UpperBounded,
};

/// The family of [prime][super] numbers, also known as [`P`][super::P].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prime {
    _8(Prime8),
    _16(Prime16),
    _32(Prime32),
    _64(Prime64),
    _128(Prime128),
    // Big(PrimeBig),
}

/* Numbers impl */

/// This implementation is no-op.
impl Numbers for Prime {
    type InnerRepr = Self;
    type InnermostRepr = Self;

    /// Returns `value` unchanged.
    #[inline]
    fn from_inner_repr(value: Prime) -> NumeraResult<Self> {
        Ok(value)
    }
    /// Returns `value` unchanged.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
    unsafe fn from_inner_repr_unchecked(value: Prime) -> Self {
        value
    }

    /// Returns `value` unchanged.
    #[inline]
    fn from_innermost_repr(value: Prime) -> NumeraResult<Self> {
        Ok(value)
    }
    /// Returns `value` unchanged.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
    unsafe fn from_innermost_repr_unchecked(value: Prime) -> Self {
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
impl Bound for Prime {
    fn is_lower_bounded(&self) -> bool {
        use Prime::*;
        match self {
            _8(p) => p.is_lower_bounded(),
            _16(p) => p.is_lower_bounded(),
            _32(p) => p.is_lower_bounded(),
            _64(p) => p.is_lower_bounded(),
            _128(p) => p.is_lower_bounded(),
        }
    }
    fn is_upper_bounded(&self) -> bool {
        use Prime::*;
        match self {
            _8(p) => p.is_upper_bounded(),
            _16(p) => p.is_upper_bounded(),
            _32(p) => p.is_upper_bounded(),
            _64(p) => p.is_upper_bounded(),
            _128(p) => p.is_upper_bounded(),
        }
    }
    fn lower_bound(&self) -> Option<Self> {
        use Prime::*;
        match self {
            _8(p) => p.lower_bound().map(|p| p.into()),
            _16(p) => p.lower_bound().map(|p| p.into()),
            _32(p) => p.lower_bound().map(|p| p.into()),
            _64(p) => p.lower_bound().map(|p| p.into()),
            _128(p) => p.lower_bound().map(|p| p.into()),
        }
    }
    fn upper_bound(&self) -> Option<Self> {
        use Prime::*;
        match self {
            _8(p) => p.upper_bound().map(|p| p.into()),
            _16(p) => p.upper_bound().map(|p| p.into()),
            _32(p) => p.upper_bound().map(|p| p.into()),
            _64(p) => p.upper_bound().map(|p| p.into()),
            _128(p) => p.upper_bound().map(|p| p.into()),
        }
    }
}

impl LowerBounded for Prime {
    /// Returns a [`Prime8::new_min()`][Prime8#method.new_min].
    #[inline]
    fn new_min() -> Prime {
        Prime::_8(Prime8::new_min())
    }
}
impl ConstLowerBounded for Prime {
    const MIN: Self = Prime::_8(Prime8::MIN);
}
impl UpperBounded for Prime {
    /// Returns a [`Prime128::new_max()`][Prime128#method.new_max].
    #[inline]
    fn new_max() -> Prime {
        Prime::_128(Prime128::new_max())
    }
}
impl ConstUpperBounded for Prime {
    const MAX: Self = Prime::_32(Prime32::MAX);
}

impl Count for Prime {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}
impl Countable for Prime {
    fn next(&self) -> NumeraResult<Self> {
        use Prime::*;
        match self {
            _8(p) => p.next().map(|p| p.into()),
            _16(p) => p.next().map(|p| p.into()),
            #[cfg(feature = "std")]
            _32(p) => p.next().map(|p| p.into()),
            // IMPROVE for bigger sized
            // IMPROVE for no-std
            // #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraError::NotImplemented),
        }
    }
    fn previous(&self) -> NumeraResult<Self> {
        use Prime::*;
        match self {
            // Prime8(p) => Prime8(p.),
            _8(p) => p.previous().map(|p| p.into()),
            _16(p) => p.previous().map(|p| p.into()),
            #[cfg(feature = "std")]
            _32(p) => p.previous().map(|p| p.into()),
            // IMPROVE for bigger sized
            // IMPROVE for no-std
            // #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraError::NotImplemented),
        }
    }
}

#[rustfmt::skip]
impl Ident for Prime {
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
impl NonZero for Prime {}
impl NonOne for Prime {}

#[rustfmt::skip]
impl Sign for Prime {
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
impl Positive for Prime {}
impl NonNegative for Prime {}
