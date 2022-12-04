//
//! The absence of a number.
//

use crate::{
    error::Result,
    number::traits::{Bound, Count, Ident, Number, Sign},
};

/// A zero-sized type that represents the absence of a number.
///
/// It is used together with [`AnyNumbers`][super::AnyNumbers] in
/// [`Numbers`][super::Numbers] for convenience.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoNumber;

#[rustfmt::skip]
impl Number for NoNumber {
    type Inner = NoNumber;

    fn new(value: NoNumber) -> Result<Self> { Ok(value) }
    unsafe fn new_unchecked(value: NoNumber) -> Self { value }
    fn into_inner(self) -> Self::Inner { NoNumber {} }
    fn ref_inner(&self) -> &Self::Inner { &NoNumber {} }
}
#[rustfmt::skip]
impl Bound for NoNumber {
    fn is_lower_bounded(&self) -> bool { true }
    fn is_upper_bounded(&self) -> bool { true }
    fn lower_bound(&self) -> Option<Self> where Self: Sized { Some(NoNumber) }
    fn upper_bound(&self) -> Option<Self> where Self: Sized { Some(NoNumber) }
}
// RATIONALE:
// If we would regard NoNumber equivalent to the empty set, we would implement
// Countable, returning itself. That's it, counted in terms of itself, which...
// is not a number.
// Otherwise we could say NoNumber is neither countable nor uncountable. This
// is I think the simplest solution.
#[rustfmt::skip]
impl Count for NoNumber {
    fn is_countable(&self) -> bool { false }
}
// impl Uncountable for NoNumber {}
// impl Countable for NoNumber {
//     fn next(&self) -> Result<Self> { Ok(NoNumber) }
//     fn previous(&self) -> Result<Self> { Ok(NoNumber) }
// }
#[rustfmt::skip]
impl Ident for NoNumber {
    fn can_zero(&self) -> bool { false }
    fn can_one(&self) -> bool { false }
    fn can_neg_one(&self) -> bool { false }
    fn is_zero(&self) -> bool { false }
    fn is_one(&self) -> bool { false }
    fn is_neg_one(&self) -> bool { false }
}

#[rustfmt::skip]
impl Sign for NoNumber {
    fn can_positive(&self) -> bool { false }
    fn can_negative(&self) -> bool { false }
    fn is_positive(&self) -> bool { false }
    fn is_negative(&self) -> bool { false }
}
