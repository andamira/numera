// numera::integer::impl_number
//
//! implements the `Number` trait for all integer types.
//

use super::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::{
    error::{Error, IntegerError, Result},
    traits::{Number, Signed},
};

// Z: Received values are always valid.
#[rustfmt::skip]
impl<N: Number + Signed> Number for Integer<N> {
    type Inner = N;
    /// Returns a new `Integer`.
    #[inline]
    fn new(value: Self::Inner) -> Self { Self(value) }
    /// Returns some new `Integer`.
    #[inline]
    fn new_checked(value: Self::Inner) -> Option<Self> { Some(Self(value)) }

    #[inline]
    fn get_inner(&self) -> Self::Inner { self.0.clone() }

    #[inline]
    fn set_inner(&mut self, value: Self::Inner) { self.0 = value; }
    #[inline]
    fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
        self.0 = value; Ok(())
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

// N0z: Received values are only valid iff `!= 0`.
#[rustfmt::skip]
impl<N: Number + Signed> Number for NonZeroInteger<N> {
    type Inner = N;
    /// Returns a new `NonZeroInteger`.
    ///
    /// # Panics
    /// Panics if `value == 0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![!value.is_zero()];
        Self(value)
    }
    /// Returns some new `NonZeroInteger` or `None` if `value == 0`.
    #[inline]
    fn new_checked(value: Self::Inner) -> Option<Self> {
        if value.is_zero() { None } else { Some(Self(value)) }
    }

    #[inline]
    fn get_inner(&self) -> Self::Inner { self.0.clone() }

    #[inline]
    fn set_inner(&mut self, value: Self::Inner) {
        assert![!value.is_zero()];
        self.0 = value;
    }
    #[inline]
    fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
        if value.is_zero() { Err(Error::Integer(IntegerError::Zero)) }
        else { self.0 = value; Ok(()) }
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

// Npi: Received values are valid iff `<= 0`.
#[rustfmt::skip]
impl<N: Number> Number for NonPositiveInteger<N> {
    type Inner = N;
    /// Returns a new *non-positive* `Integer`.
    ///
    /// Panics if `value` > `0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![!value.is_positive()];
        Self(value)
    }
    /// Returns some new `NonPositiveInteger` or `None` if `value > 0`.
    #[inline]
    fn new_checked(value: Self::Inner) -> Option<Self> {
        if value.is_positive() { None } else { Some(Self(value)) }
    }

    #[inline]
    fn get_inner(&self) -> Self::Inner { self.0.clone() }

    #[inline]
    fn set_inner(&mut self, value: Self::Inner) {
        assert![!value.is_positive()];
        self.0 = value;
    }
    #[inline]
    fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
        if value.is_positive() { Err(IntegerError::Zero.into()) }
        else { self.0 = value; Ok(()) }
    }

    #[inline]
    fn can_negative() -> bool { true }
    #[inline]
    fn can_positive() -> bool { false }
    #[inline]
    fn is_negative(&self) -> bool { self.0.is_negative() }
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

// Received values are valid iff `< 0`.
#[rustfmt::skip]
impl<N: Number> Number for NegativeInteger<N> {
    type Inner = N;
    /// Returns a new *negative* `Integer`.
    ///
    /// Panics if `value >= 0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![value.is_negative()];
        Self(value)
    }
    /// Returns some new `NegativeInteger` or `None` if `value >= 0`.
    #[inline]
    fn new_checked(value: Self::Inner) -> Option<Self> {
        if value.is_negative() { Some(Self(value)) } else { None}
    }

    #[inline]
    fn get_inner(&self) -> Self::Inner { self.0.clone() }

    #[inline]
    fn set_inner(&mut self, value: Self::Inner) {
        assert![value.is_negative()];
        self.0 = value;
    }
    #[inline]
    fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
        if value.is_negative() { self.0 = value; Ok(()) }
        else { Err(IntegerError::ZeroOrMore.into()) }
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

// Received values are valid iff `> 0`.
#[rustfmt::skip]
impl<N: Number> Number for NonNegativeInteger<N> {
    type Inner = N;
    /// Returns a new *non-negative* `Integer`.
    ///
    /// Panics if `value < 0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![!value.is_negative()];
        Self(value)
    }
    /// Returns some new `NonNegativeInteger` or `None` if `value < 0`.
    #[inline]
    fn new_checked(value: Self::Inner) -> Option<Self> {
        if value.is_negative() { None } else { Some(Self(value)) }
    }

    #[inline]
    fn get_inner(&self) -> Self::Inner { self.0.clone() }

    #[inline]
    fn set_inner(&mut self, value: Self::Inner) {
        assert![!value.is_negative()];
        self.0 = value;
    }
    #[inline]
    fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
        if value.is_negative() { Err(IntegerError::ZeroOrLess.into()) }
        else { self.0 = value; Ok(()) }
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

