// numera::integer::impl_number
//
//! implements the `Number` trait for all integer types.
//

use super::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{Number, Signed};

#[rustfmt::skip]
impl<N: Number + Signed> Number for Integer<N> {
    type Value = N;
    /// Returns a new `Integer`.
    #[inline]
    fn new(value: Self::Value) -> Self { Self(value) }

    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn can_positive() -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { Signed::is_negative(&self.0) }
    #[inline]
    fn is_positive(&self) -> bool { Signed::is_positive(&self.0) }

    #[inline]
    fn can_zero() -> bool { true }
    #[inline]
    fn can_one() -> bool { true }
    #[inline]
    fn can_neg_one() -> bool { true }
    #[inline]
    fn is_zero(&self) -> bool { self.0.is_zero() }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
    #[inline]
    fn is_neg_one(&self) -> bool { self.0.is_neg_one() }
}

#[rustfmt::skip]
impl<N: Number + Signed> Number for NonZeroInteger<N> {
    type Value = I;
    /// Returns a new `NonZeroInteger`.
    ///
    /// Panics if `value` == `0`.
    #[inline]
    fn new(value: Self::Value) -> Self {
        assert![!value.is_zero()];
        Self(value)
    }
    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn can_positive() -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { Signed::is_negative(&self.0) }
    #[inline]
    fn is_positive(&self) -> bool { Signed::is_positive(&self.0) }

    #[inline]
    fn can_zero() -> bool { false }
    #[inline]
    fn can_one() -> bool { true }
    #[inline]
    fn can_neg_one() -> bool { true }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
    #[inline]
    fn is_neg_one(&self) -> bool { self.0.is_neg_one() }
}

#[rustfmt::skip]
impl<N: Number + Signed> Number for NonPositiveInteger<N> {
    type Value = I;
    /// Returns a new *non-positive* `Integer`.
    ///
    /// Panics if `value` > `0`.
    #[inline]
    fn new(value: Self::Value) -> Self {
        assert![!Signed::is_positive(&value)];
        Self(value)
    }
    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn can_positive() -> bool { false }
    #[inline]
    fn is_negative(&self) -> bool { Signed::is_negative(&self.0) }
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
impl<N: Number + Signed> Number for NegativeInteger<N> {
    type Value = I;
    /// Returns a new *negative* `Integer`.
    ///
    /// Panics if `value` >= `0`.
    #[inline]
    fn new(value: Self::Value) -> Self {
        assert![Signed::is_negative(&value)];
        assert![!value.is_zero()];
        Self(value)
    }
    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn can_positive() -> bool { false }
    #[inline]
    fn is_negative(&self) -> bool { true }
    #[inline]
    fn is_positive(&self) -> bool { false }

    #[inline]
    fn can_zero() -> bool { false }
    #[inline]
    fn can_one() -> bool { false }
    #[inline]
    fn can_neg_one() -> bool { true }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { false }
    #[inline]
    fn is_neg_one(&self) -> bool {self.0.is_neg_one() }
}

#[rustfmt::skip]
impl<N: Number> Number for NonNegativeInteger<N> {
    type Value = I;
    /// Returns a new *non-negative* `Integer`.
    ///
    /// Panics if `value` < `0`.
    #[inline]
    fn new(value: Self::Value) -> Self {
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
    fn can_one() -> bool { true }
    #[inline]
    fn can_neg_one() -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { self.0.is_zero() }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}

#[rustfmt::skip]
impl<N: Number> Number for PositiveInteger<N> {
    type Value = I;
    /// Returns a new *positive* `Integer`.
    ///
    /// Panics if `value` <= `0`.
    #[inline]
    fn new(value: Self::Value) -> Self {
        assert![value.is_positive()];
        assert![!value.is_zero()];
        Self(value)
    }
    #[inline]
    fn can_negative() -> bool { false }
    #[inline]
    fn can_positive() -> bool { true }
    #[inline]
    fn is_negative(&self) -> bool { false }
    #[inline]
    fn is_positive(&self) -> bool { self.0.is_positive() }

    #[inline]
    fn can_zero() -> bool { false }
    #[inline]
    fn can_one() -> bool { true }
    #[inline]
    fn can_neg_one() -> bool { false }
    #[inline]
    fn is_zero(&self) -> bool { false }
    #[inline]
    fn is_one(&self) -> bool { self.0.is_one() }
    #[inline]
    fn is_neg_one(&self) -> bool { false }
}
