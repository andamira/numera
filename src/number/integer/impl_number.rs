// numera::number::integer::impl_number
//
//!
//

use super::{Integer, Negative, NonNegative, NonPositive, Positive};
use crate::number::traits::{InnerNumber, Number};
use core::ops::Neg;

#[rustfmt::skip]
impl<I: InnerNumber> Number for Integer<I> {
    type Inner = I;
    /// Returns a new `Integer`.
    #[inline]
    fn new(value: Self::Inner) -> Self { Self(value) }
    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { self.0.is_negative() }
    #[inline]
    fn can_positive() -> bool { true }
    #[inline]
    fn is_positive(&self) -> bool { self.0.is_positive() }
    #[inline]
    fn can_zero() -> bool { true }
    #[inline]
    fn is_zero(&self) -> bool { self.0.is_zero() }
    #[inline]
    fn can_one() -> bool { true }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
}

#[rustfmt::skip]
impl<I: InnerNumber> Number for NonNegative<I> {
    type Inner = I;
    /// Returns a new *non-negative* `Integer`.
    /// The smallest value saturates to `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        Self(if value < Self::Inner::new_zero() {
            Self::Inner::new_zero()
        } else {
            value
        })
    }
    #[inline]
    fn can_negative() -> bool { false }
    #[inline]
    fn is_negative(&self) -> bool { false }
    #[inline]
    fn can_positive() -> bool { true }
    #[inline]
    fn is_positive(&self) -> bool { self.0.is_positive() }
    #[inline]
    fn can_zero() -> bool { true }
    #[inline]
    fn is_zero(&self) -> bool { self.0.is_zero() }
    #[inline]
    fn can_one() -> bool { true }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
}

#[rustfmt::skip]
impl<I: InnerNumber> Number for Positive<I> {
    type Inner = I;
    /// Returns a new *positive* `Integer`.
    /// The smallest value saturates to `1`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        Self(if value < Self::Inner::new_one() {
            Self::Inner::new_one()
        } else {
            value
        })
    }
    #[inline]
    fn can_negative() -> bool { false }
    #[inline]
    fn is_negative(&self) -> bool { false }
    #[inline]
    fn can_positive() -> bool { true }
    #[inline]
    fn is_positive(&self) -> bool { self.0.is_positive() }
    #[inline]
    fn can_zero() -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn can_one() -> bool { true }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
}

#[rustfmt::skip]
impl<I: InnerNumber> Number for NonPositive<I> {
    type Inner = I;
    /// Returns a new *non-positive* `Integer`.
    /// The largest value Saturates to `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        Self(if value > Self::Inner::new_zero() {
            Self::Inner::new_zero()
        } else {
            value
        })
    }
    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { self.0.is_negative() }
    #[inline]
    fn can_positive() -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { false }
    #[inline]
    fn can_zero() -> bool { true }
    #[inline]
    fn is_zero(&self) -> bool { self.0.is_zero() }
    #[inline]
    fn can_one() -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
}

#[rustfmt::skip]
impl<I: InnerNumber + Neg<Output = I>> Number for Negative<I> {
    type Inner = I;
    /// Returns a new *negative* `Integer`.
    /// The largest value saturates to `-1`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        Self(if value >= Self::Inner::new_zero() {
            Self::Inner::new_one().neg()
        } else {
            value
        })
    }
    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { true }
    #[inline]
    fn can_positive() -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { false }
    #[inline]
    fn can_zero() -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn can_one() -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
}