// Received values are valid iff `>= 0`.
#[rustfmt::skip]
impl<N: Number> Number for PositiveInteger<N> {
    type Inner = N;
    /// Returns a new *positive* `Integer`.
    ///
    /// Panics if `value <= 0`.
    #[inline]
    fn new(value: Self::Inner) -> Self {
        assert![value.is_positive()];
        Self(value)
    }
    /// Returns some new `PositiveInteger` or `None` if `value <= 0`.
    #[inline]
    fn new_checked(value: Self::Inner) -> Option<Self> {
        if value.is_positive() { Some(Self(value)) } else { None }
    }

    #[inline]
    fn get_inner(&self) -> Self::Inner { self.0.clone() }

    #[inline]
    fn set_inner(&mut self, value: Self::Inner) {
        assert![value.is_positive()];
        self.0 = value;
    }
    #[inline]
    fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
        if value.is_positive() { self.0 = value; Ok(()) }
        else { Err(IntegerError::LessThanZero.into()) }
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

/// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::integer::a::*;

    /// tests the checked constructor for all integers
    #[test]
    fn new_checked() {
        // Integer
        assert![Z::new_checked(-1).is_some()];
        assert![Z::new_checked(0).is_some()];
        assert![Z::new_checked(1).is_some()];
        // NonZeroInteger
        assert![N0z::new_checked(-1).is_some()];
        assert![N0z::new_checked(0).is_none()];
        assert![N0z::new_checked(1).is_some()];
        // NegativeInteger
        assert![Nz::new_checked(-1).is_some()];
        assert![Nz::new_checked(0).is_none()];
        assert![Nz::new_checked(1).is_none()];
        // NonPositiveInteger
        assert![Npz::new_checked(-1).is_some()];
        assert![Npz::new_checked(0).is_some()];
        assert![Npz::new_checked(1).is_none()];
        // PositiveInteger
        assert![Pz::new_checked(-1).is_none()];
        assert![Pz::new_checked(0).is_none()];
        assert![Pz::new_checked(1).is_some()];
        // NonNegativeInteger
        assert![Nnz::new_checked(-1).is_none()];
        assert![Nnz::new_checked(0).is_some()];
        assert![Nnz::new_checked(1).is_some()];
    }

    /// tests the panicking constructor for all integers.
    #[test]
    #[cfg(feature = "std")]
    fn new_panic() {
        use std::panic::catch_unwind;
        // Integer
        assert![catch_unwind(|| { Z::new(-1) }).is_ok()];
        assert![catch_unwind(|| { Z::new(0) }).is_ok()];
        assert![catch_unwind(|| { Z::new(1) }).is_ok()];
        // NonZeroInteger
        assert![catch_unwind(|| { N0z::new(-1) }).is_ok()];
        assert![catch_unwind(|| { N0z::new(0) }).is_err()];
        assert![catch_unwind(|| { N0z::new(1) }).is_ok()];
        // NegativeInteger
        assert![catch_unwind(|| { Nz::new(-1) }).is_ok()];
        assert![catch_unwind(|| { Nz::new(0) }).is_err()];
        assert![catch_unwind(|| { Nz::new(1) }).is_err()];
        // NonPositiveInteger
        assert![catch_unwind(|| { Npz::new(-1) }).is_ok()];
        assert![catch_unwind(|| { Npz::new(0) }).is_ok()];
        assert![catch_unwind(|| { Npz::new(1) }).is_err()];
        // PositiveInteger
        assert![catch_unwind(|| { Pz::new(-1) }).is_err()];
        assert![catch_unwind(|| { Pz::new(0) }).is_err()];
        assert![catch_unwind(|| { Pz::new(1) }).is_ok()];
        // NonNegativeInteger
        assert![catch_unwind(|| { Nnz::new(-1) }).is_err()];
        assert![catch_unwind(|| { Nnz::new(0) }).is_ok()];
        assert![catch_unwind(|| { Nnz::new(1) }).is_ok()];
    }
}
