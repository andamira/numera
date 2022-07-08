// numera::number::integer::impl_number
//
//! implements the `Number` trait for all integer types.
//

use super::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::number::traits::{NegOne, Number, NumberAble, One, Zero};

#[rustfmt::skip]
impl<I: NumberAble + Zero + One + NegOne> Number for Integer<I> {
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
    #[inline]
    fn can_neg_one() -> bool { true }
    #[inline]
    fn is_neg_one(&self) -> bool { self.0.is_neg_one() }
}

#[rustfmt::skip]
impl<I: NumberAble + One + NegOne> Number for NonZeroInteger<I> {
    type Inner = I;
    /// Returns a new `NonZeroInteger`.
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
    fn can_zero() -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn can_one() -> bool { true }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
    #[inline]
    fn can_neg_one() -> bool { true }
    #[inline]
    fn is_neg_one(&self) -> bool { self.0.is_neg_one() }
}

#[rustfmt::skip]
impl<I: NumberAble + Zero + One> Number for NonNegativeInteger<I> {
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
    #[inline]
    fn can_neg_one() -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}

#[rustfmt::skip]
impl<I: NumberAble + One> Number for PositiveInteger<I> {
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
    #[inline]
    fn can_neg_one() -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}

#[rustfmt::skip]
impl<I: NumberAble + Zero + NegOne> Number for NonPositiveInteger<I> {
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
    #[inline]
    fn can_neg_one() -> bool { true }
    #[inline]
    fn is_neg_one(&self) -> bool {self.0.is_neg_one() }
}

#[rustfmt::skip]
impl<I: NumberAble + NegOne + Zero> Number for NegativeInteger<I> {
    type Inner = I;
    /// Returns a new *negative* `Integer`.
    /// The largest value saturates to `-1`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        Self(if value >= Self::Inner::new_zero() {
            Self::Inner::new_neg_one()
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
    #[inline]
    fn can_neg_one() -> bool { true }
    #[inline]
    fn is_neg_one(&self) -> bool {self.0.is_neg_one() }
}
