//
//! Implementing `Number` for the unit type [`()`] in order to be able
//! to represent the absence of a number, where a number is expected.
//

use crate::{
    error::Result,
    number::traits::{Bound, Count, Ident, Number, Sign},
};

#[rustfmt::skip]
impl Number for () {
    type Inner = ();

    fn new(value: ()) -> Result<Self> { Ok(value) }
    unsafe fn new_unchecked(value: ()) -> Self { value }
    fn into_inner(self) -> Self::Inner { }
    fn ref_inner(&self) -> &Self::Inner { &() }
}
#[rustfmt::skip]
impl Bound for () {
    fn is_lower_bounded(&self) -> bool { true }
    fn is_upper_bounded(&self) -> bool { true }
    fn lower_bound(&self) -> Option<Self> where Self: Sized { Some(()) }
    fn upper_bound(&self) -> Option<Self> where Self: Sized { Some(()) }
}

#[rustfmt::skip]
impl Count for () {
    fn is_countable(&self) -> bool { false }
}

#[rustfmt::skip]
impl Ident for () {
    fn can_zero(&self) -> bool { false }
    fn can_one(&self) -> bool { false }
    fn can_neg_one(&self) -> bool { false }
    fn is_zero(&self) -> bool { false }
    fn is_one(&self) -> bool { false }
    fn is_neg_one(&self) -> bool { false }
}

#[rustfmt::skip]
impl Sign for () {
    fn can_positive(&self) -> bool { false }
    fn can_negative(&self) -> bool { false }
    fn is_positive(&self) -> bool { false }
    fn is_negative(&self) -> bool { false }
}
