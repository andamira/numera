// numera::integer::impl_sign
//
//! implements the `Signed` trait for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{NegSigned, Number, Signed, Unsigned};

// can hold both signs

#[rustfmt::skip]
impl<N: Number + Signed> Signed for Integer<N> {
    fn is_negative(&self) -> bool { Signed::is_negative(&self.0) }
    fn is_positive(&self) -> bool { Signed::is_positive(&self.0) }
}

#[rustfmt::skip]
impl<N: Number + Signed> Signed for NonZeroInteger<N> {
    fn is_negative(&self) -> bool { Signed::is_negative(&self.0) }
    fn is_positive(&self) -> bool { Signed::is_positive(&self.0) }
}

// can't hold negative values

impl<N: Number> Unsigned for NonNegativeInteger<N> {}
impl<N: Number> Unsigned for PositiveInteger<N> {}

// can't hold positive values

impl<N: Number> NegSigned for NonPositiveInteger<N> {
    type Inner = N;
    fn new_neg(value: Self::Inner) -> Self {
        Self(value)
    }
}
impl<N: Number + Unsigned> NegSigned for NegativeInteger<N> {
    type Inner = N;
    fn new_neg(value: Self::Inner) -> Self {
        Self(value)
    }
}
