// numera::integer::impl_sign
//
//! implements the `Sign` trait for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{NumberAble, Sign};

#[rustfmt::skip]
impl<I: NumberAble> Sign for Integer<I> {
    fn can_negative() -> bool { true }
    fn can_positive() -> bool { true }
    fn is_negative(&self) -> bool { self.0.is_negative() }
    fn is_positive(&self) -> bool { self.0.is_positive() }
    fn inverse(&self) -> Option<Self> { self.0.inverse().map(|n| Self(n)) }
    fn abs(&self) -> Option<Self> { self.0.abs().map(|n| Self(n)) }
}
#[rustfmt::skip]
impl<I: NumberAble> Sign for NonZeroInteger<I> {
    fn can_negative() -> bool { true }
    fn can_positive() -> bool { true }
    fn is_negative(&self) -> bool { self.0.is_negative() }
    fn is_positive(&self) -> bool { self.0.is_positive() }
    fn inverse(&self) -> Option<Self> { self.0.inverse().map(|n| Self(n)) }
    fn abs(&self) -> Option<Self> { self.0.abs().map(|n| Self(n)) }
}

#[rustfmt::skip]
impl<I: NumberAble> Sign for NonNegativeInteger<I> {
    fn can_negative() -> bool { false }
    fn can_positive() -> bool { true }
    fn is_negative(&self) -> bool { false }
    fn is_positive(&self) -> bool { true }
    fn inverse(&self) -> Option<Self> { None }
    fn abs(&self) -> Option<Self> { Some(Self(self.0.clone())) }
    // fn abs(&self) -> Option<Self> { self.0.abs().map(|n|Self(n)) } // !unsigned?
}
#[rustfmt::skip]
impl<I: NumberAble> Sign for PositiveInteger<I> {
    fn can_negative() -> bool { false }
    fn can_positive() -> bool { true }
    fn is_negative(&self) -> bool { false }
    fn is_positive(&self) -> bool { true }
    fn inverse(&self) -> Option<Self> { None }
    fn abs(&self) -> Option<Self> { Some(Self(self.0.clone())) }
    // fn abs(&self) -> Option<Self> { self.0.abs().map(|n|Self(n)) }
}

#[rustfmt::skip]
impl<I: NumberAble> Sign for NonPositiveInteger<I> {
    fn can_negative() -> bool { true }
    fn can_positive() -> bool { false }
    fn is_negative(&self) -> bool { true }
    fn is_positive(&self) -> bool { false }
    fn inverse(&self) -> Option<Self> { None }
    fn abs(&self) -> Option<Self> { None }
}
#[rustfmt::skip]
impl<I: NumberAble> Sign for NegativeInteger<I> {
    fn can_negative() -> bool { true }
    fn can_positive() -> bool { false }
    fn is_negative(&self) -> bool { true }
    fn is_positive(&self) -> bool { false }
    fn inverse(&self) -> Option<Self> { None }
    fn abs(&self) -> Option<Self> { None }
}
