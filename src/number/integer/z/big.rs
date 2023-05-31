// numera::number::integer::z::big
//
//!
//
// TOC
//
// - definition
// - impls
//   - Sign
//   - Bound
//   - Count
//   - Ident
//   - Number

use crate::{
    error::{NumeraError, NumeraResult},
    number::traits::{
        Bound, Count, Countable, Ident, NegOne, Negative, NonLowerBounded, NonUpperBounded, Numbers,
        One, Positive, Sign, Zero,
    },
};
use core::{fmt, str::FromStr};
use ibig::IBig;

/* definition */

/// A big integer number, from the set $\\Z$,
/// also known as [`ZBig`][super::ZBig].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerBig(pub IBig);

impl fmt::Display for IntegerBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for IntegerBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZBig({})", self.0)
    }
}

impl IntegerBig {
    /// Returns a new `IntegerBig`.
    #[inline]
    #[must_use]
    pub fn new(value: i128) -> IntegerBig {
        Self(IBig::from(value))
    }

    /// Returns a new `IntegerBig` from a string in base 10.
    ///
    /// # Errors
    /// If the number is unparseable.
    #[inline]
    pub fn from_string(value: &str) -> NumeraResult<IntegerBig> {
        Ok(Self(IBig::from_str_radix(value, 10)?))
    }

    /// Returns a new `IntegerBig` from a string in the given `base`,
    /// which must be between 2 and 36, inclusive.
    ///
    /// `value` may contain an optional `+` prefix.
    /// Digits 10-35 are represented by `a-z` or `A-Z`.
    ///
    ///
    /// # Panics
    /// If base is <2 or >36.
    // FIXME: make this an error.
    ///
    /// # Errors
    /// If the number is unparseable.
    #[inline]
    pub fn from_str_with_base(value: &str, base: u32) -> NumeraResult<IntegerBig> {
        Ok(Self(IBig::from_str_radix(value, base)?))
    }
}

impl FromStr for IntegerBig {
    type Err = NumeraError;

    fn from_str(s: &str) -> NumeraResult<IntegerBig> {
        Self::from_string(s)
    }
}

/* sign */

#[rustfmt::skip]
impl Sign for IntegerBig {
    #[inline]
    fn can_negative(&self) -> bool { true }
    #[inline]
    fn can_positive(&self) -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { self.0.is_negative() }
    #[inline]
    fn is_positive(&self) -> bool { self.0.is_positive() }
}
impl Positive for IntegerBig {}
impl Negative for IntegerBig {}

/* bound */

#[rustfmt::skip]
impl Bound for IntegerBig {
    #[inline]
    fn is_lower_bounded(&self) -> bool { false }
    #[inline]
    fn is_upper_bounded(&self) -> bool { false }
    #[inline]
    fn lower_bound(&self) -> Option<Self> { None }
    #[inline]
    fn upper_bound(&self) -> Option<Self> { None }
}
impl NonLowerBounded for IntegerBig {}
impl NonUpperBounded for IntegerBig {}

/* count */

impl Count for IntegerBig {
    #[inline]
    fn is_countable(&self) -> bool {
        true
    }
}

impl Countable for IntegerBig {
    #[inline]
    fn next(&self) -> NumeraResult<Self> {
        self.0.next().map(Self)
    }
    #[inline]
    fn previous(&self) -> NumeraResult<Self> {
        self.0.previous().map(Self)
    }
}

/* ident */

#[rustfmt::skip]
impl Ident for IntegerBig {
    #[inline]
    fn can_zero(&self) -> bool { true }
    #[inline]
    fn can_one(&self) -> bool { true }
    #[inline]
    fn can_neg_one(&self) -> bool { true }

    #[inline]
    fn is_zero(&self) -> bool { self.0.is_zero() }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
    #[inline]
    fn is_neg_one(&self) -> bool { self.0.is_neg_one() }
}
impl Zero for IntegerBig {
    #[inline]
    fn new_zero() -> Self {
        Self(IBig::new_zero())
    }
}
impl One for IntegerBig {
    #[inline]
    fn new_one() -> Self {
        Self(IBig::new_one())
    }
}
impl NegOne for IntegerBig {
    #[inline]
    fn new_neg_one() -> Self {
        Self(IBig::new_neg_one())
    }
}

/* Numbers */

impl Numbers for IntegerBig {
    type Parts = IBig;

    /// Returns a new `IntegerBig` from the constituent parts.
    ///
    /// # Errors
    /// This function can't fail.
    #[inline]
    fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
        Ok(Self(value))
    }

    /// Returns a new `IntegerBig` from the constituent parts.
    ///
    /// # Safety
    /// This function is safe.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
        Self(value)
    }
}
