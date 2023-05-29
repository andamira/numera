//
//! Not a number.
//
// TOC
//
// - Number
// - Bound
// - Count
// - Ident
// - Sign

use crate::{error::NumeraResult, number::traits::*};

/* Number */

/// This implementation allows to use the unit type for representing
/// the absence of a number, where a number is expected.
#[rustfmt::skip]
impl Number for () {
    type Parts = ();
    #[inline]
    fn from_parts(value: ()) -> NumeraResult<Self> { Ok(value) }
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: ()) -> Self { value }
}

/* Bound */

#[rustfmt::skip]
impl Bound for () {
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    fn is_upper_bounded(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Option<Self> { Some(()) }
    #[inline]
    fn upper_bound(&self) -> Option<Self> { Some(()) }
}
// these imply Bounded:
impl LowerBounded for () {
    #[inline]
    fn new_min() -> Self {}
}
impl UpperBounded for () {
    #[inline]
    fn new_max() -> Self {}
}
// these imply ConstBounded:
#[rustfmt::skip]
impl ConstLowerBounded for () { const MIN: Self = (); }
#[rustfmt::skip]
impl ConstUpperBounded for () { const MAX: Self = (); }
// these imply NonBounded:
impl NonLowerBounded for () {}
impl NonUpperBounded for () {}

/* Count */

#[rustfmt::skip]
impl Count for () {
    #[inline]
    fn is_countable(&self) -> bool { false }
}
#[rustfmt::skip]
impl Countable for () {
    #[inline]
    fn next(&self) -> NumeraResult<Self> { Ok(()) }
    #[inline]
    fn previous(&self) -> NumeraResult<Self> { Ok(()) }
}
impl Uncountable for () {}

/* Ident */

#[rustfmt::skip]
impl Ident for () {
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
impl NonZero for () {}
#[rustfmt::skip]
impl ConstOne for () { const ONE: Self = (); }
#[rustfmt::skip]
impl ConstZero for () { const ZERO: Self = (); }
#[rustfmt::skip]
impl ConstNegOne for () { const NEG_ONE: Self = (); }
impl One for () {
    #[inline]
    fn new_one() -> Self {}
}
impl Zero for () {
    #[inline]
    fn new_zero() -> Self {}
}
impl NegOne for () {
    #[inline]
    fn new_neg_one() -> Self {}
}

/* Sign */

#[rustfmt::skip]
impl Sign for () {
    #[inline]
    fn can_positive(&self) -> bool { false }
    #[inline]
    fn can_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { false }
    #[inline]
    fn is_negative(&self) -> bool { false }
}
// these imply Signed:
impl Positive for () {}
impl Negative for () {}
// these imply Unsigned, NonOne, NonNegOne:
impl NonNegative for () {}
#[rustfmt::skip]
impl NonPositive for () {
    type Parts = ();
    #[inline]
    fn new_neg(value: Self::Parts) -> NumeraResult<Self::Parts> { Ok(value) }
}
