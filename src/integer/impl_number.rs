// numera::integer::impl_number
//
//! implements the `Number` trait for all integer types.
//

use super::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::Number;

#[rustfmt::skip]
impl<N: Number> Number for Integer<N> {
    type Inner = N;
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
impl<I: Number> Number for NonZeroInteger<I> {
    type Inner = I;
    /// Returns a new `NonZeroInteger`.
    ///
    /// Panics if `value` == `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![!value.is_zero()];
        Self(value)
    }
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
impl<I: Number> Number for NonNegativeInteger<I> {
    type Inner = I;
    /// Returns a new *non-negative* `Integer`.
    ///
    /// Panics if `value` < `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![value.is_positive()];
        Self(value)
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
impl<I: Number> Number for PositiveInteger<I> {
    type Inner = I;
    /// Returns a new *positive* `Integer`.
    ///
    /// Panics if `value` <= `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![value.is_positive() && !value.is_zero()];
        Self(value)
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
impl<I: Number> Number for NonPositiveInteger<I> {
    type Inner = I;
    /// Returns a new *non-positive* `Integer`.
    ///
    /// Panics if `value` > `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![!value.is_positive()];
        Self(value)
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
impl<I: Number> Number for NegativeInteger<I> {
    type Inner = I;
    /// Returns a new *negative* `Integer`.
    ///
    /// Panics if `value` >= `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![value.is_negative() && !value.is_zero()];
        Self(value)
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
