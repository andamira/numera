// numera::number::integer::z::define_big
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
    error::NumeraResult,
    number::traits::{
        Bound, Count, Countable, Ident, NegOne, NonLowerBounded, NonUpperBounded, Number, One,
        Sign, Signed, Zero,
    },
};
use ibig::IBig;

/* definition */

/// A big integer number, from the set $\\Z$.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerBig(pub(crate) IBig);

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
impl Signed for IntegerBig {}

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

/* number */

impl Number for IntegerBig {
    type Inner = IBig;

    #[inline]
    fn new(value: Self::Inner) -> NumeraResult<Self> {
        Ok(Self(value))
    }

    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
}
