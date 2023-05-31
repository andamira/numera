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

/// Represents the absence of a number.
///
/// This unit type alias implements every number trait, in order to be able to
/// be used wherever a generic number is expected.
pub type NoNumber = ();

/* Number */

#[rustfmt::skip]
impl Number for NoNumber {
    /// Itself.
    type Parts = NoNumber;
    /// Returns itself.
    #[inline]
    fn from_parts(value: NoNumber) -> NumeraResult<Self> { Ok(value) }
    /// Returns itself.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: NoNumber) -> Self { value }
    /// Returns itself.
    #[inline]
    fn try_from_parts(_value: impl Into<Self::Parts>) -> NumeraResult<Self> { Ok(()) }
}

/* Bound */

#[rustfmt::skip]
impl Bound for NoNumber {
    /// Lowerbounded by itself.
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    /// Upperbounded by itself.
    fn is_upper_bounded(&self) -> bool { true }
    /// Returns itself.
    #[inline]
    fn lower_bound(&self) -> Option<Self> { Some(()) }
    /// Returns itself.
    #[inline]
    fn upper_bound(&self) -> Option<Self> { Some(()) }
}
// these imply Bounded:
impl LowerBounded for NoNumber {
    /// Returns itself.
    #[inline]
    fn new_min() -> Self {}
}
impl UpperBounded for NoNumber {
    /// Returns itself.
    #[inline]
    fn new_max() -> Self {}
}
// these imply ConstBounded:
/// Returns itself.
impl ConstLowerBounded for NoNumber {
    /// Itself.
    const MIN: NoNumber = ();
}
/// Returns itself.
impl ConstUpperBounded for NoNumber {
    /// Itself.
    const MAX: NoNumber = ();
}
// these imply NonBounded:
impl NonLowerBounded for NoNumber {}
impl NonUpperBounded for NoNumber {}

/* Count */

#[rustfmt::skip]
impl Count for NoNumber {
    #[inline]
    fn is_countable(&self) -> bool { false }
}
#[rustfmt::skip]
impl Countable for NoNumber {
    /// Returns itself.
    #[inline]
    fn next(&self) -> NumeraResult<Self> { Ok(()) }
    /// Returns itself.
    #[inline]
    fn previous(&self) -> NumeraResult<Self> { Ok(()) }
}
impl Uncountable for NoNumber {}

/* Ident */

#[rustfmt::skip]
impl Ident for NoNumber {
    /// Returns `false`.
    #[inline]
    fn can_zero(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn can_one(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn can_neg_one(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn is_zero(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn is_one(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}
impl NonZero for NoNumber {}
/// Returns itself.
impl ConstOne for NoNumber {
    /// Itself.
    const ONE: NoNumber = ();
}
/// Returns itself.
impl ConstZero for NoNumber {
    /// Itself.
    const ZERO: NoNumber = ();
}
/// Returns itself.
impl ConstNegOne for NoNumber {
    /// Itself.
    const NEG_ONE: NoNumber = ();
}
impl One for NoNumber {
    /// Returns itself.
    #[inline]
    fn new_one() -> Self {}

    /// No op.
    #[inline]
    fn set_one(&mut self) {}
}
impl Zero for NoNumber {
    /// Returns itself.
    #[inline]
    fn new_zero() -> Self {}

    /// No op.
    #[inline]
    fn set_zero(&mut self) {}
}
impl NegOne for NoNumber {
    /// Returns itself.
    #[inline]
    fn new_neg_one() -> Self {}

    /// No op.
    #[inline]
    fn set_neg_one(&mut self) {}
}

/* Sign */

#[rustfmt::skip]
impl Sign for NoNumber {
    /// Returns `false`.
    #[inline]
    fn can_positive(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn can_negative(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn is_positive(&self) -> bool { false }
    /// Returns `false`.
    #[inline]
    fn is_negative(&self) -> bool { false }
}
// these imply Signed:
impl Positive for NoNumber {}
impl Negative for NoNumber {}
// these imply Unsigned, NonOne, NonNegOne:
impl NonNegative for NoNumber {}
#[rustfmt::skip]
impl NonPositive for NoNumber {
    /// Itself.
    type Parts = NoNumber;
    /// Returns itself.
    #[inline]
    fn new_neg(value: Self::Parts) -> NumeraResult<Self::Parts> { Ok(value) }
}
