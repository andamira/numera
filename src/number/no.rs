//
//!
//

use crate::{
    error::NumeraResult as Result,
    number::traits::{Bound, Count, Ident, Number, Sign},
};

/// This implementation allows to use the unit type for representing
/// the absence of a number, where a number is expected.
#[rustfmt::skip]
impl Number for () {
    type Parts = ();

    #[inline]
    fn from_parts(value: ()) -> Result<Self> { Ok(value) }

    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: ()) -> Self { value }
}
#[rustfmt::skip]
impl Bound for () {
    #[inline]
    fn is_lower_bounded(&self) -> bool { true }
    #[inline]
    fn is_upper_bounded(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Option<Self> where Self: Sized { Some(()) }
    #[inline]
    fn upper_bound(&self) -> Option<Self> where Self: Sized { Some(()) }
}

#[rustfmt::skip]
impl Count for () {
    #[inline]
    fn is_countable(&self) -> bool { false }
}

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
